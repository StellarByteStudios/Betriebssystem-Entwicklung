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

use core::panic::PanicInfo;

use alloc::{string::ToString, vec};
use consts::{KERNEL_HEAP_SIZE, PAGE_FRAME_SIZE, TEMP_HEAP_SIZE};
use devices::{cga, fonts::font_8x8, keyboard::Keyboard, pit, vga};
use kernel::{
    allocator::{self},
    cpu, interrupts,
    paging::{
        frames::{self, dump_kernal_frames, dump_user_frames, pf_alloc, pf_free},
        pages,
    },
    syscall,
    threads::{self, scheduler::Scheduler, sec_idle_thread},
};
use mylib::input;
// Funktioniert nicht mehr wegen neuer Threads
use user::{
    applications::{self, graphic_console::graphic_console_printer, keyboard_handler},
    syscall_testthreads::{
        get_last_key_thread, get_thread_id, hello_world_thread, write_in_buffer_thead,
    }, //aufgabe1::text_demo,
       //aufgabe2::heap_demo,
       //aufgabe3::keyboard_irq_demo,
       //aufgabe4, aufgabe5, aufgabe6, aufgabe7,
};

extern crate alloc;
extern crate spin; // we need a mutex in devices::cga_print

// insert other modules
#[macro_use] // import macros, too
mod devices;
#[macro_use] // import macros, too
mod mylib;

#[macro_use]
extern crate bitflags;

mod consts;
mod kernel;

mod boot;
mod user;

fn own_tests() {
    keyboard_handler::run();
}

fn init_all(mbi: u64) {
    kprintln!("OS initializing...");

    // init allocator
    allocator::init(allocator::HEAP_START, consts::HEAP_SIZE); // Konstruktor geändert

    // Multiboot-Infos für Grafik auslesen, falls vorhanden
    check_graphics_mode(mbi);

    // init interrupts
    interrupts::init();

    // register keyboard ISR
    Keyboard::plugin();

    // Timer Interupt registrieren
    pit::plugin();

    // CPU enable ints
    cpu::enable_int();

    // Clear Screen
    cga::clear();

    kprintln!("Initializing finished!");
}

/*
fn aufgabe1() {
    //cga::clear();
    text_demo::run();
    kprintln!("Textdemo run");
    //keyboard_demo::run();
}

fn aufgabe2() {
    heap_demo::run();
    //cga::clear();
    //sound_demo::run();
}

fn aufgabe3() {
    cga::clear();

    /*
    pic::forbid(IRQ_KEYBOARD);
    pic::forbid(IRQ_TIMER);

    kprintln!("Beide Interrupts sind jetzt deaktiviert");
    kprintln!("Status Keyboard {}", pic::status(IRQ_KEYBOARD));
    kprintln!("Status Timer {}", pic::status(IRQ_TIMER));

    pic::allow(IRQ_KEYBOARD);
    pic::allow(IRQ_TIMER);

    kprintln!("Beide Interrupts sind jetzt wieder aktiviert");
    kprintln!("Status Keyboard {}", pic::status(IRQ_KEYBOARD));
    kprintln!("Status Timer {}", pic::status(IRQ_TIMER));
     */

    // Cursor muss in Keyboard::KeyboardISR::trigger festgesetzt werden!!!
    keyboard_irq_demo::run();
}

fn aufgabe4() {
    cga::clear();

    // Lied abspielen
    //pcspk::alle_meine_entchen();
    //pcspk::starwars_imperial();
    //pcspk::super_mario();
    //pcspk::doom();
    //pcspk::tetris();

    // Threads Initialisieren
    //init_all_threads();

    // Scheduler aufsetzen
    //Scheduler::schedule();

    //aufgabe4::corouts_demo::run();
    //aufgabe4::queue_tests::run();
}

fn aufgabe5() {
    cga::clear();

    // Threads Initialisieren
    init_all_threads_preempt();

    // Scheduler aufsetzen
    Scheduler::schedule();
}

fn aufgabe6() {
    cga::clear();

    // Threads Initialisieren
    init_all_threads_sync();

    // Scheduler aufsetzen
    Scheduler::schedule();
}

fn aufgabe7() {
    threads::idle_thread::init();
    /*
    applications::graphic_console::gc_programms::clock::init(vec![
        "clock".to_string(),
        "start".to_string(),
    ]); */
    aufgabe7::test_console::init();

    // Scheduler aufsetzen
    Scheduler::schedule();
}

fn init_all_threads() {
    threads::idle_thread::init();
    aufgabe4::hello_world_thread::init();
    //applications::music_thread::init();
    //aufgabe4::coop_thread_loop::init();
    //aufgabe4::coop_thread_demo::init();
}

fn init_all_threads_preempt() {
    threads::idle_thread::init();
    aufgabe4::hello_world_thread::init();
    aufgabe5::thread_demo::init();
    //applications::music_thread::init();
}

fn init_all_threads_sync() {
    threads::idle_thread::init();
    aufgabe6::semaphore_launch_thread::init();
    //applications::music_thread::init();
}

fn print_main_screen() {
    cga::clear();
    println!("Byte OS: 1.0");
    println!("------------------------------------\n");
    println!("Aktuelle Funktionalitaeten:");
    print!("    Bildschirmausgabe ");
    cga::set_attribute(cga::Color::Blue, cga::Color::Yellow, true);
    println!("(auch bunt)");
    cga::set_default_attribute();
    println!("    Heapverwaltung (mit Freispeicherliste)");
    println!("    Interrupts");
    println!("    Tastatureingabe (Ueber Interrupts)");
    println!("    Koroutinen (Kooperativ - verkettet)");
    println!("    Queue (Für die Threads)");
    println!("    Scheduler (Kooperativ)");
    println!("    Threads (Kooperativ)");
    println!("    Musik");
}

fn print_main_graphic() {
    let text_h = font_8x8::CHAR_HEIGHT;

    vga::draw_string(0, 0, vga::rgb_24(0, 255, 0), "Byte OS: 1.0");
    vga::draw_string(
        0,
        text_h,
        vga::rgb_24(0, 255, 0),
        "------------------------------------\n",
    );
    vga::draw_string(
        0,
        text_h * 2,
        vga::rgb_24(0, 255, 0),
        "Aktuelle Funktionalitaeten:",
    );
    vga::draw_string(
        0,
        text_h * 3,
        vga::rgb_24(0, 255, 0),
        "    - Bildschirmausgabe ",
    );
    vga::draw_string(
        0,
        text_h * 4,
        vga::rgb_24(0, 255, 0),
        "    - Heapverwaltung (mit Freispeicherliste)",
    );
    vga::draw_string(0, text_h * 5, vga::rgb_24(0, 255, 0), "    - Interrupts");
    vga::draw_string(
        0,
        text_h * 6,
        vga::rgb_24(0, 255, 0),
        "    - Tastatureingabe (Ueber Interrupts)",
    );

    vga::draw_string(
        0,
        text_h * 7,
        vga::rgb_24(0, 255, 0),
        "    - Koroutinen (Kooperativ - verkettet)",
    );
    vga::draw_string(
        0,
        text_h * 8,
        vga::rgb_24(0, 255, 0),
        "    - Queue (Für die Threads)",
    );
    vga::draw_string(
        0,
        text_h * 9,
        vga::rgb_24(0, 255, 0),
        "    - Scheduler (Kooperativ)",
    );
    vga::draw_string(0, text_h * 10, vga::rgb_24(0, 255, 0), "    - Musik");
    vga::draw_string(
        0,
        text_h * 11,
        vga::rgb_24(0, 255, 0),
        "    - Shellfunktionalität",
    );
    vga::draw_string(
        0,
        text_h * 12,
        vga::rgb_24(34, 80, 200),
        "        * Ein und ausgabe von Text",
    );
    vga::draw_string(
        0,
        text_h * 13,
        vga::rgb_24(34, 80, 200),
        "        * Auswahl von Musik",
    );
    vga::draw_string(
        0,
        text_h * 14,
        vga::rgb_24(34, 80, 200),
        "        * Fraktalberechnung",
    );
    vga::draw_string(
        0,
        text_h * 15,
        vga::rgb_24(34, 80, 200),
        "        * und vieles mehr ...",
    )
}
 */
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

/* Ursprüngliche Funktion, welche beim Startup lief */
#[no_mangle]
pub extern "C" fn startup(mbi: u64) {
    kprintln!("OS startup...");

    init_all(mbi);

    //print_main_screen();
    //print_main_graphic();
    //kprintln!("Die Aktuelle Zeit: {}", rtc::get_time());
    //draw_newton();

    //pcspk::intro();

    input::wait_for_return();

    cga::clear();

    //aufgabe1();
    //aufgabe2();
    //aufgabe3();
    //aufgabe4();
    //aufgabe5();
    //aufgabe6();
    //aufgabe7();

    own_tests();

    kprintln!(" = = Closing OS = =");

    loop {
        //let mut code = key_hit();

        //keyboard_handler::handle_keystroke(code.get_ascii());
    }
}

/* Neuer Startupblock
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*/

// Konstanten im Linker-Skript
extern "C" {
    static ___KERNEL_DATA_START__: u64;
    static ___KERNEL_DATA_END__: u64;
}

use crate::boot::multiboot::PhysRegion;
use boot::multiboot;
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

#[no_mangle]
pub extern "C" fn kmain(mbi: u64) {
    kprintln!("kmain");

    let kernel_region = get_kernel_image_region();
    kprintln!("   kernel_region: {:?}", kernel_region);

    // Verfuegbaren physikalischen Speicher ermitteln (exklusive Kernel-Image und Heap)
    let heap_region = create_temp_heap(kernel_region.end as usize);
    kprintln!("kmain, heap: {:?}", heap_region);

    // Multiboot-Infos für Grafik auslesen, falls vorhanden
    check_graphics_mode(mbi);

    // Verfuegbaren physikalischen Speicher ermitteln (exklusive Kernel-Image und Heap)
    let phys_mem = multiboot::get_free_memory(mbi, kernel_region, heap_region);
    kprintln!("kmain, free physical memory: {:?}", phys_mem);

    // Multiboot-Infos ausgeben
    multiboot::dump(mbi);

    // Page-Frame-Management einrichten
    frames::pf_init(phys_mem);

    // Paging fuer den Kernel aktivieren
    let pml4_addr = pages::pg_init_kernel_tables();
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

    // Erstmal in den Speicher gucken
    vprintln!("= = = Kernal Frames = = =");
    dump_kernal_frames();
    //vprintln!("\n= = = User Frames = = =");
    //kernel::paging::frames::dump_user_frames();

    vprintln!("\n= = = Vordere ein Wenig speicher an = = =");

    let pf_alloc1 = pf_alloc(5, true);
    let pf_alloc2 = pf_alloc(30, true);
    let pf_alloc3 = pf_alloc(17, true);

    //kernel::paging::frames::pf_alloc(1, false);
    //kernel::paging::frames::pf_alloc(10, false);

    // Erstmal in den Speicher gucken
    vprintln!("\n\n= = = Kernal Frames = = =");
    dump_kernal_frames();
    //vprintln!("\n= = = User Frames = = =");
    //kernel::paging::frames::dump_user_frames();

    vprintln!("\n= = = gebe Teil davon frei = = =");
    pf_free(pf_alloc1, 5);

    vprintln!("\n\n= = = Kernal Frames = = =");
    dump_kernal_frames();

    vprintln!("\n= = = noch mehr Freigeben = = =");
    pf_free(pf_alloc2, 30);

    vprintln!("\n\n= = = Kernal Frames = = =");
    dump_kernal_frames();
    //vprintln!("\n= = = User Frames = = =");
    //kernel::paging::frames::dump_user_frames();

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

    // Idle-Thread eintragen
    /*let idle_thread = Thread::new(
        scheduler::next_thread_id(),
        sec_idle_thread::idle_thread_entry,
        true,
    );

    scheduler::Scheduler::ready(idle_thread);*/
    sec_idle_thread::init();

    // HelloWorld-Thread eintragen
    //hello_world_thread::init();

    // Andere Threads testen
    //get_last_key_thread::init();
    //get_thread_id::init();
    //write_in_buffer_thead::init();

    // Scheduler starten & Interrupts erlauben
    Scheduler::schedule();
}

/*
* Panic Handler
*/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //kprintln!("{:?}", Backtrace::new());
    loop {}
}
