/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: startup                                                         ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Here is the main function called first from the boot code as    ║
   ║         well as the panic handler. All features are set and all modules ║
   ║         are imported.                                                   ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoettner, Univ. Duesseldorf, 5.2.2024                 ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![no_std]
#![allow(dead_code)] // avoid warnings
#![allow(unused_variables)] // avoid warnings
#![allow(unused_imports)]
#![allow(unused_macros)]
#![feature(allocator_api)]
// Iso-Neu
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
// Nach Rust Update
#![allow(static_mut_refs)]

use alloc::{boxed::Box, string::ToString, vec, vec::Vec};
use core::{panic::PanicInfo, ptr};

use boot::{appregion, multiboot};
use consts::{KERNEL_HEAP_SIZE, PAGE_FRAME_SIZE, TEMP_HEAP_SIZE};
use devices::{
    cga,
    graphical::{fonts::font_8x8, graphic_console_printer, vga},
    keyboard::Keyboard,
    pit,
};
use kernel::{
    cpu, interrupts,
    paging::{
        frames::{self, dump_kernal_frames, dump_user_frames, pf_alloc, pf_free},
        pages,
    },
    syscall,
    systemallocator::allocator,
    threads::{self, idle_thread, scheduler::Scheduler, thread::Thread},
};
use log::info;

use crate::{
    boot::{appregion::AppRegion, multiboot::PhysRegion},
    kernel::{logger, paging::physical_addres::PhysAddr, shell::shell_process, threads::scheduler},
};

extern crate alloc;

// insert other modules
#[macro_use] // import macros, too
mod devices;

mod consts;
mod kernel;

mod boot;
mod utility;

// Konstanten im Linker-Skript
extern "C" {
    static ___KERNEL_DATA_START__: u64;
    static ___KERNEL_DATA_END__: u64;
}

fn init_all(mbi: u64) {
    kprintln!("OS initializing...");

    let kernel_region = get_kernel_image_region();

    // Verfuegbaren physikalischen Speicher ermitteln (exklusive Kernel-Image und Heap)
    let heap_region = create_temp_heap(kernel_region.end as usize);

    // Multiboot-Infos für Grafik auslesen, falls vorhanden
    check_graphics_mode(mbi);

    // Verfuegbaren physikalischen Speicher ermitteln (exklusive Kernel-Image und Heap)
    let phys_mem = multiboot::get_free_memory(mbi, kernel_region, heap_region);

    // Multiboot-Infos ausgeben
    multiboot::dump(mbi);

    // initialize the logger
    logger::init();
    info!("Logger active");

    // Page-Frame-Management einrichten
    frames::pf_init(phys_mem);

    // Paging fuer den Kernel aktivieren
    let pml4_addr = pages::pg_init_kernel_tables(mbi);
    pages::pg_set_cr3(pml4_addr);

    // Nochmal richtig Kernal-Heap initialisieren
    ini_kernel_heap();

    // Interrupt-Strukturen initialisieren
    interrupts::init();

    // Trapgate initialisieren
    syscall::syscall_dispatcher::init();

    // Tastatur-Unterbrechungsroutine 'einstoepseln'
    Keyboard::plugin();

    // Zeitgeber-Unterbrechungsroutine 'einstoepseln'
    pit::plugin();

    // Bildschirm frei machen
    graphic_console_printer::clear_screen();

    kprintln!("Initializing finished!");
}

// Pruefen, ob wir in einem Grafikmodus sind
// Falls ja setzen der Infos in VGA
fn check_graphics_mode(mbi: u64) -> bool {
    unsafe {
        let ptr = mbi;

        let flags = *(mbi as *mut u32);

        // 12 Bit in Flags zeigt an, ob Framebuffer-Infos vorhanden sind
        if flags & 0x1000 == 0 {
            return false;
        }

        let addr = *((mbi + 88) as *mut u64);
        let pitch = *((mbi + 96) as *mut u32);
        let width = *((mbi + 100) as *mut u32);
        let height = *((mbi + 104) as *mut u32);
        let bpp = *((mbi + 108) as *mut u8);
        vga::VGA::init(addr, pitch, width, height, bpp);
    }
    true
}

// Start- und Endadresse des Kernel-Images ermitteln,
// aufrunden auf das naechste volle MB und zurueckgeben
fn get_kernel_image_region() -> multiboot::PhysRegion {
    let kernel_start: usize;
    let kernel_end: usize;

    unsafe {
        kernel_start = &___KERNEL_DATA_START__ as *const u64 as usize;
        kernel_end = &___KERNEL_DATA_END__ as *const u64 as usize;
    }

    // Kernel-Image auf das naechste MB aufrunden
    let mut kernel_rounded_end = kernel_end & 0xFFFFFFFFFFF00000;
    kernel_rounded_end += 0x100000 - 1; // 1 MB aufaddieren

    PhysRegion {
        start: kernel_start as u64,
        end: kernel_rounded_end as u64,
    }
}

// Einen temperoraeren Heap anlegen, nach dem Ende des Kernel-Images
fn create_temp_heap(kernel_end: usize) -> multiboot::PhysRegion {
    let heap_start = kernel_end + 1;

    // Temporaeren Heap einrichten, nach dem Kernel-Image
    allocator::init(heap_start, TEMP_HEAP_SIZE);

    PhysRegion {
        start: heap_start as u64,
        end: (heap_start + TEMP_HEAP_SIZE - 1) as u64,
    }
}

fn ini_kernel_heap() {
    // Wie viele Kachel brauchen wir
    let heap_frames_count: usize = KERNEL_HEAP_SIZE / PAGE_FRAME_SIZE;

    // Genug Speicher Anfordern
    let kernal_heap_adress = pf_alloc(heap_frames_count, true);

    // Allokator neu intitialisieren
    allocator::init(kernal_heap_adress.raw() as usize, KERNEL_HEAP_SIZE);
}

fn ph_allocator_testing() {
    // Erstmal in den Speicher gucken
    vprintln!("= = = Kernal Frames = = =");
    dump_kernal_frames();

    vprintln!("\n= = = Vordere ein Wenig speicher an = = =");

    let pf_alloc1 = pf_alloc(5, true);
    let pf_alloc2 = pf_alloc(30, true);
    let pf_alloc3 = pf_alloc(17, true);

    // Erstmal in den Speicher gucken
    vprintln!("\n\n= = = Kernal Frames = = =");
    dump_kernal_frames();
    vprintln!("\n= = = gebe Teil davon frei = = =");
    pf_free(pf_alloc1, 5);

    vprintln!("\n\n= = = Kernal Frames = = =");
    dump_kernal_frames();

    vprintln!("\n= = = noch mehr Freigeben = = =");
    pf_free(pf_alloc2, 30);

    vprintln!("\n\n= = = Kernal Frames = = =");
    dump_kernal_frames();

    vprintln!("\n-------------------------------------------------------\n= = = Userframes = = =");
    dump_user_frames();

    vprintln!("\n= = = Vordere ein Wenig speicher an = = =");
    let useralloc1 = pf_alloc(10, false);
    let useralloc2 = pf_alloc(20, false);
    let useralloc3 = pf_alloc(30, false);
    let useralloc4 = pf_alloc(40, false);
    let useralloc5 = pf_alloc(50, false);
    let useralloc6 = pf_alloc(60, false);

    vprintln!("\n= = = Userframes nach dem Anfordern = = =");
    dump_user_frames();

    vprintln!("\n= = = Gebe durcheinander frei = = =");
    pf_free(useralloc5, 50);
    pf_free(useralloc1, 10);
    pf_free(useralloc3, 30);

    vprintln!("\n= = = Userframes nach paar freigeben = = =");
    dump_user_frames();

    pf_free(useralloc4, 40);
    pf_free(useralloc6, 60);
    pf_free(useralloc2, 20);

    vprintln!("\n= = = Jetzt sollte wieder der User-Space sein wie vorher = = =");
    dump_user_frames();
}

fn print_frames_with_headline(headline: &str) {
    vprintln!(
        "\n-------------------------------------------------------\n{}",
        headline
    );
    vprintln!("\n\n= = = Kernal Frames = = =");
    dump_kernal_frames();
    vprintln!("\n\n= = = User Frames = = =");
    dump_user_frames();

    vprintln!("\n-------------------------------------------------------\n");
}

#[no_mangle]
pub extern "C" fn kmain(mbi: u64) {
    kprintln!("kmain");

    kprintln!("-- Noch vor dem Init. MBI-Pointer: {:#x}\n", mbi);

    // Alles Wichtige Initialisieren
    init_all(mbi);

    // Kernel-Prozess mit Idle-Thread erzeugen und im Scheduler registrieren
    scheduler::spawn_kernel();

    // TODO: Wenn auskommentiert sehr wilde ausgabe
    kprintln!("-- Nach Spawn kernel. MBI-Pointer: {:#x}\n", mbi);

    // Apps aus initrd.tar extrahieren
    let opt_apps: Option<Vec<AppRegion>> = appregion::get_apps_from_tar(mbi);

    // Wurde was geladen?
    if opt_apps.is_none() {
        kprintln!("!=!=!=!=!=!=!=!=!=!=!=!=!=!=! No apps found !=!=!=!=!=!=!=!=!=!=!=!=!=!=!");
        // Dauerloop
        loop {}
    }

    // Environment Test
    usrlib::kernel::runtime::env_variables::env_insert("Hello", "Super Toller Inhalt");
    usrlib::kernel::runtime::env_variables::env_insert("python", "/usr/local/bin/python");

    // Schellprogramm starten
    shell_process::spawn_shell_process(opt_apps.clone().unwrap());

    // Scheduler starten & Interrupts erlauben
    Scheduler::schedule();
}

/*
* Panic Handler
*/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    loop {}
}

// Dummy-Main Methode wegen der Dependency von Usrlib
#[no_mangle]
pub fn main() {
    kprintln!("Should never be reached");
}
