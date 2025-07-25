/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pit                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Programmable Interval Timer.                                    ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author:  Michael Schoettner, HHU, 15.6.2023                             ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![allow(dead_code)]

use alloc::{boxed::Box, format, string::ToString};
use core::{
    ptr::null_mut,
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
};

use usrlib::time::rtc_date_time::RtcDateTime;

use super::cga_print;
use crate::{
    consts::{CLOCK_POS, GRAPHIC_BYTE_CLOCK_POS, GRAPHIC_CLOCK_POS},
    devices::{
        cga,
        graphical::graphic_console_printer,
        rtc::{get_current_date, get_current_time, get_date_time},
    },
    kernel::{
        cpu,
        interrupts::{intdispatcher, intdispatcher::INT_VEC_TIMER, isr, pic, pic::IRQ_TIMER},
        threads::{
            scheduler,
            scheduler::{Scheduler, SCHEDULER},
            thread,
            thread::Thread,
        },
    },
};

// read systime
pub fn get_systime() -> u64 {
    SYS_TIME.load(Ordering::SeqCst)
}

pub fn get_systime_intervall() -> u64 {
    return SYS_TICK_LENGHT as u64;
}

// Ports
const PORT_CTRL: u16 = 0x43;
const PORT_DATA0: u16 = 0x40;

const SYS_TICK_LENGHT: u32 = 10; // Tickspeed in ms

// system time ticks (each 10ms one incremented)
static SYS_TIME: AtomicU64 = AtomicU64::new(0);

// index for displaying spinner
static SYS_TIME_DISPLAY: AtomicUsize = AtomicUsize::new(0);

static CLOCK_SYMBOLS: [u8; 4] = [b'|', b'/', b'-', b'\\'];

/**
  Description: Configure pit to fire an interrupt after `x` microseconds. \

*/
pub fn interval(tick_lenght: u32) {
    // Counter ausrechnen
    let freq: f32 = 1.0 / (tick_lenght as f32 / 1000.0);
    let counter: u16 = (1_193_182_f32 / freq) as u16;

    // Command zusammenbauen
    // (00)Channel 0 | (11)Access-Mode: high/low Byte | (011)Timer_mode 3 = (square wave generator) | (0)Conter-Mode: Binary
    let pit_command: u8 = 0b00_11_011_0;

    cpu::outb(PORT_CTRL, pit_command);
    cpu::outb(PORT_DATA0, counter as u8);
    cpu::outb(PORT_DATA0, (counter >> 8) as u8);
}

/**
 Description: Configure pit using `interval` to fire an interrupt each 10ms.  \
              Then register `trigger` in interrupt dispatcher and allow the \
              timer IRQ in the PIC.

 Parameters: \
            `f` frequency of musical note \
            `d` duration in ms
*/
pub fn plugin() {
    // PIT initialisieren
    interval(SYS_TICK_LENGHT);

    // Pic Bit freigeben
    pic::allow(IRQ_TIMER);

    // Registrieren der Tastatur
    intdispatcher::register(INT_VEC_TIMER, Box::new(PitISR));
}

struct PitISR;

impl isr::ISR for PitISR {
    /**
     Description: ISR of the pit.
    */
    fn trigger(&self) {
        // Einen Tick speichern
        let systime: u64 = SYS_TIME.fetch_add(1, Ordering::SeqCst);

        // Rotate the spinner each 100 ticks. One tick is 10ms, so the spinner
        // rotates 360 degress in about 1s

        // Müssen wir die Uhr aktuallisieren?
        if systime % 100 == 0 {
            /* Grafikmodus  version */
            // Interrupts zwischendrin disablen
            let ie: bool = cpu::disable_int_nested();

            // Lock vom Cursor freigeben
            unsafe { graphic_console_printer::forceunlock_cursor() }

            // Position festsetzen vom Bildschirm
            //let clock_cursor_pos: (u32, u32) = GRAPHIC_BYTE_CLOCK_POS;
            let clock_cursor_pos: (u32, u32) = GRAPHIC_CLOCK_POS;

            // Systemzeit holen
            let timestamp = get_date_time();

            // Berechnen welches Zeichen überhaupt ausgeben
            let clock_index: usize = (SYS_TIME_DISPLAY.fetch_add(1, Ordering::SeqCst)) % 4;
            let clock_char: u8 = CLOCK_SYMBOLS[clock_index];

            // Alte Cursor-Position speicher
            let old_cursor_pos: (u32, u32) = graphic_console_printer::get_pos();

            // Position der Uhr Setzen
            graphic_console_printer::set_pos(clock_cursor_pos.0, clock_cursor_pos.1);

            // Uhr ausgeben
            //graphic_console_printer::print_char(clock_char as char);
            graphic_console_printer::print_string_on_position(
                clock_cursor_pos.0 as u64,
                clock_cursor_pos.1 as u64,
                timestamp.format().as_str(),
            );

            // Cursor wieder an richtige Stelle setzen
            graphic_console_printer::set_pos(old_cursor_pos.0, old_cursor_pos.1);

            // Interrupts wieder freischalten
            cpu::enable_int_nested(ie);
        }

        // Prüfen, ob der Scheduler grade frei ist
        let mut scheduler: Option<spin::MutexGuard<Scheduler>> = SCHEDULER.try_lock();
        if scheduler.is_none() {
            // Scheduler wieder freigeben
            drop(scheduler);
            return;
        }

        // Threads holen
        let threads2switch: (*mut Thread, *mut Thread) =
            scheduler.as_mut().unwrap().prepare_preempt();

        // Scheduler wieder freigeben
        drop(scheduler);

        // kam was bei rum?
        if threads2switch.0.is_null() {
            return;
        }

        // Ansonsten switchen
        Thread::switch(threads2switch.0, threads2switch.1);
    }
}
