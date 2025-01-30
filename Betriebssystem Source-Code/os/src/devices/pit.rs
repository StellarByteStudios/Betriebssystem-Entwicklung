/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pit                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Programmable Interval Timer.                                    ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author:  Michael Schoettner, HHU, 15.6.2023                             ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![allow(dead_code)]

use alloc::boxed::Box;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::consts::{CLOCK_POS, GRAPHIC_BYTE_CLOCK_POS, GRAPHIC_CLOCK_POS};
use crate::devices::cga;
use crate::devices::graphical::graphic_console_printer;
use crate::kernel::cpu;
use crate::kernel::interrupts::intdispatcher;
use crate::kernel::interrupts::intdispatcher::INT_VEC_TIMER;
use crate::kernel::interrupts::isr;
use crate::kernel::interrupts::pic;
use crate::kernel::interrupts::pic::IRQ_TIMER;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::scheduler::Scheduler;
use crate::kernel::threads::scheduler::SCHEDULER;
use crate::kernel::threads::thread;
use crate::kernel::threads::thread::Thread;

use super::cga_print;

// read systime
pub fn get_systime() -> u64 {
    SYS_TIME.load(Ordering::SeqCst)
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

    //kprintln!("tick_length: {}", tick_lenght);
    //kprintln!("(tick_lenght / 1000: {})", (tick_lenght as f32 / 1000.0));
    //kprintln!("freq: {}", freq);
    //kprintln!("Counter: {}", counter);

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
        // progress system time by one tick

        // Einen Tick speichern
        let systime: u64 = SYS_TIME.fetch_add(1, core::sync::atomic::Ordering::SeqCst);

        //kprintln!("System Tick interrupt: {}", systime);

        // Rotate the spinner each 100 ticks. One tick is 10ms, so the spinner
        // rotates 360 degress in about 1s

        // Müssen wir die Uhr aktuallisieren?

        if systime % 100 == 0 {
            /* Text Mode version
            // Interrupts zwischendrin disablen
            let ie: bool = cpu::disable_int_nested();

            // Position festsetzen vom Bildschirm
            let clock_cursor_pos: (u32, u32) = CLOCK_POS;

            // Berechnen welches Zeichen überhaupt ausgeben
            let clock_index: usize =
                (SYS_TIME_DISPLAY.fetch_add(1, core::sync::atomic::Ordering::SeqCst)) % 4;
            let clock_char: u8 = CLOCK_SYMBOLS[clock_index];

            // Alte Cursor-Position speicher
            let old_cursor_pos: (u32, u32) = cga::getpos();

            // Position der Uhr Setzen
            cga::setpos(clock_cursor_pos.0, clock_cursor_pos.1);

            // Uhr ausgeben
            cga::print_byte(clock_char);
            // Cursor wieder an richtige Stelle setzen
            cga::setpos(old_cursor_pos.0, old_cursor_pos.1);

            // Interrupts wieder freischalten
            cpu::enable_int_nested(ie);
            */

            /* Grafikmodus  version */
            /*
            // Interrupts zwischendrin disablen
            let ie: bool = cpu::disable_int_nested();

            // Position festsetzen vom Bildschirm
            let clock_cursor_pos: (u32, u32) = GRAPHIC_BYTE_CLOCK_POS;

            // Berechnen welches Zeichen überhaupt ausgeben
            let clock_index: usize =
                (SYS_TIME_DISPLAY.fetch_add(1, core::sync::atomic::Ordering::SeqCst)) % 4;
            let clock_char: u8 = CLOCK_SYMBOLS[clock_index];

            // Alte Cursor-Position speicher
            let old_cursor_pos: (u32, u32) = graphic_console_printer::get_pos();

            // Position der Uhr Setzen
            graphic_console_printer::set_pos(clock_cursor_pos.0, clock_cursor_pos.1);

            // Uhr ausgeben
            graphic_console_printer::print_char(clock_char as char);
            // Cursor wieder an richtige Stelle setzen
            graphic_console_printer::set_pos(old_cursor_pos.0, old_cursor_pos.1);

            // Interrupts wieder freischalten
            cpu::enable_int_nested(ie);
             */
        }

        // We try to switch to the next thread
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

        //kprintln!("Zwei Threads aus threads2switch {:?},  {:?};     Zeit: {}", threads2switch.0, threads2switch.1, get_systime());
        //kprintln!("Zeit: {}", get_systime());
        // kam was bei rum?
        if threads2switch.0.is_null() {
            return;
        }

        // Ansonsten switchen
        Thread::switch(threads2switch.0, threads2switch.1);
    }
}
