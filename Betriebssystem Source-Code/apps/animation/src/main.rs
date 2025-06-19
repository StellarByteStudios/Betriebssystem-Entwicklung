#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use core::str::from_utf8_unchecked;

use usrlib::{self, gprintln, graphix::picturepainting::animate::animate_charmander, kernel::{
    allocator::allocator::init,
    runtime::runtime::HEAP_SIZE,
    syscall::user_api::{usr_get_pid, usr_read_process_name},
}, print_setpos};
use usrlib::kernel::runtime::environment::args_as_vec;
use usrlib::kernel::syscall::user_api::usr_get_screen_width;

mod custom_animations;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Allokator initialisieren
    let pid: usize = usr_get_pid() as usize;

    init(pid, HEAP_SIZE);

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


    // Laden welcher Song gespielt werden muss
    let args = args_as_vec();

    if args.len() < 3 {
        gprintln!("Nicht genug argumente um Position und Animation auszuwaehlen");
        return;
    }

    // Parsen der songnummer
    let x_result = args.get(1).unwrap().parse::<u32>();
    let y_result = args.get(2).unwrap().parse::<u32>();


    // War das Parsen erfolgreich
    if x_result.is_err() || y_result.is_err() {
        gprintln!(
            "Die Koordinaten muessen eine richtige Zahl sein. Deine Eingabe: {:?}, {:?}",
            args.get(1),
            args.get(2)
        );
        return;
    }

    let x = x_result.clone().unwrap();
    let y = y_result.clone().unwrap();

    // Ist die Zahl sinnvoll
    if x > 1280 || x < 0 || y > 720 || y < 0 {
        gprintln!("Keine sinnvolle Position x in [0, 1280]; y in [0, 720]");
        return;
    }


    // Ausgabe
    print_setpos!(50, 36, "Name: {}; pid: {}", actual_name, pid);

    // Animation
    //animate_charmander(500, 400);
    //animate_ghost(500, 400);
    custom_animations::animate::animate_blue_flame(x, y);

}
