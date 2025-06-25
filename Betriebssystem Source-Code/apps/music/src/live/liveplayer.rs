use alloc::vec;

use usrlib::{
    gprintln,
    kernel::{
        shell::shell_handler::{activate_shell, deactivate_shell},
        syscall::user_api::usr_getlastkey,
    },
    music::player::play_notes,
};

use crate::play::player::key_char_to_note;

pub fn play_live() {
    // Shell deaktivieren
    deactivate_shell();

    gprintln!("Du kanns jetzt richtig ein Keyboard benutzten. \"q\" zum beenden");

    loop {
        // Key holen
        let note = usr_getlastkey() as u8 as char;

        // Fehlerfall
        if note == 'q' {
            gprintln!("App wird beendent");
            break;
        }

        // Ansonsten Ton parsen
        let true_note = key_char_to_note(note);

        // Ist das valide?
        if true_note.is_none() {
            continue;
        }

        // Ton abspielen
        let noten_vector = vec![true_note.unwrap()];

        // Noten abspielen
        play_notes(&noten_vector);
    }

    // Shell wieder freigeben
    activate_shell();
}
