#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use core::{panic::PanicInfo, str::from_utf8_unchecked};

use usrlib::{
    self,
    graphix::picturepainting::animate::animate_charmander,
    kernel::{
        allocator::allocator::{init, HEAP_SIZE},
        syscall::user_api::{usr_get_pid, usr_read_process_name},
    },
    print_setpos,
};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Allokator initialisieren
    let pid: usize = usr_get_pid() as usize;

    init(pid, HEAP_SIZE);

    const BUFFERLENGH: usize = 255;

    // Daten holen
    let pid = usr_get_pid();
    let mut namebuffer: [u8; BUFFERLENGH] = [0; BUFFERLENGH];
    usr_read_process_name(namebuffer.as_mut_ptr(), BUFFERLENGH) as usize;
    let actual_name: &str = unsafe {
        from_utf8_unchecked(
            namebuffer
                .as_slice()
                .split(|&b| b == 0)
                .next()
                .unwrap_or(&[]),
        )
    };

    // Ausgabe
    print_setpos!(50, 36, "Name: {}; pid: {}", actual_name, pid);


    // Animation
    animate_charmander(500, 400);

    loop {}
}

/*
* Panic Handler
*/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
