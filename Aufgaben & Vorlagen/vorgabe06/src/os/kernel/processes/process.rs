use alloc::boxed::Box;
use alloc::collections::btree_map;

use crate::kernel::paging::frames::PhysAddr;
use crate::kernel::threads::thread;
use crate::boot::multiboot::AppRegion;
use alloc::string::String; 
use core::sync::atomic::{Ordering, AtomicU64};


pub static mut PROCESSES: Option<btree_map::BTreeMap<u64, Box<Process>>> = None;

// Prozessverwaltung anlegen; wird nur 1x aufgerufen
pub fn init() {
    unsafe {
        PROCESSES = Some(btree_map::BTreeMap::new());
    }
}

// Neuen Prozess registrieren
pub fn add_process(new_proc: Box<Process>) {
    unsafe {
        if PROCESSES.is_none() {
            init();
        }
    }

    // Prozess registrieren
    /* 
     * Hier  muss Code eingefuegt werden
     */
    
}

// App-Name abfragen
pub fn get_app_name(pid: u64) -> Option<String> {

    /* 
     * Hier  muss Code eingefuegt werden
     */

}



static NEXT_PID:AtomicU64 = AtomicU64::new(0);

// Verwaltungsstruktur fuer einen Process
#[repr(C)]
#[derive(Debug)]
pub struct Process {
    pub pid: u64,
    pub file_name: String, 
}

impl Process {

    // Neuen Prozess anlegen
    pub fn new(fname:String) -> Box<Process> {
        Box::new(Process {
            pid: NEXT_PID.fetch_add(1, Ordering::SeqCst), 
            file_name: fname,
        })
    }

}
