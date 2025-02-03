#![no_std]
#![allow(unused_variables)] // avoid warnings

use core::panic::PanicInfo;
use core::str::from_utf8_unchecked;
#[macro_use]
use usrlib;
use usrlib::print_setpos;
use usrlib::kernel::syscall::user_api::{usr_get_pid, usr_read_process_name};
use usrlib::utility::delay::delay;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    
    let mut i: u64 = 0;
    loop {

        const BUFFERLENGH: usize = 255;

        // Daten holen
        let pid = usr_get_pid();
        let mut namebuffer: [u8; BUFFERLENGH] = [0; BUFFERLENGH];
        usr_read_process_name(namebuffer.as_mut_ptr(), BUFFERLENGH as u64) as usize;
        let actual_name: &str = unsafe {
            from_utf8_unchecked(namebuffer
                .as_slice()
                .split(|&b| b == 0)
                .next()
                .unwrap_or(&[]))
        };


        // Ausgabe
        print_setpos!(90, 30, "Name: {}; pid: {}", actual_name, pid);
        print_setpos!(90, 31, "Extra Counter {}", i);


        // Counter verschieben
        i = i + 1;

        // kurz warten
        delay(7);

    }
}

/*
* Panic Handler
*/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
