#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use alloc::boxed::Box;

use usrlib::{
    self,
    kernel::syscall::{
        process_management::get_process_name,
        user_api::{usr_dump_active_vmas, usr_get_pid},
    },
    print_setpos,
    utility::{delay::delay, mathadditions::fibonacci::calculate_fibonacci_rec},
};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // VMAs Ausgeben
    usr_dump_active_vmas();

    // Allokator benutzen
    let alloc_box: Box<u64> = Box::new(6);

    // Counter starten
    let mut i: u64 = 0;
    loop {
        // Daten holen
        let pid = usr_get_pid() as usize;
        let actual_name = get_process_name();

        // Ausgabe
        print_setpos!(10, 30, "Name: {}; pid: {}", actual_name, pid);
        print_setpos!(10, 31, "Counter {}", i);

        //let add_summand: u64 = 1000;
        let add_summand: u64 = 0;

        // Fibonacci berechnen
        let fibonacci_value = calculate_fibonacci_rec(i + add_summand);

        // fibonacci Wert ausgeben
        print_setpos!(
            10,
            32,
            "{}th fibonacci value: {}",
            i + add_summand,
            fibonacci_value
        );

        // Counter verschieben
        i = i + 1;

        // kurz warten
        delay(10);
    }
}
