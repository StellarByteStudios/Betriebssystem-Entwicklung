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
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::interrupts::intdispatcher;
use crate::kernel::interrupts::isr;
use crate::kernel::interrupts::pic;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::scheduler::SCHEDULER;
use crate::kernel::threads::thread;

// read systime
pub fn get_systime() -> u64 {
    SYS_TIME.load(Ordering::SeqCst)
}

// Ports
const PORT_CTRL: u16 = 0x43;
const PORT_DATA0: u16 = 0x40;

// system time ticks (each 10ms one incremented)
static SYS_TIME: AtomicU64 = AtomicU64::new(0);

// index for displaying spinner
static SYS_TIME_DISPLAY: AtomicUsize = AtomicUsize::new(0);

/**
  Description: Configure pit to fire an interrupt after `x` microseconds. \

*/
pub fn interval(x: u32) {

    /* Hier muss Code eingefuegt werden */

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

   /* Hier muss Code eingefuegt werden */

}

struct PitISR;

impl isr::ISR for PitISR {
    /**
     Description: ISR of the pit.
    */
    fn trigger(&self) {
           
        // progress system time by one tick

        /* Hier muss Code eingefuegt werden */

        // Rotate the spinner each 100 ticks. One tick is 10ms, so the spinner
        // rotates 360 degress in about 1s
 
        /* Hier muss Code eingefuegt werden */

        // We try to switch to the next thread
 
        /* Hier muss Code eingefuegt werden */

    }
}
