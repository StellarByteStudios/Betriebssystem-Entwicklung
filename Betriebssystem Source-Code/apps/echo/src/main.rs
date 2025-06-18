#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

#[allow(unused_imports)]
use usrlib::kernel::runtime::*;
use usrlib::{
    gprint, gprintln,
    kernel::runtime::environment::{args_as_vec, args_len},
    kprint, kprintln,
};

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

    // Kernel Print test
    kprintln!("Syscall kprint: args={:?}", arguments);

    drop(arguments);

    kprintln!("Arguments Dropped");
    return 0;
}
