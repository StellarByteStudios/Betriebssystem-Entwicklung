use usrlib::kernel::syscall::keyboard::KeyEvent;
use crate::devices::keyboard::{get_last_keyevent, get_lastkey};
use crate::kernel::shell::shell_logic;

#[no_mangle]
pub extern "C" fn sys_getlastkey() -> usize {
    let key: KeyEvent = get_last_keyevent();
    return key.into();
}

pub extern "C" fn sys_activate_shell() {
    shell_logic::activate_shell();
}

pub extern "C" fn sys_deactivate_shell() {
    shell_logic::deactivate_shell();
}

