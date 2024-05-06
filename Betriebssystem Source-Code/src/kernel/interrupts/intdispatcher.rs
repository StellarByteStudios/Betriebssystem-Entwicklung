/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: intdispatcher                                                   ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Interrupt dispatching in Rust. The main function is 'int_disp'  ║
   ║         which is called for any interrupt and calls a registered ISR    ║
   ║         of device driver, e.g. the keyboard.                            ║
   ║                                                                         ║
   ║         'int_disp' is called from 'interrupts.asm' where all the x86    ║
   ║         low-level stuff is handled.                                     ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 7.3.2022                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
extern crate spin;

use crate::kernel::cpu;
use crate::kernel::interrupts::isr;
use alloc::{boxed::Box, vec::Vec};
use spin::Mutex;

pub const INT_VEC_TIMER: usize = 32;
pub const INT_VEC_KEYBOARD: usize = 33;

/**
 Description:
    This function is the main interrupt dispatcher in Rust.
    It is called from `interrupts.asm`

 Parameters: \
   `vector` vector number of interrupt
*/
#[no_mangle]
pub extern "C" fn int_disp(vector: u32) {
    if report(vector as usize) == false {
        kprint!("Panic: unexpected interrupt nr = {}", vector);
        kprint!(" - processor halted.");
        cpu::halt();
    }
}

const MAX_VEC_NUM: usize = 256;

static INT_VECTORS: Mutex<IntVectors> = Mutex::new(IntVectors { map: Vec::new() });

// Interrupt vector map
struct IntVectors {
    map: Vec<Box<dyn isr::ISR>>,
}

// required by the compiler for gloabl 'INT_DISPATCHER'
unsafe impl Send for IntVectors {}
unsafe impl Sync for IntVectors {}

/**
 Description:
    Initializing the ISR map with MAX_VEC_NUM default ISRs.
    Specific ISRs can be overwritten by calling `assign`.
*/
pub fn init() {
    let mut vectors = INT_VECTORS.lock();

    for _ in 0..MAX_VEC_NUM {
        vectors.map.push(Box::new(isr::Default));
    }
}

/**
 Description:
    Register an ISR.

 Parameters: \
    `vector` vector number of interrupt
    `isr` the isr to be registered
*/
pub fn register(vector: usize, isr: Box<dyn isr::ISR>) -> bool {
    /* Hier muss Code eingefuegt werden */
    return false;
}

/**
Description:
   Check if an ISR is registered for `vector`. If so, call it.

Parameters: \
   `vector` vector of the interrupt which was fired.
*/
pub fn report(vector: usize) -> bool {
    /* Hier muss Code eingefuegt werden */
    return false;
}
