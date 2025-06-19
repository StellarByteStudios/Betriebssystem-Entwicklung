#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use usrlib::{gprintln, kernel::syscall::user_api::usr_print_all_apps};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    gprintln!("Verfügbare Apps: ");
    // Einfach nur Apps ausgeben
    usr_print_all_apps();
}
