use alloc::string::String; // String-Klasse für einfacheres Matching der Commands

use crate::devices::cga;
use crate::devices::cga::print_byte;
// shortcut for cga
//use crate::devices::cga::print_backspace;
//use crate::devices::cga::print_byte;
use crate::devices::cga_print; // used to import code needed by println!
use crate::devices::key;
use crate::devices::keyboard;
use crate::kernel::cpu; // shortcut for keyboard

use crate::mylib::input;

// Enum für Pfeiltasten
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

static mut COMMANDLINE_ENABLED: bool = false;

// ===== ggf noch einen Buffer der Asciis einbauen für befehle
pub fn run() {
    // Terminalfarbe wählen
    cga::set_attribute(cga::Color::Black, cga::Color::Green, false);

    // Komandozeile aktivieren
    unsafe {
        COMMANDLINE_ENABLED = true;
    }

    kprintln!("In dem keyboard_handler");

    // ================= NOTIZ: Pfeiltasten werden nicht richtig übersetzt ================= //
    //kprint!("Die gedrückten Tasten sind: ");

    loop {
        let key = input::getch();

        handle_keystroke(key);
    }
    /*
    // Dauerhaft die Tastatur einlesen
    loop {
        // Warten bis ein Valid Key da ist
        let mut key: key::Key;
        loop {
            key = keyboard::key_hit();

            if key.valid() {
                break;
            }
        }

        // Das Symbol auslesen
        let ascii_byte: u8 = key.get_ascii();

        //kprint!("{:#2x} ", ascii_byte);

        // Handler aufrufen
        let error_code: bool = handle_keystroke(ascii_byte);

        // Im fehlerfall abbrechen
        if error_code {
            cpu::halt();
            return;
        }
    }*/
}

// === Behandelt je nach Zeichen, was gemacht werden soll
pub fn handle_keystroke(code: u8) -> bool {
    let mut error_code: bool = false;

    match code {
        0xd => error_code = handle_enter(), // Newline
        0x8 => cga::print_backspace(),      // Backspace
        //0x0 => print!("    "),              // Tab

        // Pfeiltasten funktionieren nicht. Daher erstmal nummernblock zum ausprobieren
        0x38 => move_cursor(Direction::Up),
        0x32 => move_cursor(Direction::Down),
        0x34 => move_cursor(Direction::Left),
        0x36 => move_cursor(Direction::Right),

        _ => print_byte(code), // normale Zeichen
    }

    return error_code;
}

fn move_cursor(dir: Direction) {
    // Cursorpossition bestimmen
    let pos: (u32, u32) = cga::getpos();

    // 4 Fälle für Cursorbewegungen
    match dir {
        // Hoch
        Direction::Up => {
            // Testen ob noch Platz ist
            if pos.1 <= 0 {
                return;
            }
            cga::setpos(pos.0, pos.1 - 1);
            return;
        }
        // Runter
        Direction::Down => {
            // Testen ob noch Platz ist
            if pos.1 >= cga::get_screen_height() - 1 {
                return;
            }
            cga::setpos(pos.0, pos.1 + 1);
            return;
        }
        // Links
        Direction::Left => {
            // Testen ob noch Platz ist
            if pos.0 <= 0 {
                return;
            }
            cga::setpos(pos.0 - 1, pos.1);
            return;
        }
        // Rechts
        Direction::Right => {
            // Testen ob noch Platz ist
            if pos.0 >= cga::get_screen_width() - 1 {
                return;
            }
            cga::setpos(pos.0 + 1, pos.1);
            return;
        }
    }
}

fn handle_enter() -> bool {
    cga::print_byte('\n' as u8);
    return false;

    /*
    // ============= Geht noch nicht, weil es noch keinen Heap gibt... ===================== //
    unsafe{
        if !COMMANDLINE_ENABLED {
            cga::print_byte('\n' as u8);
            return false;
        }
    }

    // Bildschirmbreite speichern
    const SCREEN_WIDTH: u32 = cga::get_screen_width();

    // Buffer erzeugen
    let mut command_buffer: [char; SCREEN_WIDTH as usize] =  [0 as char; SCREEN_WIDTH as usize];

    kprintln!("Erfolgreich Buffer angelegt");

    // Aktuelle Zeile in Buffer einlesen
    let command_size: u32 = cga::read_line_into_buffer(&mut command_buffer);

    if command_size < 1 {
        cga::print_byte('\n' as u8);
        return false;
    }

    // Zu string für Matching umbauen
    let command_string: String = command_buffer.iter().collect();

    // = = = Befehl matchen
    // Konsole beenden
    if command_string == "exit" || command_string == "quit"{
        return true;
    }

    // Matching auf andere Befehle
    match command_string.as_str(){
        "scream" => println!("AAAAAAAHHHHHHH!!!!!"),                                        // Lustige Textausgabe
        "greet" => println!("I Greet you fellow User. I hope you have fun with Byte OS"),   // Andere lustige Textausgabe
        "clear" => command_clear(),                                                         // Bildschirm freiräumen
        _ => print!("")     //cga::print_byte(0xa as u8)                                    // Einfach nur Newline ausgeben
    }

    // Normaler verlauf
    return false;

     */
}

fn command_clear() {
    cga::clear();
}
