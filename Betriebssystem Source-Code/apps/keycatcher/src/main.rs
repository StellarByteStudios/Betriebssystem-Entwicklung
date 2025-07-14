#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use usrlib::{self, gprintln, kernel::{
    shell::shell_handler::{activate_shell, deactivate_shell},
}, kprintln};
use usrlib::kernel::syscall::keyboard::{get_last_key, get_new_key_event, KeyEvent};
use usrlib::utility::delay::delay;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Nachricht
    gprintln!("Keyboard Highjacked. Press q to exit.");

    // Shell deaktivieren
    deactivate_shell();

    loop {

        // KeyEvent holen
        let key_event = get_new_key_event();


        // Gabs was neues?
        if key_event == KeyEvent::NoEvent {
            continue;
        }


        // Key auspacken
        let char: char = key_event.as_char();

        // Fälle prüfen
        if char == 'q' {
            gprintln!("Du bist frei!");
            activate_shell();
            return;
        }

        gprintln!("Du hast ein \"{}\" eingegeben :)", char);
    }
}
