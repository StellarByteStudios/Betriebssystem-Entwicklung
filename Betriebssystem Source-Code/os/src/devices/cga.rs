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

use crate::kernel::cpu;

use super::{cga_print::print, kprint};

// make type comparable, printable and enable copy semantics
#[allow(dead_code)] // avoid warnings for unused colors
#[repr(u8)] // store each enum variant as an u8
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Pink = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightPink = 13,
    Yellow = 14,
    White = 15,
}

const CGA_STD_ATTR: u8 = (Color::Black as u8) << 4 | (Color::Green as u8);

const CGA_BASE_ADDR: u32 = 0xb8000;
const CGA_ROWS: u32 = 25;
const CGA_COLUMNS: u32 = 80;

const CGA_INDEX_PORT: u16 = 0x3d4; // select register
const CGA_DATA_PORT: u16 = 0x3d5; // read/write register
const CGA_HIGH_BYTE_CMD: u8 = 14; // cursor high byte
const CGA_LOW_BYTE_CMD: u8 = 15; // cursor high byte

// Attribut mit welchem man Dynamisch die Farben auswählen kann
static mut CGA_DYN_ATTR: u8 = CGA_STD_ATTR;

/**
 Description: Clear text screen
*/
pub fn clear() {
    // Cursor an Anfang setzten
    setpos(0, 0);

    // Kompletten Bildschirm mit Leerzeichen füllen
    // Für alle Zeilen
    for row in 0..CGA_ROWS {
        // Jedes Zeichen pro Spalte
        for column in 0..CGA_COLUMNS {
            // Leerzeichen schreiben
            print_byte('\0' as u8);
        }
    }

    // Cursor wieder an Anfang setzten
    setpos(0, 0);
}

/**
 Description: Display the `character` at the given position `x`,`y` with attribute `attrib`
*/
pub fn show(x: u32, y: u32, character: char, attrib: u8) {
    let pos: u32;

    if x > CGA_COLUMNS || y > CGA_ROWS {
        return;
    }

    pos = (y * CGA_COLUMNS + x) * 2;

    unsafe {
        *((CGA_BASE_ADDR + pos) as *mut u8) = character as u8;
        *((CGA_BASE_ADDR + pos + 1) as *mut u8) = attrib;
    }
}

/**
 Description: Return cursor position `x`,`y`
*/
pub fn getpos() -> (u32, u32) {
    //let ie = cpu::disable_int_nested();
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

    //cpu::enable_int_nested(ie);
    return (x, y);
}

/**
 Description: Set cursor position `x`,`y`
*/
pub fn setpos(x: u32, y: u32) {
    // Ist der Cursor im Scope des Bildschirms
    if x > CGA_COLUMNS {
        return;
    }
    if y > CGA_ROWS {
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
pub fn print_byte(b: u8) {
    // Muss man vielleicht hochscrollen?
    scroll_with_check();

    // Possition des Cursers holen
    let pos: (u32, u32) = getpos();

    // An diese Stelle das Byte Printen
    // Prüfen ob es eine neue Zeile ist
    if b == '\n' as u8 {
        setpos(0, pos.1 + 1);
        scroll_with_check();
        return;
    }

    // Ansonsten normal Ausgeben
    // unsafe, wegen dem Dynamischen Farb Attribut
    unsafe {
        show(pos.0, pos.1, b as char, CGA_DYN_ATTR);
    }

    // Curser normal eins weiter setzten
    setpos(pos.0 + 1, pos.1);

    // Line-Wrap wenn Zeile voll
    if pos.0 >= CGA_COLUMNS {
        setpos(0, pos.1 + 1);
    }
}

/**
 Description: Scroll text lines by one to the top.
*/
pub fn scrollup() {
    // Cursor an erste Possition stellen
    setpos(0, 0);

    // Für alle Zeilen die darunterliegenden Werte Kopieren
    for row in 0..CGA_ROWS {
        // Einzelne Zeichen kopieren
        for column in 0..CGA_COLUMNS {
            // Zeichen untendrunter holen
            setpos(column, row + 1);
            let symbol_tupel: (u8, u8) = get_symbol_of_screen();

            // Zeichen eins drüber schreiben
            show(column, row, symbol_tupel.0 as char, symbol_tupel.1);
        }
    }

    // Letzte Zeile ausschwärzen
    for i in 0..CGA_COLUMNS - 1 {
        setpos(i, CGA_ROWS - 1);
        print_byte(' ' as u8);
    }

    // Cursor wieder eine Zeile nach oben setzen
    setpos(0, CGA_ROWS - 1);
}

// = = = NOTE: Blinkenbit verstellt nur die Backgroundfarbe = = = //
// ==> Bit 0-3 Vordergrund. Bit 4-7 Hintergrund
/**
 Description: Helper function returning an attribute byte for the given
              parameters `bg`, `fg`, and `blink`
*/
pub fn attribute(bg: Color, fg: Color, blink: bool) -> u8 {
    let fg_byte: u8 = fg as u8;
    let bg_byte: u8 = (bg as u8) << 4;
    let blink_byte: u8 = (blink as u8) << 7;

    return bg_byte | fg_byte | blink_byte;
}

// * * * * Eigene Helperfunktionen * * * * //
/**
 Description: Zerlegt einen 16 Bit Integer in ein 2 Byte Tupel
            ret.0 ist erstes Byte (smallest) ret.1 zweites Byte
*/
pub fn get_bytes(num: u16) -> (u8, u8) {
    return (num as u8, (num >> 8) as u8);
}

/**
 Description: Zerlegt einen 16 Bit Integer in ein 2 Byte Tupel
            ret.0 ist erstes Byte (smallest) ret.1 zweites Byte
*/
pub fn clue_bytes(low: u8, high: u8) -> u16 {
    // Beide Bytes zu u16 Konvertieren
    let low_big: u16 = low as u16;
    // 8 hits in das high-Byte shiften
    let high_big: u16 = (high as u16) << 8;

    // Beide Bytes verodern
    return high_big | low_big;
}

/**
Description: Holt ein Zeichen auf dem Bildschirm
   ret.0 Zeichen; ret.1 Attribut
*/
pub fn get_symbol_of_screen() -> (u8, u8) {
    // Possitionnsoffset berechen
    let cursor_pos: (u32, u32) = getpos();
    let pos_offset = (cursor_pos.1 * CGA_COLUMNS + cursor_pos.0) * 2;

    // Daten aus Speicher lesen
    let character: u8;
    let attr: u8;

    unsafe {
        character = *((CGA_BASE_ADDR + pos_offset) as *mut u8);
        attr = *((CGA_BASE_ADDR + pos_offset + 1) as *mut u8);
    }

    return (character, attr);
}

/**
Description: Checkt ob gescrollt werden muss und macht das dann ggf
*/
pub fn scroll_with_check() {
    // Cursorpossition holen
    let cursor_pos: (u32, u32) = getpos();
    // Scoll wenn Bildschirm voll
    if cursor_pos.1 >= CGA_ROWS {
        scrollup();
    }
}

/**
Description: Löscht letztes Zeichen
Note: Springt einfach in der Zeile nach oben, auch wenn diese am Ende noch leer ist
*/
pub fn print_backspace() {
    // Cursor-Possition holen
    let cursor_pos: (u32, u32) = getpos();

    // Neue Possition berechnen
    let new_pos: (u32, u32);

    // EdgeCase bei Linewrap
    if cursor_pos.0 == 0 {
        new_pos = (CGA_COLUMNS - 1, cursor_pos.1 - 1);
    } else {
        // Normal Case
        new_pos = (cursor_pos.0 - 1, cursor_pos.1);
    }

    // Position setzten
    setpos(new_pos.0, new_pos.1);

    // Byte Löschen
    print_byte('\0' as u8);

    // Nochmal zurück gehen (print_byte geht wieder eins vor)
    setpos(new_pos.0, new_pos.1);
}

/* * == Legt fest, wie zukünftig Symbole auf dem Bildschirm gezeigt werden sollen  == * */
pub fn set_attribute(bg: Color, fg: Color, blink: bool) {
    // Neues Format-Byte zusammenbauen
    let new_attr: u8 = attribute(bg, fg, blink);

    // Neues Format abspeichern
    unsafe {
        CGA_DYN_ATTR = new_attr;
    }
}

/* * == Setzt wieder das Standardformat  == * */
pub fn set_default_attribute() {
    // Standardformat speichern
    unsafe {
        CGA_DYN_ATTR = CGA_STD_ATTR;
    }
}

/* * == Gibt die Breite des Bildschirms zurück  == * */
pub const fn get_screen_width() -> u32 {
    return CGA_COLUMNS;
}

/* * == Gibt die Größe des Bildschirms zurück  == * */
pub const fn get_screen_height() -> u32 {
    return CGA_ROWS;
}

/* * == liest die letzte Zeile in dem Buffer und gibt die Größe zurück  == * */
// Wurde nie benutzt
pub fn read_line_into_buffer(buffer: &mut [char; CGA_COLUMNS as usize]) -> u32 {
    // Ende der Zeile Finden (Leerzeicheichen überspringen)
    // Dabei immer weiter mit dem Cursor nach hinten springen
    loop {
        let symbol: (u8, u8) = get_symbol_of_screen();
        // Wurde etwas anderes als ein Leerzeichen gefunden
        if symbol.0 as char != ' ' {
            break;
        }
        // Cursorpossition holen
        let pos: (u32, u32) = getpos();
        // Aber sind wir vielleicht am Ende der Zeile?
        if pos.0 <= 0 {
            return 0;
        }
        // Ansonsten Cursor eins zurück setzen
        setpos(pos.0 - 1, pos.1);
    }

    // = = =  Cursor ist jetzt beim ersten Zeichen, welches kein Leerzeichen ist

    // Possition des Cursers bestimmen, damit wir rückwärts laufen können
    let command_size = getpos().0;

    // Command einlesen
    // Iteratorvariable
    let mut i: u32 = 0;

    loop {
        // Symbol holen
        let symbol: (u8, u8) = get_symbol_of_screen();

        // Symbol an richtiger Stelle im Buffer abspeichern
        buffer[(command_size - i) as usize] = symbol.0 as char;

        // Iterator weiterschieben
        i += 1;

        // Weiter zurück gehen oder Ende der Zeile erreicht??
        let pos: (u32, u32) = getpos();
        // Aber sind wir vielleicht am Ende der Zeile?
        if pos.0 <= 0 {
            break;
        }
        // Ansonsten Cursor eins zurück setzen
        setpos(pos.0 - 1, pos.1);
    }

    return command_size;
}
