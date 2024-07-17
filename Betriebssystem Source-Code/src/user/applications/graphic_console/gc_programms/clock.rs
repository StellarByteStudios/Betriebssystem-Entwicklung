use core::sync::atomic::AtomicBool;

use crate::consts::GRAPHIC_CLOCK_POS;
use crate::devices::rtc;
use crate::kernel::cpu;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::scheduler::Scheduler;
use crate::kernel::threads::thread;
use crate::kernel::threads::thread::Thread;
use crate::user::applications::graphic_console::graphic_console_printer;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

static CLOCK_RUNNING: AtomicBool = AtomicBool::new(false);

#[no_mangle]
extern "C" fn graphic_console_clock(myself: *mut thread::Thread) {
    // Argumente holen
    let args: Vec<String> = Thread::get_args(myself);

    // ist überhaupt was drin?
    if args.len() < 2 {
        clock_loop();
        Scheduler::exit();
        return;
    }

    // Zweites Argument matchen (erstes ist programmname)
    match args[1].as_str() {
        "start" => clock_loop(),
        "timezone" => set_new_timezone(args),
        _ => clock_loop(),
    }

    Scheduler::exit();
}

pub fn init(args: Vec<String>) {
    let clock_thread: Box<Thread> =
        thread::Thread::new_with_args(scheduler::next_thread_id(), graphic_console_clock, args);
    scheduler::Scheduler::ready(clock_thread);
}

pub fn print_help() {
    vprintln!("Starts the Clock shown on the top left corner");
    vprintln!(" - clock start (starts clock)");
    vprintln!(" - clock timezone [number] (changes Timezone; ATM no negatives)");
}

fn set_new_timezone(args: Vec<String>) {
    vprintln!("Not supported ATM");
    return;

    /*
    // Checken ob genug Werte in Args sind
    if args.len() < 3 {
        vprintln!("Not enough Arguments given for timezone");
        return;
    }

    // Checken ob der Wert eine Zahl ist
    let new_timezone = args[2].parse::<u8>();
    if new_timezone.is_err() {
        vprintln!(
            "{} is not right value for timezones (currently unable to do negatives)",
            args[2]
        );
        return;
    }

    // Zeitzone in RTC ändern
    let success: bool = rtc::set_timezone(new_timezone.clone().unwrap());
    if !success {
        vprintln!(
            "{} is not right value for timezones (currently unable to do negatives)",
            new_timezone.unwrap()
        );
        return;
    } */
}

// Loops and updates the Clock in the top right
fn clock_loop() {
    // Check if Clock is already runing
    let result: Result<bool, bool> = CLOCK_RUNNING.compare_exchange(
        false,
        true,
        core::sync::atomic::Ordering::SeqCst,
        core::sync::atomic::Ordering::SeqCst,
    );

    // return if clock is already running
    if result.is_err() {
        // Clock is running
        vprintln!("Clock is already running");
        return;
    }

    loop {
        // Interrupts zwischendrin disablen
        let ie: bool = cpu::disable_int_nested();

        // Position festsetzen vom Bildschirm
        let clock_cursor_pos: (u32, u32) = GRAPHIC_CLOCK_POS;

        // Uhrzeit holen
        let datetime: rtc::DateTime = rtc::get_time();

        // Alte Cursor-Position speicher
        let old_cursor_pos: (u32, u32) = graphic_console_printer::get_pos();

        // Position der Uhr Setzen
        graphic_console_printer::set_pos(clock_cursor_pos.0, clock_cursor_pos.1);

        // Uhr ausgeben
        graphic_console_printer::print_string(format!("{}", datetime).as_str());
        // Cursor wieder an richtige Stelle setzen
        graphic_console_printer::set_pos(old_cursor_pos.0, old_cursor_pos.1);

        // Interrupts wieder freischalten
        cpu::enable_int_nested(ie);

        scheduler::Scheduler::yield_cpu();
    }
}
