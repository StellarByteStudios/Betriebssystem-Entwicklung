use usrlib::time::rtc_date_time::RtcDateTime;
use crate::devices::{graphical::vga, pit};
use crate::devices::rtc::{get_current_date, get_current_time};

#[no_mangle]
pub extern "C" fn sys_get_systime() -> u64 {
    return pit::get_systime();
}

#[no_mangle]
pub extern "C" fn sys_get_screen_witdh() -> u64 {
    return vga::get_res().0 as u64;
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