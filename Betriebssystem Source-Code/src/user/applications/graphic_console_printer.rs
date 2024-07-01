use alloc::string::{self, String};
use spin::Mutex;

use crate::{
    consts,
    devices::vga::{self, draw_pixel},
};

// Possition des Cursors
static CURSOR: Mutex<(u32, u32)> = Mutex::new((0, 0));
static PRINTER: Mutex<bool> = Mutex::new(false);

const VGA_ROWS: u32 = consts::SCREEN_HEIGHT / 10;
const VGA_COLUMNS: u32 = consts::SCREEN_WIDTH / 10;

const MAIN_COLOR: u32 = vga::rgb_24(0, 255, 0);
const BG_COLOR: u32 = vga::rgb_24(0, 0, 0);

// Position setzen
fn set_pos(x: u32, y: u32) {
    // Ist der Cursor im Scope des Bildschirms
    if x > VGA_COLUMNS {
        return;
    }
    if y > VGA_ROWS {
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

    // Hintergrund einfärben
    for dx in 0..10 {
        for dy in 0..10 {
            draw_pixel(cursor.0 * 10 + dx, cursor.1 * 10 + dy, BG_COLOR)
        }
    }

    // normal Ausgeben
    vga::draw_string(
        cursor.0 * 10,
        cursor.1 * 10,
        MAIN_COLOR,
        String::from(b).as_str(),
    );

    // Curser normal eins weiter setzten
    set_pos(cursor.0 + 1, cursor.1);

    // Line-Wrap wenn Zeile voll
    if cursor.0 >= VGA_COLUMNS {
        set_pos(0, cursor.1 + 1);
    }

    // Lock wieder freigeben
    drop(printlock);
}

// Ganzen String Ausgeben
pub fn print_string(string: &str) {
    for c in string.chars() {
        print_char(c);
    }
}

// Bildschirm Clearen
pub fn clear_screen() {
    // Kompletten Bildschirm mit Leerzeichen füllen
    // Für alle Zeilen
    for y in 0..consts::SCREEN_HEIGHT {
        // Jedes Zeichen pro Spalte
        for x in 0..consts::SCREEN_WIDTH {
            // Leerzeichen schreiben
            vga::draw_pixel(x, y, BG_COLOR);
        }
    }

    // Cursor wieder an Anfang setzten
    set_pos(0, 0);
}
