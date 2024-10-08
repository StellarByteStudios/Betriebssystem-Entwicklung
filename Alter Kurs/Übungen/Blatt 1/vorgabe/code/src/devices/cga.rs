/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: cga                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: This module provides functions for doing output on the CGA text ║
   ║         screen. It also supports a text cursor position stored in the   ║
   ║         hardware using ports.                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 6.2.2024                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use crate::kernel::cpu as cpu;


// make type comparable, printable and enable copy semantics
#[allow(dead_code)]   // avoid warnings for unused colors
#[repr(u8)]           // store each enum variant as an u8
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}


pub const CGA_STD_ATTR: u8       = (Color::Black as u8) << 4 | (Color::Green as u8);

const CGA_BASE_ADDR: u32     = 0xb8000;
const CGA_ROWS   : u32       = 25;
const CGA_COLUMNS: u32       = 80;

const CGA_INDEX_PORT: u16    = 0x3d4;  // select register
const CGA_DATA_PORT: u16     = 0x3d5;  // read/write register
const CGA_HIGH_BYTE_CMD: u8  = 14;     // cursor high byte
const CGA_LOW_BYTE_CMD: u8   = 15;     // cursor high byte


/**
 Description: Clear text screen
*/
pub fn clear() {

   /* Hier muss Code eingefuegt werden */

}


/**
 Description: Display the `character` at the given position `x`,`y` with attribute `attrib`
*/
pub fn show (x: u32, y: u32, character: char, attrib: u8) {
    let pos: u32;

    if x>CGA_COLUMNS || y>CGA_ROWS
    {    
		return ; 
    }
    
    pos = (y * CGA_COLUMNS + x) * 2;

    unsafe {
        *((CGA_BASE_ADDR + pos) as *mut u8)     = character as u8;
        *((CGA_BASE_ADDR + pos + 1) as *mut u8) = attrib;
    }
}


/**
 Description: Return cursor position `x`,`y` 
*/
pub fn getpos () -> (u32, u32) {

   /* Hier muss Code eingefuegt werden */

   (0,0) // Platzhalter, entfernen und durch sinnvollen Rueckgabewert ersetzen 
}


/**
 Description: Set cursor position `x`,`y` 
*/
pub fn setpos (x:u32, y:u32) {

   /* Hier muss Code eingefuegt werden */

}

 
/**
 Description: Print byte `b` at actual position cursor position `x`,`y` 
*/
pub fn print_byte (b: u8) {

   /* Hier muss Code eingefuegt werden */

}


/**
 Description: Scroll text lines by one to the top.
*/
pub fn scrollup () {

   /* Hier muss Code eingefuegt werden */

}
 
 
/**
 Description: Helper function returning an attribute byte for the given 
              parameters `bg`, `fg`, and `blink`
*/
pub fn attribute (bg: Color, fg: Color, blink: bool) -> u8 {

   /* Hier muss Code eingefuegt werden */
   
   0 // Platzhalter, entfernen und durch sinnvollen Rueckgabewert ersetzen 
}
