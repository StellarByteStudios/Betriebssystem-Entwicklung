/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: vga                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: The graphics mode is turned on by grub. The configuration is in ║
   ║         'boot.asm' in the multiboot header. The config. is read in      ║
   ║         'startup.rs' and used here and in the graphic demo thread.      ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 26.6.2023                 ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![allow(dead_code)]

use crate::devices::graphical::fonts::font_8x8;

// Global VGA struct
static mut VGA: Option<VGA> = None;

pub fn draw_pixel(x: u32, y: u32, col: u32) {
    unsafe {
        if VGA.is_none() {
            return;
        }
        VGA.as_ref().unwrap().draw_pixel(x, y, col);
    }
}

pub fn get_pixel(x: u32, y: u32) -> u32 {
    unsafe {
        if VGA.is_none() {
            return 0;
        }
        return VGA.as_ref().unwrap().get_pixel(x, y);
    }
}

pub fn draw_string(x: u32, y: u32, col: u32, string: &str) {
    unsafe {
        if VGA.is_none() {
            return;
        }
        VGA.as_ref().unwrap().draw_string(x, y, col, string);
    }
}

pub fn draw_byte(x: u32, y: u32, col: u32, byte: u8) {
    unsafe {
        if VGA.is_none() {
            return;
        }
        VGA.as_ref().unwrap().draw_byte(x, y, col, byte);
    }
}

pub fn draw_bitmap(x: u32, y: u32, width: u32, height: u32, bitmap: &[u8], bpp: u32) {
    unsafe {
        if VGA.is_none() {
            return;
        }
        VGA.as_ref()
            .unwrap()
            .draw_bitmap(x, y, width, height, bitmap, bpp);
    }
}

pub fn get_res() -> (u32, u32) {
    unsafe {
        if VGA.is_none() {
            return (0, 0);
        }
        (VGA.as_ref().unwrap().width, VGA.as_ref().unwrap().height)
    }
}

// Farbwert für 24/32 Bit Farbmodus
pub const fn rgb_24(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) + ((g as u32) << 8) + (b as u32)
}

pub struct VGA {
    addr: u64,   // start address of linear framebuffer
    pitch: u32,  // number of bytes per line (often: pitch = xres, but not always)
    width: u32,  // number of horizontal pixels
    height: u32, // number of vertical pixels
    bpp: u8,     // color depth (number of bits per pixel
}

impl VGA {
    pub fn init(a: u64, p: u32, w: u32, h: u32, b: u8) {
        unsafe {
            VGA = Some(VGA {
                addr: a,
                pitch: p,
                width: w,
                height: h,
                bpp: b,
            });
        }
    }

    /**
     Description: Draw a bitmap at Position `x`, `y`
                  (`bitmap`: pixel array, `bpp` bits per pixel)
    */
    pub fn draw_bitmap(&self, x: u32, y: u32, width: u32, height: u32, bitmap: &[u8], bpp: u32) {
        let xpos: u32 = x;
        let ypos: u32 = y;
        let mut idx: usize = 0;
        let mut r: u8;
        let mut g: u8;
        let mut b: u8;

        // Pixel ausserhalb des sichtbaren Bereichs?
        if x >= self.width || y >= self.height {
            return;
        }

        // Bitmap zeichnen
        for y in 0..height {
            for x in 0..width {
                r = bitmap[idx];
                idx = idx + 1;
                g = bitmap[idx];
                idx = idx + 1;
                b = bitmap[idx];
                idx = idx + 1;

                // Alphakanal:
                if bpp == 4 {

                    // Unter Schwelle: nicht zeichnen
                    if bitmap[idx] <= 0xFF/2 {
                        idx = idx + 1;
                        continue;
                    }

                    // Ansonsten normal weiter machen
                    idx = idx + 1;
                }

                self.draw_pixel(xpos + x, ypos + y, rgb_24(r, g, b));
            }
        }
    }

    /**
     Description: Draw a pixel at Position `x`, `y` using colour `col`.
    */
    fn draw_pixel(&self, x: u32, y: u32, col: u32) {
        let mut ptr: u64;

        // Pixel ausserhalb des sichtbaren Bereichs?
        if x >= self.width || y >= self.height {
            return;
        }

        // Adresse des Pixels berechnen und Inhalt schreiben
        match self.bpp {
            8 => {
                ptr = self.addr + (x + y * self.width) as u64;
                unsafe {
                    *(ptr as *mut u8) = col as u8;
                }
            }
            15 | 16 => {
                ptr = self.addr + (2 * x + 2 * y * self.width) as u64;
                unsafe {
                    *(ptr as *mut u8) = col as u8;
                }
            }
            24 => {
                ptr = self.addr + (3 * x + 3 * y * self.width) as u64;
                unsafe {
                    *(ptr as *mut u8) = (col & 0xFF) as u8;
                    ptr = ptr + 1;
                    *(ptr as *mut u8) = ((col >> 8) & 0xFF) as u8;
                    ptr = ptr + 1;
                    *(ptr as *mut u8) = ((col >> 16) & 0xFF) as u8;
                }
            }
            32 => {
                ptr = self.addr + (4 * x + 4 * y * self.width) as u64;
                unsafe {
                    *(ptr as *mut u8) = (col & 0xFF) as u8;
                    ptr = ptr + 1;
                    *(ptr as *mut u8) = ((col >> 8) & 0xFF) as u8;
                    ptr = ptr + 1;
                    *(ptr as *mut u8) = ((col >> 16) & 0xFF) as u8;
                }
            }
            _ => {}
        }
    }

    /**
     Description: Returns the color at Position `x`, `y`.
    */
    fn get_pixel(&self, x: u32, y: u32) -> u32 {
        let mut ptr: u64;

        // Pixel ausserhalb des sichtbaren Bereichs?
        if x >= self.width || y >= self.height {
            return 0;
        }

        // Adresse des Pixels berechnen und Inhalt holen
        ptr = self.addr + (4 * x + 4 * y * self.width) as u64;

        unsafe {
            let b = *(ptr as *mut u8);
            ptr = ptr + 1;
            let g = *(ptr as *mut u8);
            ptr = ptr + 1;
            let r = *(ptr as *mut u8);

            return rgb_24(r, g, b);
        }
    }

    // Slice auf Pixel-Daten eines Zeichens
    fn get_char(&self, data: &[u8], c: u8) -> &[u8] {
        let char_mem_size = (font_8x8::CHAR_WIDTH + (8 >> 1)) / 8 * font_8x8::CHAR_HEIGHT;
        let start_idx = (char_mem_size * c as u32) as usize;
        let end_idx = start_idx + char_mem_size as usize;

        &font_8x8::DATA[start_idx..end_idx]
    }

    /**
     Description: Draw a string at Position `x`, `y` using `font_8x8` with colour `col`
    */
    fn draw_string(&self, x: u32, y: u32, col: u32, string: &str) {
        let char_width = font_8x8::CHAR_WIDTH;
        let char_height = font_8x8::CHAR_HEIGHT;

        let mut width_byte: u32 = char_width / 8;
        if (char_width % 8) != 0 {
            width_byte = width_byte + 1;
        }

        let mut x2 = x;

        // Pixel ausserhalb des sichtbaren Bereichs?
        if x >= self.width || y >= self.height {
            return;
        }

        // Iteriere ueber alle Zeichen
        for ch in string.bytes() {
            let chpix = self.get_char(font_8x8::DATA, ch);

            // ein Zeichnen ausgeben
            let mut idx = 0;
            for yoff in 0..char_height {
                let mut xpos = x2;
                let ypos = y + yoff;
                for xb in 0..width_byte {
                    for src in (0..8).rev() {
                        if ((1 << src) & chpix[idx]) != 0 {
                            draw_pixel(xpos, ypos, col);
                        }
                        xpos = xpos + 1;
                    }
                }
                idx = idx + 1;
            }
            x2 = x2 + char_width;
        }
    }

    fn draw_byte(&self, x: u32, y: u32, col: u32, ch: u8) {
        let char_width = font_8x8::CHAR_WIDTH;
        let char_height = font_8x8::CHAR_HEIGHT;

        let mut width_byte: u32 = char_width / 8;
        if (char_width % 8) != 0 {
            width_byte = width_byte + 1;
        }

        let x2 = x;

        // Pixel ausserhalb des sichtbaren Bereichs?
        if x >= self.width || y >= self.height {
            return;
        }

        let chpix = self.get_char(font_8x8::DATA, ch);

        // ein Zeichnen ausgeben
        let mut idx = 0;
        for yoff in 0..char_height {
            let mut xpos = x2;
            let ypos = y + yoff;
            for xb in 0..width_byte {
                for src in (0..8).rev() {
                    if ((1 << src) & chpix[idx]) != 0 {
                        draw_pixel(xpos, ypos, col);
                    }
                    xpos = xpos + 1;
                }
            }
            idx = idx + 1;
        }
    }
}
