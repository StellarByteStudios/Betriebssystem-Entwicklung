use crate::consts::GRAPHIC_CLOCK_POS;
use crate::devices::rtc;
use crate::kernel::cpu;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread;
use crate::kernel::threads::thread::Thread;
use crate::user::applications::graphic_console::graphic_console_printer;
use alloc::boxed::Box;
use alloc::format;

pub fn init() {
    let clock_thread: Box<Thread> =
        thread::Thread::new(scheduler::next_thread_id(), clock_thread_entry);
    scheduler::Scheduler::ready(clock_thread);
}

#[no_mangle]
extern "C" fn clock_thread_entry(myself: *mut thread::Thread) {
    scheduler::set_initialized();

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
