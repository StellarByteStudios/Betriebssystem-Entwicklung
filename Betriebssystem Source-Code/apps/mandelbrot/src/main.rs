#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

#[allow(unused_imports)]
use usrlib::kernel::runtime::*;
use usrlib::{gprint, gprintln, kernel::runtime::environment::args_as_vec};

use crate::mandelbrot::drawer;

mod mandelbrot;

const FIXED_HEIGHT: u32 = 300;
const FIXED_WIDTH: u32 = 450;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Laden der Argumente
    let args = args_as_vec();

    if args.len() < 3 {
        gprintln!("Nicht genug argumente um Position und Bild auszuwaehlen");
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
    if x > 1280 || y > 720 {
        gprintln!("Keine sinnvolle Position x in [0, 1280]; y in [0, 720]");
        return;
    }

    // Neue Zeile zum Abschluss
    gprintln!("Zeichne Mandelbrot");

    drawer::draw_mandelbrot(FIXED_WIDTH, FIXED_HEIGHT, (x, y))
}
