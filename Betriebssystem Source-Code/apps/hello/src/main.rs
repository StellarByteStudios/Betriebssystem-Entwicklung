#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::ToString;
use core::str::from_utf8_unchecked;

use usrlib::{
    self,
    kernel::{
        allocator::allocator::init,
        runtime::runtime::HEAP_SIZE,
        syscall::user_api::{usr_dump_active_vmas, usr_get_pid, usr_read_process_name},
    },
    print_setpos,
    utility::{delay::delay, mathadditions::fibonacci::calculate_fibonacci_rec},
};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // VMAs Ausgeben
    usr_dump_active_vmas();

    // Allokator initialisieren
    let pid: usize = usr_get_pid() as usize;

    init(pid, HEAP_SIZE);

    // Allokator benutzen
    let alloc_box: Box<u64> = Box::new(6);

    // Counter starten
    let mut i: u64 = 0;
    loop {
        const BUFFERLENGH: usize = 255;

        // Daten holen
        let pid = usr_get_pid();

        let mut namebuffer: [u8; BUFFERLENGH] = [0; BUFFERLENGH];
        usr_read_process_name(namebuffer.as_mut_ptr(), BUFFERLENGH) as usize;

        let actual_name: &str = unsafe {
            from_utf8_unchecked(
                namebuffer
                    .as_slice()
                    .split(|&b| b == 0)
                    .next()
                    .unwrap_or(&[]),
            )
        };

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
