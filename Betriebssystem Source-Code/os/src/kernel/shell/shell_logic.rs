use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{ops::Deref, ptr::null, sync::atomic::AtomicBool};

use spin::Mutex;

use crate::{
    boot::appregion::AppRegion,
    consts,
    devices::keyboard,
    kernel::{threads::scheduler},
};
use crate::devices::graphical::graphic_console_printer;

// Gibt an, ob die Kommandozeile schon aktiviert ist
static KEYBOARD_ENABLED: AtomicBool = AtomicBool::new(false);

// Speichern der getippten Befehle
//(String der eingegebenen Chars, Index wie viele Zeichen der Command hat)
static COMMAND_BUFFER: Mutex<(String, u32)> = Mutex::new((String::new(), 0));

static APPS: Mutex<Vec<AppRegion>> = Mutex::new(Vec::new());

// Liste aller Commands bei denen man die Apps angezeigt bekommt, die geladen sind
const LIST_ALL_COMMANDS: [&str; 3] = ["programms", "app", "apps"];

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

fn load_app_by_name(name: &str) -> Option<AppRegion> {
    let apps = APPS.lock(); // Sperren des Mutex

    // Zielname im erwarteten Format bauen
    let expected_name = format!("./{}.bin", name);

    // Suchen nach passender App
    apps.iter()
        .find(|app| app.file_name == expected_name)
        .cloned()
}
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

    // = = = Befehl aufspalten für ggf argumente = = = //
    let command_array: Vec<String> = command.split(" ").map(str::to_string).collect();

    // Hat das aufspalten funktioniert? Ansonsten abbruch
    if command_array.get(0).is_none() {
        return false;
    }
    // Speichern als Abkürzung
    let main_command = command_array.get(0).unwrap();

    // = = = Befehl matchen = = = //

    // Erstmal neue Zeile für den Befehl
    graphic_console_printer::print_char('\n');

    // Gebe einfach die die Befehle aus.
    if LIST_ALL_COMMANDS.contains(&main_command.as_str()) {
        let app_names: Vec<String> = return_loaded_apps();
        graphic_console_printer::print_string("Geladene Apps:\n");
        for name in app_names.iter() {
            graphic_console_printer::print_string("   - ");
            graphic_console_printer::print_string(name.as_str());
            graphic_console_printer::print_char('\n');
        }
        return false;
    }

    // TODO: Exitbefehl

    // App Laden
    let loaded_app = load_app_by_name(main_command.as_str());

    if loaded_app.is_none() {
        // Befehl existiert nicht
        graphic_console_printer::print_string("Der eingegebene Befehl \"");
        graphic_console_printer::print_string(command_array.get(0).unwrap());
        graphic_console_printer::print_string("\" existiert leider nicht :(\n");
        return false;
    }

    // Wenn die App gefunden wurde, starte sie jetzt
    scheduler::spawn_app(loaded_app.unwrap(), command_array);
    return false;
}
