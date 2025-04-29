use crate::devices::keyboard;

const KEY_LF: u8 = 10;
const KEY_CR: u8 = 13;

// Wartet bis neue Tastatureingabe kommt
pub fn getchar() -> u8 {
    let mut k: u8;

    loop {
        k = keyboard::get_lastkey();
        if k != 0 {
            break;
        }
    }
    k
}
/* Alte Variante. Funktioniert nicht mit neuen Threads

use crate::{devices::keyboard, user::applications::keyboard_handler};

pub fn getch() -> u8 {
    let mut k: u8;

    loop {
        k = keyboard::get_lastkey();
        if k != 0 {
            break;
        }
    }
    return k;
}

pub fn wait_for_return() {
    //kprintln!("Called wait_for_return");
    loop {
        let wanted_char: u8 = 0xd;

        let ch = keyboard::get_lastkey();

        if ch != 0 {
            keyboard_handler::handle_keystroke(ch);
        }

        if ch == wanted_char {
            break;
        }
    }
    //kprintln!("Ended wait_for_return");
}

*/
