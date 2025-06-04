#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

#[allow(unused_imports)]
use usrlib::kernel::runtime::*;
use usrlib::{
    gprint, gprintln,
    kernel::{runtime::environment::args_as_vec, syscall::user_api::usr_hello_world_print},
};

#[link_section = ".main"]
#[no_mangle]
pub fn main() -> isize {

    // Argumente holen
    let arguments = args_as_vec();

    // Argumente Ausgeben
    for argument in &arguments {
        gprint!("{} ", argument);
    }

    // Neue Zeile zum Abschluss
    gprintln!("");

    loop {}
    return 0;
}
