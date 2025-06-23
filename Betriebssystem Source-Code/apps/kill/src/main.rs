#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use usrlib::{
    gprintln,
    kernel::{runtime::environment::args_as_vec, syscall::process_management::kill_process},
};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Laden der Argumente
    let args = args_as_vec();

    if args.len() < 2 {
        gprintln!("Übergebe die Pid des Prozesses den du beenden möchtest");
        return;
    }

    // Parsen der Position'
    let pid_result = args.get(1).unwrap().parse::<u32>();

    // War das Parsen erfolgreich
    if pid_result.is_err() {
        gprintln!(
            "Die PID muss eine richtige Zahl sein und das hat bei {:?} nicht funktioniert",
            args.get(1)
        );
        return;
    }

    // Prozess mit Pid beenden
    kill_process(pid_result.unwrap() as usize);
}
