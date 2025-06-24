#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use usrlib::{
    self, gprintln, graphix::picturepainting::paint, kernel::runtime::environment::args_as_vec,
};

use crate::custom_pictures::{
    crumpy_cat::get_crumpy, cute_cat::get_cute_cat, doge::get_doge, orion::get_orion,
};

mod custom_pictures;

const PICTURES: &'static [&'static str] = &["cat", "crumpy", "doge", "orion"];

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Laden der Argumente
    let args = args_as_vec();

    if args.len() < 4 {
        gprintln!("Nicht genug argumente um Position und Bild auszuwaehlen");
        return;
    }

    // Parsen der Position'
    let x_result = args.get(1).unwrap().parse::<usize>();
    let y_result = args.get(2).unwrap().parse::<usize>();

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

    // Raussuchen welche Animation gemeint wird
    match args.get(3).unwrap().to_ascii_lowercase().as_str() {
        "cat" | "cutecat" | "cute_cat" => paint::draw_picture(x, y, &get_cute_cat()),
        "doge" => paint::draw_picture(x, y, &get_doge()),
        "crumpy" | "crumpycat" | "crumpy_cat" => paint::draw_picture(x, y, &get_crumpy()),
        "orion" | "nebel" | "nebular" => paint::draw_picture(x, y, &get_orion()),
        // gibt eine Liste aller Bilder aus
        "song" | "songs" => {
            gprintln!("Folgende Bilder sind verfügbar: ");
            for pic in PICTURES {
                gprintln!("    - {}", pic);
            }
            return;
        }
        // Fehlerfall
        _ => {
            gprintln!("Picture not avaiable... :(");
            return;
        }
    }

    // Ausgabe
    gprintln!("Bild: \"{}\" gezeichnet", args.get(3).unwrap().as_str());
}
