#![no_std]
#![allow(unused_variables)] // avoid warnings

use core::mem;
use core::panic::PanicInfo;
use core::str::{from_utf8, from_utf8_unchecked};
// Sobald usrlib importiert wird "error: no global memory
// allocator found but one is required; link to std or add `#[global_allocator]`
// to a static item that implements the GlobalAlloc trait"
use usrlib;
// Man muss beide Imporieren, da sie sonst nicht richtig aufklappen
use usrlib::{gprint, gprintln, print_setpos};
use usrlib::kernel::syscall::user_api::{usr_dump_active_vmas, usr_get_pid, usr_read_process_name};
use usrlib::utility::delay::delay;


#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    
    // VMAs Ausgeben
    usr_dump_active_vmas();

    // Counter starten
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
        print_setpos!(10, 30, "Name: {}; pid: {}", actual_name, pid);  
        //print_setpos!(10, 30, " pid: {}", pid);
        print_setpos!(10, 31, "Counter {}", i);
        
 
        // Counter verschieben
        i = i + 1;
        
        // kurz warten
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
