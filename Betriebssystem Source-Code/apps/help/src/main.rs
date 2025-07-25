#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use usrlib::{gprintln, kernel::syscall::user_api::usr_print_all_apps};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Einfache Ausgabe
    gprintln!("Hello, this is a rudimentary shell. All running apps use an API to the userlib\nWant to use it? Type one of these commands:");

    // Ausgabe der Apps
    usr_print_all_apps();
}
