use core::{
    fmt,
    sync::atomic::{AtomicU32, AtomicU8, Ordering::SeqCst},
};

use crate::kernel::cpu;

const CURRENT_YEAR: u32 = 2024; // Change this each year!

const TIMEZONE: u8 = 2; // Time Zone (Summertime Berlin)

static CENTURY_REGISTER: AtomicU8 = AtomicU8::new(0); // Set by ACPI table parsing code if possible

static SECOND: AtomicU8 = AtomicU8::new(0);
static MINUTE: AtomicU8 = AtomicU8::new(0);
static HOUR: AtomicU8 = AtomicU8::new(0);
static DAY: AtomicU8 = AtomicU8::new(0);
static MONTH: AtomicU8 = AtomicU8::new(0);
static YEAR: AtomicU32 = AtomicU32::new(0);

const CMOS_ADDRESS: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

pub struct DateTime {
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u32,
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

pub fn get_time() -> DateTime {
    read_rtc();

    let second: u8 = SECOND.load(SeqCst);
    let minute: u8 = MINUTE.load(SeqCst);
    let hour: u8 = HOUR.load(SeqCst);
    let day: u8 = DAY.load(SeqCst);
    let month: u8 = MONTH.load(SeqCst);
    let year: u32 = YEAR.load(SeqCst);

    return DateTime {
        second,
        minute,
        hour,
        day,
        month,
        year,
    };
}

fn get_update_in_progress_flag() -> bool {
    cpu::outb(CMOS_ADDRESS, 0x0A);
    return cpu::inb(CMOS_DATA) & 0x80 != 0;
}

fn get_rtc_register(reg: u8) -> u8 {
    cpu::outb(CMOS_ADDRESS, reg);
    return cpu::inb(CMOS_DATA);
}

fn store_all_values() {
    SECOND.store(get_rtc_register(0x00), SeqCst);
    MINUTE.store(get_rtc_register(0x02), SeqCst);
    HOUR.store(get_rtc_register(0x04), SeqCst);
    DAY.store(get_rtc_register(0x07), SeqCst);
    MONTH.store(get_rtc_register(0x08), SeqCst);
    YEAR.store(get_rtc_register(0x09) as u32, SeqCst);
}

fn read_rtc() {
    let mut century: u8 = 0;
    let mut last_second: u8;
    let mut last_minute: u8;
    let mut last_hour: u8;
    let mut last_day: u8;
    let mut last_month: u8;
    let mut last_year: u8;
    let mut last_century: u8;
    let register_b: u8;

    // Note: This uses the "read registers until you get the same values twice in a row" technique
    // to avoid getting dodgy/inconsistent values due to RTC updates

    while get_update_in_progress_flag() {}

    SECOND.store(get_rtc_register(0x00), SeqCst);
    MINUTE.store(get_rtc_register(0x02), SeqCst);
    HOUR.store(get_rtc_register(0x04), SeqCst);
    DAY.store(get_rtc_register(0x07), SeqCst);
    MONTH.store(get_rtc_register(0x08), SeqCst);
    YEAR.store(get_rtc_register(0x09) as u32, SeqCst);

    if CENTURY_REGISTER.load(SeqCst) != 0 {
        century = get_rtc_register(CENTURY_REGISTER.load(SeqCst));
    }

    loop {
        last_second = SECOND.load(SeqCst);
        last_minute = MINUTE.load(SeqCst);
        last_hour = HOUR.load(SeqCst);
        last_day = DAY.load(SeqCst);
        last_month = MONTH.load(SeqCst);
        last_year = YEAR.load(SeqCst) as u8;
        last_century = century;

        while get_update_in_progress_flag() {}

        SECOND.store(get_rtc_register(0x00), SeqCst);
        MINUTE.store(get_rtc_register(0x02), SeqCst);
        HOUR.store(get_rtc_register(0x04), SeqCst);
        DAY.store(get_rtc_register(0x07), SeqCst);
        MONTH.store(get_rtc_register(0x08), SeqCst);
        YEAR.store(get_rtc_register(0x09) as u32, SeqCst);

        if CENTURY_REGISTER.load(SeqCst) != 0 {
            century = get_rtc_register(CENTURY_REGISTER.load(SeqCst));
        }

        if last_second == SECOND.load(SeqCst)
            && last_minute == MINUTE.load(SeqCst)
            && last_hour == HOUR.load(SeqCst)
            && last_day == DAY.load(SeqCst)
            && last_month == MONTH.load(SeqCst)
            && last_year == YEAR.load(SeqCst) as u8
            && last_century == century
        {
            break;
        }
    }

    register_b = get_rtc_register(0x0B);

    // Convert BCD to binary values if necessary

    if register_b & 0x04 == 0 {
        SECOND.store(
            (SECOND.load(SeqCst) & 0x0F) + ((SECOND.load(SeqCst) / 16) * 10),
            SeqCst,
        );
        MINUTE.store(
            (MINUTE.load(SeqCst) & 0x0F) + ((MINUTE.load(SeqCst) / 16) * 10),
            SeqCst,
        );
        HOUR.store(
            ((HOUR.load(SeqCst) & 0x0F) + (((HOUR.load(SeqCst) & 0x70) / 16) * 10))
                | (HOUR.load(SeqCst) & 0x80),
            SeqCst,
        );
        DAY.store(
            (DAY.load(SeqCst) & 0x0F) + ((DAY.load(SeqCst) / 16) * 10),
            SeqCst,
        );
        MONTH.store(
            (MONTH.load(SeqCst) & 0x0F) + ((MONTH.load(SeqCst) / 16) * 10),
            SeqCst,
        );
        YEAR.store(
            (YEAR.load(SeqCst) & 0x0F) + ((YEAR.load(SeqCst) / 16) * 10),
            SeqCst,
        );

        if CENTURY_REGISTER.load(SeqCst) != 0 {
            century = (century & 0x0F) + ((century / 16) * 10);
        }
    }

    // Convert 12 hour clock to 24 hour clock if necessary

    if register_b & 0x02 == 0 && HOUR.load(SeqCst) & 0x80 != 0 {
        HOUR.store(((HOUR.load(SeqCst) & 0x7F) + 12) % 24, SeqCst);
    }

    // Calculate with Timezones
    HOUR.fetch_add(TIMEZONE, SeqCst);

    // Calculate the full (4-digit) year

    if CENTURY_REGISTER.load(SeqCst) != 0 {
        YEAR.fetch_add((century as u32) * 100, SeqCst);
    } else {
        YEAR.fetch_add((CURRENT_YEAR / 100) * 100, SeqCst);
        if YEAR.load(SeqCst) < CURRENT_YEAR {
            YEAR.fetch_add(100, SeqCst);
        }
    }
}
