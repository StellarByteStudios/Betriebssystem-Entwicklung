use alloc::{string::String, vec::Vec};

use usrlib::{
    gprintln, kprintln,
    music::{note::Note, player::play_notes},
};

use crate::play::scale::*;

enum PlayerMode {
    True,
    Key,
    Error,
}

// = = Spielt Dynamisch übergebene Noten ab = = //
pub fn play_args(args: Vec<String>) {
    // Valide Argumente?
    let args_checked: PlayerMode = check_arguments(&args);

    // Matchen wie der Player benutzt wird
    match args_checked {
        PlayerMode::True => play_true_notes(args.get(3).unwrap().clone()),
        PlayerMode::Key => play_key_notes(args.get(3).unwrap().clone()),
        PlayerMode::Error => gprintln!("play Kommand nicht richtig benutzt \nmusic play <true/key> <Notes> \n   wobei Notes als Notenname (cdefgab x) oder als keys (asdfjkl b) eingegeben werden"),
    }
}

// = = Prüft ob noch genug restargumente vorhanden sind = = //
fn check_arguments(args: &Vec<String>) -> PlayerMode {
    // Länge genug?
    if args.len() < 4 {
        return PlayerMode::Error;
    }

    // True Mode
    if args.get(2).unwrap().to_ascii_lowercase().contains("true") {
        return PlayerMode::True;
    }

    // Key Mode
    if args.get(2).unwrap().to_ascii_lowercase().contains("key") {
        return PlayerMode::Key;
    }

    // Kein Valider Modus
    return PlayerMode::Error;
}

// = = Spielt die Noten nach echten Namen = = //
fn play_true_notes(notes: String) {
    // String zu einzelnen Chars zerlegen
    let true_notes: Vec<Note> = notes.chars().filter_map(|c| true_char_to_note(c)).collect();

    // Ausgabe
    gprintln!("Dein Song \"{}\" wird abgespielt", notes);

    // Als Slice abspielen
    play_notes(&true_notes);
}

// = = Mapped einen Buchstaben auf echte Notennamen = = //
fn true_char_to_note(note: char) -> Option<Note> {
    let freq = match note {
        // Niedere Oktave
        'C' => Some(C0),
        'D' => Some(D0),
        'E' => Some(E0),
        'F' => Some(F0),
        'G' => Some(G0),
        'A' => Some(A0),
        'B' => Some(B0),
        // Höhere Oktave
        'c' => Some(C1),
        'd' => Some(D1),
        'e' => Some(E1),
        'f' => Some(F1),
        'g' => Some(G1),
        'a' => Some(A1),
        'b' => Some(B1),
        // Pause
        'x' | 'X' | ' ' => Some(PAUSE),
        // Keine Valide Note
        _ => return None,
    };

    return Some(Note {
        frequency: freq.unwrap() as u32,
        duration: 250,
    });
}

// = = Spielt die Noten nach Keyboard Layout = = //
fn play_key_notes(notes: String) {
    // String zu einzelnen Chars zerlegen
    let true_notes: Vec<Note> = notes
        .chars()
        .filter_map(|c| key_char_to_note(c)) // <- filtert None automatisch raus
        .collect();

    // Ausgabe
    gprintln!("Dein Song \"{}\" wird abgespielt", notes);

    // Noten abspielen
    play_notes(&true_notes);
}

// = = Mapped einen Buchstaben aus Keyboard Layout = = //
pub fn key_char_to_note(note: char) -> Option<Note> {
    let freq = match note {
        // Niedere Oktave
        'A' => Some(C0),
        'W' => Some(C0X),
        'S' => Some(D0),
        'E' => Some(D0X),
        'D' => Some(E0),
        'F' => Some(F0),
        'T' => Some(F0X),
        'G' => Some(G0),
        'Z' => Some(G0X),
        'H' => Some(A0),
        'U' => Some(A0X),
        'J' => Some(B0),
        // Höhere Oktave
        'a' => Some(C1),
        'w' => Some(C1X),
        's' => Some(D1),
        'e' => Some(D1X),
        'd' => Some(E1),
        'f' => Some(F1),
        't' => Some(F1X),
        'g' => Some(G1),
        'z' => Some(G1X),
        'h' => Some(A1),
        'u' => Some(A1X),
        'j' => Some(B1),
        // Pause
        'b' | 'B' | ' ' => Some(PAUSE),
        // Keine Valide Note
        _ => {
            kprintln!("Key \"{}\" not recognized", note);
            return None;
        }
    };

    return Some(Note {
        frequency: freq.unwrap() as u32,
        duration: 250,
    });
}
