use usrlib::time::rtc_date_time::{RtcDate, RtcDateTime, RtcTime};

use crate::cpu::{inb, outb};

const CMOS_ADDRESS: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

const TIMEZONE: u8 = 2;

fn read_cmos(reg: u8) -> u8 {
    outb(CMOS_ADDRESS, reg);
    inb(CMOS_DATA)
}

fn wait_update_finished() {
    // Bit 7 im Statusregister A (0x0A) zeigt Update in Progress an
    while (read_cmos(0x0A) & 0x80) != 0 {}
}

fn bcd_to_binary(bcd: u8) -> u8 {
    ((bcd / 16) * 10) + (bcd & 0xf)
}

pub fn get_current_date() -> RtcDate {
    wait_update_finished();
    let day: u8 = bcd_to_binary(read_cmos(0x07));
    let month: u8 = bcd_to_binary(read_cmos(0x08));
    let year: u8 = bcd_to_binary(read_cmos(0x09));
    RtcDate { day, month, year }
}

pub fn get_current_time() -> RtcTime {
    wait_update_finished();
    let seconds: u8 = bcd_to_binary(read_cmos(0x00));
    let minutes: u8 = bcd_to_binary(read_cmos(0x02));
    let hours: u8 = bcd_to_binary(read_cmos(0x04)) + TIMEZONE; // for +2 UTC time

    RtcTime {
        seconds,
        minutes,
        hours,
    }
}

pub fn get_date_time() -> RtcDateTime {
    let time = get_current_time();
    let date = get_current_date();
    RtcDateTime { date, time }
}
