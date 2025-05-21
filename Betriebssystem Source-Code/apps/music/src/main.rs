#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use core::str::from_utf8_unchecked;

use usrlib::{
    self,
    kernel::syscall::{
        user_api::{usr_get_pid, usr_play_song, usr_read_process_name},
        SongID,
    },
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

    // Ausgabe
    print_setpos!(50, 15, "Name: {}; pid: {}", actual_name, pid);

    usr_play_song(SongID::starwars_imperial as usize);
}
