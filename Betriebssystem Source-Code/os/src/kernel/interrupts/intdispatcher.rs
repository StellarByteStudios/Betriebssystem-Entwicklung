/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: int_dispatcher                                                  ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Interrupt dispatching in Rust. The main function is 'int_disp'  ║
   ║         which is called for any interrupt and calls a registered ISR    ║
   ║         of device driver, e.g. the keyboard.                            ║
   ║                                                                         ║
   ║         'int_disp' is called from 'interrupts.asm' where all the x86    ║
   ║         low-level stuff is handled.                                     ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 1.6.2024                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use alloc::{boxed::Box, vec::Vec};
use core::{
    arch::asm,
    sync::atomic::{AtomicUsize, Ordering},
};

use x86_64::registers::control::{Cr2, Cr3};

use crate::{
    consts,
    devices::kprint,
    kernel::{
        cpu, interrupts,
        interrupts::isr,
        paging::{pages, pages::where_physical_address, physical_addres::PhysAddr},
        processes::process_handler,
        threads::scheduler,
    },
    utility::delay::delay,
};

pub const INT_VEC_TIMER: usize = 32;
pub const INT_VEC_KEYBOARD: usize = 33;
pub const INT_VEC_SB16: usize = 37;

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

    // 'report' calls registered ISR
    if report(vector as usize) == true {
        return;
    }

    if vector < 32 {
        print_exception(vector);
    } else {
        kprint!("Panic: unexpected interrupt nr = {}", vector);
        kprint!(" - processor halted.");
        cpu::halt();
    }
}

const MAX_VEC_NUM: usize = 256;

static mut INT_VECTORS: Option<IntVectors> = None;
static INT_VECTORS_INITIALIZED: AtomicUsize = AtomicUsize::new(0);

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
    kprintln!("int_dispatcher::init");
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

// Interrupt vector map
struct IntVectors {
    map: Vec<Box<dyn isr::ISR>>,
}

// required by the compiler for gloabl 'INT_DISPATCHER'
unsafe impl Send for IntVectors {}
unsafe impl Sync for IntVectors {}

/**
 Description:
    Register an ISR. Must be synchronized agains interrupts, especially the PIT
    which could switch to another thread.

 Parameters: \
    `vector` vector number of interrupt
    `isr` the isr to be registered
*/
pub fn register(vector: usize, isr: Box<dyn isr::ISR>) -> bool {
    if vector < MAX_VEC_NUM {
        let ie = cpu::disable_int_nested();
        unsafe {
            INT_VECTORS.as_mut().unwrap().map[vector] = isr;
        }
        cpu::enable_int_nested(ie);
        return true;
    }
    return false;
}

/**
Description:
   Check if an ISR is registered for `vector`. If so, call it.
   This function is only called from 'int_disp', so within a
   interrupt handler

Parameters: \
   `vector` vector of the interrupt which was fired.
*/
fn report(vector: usize) -> bool {
    if vector < MAX_VEC_NUM {
        unsafe {
            match INT_VECTORS.as_mut().unwrap().map.get(vector) {
                Some(v) => {
                    if v.is_default_isr() {
                        return false;
                    }
                    v.trigger();
                    return true;
                }
                None => return false,
            }
        }
    }
    return false;
}

/**
Description:
   Print x86 exception.
*/
fn print_exception(vector: u32) {
    // force unlock, just to be sure
    // anyway we do not return
    unsafe {
        kprint::WRITER.force_unlock();
    }
    kprint!("Panic: ");

    match vector {
        0 => kprint!("division by zero"),
        1 => kprint!("debug exception"),
        2 => kprint!("non-maskable interrupt"),
        3 => kprint!("breakpoint exception"),
        4 => kprint!("overflow exception"),
        5 => kprint!("bound range exception"),
        6 => kprint!("invalid opcode"),
        7 => kprint!("device not available"),
        8 => kprint!("double fault"),
        10 => kprint!("invalid tss"),
        11 => kprint!("segment not present"),
        12 => kprint!("stack-segment fault"),
        13 => kprint!("general protection fault"),
        14 => kprint!("page fault"),
        16 => kprint!("x87 floating-point exception"),
        17 => kprint!("alignment check"),
        18 => kprint!("machine check"),
        19 => kprint!("SIMD floating-point exception"),
        20 => kprint!("virtualization exception"),
        21 => kprint!("control protection exception"),
        _ => kprint!("Panic: unexpected interrupt vector = {}", vector),
    }
    kprintln!(" - processor halted.");
}

/**
Description:
   Handling a general proection fault. Called from assembly 'interrupts.asm'

Parameters: \
   `rip`         address of the instruction which caused the GPF \
   `cs`          active 'cs' when GPF occurred \
   `error_code`  see x86 spec.
*/
#[no_mangle]
pub extern "C" fn int_gpf(error_code: u64, cs: u16, rip: u64) {
    // force unlock, just to be sure
    // anyway we do not return
    unsafe {
        kprint::WRITER.force_unlock();
    }
    kprintln!(
        "general protection fault: error_code = bx{:b}, cs:rip = 0x{:x}:0x{:x}, CR2 = 0x{:x}",
        error_code,
        cs,
        rip,
        Cr2::read_raw()
    );
    loop {}
}

#[no_mangle]
pub extern "C" fn int_pagefault(error_code: u64, cs: u16, rip: u64) {
    let nested = cpu::disable_int_nested();
    //cpu::disable_int();
    kprintln!("\n\n= = = = = Wir sind im Page fault! = = = = =");

    // * Erstmal testen ob es eine Stacküberschreitung ist * //
    // Addresse holen, wo der Page Fault stattgefunden hat
    let fault_address = Cr2::read_raw() as usize;

    kprintln!(
        "Fault adresse gelesen: {:#x};     cs-Register {:#x}",
        fault_address,
        cs
    );

    // Holen des Aktiven Prozesses
    let active_pid = scheduler::get_active_pid();
    //let active_process = process::get_process_by_id(active_pid);
    let active_process = unsafe {
        process_handler::PROCESSES
            .as_mut()
            .unwrap()
            .get_mut(&active_pid)
            .unwrap()
    };

    // Der Active Prozess gibt hier nur Müll raus
    //kprintln!("Aktiven Prozess geholt: {:?}", active_process);
    //kprintln!("---- Dump VMAs");
    //active_process.dump_vmas();
    //kprintln!("---- ----");

    // Addresse prüfen
    let is_part_of_stack = active_process.is_address_neighbour_page_of_stack(fault_address);

    //kprintln!("Part of stack?: {:#}", is_part_of_stack);

    //delay(10);

    // Falls Stacküberschreitung: den Stack erweitern
    if is_part_of_stack {
        // PML4 Adresse holen
        //let pml4_addr = Cr3::read().0.start_address().as_u64();
        let pml4_addr = process_handler::get_pml4_address_by_pid(active_pid).raw();
        //kprintln!("PML4 Adresse geholt: {:#x}", pml4_addr);
        // Stack erweitern
        let success =
            pages::pg_mmap_extend_user_stack(active_pid, PhysAddr::new(pml4_addr), fault_address);

        kprintln!("success?: {:#} ", success);

        // hat es funktioniert?
        if success {
            //kprintln!("Gebe Mapping der Faultadresse {:#x} aus:", fault_address);
            //where_physical_address(PhysAddr::new(pml4_addr), fault_address);

            // Aufräumen
            cpu::enable_int_nested(nested);
            //kprintln!("= = = = = Beende Pagefault = = = = =\n\n");
            //cpu::enable_int();
            // CR3 aktuallisieren für den TLB flush
            //let cr3 = Cr3::read().0.start_address().as_u64();
            //crate::kernel::paging::pages::pg_set_cr3(PhysAddr::new(pml4_addr));
            return;
        }
        // Ansonsten normalen Page-Fault weiter machen
    }

    // * Ansonsten normalen Page Fault * //

    // force unlock, just to be sure
    // anyway we do not return
    unsafe {
        kprint::WRITER.force_unlock();
    }
    kprintln!(
        "PageFault: error_code = {:#b}, cs-Register = {:#b}, rip-Register = {:#x}, CR2 = {:#x}",
        error_code,
        cs,
        rip,
        Cr2::read_raw()
    );
    let page_table_addres = PhysAddr::new(Cr3::read().0.start_address().as_u64());
    pages::where_physical_address(page_table_addres, Cr2::read_raw() as usize);
    loop {}
}
