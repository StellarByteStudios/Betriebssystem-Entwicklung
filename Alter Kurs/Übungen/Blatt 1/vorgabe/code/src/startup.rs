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
#[macro_use]   // import macros, too
mod devices;
mod kernel;
mod user;
mod consts;

use core::panic::PanicInfo;

use devices::cga;         // shortcut for cga
use devices::cga_print;   // used to import code needed by println! 
use devices::keyboard;    // shortcut for keyboard

use kernel::cpu;

use user::aufgabe1::text_demo;
use user::aufgabe1::keyboard_demo;


fn aufgabe1() {
   cga::clear();
   text_demo::run();
   keyboard_demo::run();
}


#[no_mangle]
pub extern fn startup() {
	 

    kprintln!("OS is running ...");

	//cga::clear();
	


    aufgabe1();
    
    loop{}
}




#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //	kprintln!("{:?}", Backtrace::new());
    loop {}
}

