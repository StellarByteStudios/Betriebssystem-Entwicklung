use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{ops::Deref, ptr::null, sync::atomic::AtomicBool};

use spin::Mutex;

use crate::{boot::appregion::AppRegion, consts, devices::keyboard, kernel::shell::shell_printing};

// Gibt an, ob die Kommandozeile schon aktiviert ist
static KEYBOARD_ENABLED: AtomicBool = AtomicBool::new(false);

// Speichern der getippten Befehle
//(String der eingegebenen Chars, Index wie viele Zeichen der Command hat)
static COMMAND_BUFFER: Mutex<(String, u32)> = Mutex::new((String::new(), 0));

static APPS: Mutex<Vec<AppRegion>> = Mutex::new(Vec::new());

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
            shell_printing::print_backspace();
            backspace_command()
        }
        _ => {
            // normale Zeichen
            shell_printing::print_char(code as char);
            save_command(code as char)
        }
    }

    return error_code;
}

// === Initialisierung
pub fn init_keyboardhandler(apps: Vec<AppRegion>) {
    // Command Buffer initialisieren
    reset_command();

    // Apps abspeichern
    let mut mutable_apps_vector = apps.clone();
    let mut apps_mutex = APPS.lock();
    loop {
        let app = mutable_apps_vector.pop();
        if app.is_none() {
            break;
        }
        apps_mutex.push(app.unwrap());
    }

    drop(apps_mutex);

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

fn return_loaded_apps() -> Vec<String> {
    // Mutex Holen
    let apps = APPS.lock();
    // Neuen Vektor erstellen
    let mut app_names: Vec<String> = Vec::new();
    // Alle Appnamen durchgehen
    for app in apps.iter() {
        // Name laden
        let name = &app.file_name;
        // Auseinander schneiden und gucken ob da was rauskommt
        if let Some(stripped) = name.strip_prefix("./").and_then(|s| s.strip_suffix(".bin")) {
            // Wenn ja, den beschnitten Name abspeichern
            app_names.push(stripped.to_string());
        } else {
            // Wenn das Format nicht passt, einfach originaler Name
            app_names.push(name.clone());
        }
    }
    // Vektor zurückgeben
    return app_names;
}

fn handle_enter() -> bool {
    // eingelesener Befehl ausgeben
    let command: String = read_command();
    kprintln!("Der eingelesene Befehl: {}", command);

    // Gibt es überhaupt einen Befehl?
    if command.len() < 1 {
        shell_printing::print_char('\n');
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
    shell_printing::print_char('\n');

    // TODO: Richtige Befehle rausholen
    let app_names: Vec<String> = return_loaded_apps();
    shell_printing::print_string("Geladene Apps:\n");
    for name in app_names.iter() {
        shell_printing::print_string(name.as_str());
        shell_printing::print_char('\n');
    }

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
