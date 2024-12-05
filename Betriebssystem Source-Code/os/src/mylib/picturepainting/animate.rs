use crate::{
    devices::vga,
    mylib::{
        delay::delay,
        picturepainting::pictures::{
            charmander::{charmander0, charmander1, charmander2, charmander3, charmander4},
            ghost::{
                ghost00, ghost01, ghost02, ghost03, ghost04, ghost05, ghost06, ghost07, ghost08,
                ghost09,
            },
        },
    },
};

use super::pictures::blinking::{blinking0, blinking1, blinking2, blinking3, blinking4};
use alloc::vec::Vec;

pub struct Frame {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) bpp: u32,
    pub(crate) data: Vec<u8>,
}

pub fn animate_blink(x: u32, y: u32) {
    // Bilder laden
    let frames: [Frame; 5] = [
        Frame {
            width: blinking0::WIDTH,
            height: blinking0::HEIGHT,
            bpp: blinking0::BPP,
            data: blinking0::DATA.to_vec(),
        },
        Frame {
            width: blinking1::WIDTH,
            height: blinking1::HEIGHT,
            bpp: blinking1::BPP,
            data: blinking1::DATA.to_vec(),
        },
        Frame {
            width: blinking2::WIDTH,
            height: blinking2::HEIGHT,
            bpp: blinking2::BPP,
            data: blinking2::DATA.to_vec(),
        },
        Frame {
            width: blinking3::WIDTH,
            height: blinking3::HEIGHT,
            bpp: blinking3::BPP,
            data: blinking3::DATA.to_vec(),
        },
        Frame {
            width: blinking4::WIDTH,
            height: blinking4::HEIGHT,
            bpp: blinking4::BPP,
            data: blinking4::DATA.to_vec(),
        },
    ];

    // Frames nacheinander zeichnen
    vprintln!("animating Blinking");
    loop {
        for i in 0..frames.len() {
            vga::draw_bitmap(
                x,
                y,
                frames[i].width,
                frames[i].height,
                frames[i].data.as_slice(),
                frames[i].bpp,
            );

            delay(25);
        }
    }
}

pub fn animate_charmander(x: u32, y: u32) {
    // Bilder laden
    let frames: [Frame; 5] = [
        Frame {
            width: charmander0::WIDTH,
            height: charmander0::HEIGHT,
            bpp: charmander0::BPP,
            data: charmander0::DATA.to_vec(),
        },
        Frame {
            width: charmander1::WIDTH,
            height: charmander1::HEIGHT,
            bpp: charmander1::BPP,
            data: charmander1::DATA.to_vec(),
        },
        Frame {
            width: charmander2::WIDTH,
            height: charmander2::HEIGHT,
            bpp: charmander2::BPP,
            data: charmander2::DATA.to_vec(),
        },
        Frame {
            width: charmander3::WIDTH,
            height: charmander3::HEIGHT,
            bpp: charmander3::BPP,
            data: charmander3::DATA.to_vec(),
        },
        Frame {
            width: charmander4::WIDTH,
            height: charmander4::HEIGHT,
            bpp: charmander4::BPP,
            data: charmander4::DATA.to_vec(),
        },
    ];

    // Frames nacheinander zeichnen
    vprintln!("animating Charmander");
    loop {
        for i in 0..frames.len() {
            vga::draw_bitmap(
                x,
                y,
                frames[i].width,
                frames[i].height,
                frames[i].data.as_slice(),
                frames[i].bpp,
            );

            delay(10);
        }
    }
}

pub fn animate_ghost(x: u32, y: u32) {
    // Bilder laden
    let frames: [Frame; 10] = [
        Frame {
            width: ghost00::WIDTH,
            height: ghost00::HEIGHT,
            bpp: ghost00::BPP,
            data: ghost00::DATA.to_vec(),
        },
        Frame {
            width: ghost01::WIDTH,
            height: ghost01::HEIGHT,
            bpp: ghost01::BPP,
            data: ghost01::DATA.to_vec(),
        },
        Frame {
            width: ghost02::WIDTH,
            height: ghost02::HEIGHT,
            bpp: ghost02::BPP,
            data: ghost02::DATA.to_vec(),
        },
        Frame {
            width: ghost03::WIDTH,
            height: ghost03::HEIGHT,
            bpp: ghost03::BPP,
            data: ghost03::DATA.to_vec(),
        },
        Frame {
            width: ghost04::WIDTH,
            height: ghost04::HEIGHT,
            bpp: ghost04::BPP,
            data: ghost04::DATA.to_vec(),
        },
        Frame {
            width: ghost05::WIDTH,
            height: ghost05::HEIGHT,
            bpp: ghost05::BPP,
            data: ghost05::DATA.to_vec(),
        },
        Frame {
            width: ghost06::WIDTH,
            height: ghost06::HEIGHT,
            bpp: ghost06::BPP,
            data: ghost06::DATA.to_vec(),
        },
        Frame {
            width: ghost07::WIDTH,
            height: ghost07::HEIGHT,
            bpp: ghost07::BPP,
            data: ghost07::DATA.to_vec(),
        },
        Frame {
            width: ghost08::WIDTH,
            height: ghost08::HEIGHT,
            bpp: ghost08::BPP,
            data: ghost08::DATA.to_vec(),
        },
        Frame {
            width: ghost09::WIDTH,
            height: ghost09::HEIGHT,
            bpp: ghost09::BPP,
            data: ghost09::DATA.to_vec(),
        },
    ];

    // Frames nacheinander zeichnen
    vprintln!("animating Gillbert the Ghost");
    loop {
        for i in 0..frames.len() {
            vga::draw_bitmap(
                x,
                y,
                frames[i].width,
                frames[i].height,
                frames[i].data.as_slice(),
                frames[i].bpp,
            );

            delay(5);
        }
    }
}
