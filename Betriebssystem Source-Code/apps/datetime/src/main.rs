#![no_std]
#![no_main]

extern crate alloc;

use usrlib::gprintln;
use usrlib::time::rtc_date_time::datetime;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    let dt = datetime();
    gprintln!("Aktuelle Datetime {}", dt.format());
}





