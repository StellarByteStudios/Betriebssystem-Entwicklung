use usrlib::time::rtc_date_time::RtcDateTime;

use crate::devices::{
    graphical::vga,
    pit,
    rtc::{get_current_date, get_current_time},
};

#[no_mangle]
pub extern "C" fn sys_get_systime() -> usize {
    return pit::get_systime() as usize;
}

#[no_mangle]
pub extern "C" fn sys_get_systime_intervall() -> usize {
    return pit::get_systime_intervall() as usize;
}

#[no_mangle]
pub extern "C" fn sys_get_screen_witdh() -> usize {
    return vga::get_res().0 as usize;
}

#[no_mangle]
pub extern "C" fn sys_get_screen_height() -> usize {
    return vga::get_res().1 as usize;
}

#[no_mangle]
pub extern "C" fn sys_get_datetime(buff: usize) {
    let time = get_current_time();
    let date = get_current_date();
    unsafe {
        let date_time = buff as *mut RtcDateTime;
        (*date_time).date = date;
        (*date_time).time = time;
    }
}
