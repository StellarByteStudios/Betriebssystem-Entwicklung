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
use alloc::{
    boxed::Box,
    vec::{self, Vec},
};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

use super::pic::IRQ_KEYBOARD;

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
    if is_initialized() == false {
        panic!("int_disp called but INT_VECTORS not initialized.");
    }

    if report(vector as usize) == false {
        kprint!("Panic: unexpected interrupt nr = {}", vector);
        kprint!(" - processor halted.");
        cpu::halt();
    }
}

const MAX_VEC_NUM: usize = 256;

static mut INT_VECTORS: Option<IntVectors> = None;
static INT_VECTORS_INITIALIZED: AtomicUsize = AtomicUsize::new(0);

// Interrupt vector map
struct IntVectors {
    map: Vec<Box<dyn isr::ISR>>,
}

// required by the compiler for gloabl 'INT_DISPATCHER'
unsafe impl Send for IntVectors {}
unsafe impl Sync for IntVectors {}

// used in 'int_disp' to check if interrupt dispatching tables has been initialized
fn is_initialized() -> bool {
    let v = INT_VECTORS_INITIALIZED.load(Ordering::SeqCst);
    if v == 0 {
        return false;
    }
    return true;
}

/**
 Description:
    Initializing the ISR map with MAX_VEC_NUM default ISRs.
    Specific ISRs can be overwritten by calling `assign`.
*/
pub fn init() {
    kprintln!("INT_VECTORS: init");
    unsafe {
        INT_VECTORS = Some(IntVectors { map: Vec::new() });

        for _ in 0..MAX_VEC_NUM {
            INT_VECTORS
                .as_mut()
                .unwrap()
                .map
                .push(Box::new(isr::Default));
        }
    }
    INT_VECTORS_INITIALIZED.store(1, Ordering::SeqCst);
}

/**
 Description:
    Register an ISR.

 Parameters: \
    `vector` vector number of interrupt
    `isr` the isr to be registered
*/
pub fn register(vector: usize, isr: Box<dyn isr::ISR>) -> bool {
    // Interrupts Unterdrücken
    let ie: bool = cpu::disable_int_nested();

    // Liste der Interrupts holen
    let vectors = unsafe { INT_VECTORS.as_mut().unwrap() };

    // Interrupt registrieren
    vectors.map[vector] = isr;

    // Interrupts wieder freigeben
    cpu::enable_int_nested(ie);

    return true;
}

/**
Description:
   Check if an ISR is registered for `vector`. If so, call it.

Parameters: \
   `vector` vector of the interrupt which was fired.
*/
pub fn report(vector: usize) -> bool {
    // Liste der Interrupts holen
    let vectors = unsafe { INT_VECTORS.as_mut().unwrap() };

    // Wurde ein Interrupthandler mit dieser Nummer angelegt?s
    if vectors.map[vector].is_default_isr() {
        return false;
    }

    // Ansonsten Funktion der isr ausführen
    //kprintln!("Trigger Interrupt Nr.: {}", vector);
    vectors.map[vector].trigger();
    //kprintln!("Trigger of Interrupt Ended");

    return true;
}
