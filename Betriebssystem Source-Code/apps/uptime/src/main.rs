#![no_std]
#![no_main]

extern crate alloc;

use usrlib::{
    gprintln,
    time::rtc_date_time::{datetime, systime},
};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    let dt = systime();
    gprintln!("Aktuelle Uptime {}", dt.format());
}
