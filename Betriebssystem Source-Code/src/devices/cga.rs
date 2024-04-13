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

use super::{cga_print::print, kprint};




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
   // Cursor an Anfang setzten
   setpos(0, 0);

   // Kompletten Bildschirm mit Leerzeichen füllen
   // Für alle Zeilen
   for row in 0..CGA_ROWS{
      // Jedes Zeichen pro Spalte
      for column in 0..CGA_COLUMNS{
         // Leerzeichen schreiben
         print_byte(' ' as u8);
      }

   }
   
   
   // Cursor wieder an Anfang setzten
   setpos(0, 0);

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
   // Low-Byte holen
   cpu::outb(CGA_INDEX_PORT, CGA_LOW_BYTE_CMD);
   let b_low: u8 = cpu::inb(CGA_DATA_PORT);

   // Highbyte holen
   cpu::outb(CGA_INDEX_PORT, CGA_HIGH_BYTE_CMD);
   let b_high: u8 = cpu::inb(CGA_DATA_PORT);

   // Offset zusammenkleben
   let offset: u32 = clue_bytes(b_low, b_high) as u32;

   // x-Wert berechnen
   let x: u32 = offset % CGA_COLUMNS;

   // y-Wert berechnen
   let y: u32 = (offset - x) / CGA_COLUMNS;

   return (x, y);
}


/**
 Description: Set cursor position `x`,`y` 
*/
pub fn setpos (x:u32, y:u32) {
   // Ist der Cursor im Scope des Bildschirms
   if x < 0 || x > CGA_COLUMNS {
      return;
   }
   if y < 0 || y > CGA_ROWS {
      return;
   }


   // Possitionsoffset berechnen
   let cursor_offset: u32 = y * CGA_COLUMNS + x;
   let cursor_bytes: (u8, u8) = get_bytes(cursor_offset as u16);


   // Low-Byte setzen
   // Richtige Registerstelle auswählen
   cpu::outb(CGA_INDEX_PORT, CGA_LOW_BYTE_CMD);
   // Daten (Possition) rein schreiben
   cpu::outb(CGA_DATA_PORT, cursor_bytes.0);

   // High-Byte setzen
   // Richtige Registerstelle auswählen
   cpu::outb(CGA_INDEX_PORT, CGA_HIGH_BYTE_CMD);
   // Daten (Possition) rein schreiben
   cpu::outb(CGA_DATA_PORT, cursor_bytes.1);

}

 
/**
 Description: Print byte `b` at actual position cursor position `x`,`y` 
*/
pub fn print_byte (b: u8) {

   // Possition des Cursers holen
   let pos: (u32, u32) = getpos();

   // An diese Stelle das Byte Printen
   // Prüfen ob es eine neue Zeile ist
   if b == '\n' as u8 {
      setpos(0, pos.1 + 1);
      return;
   }

   // Ansonsten normal Ausgeben
   show(pos.0, pos.1, b as char, 0x2);

   // Curser eins weiter gehen lassen
   // Line-Wrap wenn Zeile voll
   if pos.0 == CGA_COLUMNS{
      setpos(0, pos.1 + 1);
      return;
   }

   // Abruch wenn Bildschirm voll
   if pos.1 == CGA_ROWS {
      // Kompiliert nicht, weil er das Makro nicht findet
      //kprintln!("WARNING: Screen is full, wrote no Byte");
      return
   }
   // Curser normal eins weiter setzten
   setpos(pos.0 + 1, pos.1);
   
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

   // Einzelne Eigenschaften zu u8 umwandeln und auf richtige Position shiften
   
   0 // Platzhalter, entfernen und durch sinnvollen Rueckgabewert ersetzen 
}









// * * * * Eigene Helperfunktionen * * * * //
/**
 Description: Zerlegt einen 16 Bit Integer in ein 2 Byte Tupel
            ret.0 ist erstes Byte (smallest) ret.1 zweites Byte
*/
pub fn get_bytes (num: u16) -> (u8, u8){
   return( num as u8, (num >> 8) as u8);
}

/**
 Description: Zerlegt einen 16 Bit Integer in ein 2 Byte Tupel
            ret.0 ist erstes Byte (smallest) ret.1 zweites Byte
*/
pub fn clue_bytes (low: u8, high: u8) -> u16{
   // Beide Bytes zu u16 Konvertieren
   let low_big: u16 = low as u16;
   // 8 hits in das high-Byte shiften
   let high_big: u16 = (high as u16) << 8;

   // Beide Bytes verodern
   return high_big | low_big   
}


