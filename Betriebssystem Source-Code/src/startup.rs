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

extern crate alloc;
extern crate spin; // we need a mutex in devices::cga_print

// insert other modules
#[macro_use] // import macros, too
mod devices;
mod consts;
mod kernel;
mod user;

use core::panic::PanicInfo;

use devices::cga; // shortcut for cga
use devices::cga_print; // used to import code needed by println!
use devices::keyboard; // shortcut for keyboard

use kernel::cpu;

use user::applications::keyboard_handler;
use user::aufgabe1::keyboard_demo;
use user::aufgabe1::text_demo;
use user::applications; // Eigene geschriebene Anwendunden

use crate::devices::cga::attribute;
use crate::devices::cga::get_bytes;

fn own_tests() {
    keyboard_handler::run();
}

fn aufgabe1() {
    cga::clear();
    text_demo::run();
    kprintln!("Textdemo run");
    //keyboard_demo::run();
}

#[no_mangle]
pub extern "C" fn startup() {
    kprintln!("OS is running ...");

    cga::clear();

    kprintln!("Screen Cleared ...");

    aufgabe1();

    own_tests();

    kprintln!(" = = Closing OS = =");

    //loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //	kprintln!("{:?}", Backtrace::new());
    loop {}
}
