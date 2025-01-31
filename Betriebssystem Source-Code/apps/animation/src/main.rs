#![no_std]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use core::panic::PanicInfo;
use core::str::from_utf8_unchecked;
// Sobald usrlib importiert wird "error: no global memory
// allocator found but one is required; link to std or add `#[global_allocator]`
// to a static item that implements the GlobalAlloc trait"
use usrlib;
use usrlib::graphix::picturepainting::animate::{animate_blink, animate_charmander, animate_ghost};
use usrlib::graphix::picturepainting::pictures::crumpy_cat;
use usrlib::kernel::allocator::allocator::{init, HEAP_SIZE};
use usrlib::kernel::syscall::user_api::{
    usr_get_pid, usr_paint_picture_on_pos, usr_read_process_name,
};
use usrlib::print_setpos;

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
    usr_read_process_name(namebuffer.as_mut_ptr(), BUFFERLENGH as u64) as usize;
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
    //animate_ghost(500, 450);
    /*
    usr_paint_picture_on_pos(
        500,
        450,
        crumpy_cat::HEIGHT as u64,
        crumpy_cat::WIDTH as u64,
        crumpy_cat::BPP as u64,
        crumpy_cat::DATA.as_ptr(),
    );*/

    loop {}
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
