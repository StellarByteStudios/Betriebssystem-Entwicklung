#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use usrlib::{gprintln};
use usrlib::kernel::syscall::process_management::print_running_threads;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    gprintln!("Folgende Threads laufen Gerade: ");
    // Einfach nur Threads ausgeben
    print_running_threads();
}
