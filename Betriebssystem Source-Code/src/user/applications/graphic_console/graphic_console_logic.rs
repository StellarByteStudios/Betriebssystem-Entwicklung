use core::sync::atomic::AtomicBool;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use spin::Mutex; // String-Klasse für einfacheres Matching der Commands

// Funktioniert nicht mit neuen Threads
//use crate::{consts, devices::keyboard, user::applications::graphic_console::gc_programms};
use crate::{consts, devices::keyboard};

use super::graphic_console_printer;

/*
// Macro um die Programme dynamisch zu registrieren

// Liste aller Programme
const COMMANDS: &'static [&'static str] = &[
    "scream",
    "greet",
    "clear",
    "echo",
    "play",
    "mandelbrot",
    "testprint",
    "sysinfo",
    "help",
    "threads",
    "kill",
    "silence",
    "cat",
]; */

// Gibt an, ob die Kommandozeile schon aktiviert ist
static KEYBOARD_ENABLED: AtomicBool = AtomicBool::new(false);

// Speichern der getippten Befehle
//(String der eingegebenen Chars, Index wie viele Zeichen der Command hat)
static COMMAND_BUFFER: Mutex<(String, u32)> = Mutex::new((String::new(), 0));

// === Behandelt je nach Zeichen, was gemacht werden soll
pub fn handle_keystroke(code: u8) -> bool {
    // Sind wir überhaupt schon ready
    if !KEYBOARD_ENABLED.load(core::sync::atomic::Ordering::SeqCst) {
        return false;
    }

    let mut error_code: bool = false;

    match code {
        0xd => error_code = handle_enter(), // Newline
        0x8 => {
            // Backspace
            graphic_console_printer::print_backspace();
            backspace_command()
        }
        //0x0 => print!("    "),                           // Tab

        // Pfeiltasten funktionieren nicht. Daher erstmal nummernblock zum ausprobieren
        //0x38 => move_cursor(Direction::Up),
        //0x32 => move_cursor(Direction::Down),
        //0x34 => move_cursor(Direction::Left),
        //0x36 => move_cursor(Direction::Right),
        _ => {
            // normale Zeichen
            graphic_console_printer::print_char(code as char);
            save_command(code as char)
        }
    }

    return error_code;
}

// === Initialisierung
pub fn init_keyboardhandler() {
    // Lock holen
    let mut command_buffer = COMMAND_BUFFER.lock();

    // Buffer anlegen
    command_buffer.0 = String::new(); //String::with_capacity((consts::SCREEN_WIDTH / 10 + 2) as usize);

    // Counter setzen
    command_buffer.1 = 0;

    // lock freigeben
    drop(command_buffer);

    // Commands aktivieren
    KEYBOARD_ENABLED.store(true, core::sync::atomic::Ordering::SeqCst)
}

fn save_command(c: char) {
    // Lock holen
    let mut command_buffer = COMMAND_BUFFER.lock();

    // Char abspeichern
    command_buffer.0.push(c);

    // Counter eins hochzählen
    command_buffer.1 += 1;

    // lock freigeben
    drop(command_buffer);
}

fn read_command() -> String {
    // Lock holen
    let mut command_buffer = COMMAND_BUFFER.lock();

    // Command auslesen
    let command = command_buffer.0.clone();

    // Buffer löschen
    command_buffer.0 = String::new(); //String::with_capacity((consts::SCREEN_WIDTH / 10 + 2) as usize);

    // Counter wieder auf 0 setzten
    command_buffer.1 = 0;

    // lock freigeben
    drop(command_buffer);

    // Command zurückgeben
    return command;
}

fn backspace_command() {
    // Lock holen
    let mut command_buffer = COMMAND_BUFFER.lock();

    // Char löschen
    command_buffer.0.pop();

    // Counter eins hochzählen
    command_buffer.1 -= 1;

    // lock freigeben
    drop(command_buffer);
}

fn reset_command() {
    // Lock holen
    let mut command_buffer = COMMAND_BUFFER.lock();

    // Buffer löschen
    command_buffer.0 = String::new(); //String::with_capacity((consts::SCREEN_WIDTH / 10 + 2) as usize);

    // Counter wieder auf 0 setzten
    command_buffer.1 = 0;

    // lock freigeben
    drop(command_buffer);
}

fn handle_enter() -> bool {
    //graphic_console_printer::print_char('\n');

    // eingelesener Befehl ausgeben
    let command: String = read_command();

    kprintln!("Der eingelesene Befehl: {}", command);

    // Gibt es überhaupt einen Befehl?
    if command.len() < 1 {
        graphic_console_printer::print_char('\n');
        kprintln!("Lehrer befehl");
        return false;
    }

    // = = = Befehl aufspalten für ggf argumente = = = //

    let command_array: Vec<String> = command.split(" ").map(str::to_string).collect();

    // Hat das aufspalten funktioniert? Ansonsten abbruch
    if command_array.get(0).is_none() {
        return false;
    }

    // = = = Befehl matchen = = = //
    // Konsole beenden
    /*
    if command_array.get(0).unwrap() == "exit" || command_array.get(0).unwrap() == "quit" {
        graphic_console_printer::print_char('\n');
        kprintln!("System wird beendet");
        return true;
    } */

    // Erstmal neue Zeile für den Befehl
    graphic_console_printer::print_char('\n');
    /*
    // Gibt es das Programm überhaupt?
    let programm_name = command_array.get(0).unwrap();
    if !COMMANDS.contains(&programm_name.as_str()) {
        vprintln!("Command \"{}\" not avaiable", command_array.get(0).unwrap());
    }

    // Programm ausführen

    call_init!("scream"); */

    /* Altes matching
    // Matching auf andere Befehle
    // Funktioniert nicht mit neuen Threads
    match command_array.get(0).unwrap().as_str() {
        "animation" => gc_programms::animation::init(command_array),
        "scream" => gc_programms::scream::init(), // Lustige Textausgabe
        "greet" => gc_programms::greet::init(),   // Andere lustige Textausgabe
        "clear" => gc_programms::clear::init(command_array), // Bildschirm freiräumen
        "echo" => gc_programms::echo::init(command_array), // Argumente wiedergeben
        "play" => gc_programms::play::init(command_array), // Song abspielen
        "mandelbrot" => gc_programms::mandelbrot::init(command_array), // Mandelbrot malen
        "testprint" => gc_programms::macrotest::init(),
        "sysinfo" => gc_programms::sysinfo::init(),
        "help" => gc_programms::help::init(command_array),
        "kill" => gc_programms::kill::init(command_array),
        "silence" => gc_programms::silence::init(),
        "cat" => gc_programms::cat::init(),
        "doge" => gc_programms::doge::init(),
        "threads" => gc_programms::threads::init(),
        "meminfo" => gc_programms::meminfo::init(),
        "scrollup" => gc_programms::scrollup::init(),
        "clock" => gc_programms::clock::init(command_array),
        "fontchange" => gc_programms::fontchange::init(command_array),
        _ => vprintln!(
            "Command \"{}\" is not supportet",
            command_array.get(0).unwrap().as_str()
        ), // Falscher Befehl
    }
     */
    // neue Zeile nach Befehl
    //graphic_console_printer::print_char('\n');

    // Normaler verlauf
    return false;
}
