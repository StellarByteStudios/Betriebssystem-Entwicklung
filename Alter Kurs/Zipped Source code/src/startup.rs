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
#![feature(const_mut_refs)]
#![allow(dead_code)] // avoid warnings
#![allow(unused_variables)] // avoid warnings
#![allow(unused_imports)]
#![allow(unused_macros)]
#![feature(allocator_api)]

use core::panic::PanicInfo;

use alloc::{string::ToString, vec};
use devices::{cga, fonts::font_8x8, keyboard::Keyboard, pit, vga};
use kernel::{
    allocator, cpu, interrupts,
    threads::{self, scheduler::Scheduler},
};
use mylib::input;
use user::{
    applications::{self, keyboard_handler},
    aufgabe1::text_demo,
    aufgabe2::heap_demo,
    aufgabe3::keyboard_irq_demo,
    aufgabe4, aufgabe5, aufgabe6, aufgabe7,
};

extern crate alloc;
extern crate spin; // we need a mutex in devices::cga_print

// insert other modules
#[macro_use] // import macros, too
mod devices;
#[macro_use] // import macros, too
mod mylib;

mod consts;
mod kernel;

mod user;

fn own_tests() {
    keyboard_handler::run();
}

fn init_all(mbi: u64) {
    kprintln!("OS initializing...");

    // init allocator
    allocator::init();

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

#[no_mangle]
pub extern "C" fn startup(mbi: u64) {
    kprintln!("OS startup...");

    init_all(mbi);

    //print_main_screen();
    print_main_graphic();
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
    aufgabe7();

    own_tests();

    kprintln!(" = = Closing OS = =");

    loop {
        //let mut code = key_hit();

        //keyboard_handler::handle_keystroke(code.get_ascii());
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //kprintln!("{:?}", Backtrace::new());
    loop {}
}
