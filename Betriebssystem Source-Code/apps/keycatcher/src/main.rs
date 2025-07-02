#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use usrlib::{
    self, gprintln,
    kernel::{
        shell::shell_handler::{activate_shell, deactivate_shell},
        syscall::user_api::usr_getlastkey,
    },
};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Nachricht
    gprintln!("Keyboard Highjacked. Press q to exit.");

    // Shell deaktivieren
    deactivate_shell();

    loop {
        // Buchstabe laden
        let char = usr_getlastkey() as u8 as char;

        if char == 'q' {
            gprintln!("Du bist frei!");
            activate_shell();
            return;
        }

        gprintln!("Du hast ein \"{}\" eingegeben :)", char);
    }
}
