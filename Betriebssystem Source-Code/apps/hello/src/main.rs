#![no_std]
#![allow(unused_variables)] // avoid warnings

use core::panic::PanicInfo;

// Sobald usrlib importiert wird "error: no global memory 
// allocator found but one is required; link to std or add `#[global_allocator]` 
// to a static item that implements the GlobalAlloc trait"
#[macro_use]
use usrlib;
// Man muss beide Imporieren, da sie sonst nicht richtig aufklappen
use usrlib::gprintln;
use usrlib::gprint;

use usrlib::kernel::syscall::user_api;
use usrlib::utility::delay::delay;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    //let temp: u64 = 0xc0ffee;
    let temp: u64 = 0x6F6C6C6168;


    loop {
        user_api::usr_hello_world_print(133713371337);
        
        gprintln!("Hello, world!"); 

        /*
        for i in 0..999999 {
            let temp = 5;
            let value = 7;
            let conclusion = temp + value + i;
        }*/
        delay(100);

    }
}

/*
* Panic Handler
*/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //kprintln!("Panic: {}", info);
    //kprintln!("{:?}", Backtrace::new());
    loop {}
}
