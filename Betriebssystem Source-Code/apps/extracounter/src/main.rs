#![no_std]
#![allow(unused_variables)] // avoid warnings

use core::panic::PanicInfo;

#[macro_use]
use usrlib;
// Man muss beide Imporieren, da sie sonst nicht richtig aufklappen
use usrlib::{gprint, print_setpos};

use usrlib::utility::delay::delay;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    let mut i: u64 = 0;

    loop {

        print_setpos!(20, 40, "Ich bin der Extra Prozess: {}", i);
 
        i = i + 1; 

        /*
        for i in 0..999999 {  
            let temp = 5;
            let value = 7;
            let conclusion = temp + value + i;
        }*/
        delay(10);

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
