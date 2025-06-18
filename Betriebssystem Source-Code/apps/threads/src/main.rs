#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use usrlib::gprintln;
use usrlib::kernel::syscall::user_api::usr_print_running_thread;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    gprintln!("Folgende Threads laufen Gerade: ");
    // Einfach nur Threads ausgeben
    usr_print_running_thread();
}
