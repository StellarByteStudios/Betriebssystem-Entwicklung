#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

#[allow(unused_imports)]
use usrlib::kernel::runtime::*;
use usrlib::{
    gprint, gprintln,
    kernel::{runtime::environment::args_as_vec},
};
use usrlib::kernel::runtime::environment::args_len;

#[link_section = ".main"]
#[no_mangle]
pub fn main() -> isize {
    // Argumente holen
    let arguments = args_as_vec();

    // Anzahl holen
    let argc = args_len();

    // Argumente Ausgeben
    //gprintln!("Anzahl: {}, gemessen: {}", argc, arguments.len());
    for argument in &arguments {
        gprint!("{} ", argument);
    }

    // Neue Zeile zum Abschluss
    gprintln!("");

    return 0;
}
