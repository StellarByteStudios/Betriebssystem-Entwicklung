#[no_mangle]
pub extern "C" fn sys_getlastkey() -> usize {
    let key: u8 = getch();
    return key as usize;
}

pub extern "C" fn sys_activate_shell() {
    shell_logic::activate_shell();
}

pub extern "C" fn sys_deactivate_shell() {
    shell_logic::deactivate_shell();
}

// Inportiert aus der alten Library
use crate::{devices::keyboard, kernel::shell::shell_logic};

const KEY_LF: u8 = 10;
const KEY_CR: u8 = 13;

pub fn getch() -> u8 {
    let mut k: u8;

    loop {
        k = keyboard::get_lastkey();
        if k != 0 {
            break;
        }
    }
    k
}

pub fn wait_for_return() {
    loop {
        if keyboard::get_lastkey() == KEY_LF {
            break;
        }
    }
}
