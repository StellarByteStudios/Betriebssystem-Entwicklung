#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

mod play;
mod songs;

use usrlib::{
    self, gprintln,
    kernel::runtime::environment::args_as_vec,
    music::{note::Note, player::play_notes},
};
use crate::play::player;
use crate::songs::{
    daftpunk::AERODYNAMIC, doom::DOOM, entchen::ENTCHEN, nintendo::MARIO, nyancat::NYANCAT,
    starwars::STARWARS_IMPERIAL, tetris::TETRIS,
};

const SONGS: &'static [&'static str] = &[
    "nyancat",
    "starwars",
    "mario",
    "aerodynamics",
    "tetris",
    "entchen",
    "doom",
];

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Laden der Argumente
    let args = args_as_vec();

    if args.len() < 2 {
        gprintln!("Nicht genug argumente um Position und Animation auszuwaehlen");
        return;
    }

    let note_slice: &[Note];

    // Raussuchen welche Animation gemeint wird
    match args.get(1).unwrap().to_ascii_lowercase().as_str() {
        "cat" | "nyancat" => note_slice = NYANCAT,
        "starwars" | "imperial" => note_slice = STARWARS_IMPERIAL,
        "mario" => note_slice = MARIO,
        "aerodynamics" | "aero" => note_slice = AERODYNAMIC,
        "tetris" => note_slice = TETRIS,
        "entchen" | "allemeineentchen" => note_slice = ENTCHEN,
        "doom" => note_slice = DOOM,
        // Dynamischer Player
        "play" | "player" => {
            player::play_args(args);
            return;
        }
        // gibt eine Liste aller Songs aus
        "song" | "songs" => {
            gprintln!("Folgende Songs sind abspielbar: ");
            for song in SONGS {
                gprintln!("    - {}", song);
            }
            return;
        }
        // Fehlerfall
        _ => {
            gprintln!("Song not avaiable... :(");
            return;
        } // nicht registriert
    }

    // Ausgabe
    gprintln!("Playing song: \"{}\"", args.get(1).unwrap().as_str());

    // Musik abspielen
    play_notes(note_slice);
}
