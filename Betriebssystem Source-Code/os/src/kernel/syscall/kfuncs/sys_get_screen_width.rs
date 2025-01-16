use crate::devices::graphical::vga;

#[no_mangle]
pub extern "C" fn sys_get_screen_witdh() -> u64 {
    return vga::get_res().0 as u64;
}