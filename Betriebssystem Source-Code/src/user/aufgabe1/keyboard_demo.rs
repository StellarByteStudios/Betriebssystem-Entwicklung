use crate::devices::cga; // shortcut for cga
use crate::devices::cga::print_backspace;
use crate::devices::cga::print_byte;
use crate::devices::cga_print; // used to import code needed by println!
use crate::devices::key;
use crate::devices::keyboard; // shortcut for keyboard

// ===== ggf noch einen Buffer der Asciis einbauen für befehle
pub fn run() {
    // Terminalfarbe wählen
    cga::set_attribute(cga::Color::Black, cga::Color::Green, false);

    //kprint!("Die gedrückten Tasten sind: ");

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

        // Sonderfälle für bestimmte Tasten
        // Weiß noch nicht, ob ich das hier so gut finde
        match ascii_byte {
            0xd => print_byte(0xa as u8), // Newline
            0x8 => print_backspace(),     // Backspace
            _ => print_byte(ascii_byte),  // normale Zeichen
        }
    }
}
