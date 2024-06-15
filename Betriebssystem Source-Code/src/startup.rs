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

extern crate alloc;
extern crate spin; // we need a mutex in devices::cga_print

// insert other modules
#[macro_use] // import macros, too
mod devices;
mod consts;
mod kernel;
mod mylib;
mod user;

use core::panic::PanicInfo;
use core::ptr::null;
use core::ptr::null_mut;

use devices::cga; // shortcut for cga
use devices::cga_print; // used to import code needed by println!
use devices::keyboard; // shortcut for keyboard
                       //use devices::

use devices::pit;
use kernel::allocator;
use kernel::cpu;

use kernel::interrupts;
use kernel::interrupts::pic;
use kernel::interrupts::pic::IRQ_KEYBOARD;
use kernel::interrupts::pic::IRQ_TIMER;
use kernel::threads;
use kernel::threads::scheduler::Scheduler;
use kernel::threads::scheduler::SCHEDULER;
use kernel::threads::thread;
use kernel::threads::thread::Thread;
use mylib::input;
use user::applications; // Eigene geschriebene Anwendunden
use user::applications::keyboard_handler;
use user::aufgabe1::keyboard_demo;
use user::aufgabe1::text_demo;
use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;
use user::aufgabe3;
use user::aufgabe3::keyboard_irq_demo;
use user::aufgabe4;

use crate::devices::cga::attribute;
use crate::devices::cga::get_bytes;
use crate::devices::cga::set_attribute;
use crate::devices::keyboard::key_hit;
use crate::devices::keyboard::Keyboard;
use crate::kernel::interrupts::intdispatcher;
use crate::user::aufgabe2;
use crate::devices::pcspk;

fn own_tests() {
    keyboard_handler::run();
}

fn init_all() {
    kprintln!("OS initializing...");

    // init allocator
    allocator::init();

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
    pcspk::starwars_imperial();
    //pcspk::super_mario();
    //pcspk::doom();
    //pcspk::tetris();

    // Threads Initialisieren
    init_all_threads();
    
    // Scheduler aufsetzen
    Scheduler::schedule();

    //aufgabe4::corouts_demo::run();
    //aufgabe4::queue_tests::run();
}


fn init_all_threads(){
    threads::idle_thread::init();
    aufgabe4::hello_world_thread::init();
    //aufgabe4::coop_thread_loop::init();
    aufgabe4::coop_thread_demo::init();
}

fn print_main_screen() {
    cga::clear();
    println!("Byte OS: 0.4");
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
}

#[no_mangle]
pub extern "C" fn startup() {
    kprintln!("OS startup...");

    init_all();

    print_main_screen();

    input::wait_for_return();

    cga::clear();

    //aufgabe1();
    //aufgabe2();
    //aufgabe3();
    aufgabe4();

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
