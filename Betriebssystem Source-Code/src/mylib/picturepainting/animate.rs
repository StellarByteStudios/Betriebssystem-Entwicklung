use crate::{devices::vga, mylib::delay::delay};

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
        for i in 0..5 {
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
