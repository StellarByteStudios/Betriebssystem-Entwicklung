use alloc::boxed::Box;
use alloc::collections::btree_map;
use crate::kernel::threads::thread;
use alloc::string::{String, ToString}; 
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
    let pid = new_proc.pid;
    unsafe {
        PROCESSES.as_mut().unwrap().insert(pid, new_proc);
    }
}

// App-Name abfragen
pub fn get_app_name(pid: u64) -> Option<String> {

    unsafe {
        if PROCESSES.is_none() {
            return None;
        }
    }
    
    return unsafe{ Some(PROCESSES.as_ref().unwrap().get(&pid).unwrap().file_name.clone()) };
}


// Neuen Prozess erstellen und gleichzeitig einfügen
pub fn create_fresh_process(file_name: &str) -> u64 {
    // Neuen Prozess erstellen
    let new_process = Process::new(file_name.to_string());
    let process_pid = new_process.pid;
    
    // Prozess anmelden
    add_process(new_process);
    
    // Prozess ID zurückgeben
    return process_pid;
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
