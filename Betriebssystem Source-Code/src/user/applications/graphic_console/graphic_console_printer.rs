use core::sync::atomic::AtomicU32;

use alloc::string::{self, String};
use spin::Mutex;

use crate::{
    consts,
    devices::vga::{self, draw_pixel},
};

// Possition des Cursors
static CURSOR: Mutex<(u32, u32)> = Mutex::new((0, 0));
static PRINTER: Mutex<bool> = Mutex::new(false);

//const VGA_ROWS: u32 = consts::SCREEN_HEIGHT / 10;
//const VGA_COLUMNS: u32 = consts::SCREEN_WIDTH / 10;

const MAIN_COLOR: u32 = vga::rgb_24(0, 255, 0);
const BG_MAIN_COLOR: u32 = vga::rgb_24(0, 0, 0);
static FONT_COLOR: AtomicU32 = AtomicU32::new(MAIN_COLOR);
static BG_COLOR: AtomicU32 = AtomicU32::new(BG_MAIN_COLOR);

// Position setzen
fn set_pos(x: u32, y: u32) {
    // Ist der Cursor im Scope des Bildschirms
    // VGA Columns
    if x >= vga::get_res().0 / 10 {
        return;
    }
    // VGA Rows
    if y >= vga::get_res().1 / 10 {
        return;
    }

    // Lock holen
    let mut cursorlock = CURSOR.lock();

    // Positionsoffset setzen
    cursorlock.0 = x;
    cursorlock.1 = y;

    // Lock freigeben
    drop(cursorlock);
}

// Position lesen
fn get_pos() -> (u32, u32) {
    // Lock holen
    let cursorlock = CURSOR.lock();

    // Positionsoffset setzen
    let x: u32 = cursorlock.0;
    let y: u32 = cursorlock.1;

    // Lock freigeben
    drop(cursorlock);

    return (x, y);
}

// Einzelnen Char schreiben
pub fn print_char(b: char) {
    // Muss man vielleicht hochscrollen?
    //scroll_with_check();

    // Lock zum zeichnen
    let printlock = PRINTER.lock();

    // Possition des Cursers holen
    let cursor: (u32, u32) = get_pos();

    //kprintln!("Ausgegebener Character: {}", b);

    // An diese Stelle das Byte Printen
    // Prüfen ob es eine neue Zeile ist
    if b == '\n' {
        set_pos(0, cursor.1 + 1);
        //scroll_with_check();
        return;
    }

    // Formatierung holen
    //let attribute: u8 = attribute(Color::Black, Color::Green, false);

    //kprintln!("vor dem Draw");

    // Hintergrund einfärben
    for dx in 0..10 {
        for dy in 0..10 {
            draw_pixel(
                cursor.0 * 10 + dx,
                cursor.1 * 10 + dy,
                BG_COLOR.load(core::sync::atomic::Ordering::SeqCst),
            )
        }
    }

    // normal Ausgeben
    vga::draw_byte(
        cursor.0 * 10,
        cursor.1 * 10,
        FONT_COLOR.load(core::sync::atomic::Ordering::SeqCst),
        b as u8,
    );

    // Curser normal eins weiter setzten
    set_pos(cursor.0 + 1, cursor.1);

    // Line-Wrap wenn Zeile voll
    if cursor.0 >= (vga::get_res().0 / 10) - 1 {
        set_pos(0, cursor.1 + 1);
    }

    // Lock wieder freigeben
    drop(printlock);
}

// Ganzen String Ausgeben
pub fn print_string(string: &str) {
    //kprintln!("Printing");
    for c in string.chars() {
        //kprintln!("===");
        print_char(c);
    }
}

// Bildschirm Clearen
pub fn clear_screen() {
    // Kompletten Bildschirm mit Hintergrundfarbe füllen
    // Für alle Zeilen
    for y in 0..vga::get_res().1 {
        // Jedes Zeichen pro Spalte
        for x in 0..vga::get_res().0 {
            // Leerzeichen schreiben
            vga::draw_pixel(x, y, BG_COLOR.load(core::sync::atomic::Ordering::SeqCst));
        }
    }

    // Cursor wieder an Anfang setzten
    set_pos(0, 0);
}

pub fn print_backspace() {
    // Cursor-Possition holen
    let cursor_pos: (u32, u32) = get_pos();

    // Neue Possition berechnen
    let new_pos: (u32, u32);

    // EdgeCase bei Linewrap
    if cursor_pos.0 == 0 {
        new_pos = ((vga::get_res().0 / 10) - 1, cursor_pos.1 - 1);
    } else {
        // Normal Case
        new_pos = (cursor_pos.0 - 1, cursor_pos.1);
    }

    // Position setzten
    set_pos(new_pos.0, new_pos.1);

    // Byte Löschen
    print_char(' ');

    // Nochmal zurück gehen (print_byte geht wieder eins vor)
    set_pos(new_pos.0, new_pos.1);
}

// Farbe der Schrift ändern
pub fn set_font_color(r: u8, g: u8, b: u8) {
    let rgb_value = vga::rgb_24(r, g, b);
    FONT_COLOR.store(rgb_value, core::sync::atomic::Ordering::SeqCst);
}

// Farbe der schrift resetten
pub fn reset_font_color() {
    FONT_COLOR.store(MAIN_COLOR, core::sync::atomic::Ordering::SeqCst);
}

// Farbe des Hintergrunds ändern
pub fn set_bg_color(r: u8, g: u8, b: u8) {
    let rgb_value = vga::rgb_24(r, g, b);
    BG_COLOR.store(rgb_value, core::sync::atomic::Ordering::SeqCst);
}

// Farbe des Hintergrunds resetten
pub fn reset_bg_color() {
    BG_COLOR.store(BG_MAIN_COLOR, core::sync::atomic::Ordering::SeqCst);
}

// = = = Code für bunten hintergrund = = = //

pub fn clear_screen_rainbow() {
    // Kompletten Bildschirm Bunt machen
    draw_rainbow();

    // Cursor wieder an Anfang setzten
    set_pos(0, 0);
}

/**
 Description: Calculate a color value interpolated in one dimensions
*/
fn lin_inter_pol_1d(x: u32, xr: u32, l: u32, r: u32) -> u32 {
    return ((((l >> 16) * (xr - x) + (r >> 16) * x) / xr) << 16)
        | (((((l >> 8) & 0xFF) * (xr - x) + ((r >> 8) & 0xFF) * x) / xr) << 8)
        | (((l & 0xFF) * (xr - x) + (r & 0xFF) * x) / xr);
}

/**
 Description: Calculate a color value interpolated in two dimensions
*/
fn lin_inter_pol_2d(
    x: u32,
    y: u32,
    xres: u32,
    yres: u32,
    lt: u32,
    rt: u32,
    lb: u32,
    rb: u32,
) -> u32 {
    return lin_inter_pol_1d(
        y,
        yres,
        lin_inter_pol_1d(x, xres, lt, rt),
        lin_inter_pol_1d(x, xres, lb, rb),
    );
}

/**
 Description: Draw colours
*/
pub fn draw_rainbow() {
    let (xres, yres) = vga::get_res();

    for y in 0..yres {
        for x in 0..xres {
            let pix = lin_inter_pol_2d(x, y, xres, yres, 0x0000FF, 0x00FF00, 0xFF0000, 0xFFFF00);
            vga::draw_pixel(x, y, pix);
        }
    }
}
