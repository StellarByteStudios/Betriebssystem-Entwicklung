use crate::devices::{graphical::vga, pit};

#[no_mangle]
pub extern "C" fn sys_get_systime() -> u64 {
    return pit::get_systime();
}

#[no_mangle]
pub extern "C" fn sys_get_screen_witdh() -> u64 {
    return vga::get_res().0 as u64;
}
