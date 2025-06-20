#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use core::str::from_utf8_unchecked;

mod songs;

use usrlib::{
    self,
    kernel::syscall::user_api::{usr_get_pid, usr_read_process_name},
    music::player,
    print_setpos,
};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
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

    /*
    // Laden welcher Song gespielt werden muss
    let args = args_as_vec();

    if args.len() < 2 {
        gprintln!("Nicht genug argumente um ein Lied auszuwaehlen");
        return;
    }

    // Parsen der songnummer
    let song_nr = args.get(1).unwrap().parse::<u32>();

    // War das Parsen erfolgreich
    if song_nr.is_err() {
        gprintln!(
            "Die songnummer muss eine richtige Zahl sein und das hat bei {:?} nicht funktioniert",
            args.get(1)
        );
        return;
    }

    // Ist die Zahl sinnvoll
    if song_nr.clone().unwrap() > SongID::doom as u32 {
        gprintln!("Maximale Songnummer: {}", SongID::doom as usize);
        return;
    }*/

    // Ausgabe
    print_setpos!(50, 15, "Name: {}; pid: {}", actual_name, pid);
    //gprintln!("Playing Songs not implemented yet");

    //usr_play_song(song_nr.unwrap() as usize);
    player::play_notes(songs::nyancat::NYANCAT)
}
