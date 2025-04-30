use alloc::vec::{self, Vec};

use crate::devices::pcspk::{
    self, aerodynamic, alle_meine_entchen, doom, intro, nyancat, starwars_imperial, super_mario,
    tetris,
};

const SONG_LOOKUP: &[fn()] = &[
    super_mario,
    starwars_imperial,
    tetris,
    aerodynamic,
    nyancat,
    alle_meine_entchen,
    intro,
    doom,
];
fn empty_song_function() {
    kprintln!("empty song triggered!");
}

#[no_mangle]
pub extern "C" fn sys_play_song(id: u64) {
    // Id testen:
    if id >= SONG_LOOKUP.len() as u64 {
        kprintln!("Song with ID {} not available", id);
        return;
    }

    kprintln!("Playing Song with ID = {}", id);

    // Song holen
    let song_function: fn() = *SONG_LOOKUP
        .get(id as usize)
        .unwrap_or(&(empty_song_function as fn()));

    // Song abspielen
    song_function();
}
