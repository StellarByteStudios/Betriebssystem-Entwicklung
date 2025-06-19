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
use usrlib::graphix::picturepainting::animate::animate_blink;
use usrlib::kernel::runtime::environment::args_as_vec;

mod custom_animations;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {

    // Laden der Argumente
    let args = args_as_vec();

    if args.len() < 4 {
        gprintln!("Nicht genug argumente um Position und Animation auszuwaehlen");
        return;
    }

    // Parsen der Position'
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


    // Raussuchen welche Animation gemeint wird
    match args.get(3).unwrap().as_str() {
        "flame" | "Flame" | "blueflame" | "BlueFlame"=> custom_animations::animate::animate_blue_flame(x, y),
        "charmander" | "Charmander" | "pokemon" | "Pokemon" => animate_charmander(x, y),
        "blink" | "blinking" | "Blink" => animate_blink(x, y),
        _ => gprintln!("Animation not avaiable... :("), // nicht registriert
    }
}
