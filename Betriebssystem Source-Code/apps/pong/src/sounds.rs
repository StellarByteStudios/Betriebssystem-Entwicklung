use usrlib::music::{
    note::Note,
    player::{play_note, play_notes},
};

const BEEP: Note = Note {
    frequency: 800,
    duration: 10,
};

const SCORE: &[Note] = &[
    Note {
        frequency: 440,
        duration: 150,
    },
    Note {
        frequency: 523,
        duration: 150,
    },
    Note {
        frequency: 784,
        duration: 150,
    },
    Note {
        frequency: 587,
        duration: 250,
    },
];
pub fn play_simple_collision() {
    play_note(BEEP);
}

pub fn play_point_scored() {
    play_notes(SCORE);
}
