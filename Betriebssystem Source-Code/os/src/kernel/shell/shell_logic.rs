use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{ops::Deref, ptr::null, sync::atomic::AtomicBool};

use spin::Mutex;
use usrlib::kernel::shell::{
    command_parser,
    command_parser::{parse_command, EnvPutStatus},
    ENVIRONMENT_COMMAND,
};

use crate::{
    boot::appregion::AppRegion,
    consts,
    devices::{graphical::graphic_console_printer, keyboard},
    kernel::threads::scheduler,
};

// Gibt an, ob die Kommandozeile schon aktiviert ist
static KEYBOARD_ENABLED: AtomicBool = AtomicBool::new(false);

// Speichern der getippten Befehle
//(String der eingegebenen Chars, Index wie viele Zeichen der Command hat)
static COMMAND_BUFFER: Mutex<(String, u32)> = Mutex::new((String::new(), 0));

static APPS: Mutex<Vec<AppRegion>> = Mutex::new(Vec::new());

static ACTIVE: AtomicBool = AtomicBool::new(false);

// === Behandelt je nach Zeichen, was gemacht werden soll
pub fn handle_keystroke(code: u8) -> bool {
    // Ist das Keyboard schon aktiviert
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
        _ => {
            // normale Zeichen
            graphic_console_printer::print_char(code as char);
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
    KEYBOARD_ENABLED.store(true, core::sync::atomic::Ordering::SeqCst);

    // Shell aktivieren
    activate_shell();
}

pub fn get_active_status() -> bool {
    return ACTIVE.load(core::sync::atomic::Ordering::SeqCst);
}

pub fn activate_shell() {
    // Shell aktivieren
    ACTIVE.store(true, core::sync::atomic::Ordering::SeqCst);

    // Buffer räumen
    reset_command();
}

pub fn deactivate_shell() {
    ACTIVE.store(false, core::sync::atomic::Ordering::SeqCst);
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
    let command_buffer = COMMAND_BUFFER.lock();

    // Command auslesen
    let command = command_buffer.0.clone();

    // Lock Freigeben
    drop(command_buffer);

    // Buffer Resetten
    reset_command();

    // Command zurückgeben
    return command;
}

fn backspace_command() {
    // Lock holen
    let mut command_buffer = COMMAND_BUFFER.lock();

    // Lehren befehl abfangen
    if command_buffer.1 < 1 {
        // Wir haben nix zu löschen
        drop(command_buffer);
        return;
    }

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
    command_buffer.0 = String::new();

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

fn load_app_by_name(name: &str) -> Option<AppRegion> {
    let apps = APPS.lock(); // Sperren des Mutex

    // Zielname im erwarteten Format bauen
    let expected_name = format!("./{}.bin", name);

    // Suchen nach passender App
    apps.iter()
        .find(|app| app.file_name == expected_name)
        .cloned()
}

// = = Verarbeitet die Eingabe und startet ggf. die Apps = = //
pub fn print_all_apps() {
    let app_names: Vec<String> = return_loaded_apps();
    for name in app_names.iter() {
        graphic_console_printer::print_string("   - ");
        graphic_console_printer::print_string(name.as_str());
        graphic_console_printer::print_char('\n');
    }
}

// = = Parsed die Eingabe bei Enter und startet ggf. die Apps = = //
fn handle_enter() -> bool {
    // eingelesener Befehl ausgeben
    let command: String = read_command();
    kprintln!("Der eingelesene Befehl: {}", command);

    // Gibt es überhaupt einen Befehl?
    if command.len() < 1 {
        graphic_console_printer::print_char('\n');
        kprintln!("Lehrer befehl");
        return false;
    }

    // = = = Befehl parsen = = = //
    // Ist es das erstellen/updaten einer Variable
    let environment_status = command_parser::check_and_update_env_command(command.clone());
    match environment_status {
        // Standartfall: Es wird einfach normal weiter gemacht
        EnvPutStatus::NotRightCommand => {}

        // Fälle in denen etwas geändert wurde
        EnvPutStatus::Updated => {
            kprintln!("Eine vorhandene Environment-Variable wurde geupdated");
            graphic_console_printer::print_char('\n');

            return false;
        }
        EnvPutStatus::Inserted => {
            kprintln!("Eine neue Environment-Variable wurde eingefügt");
            graphic_console_printer::print_char('\n');
            return false;
        }
        EnvPutStatus::Deleted => {
            kprintln!("Eine Environment-Variable wurde entfernt");
            graphic_console_printer::print_char('\n');
            return false;
        }

        // Fehlerfälle
        EnvPutStatus::NotEnoughArguments => {
            kprintln!("put befehl nicht richtig Verwendet");
            vprintln!("\n\"{}\" ist keine valide Syntax", command);
            vprintln!("lege neue Environment Variable wie folgt an:");
            vprintln!(
                "\t{} <name_der_variable> <inhalt_der_variable>",
                ENVIRONMENT_COMMAND
            );
            return false;
        }

        _ => {
            kprintln!("Fehler in Environment-Variables");
            return true;
        }
    }

    // Läd die Environment Variablen und den Namen separat
    let parsed_command: (String, Vec<String>) = parse_command(command);

    // = = = Befehl matchen = = = //
    // Erstmal neue Zeile für den Befehl
    graphic_console_printer::print_char('\n');

    // App Laden
    let loaded_app = load_app_by_name(parsed_command.0.as_str());

    if loaded_app.is_none() {
        // Befehl existiert nicht
        graphic_console_printer::print_string("Der eingegebene Befehl \"");
        graphic_console_printer::print_string(parsed_command.0.as_str());
        graphic_console_printer::print_string("\" existiert leider nicht :(\n");
        return false;
    }

    // Wenn die App gefunden wurde, starte sie jetzt
    scheduler::spawn_app(loaded_app.unwrap(), parsed_command.1);
    return false;
}
