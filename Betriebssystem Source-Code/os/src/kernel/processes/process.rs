use alloc::boxed::Box;
use alloc::collections::{btree_map, linked_list};
use crate::kernel::threads::thread;
use alloc::string::{String, ToString}; 
use core::sync::atomic::{Ordering, AtomicUsize};
use crate::kernel::processes::vma::VMA;

pub static mut PROCESSES: Option<btree_map::BTreeMap<usize, Box<Process>>> = None;


/* * * * Statische Prozessverwaltung * * * */
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
pub fn get_app_name(pid: usize) -> Option<String> {

    unsafe {
        if PROCESSES.is_none() {
            return None;
        }
    }
    
    return unsafe{ Some(PROCESSES.as_ref().unwrap().get(&pid).unwrap().file_name.clone()) };
}


// Neuen Prozess erstellen und gleichzeitig einfügen
pub fn create_fresh_process(file_name: &str) -> usize {
    // Neuen Prozess erstellen
    let new_process = Process::new(file_name.to_string());
    let process_pid = new_process.pid;
    
    // Prozess anmelden
    add_process(new_process);
    
    // Prozess ID zurückgeben
    return process_pid;
}





/* * * * Prozessobject * * * */
static NEXT_PID:AtomicUsize = AtomicUsize::new(0);

// Verwaltungsstruktur fuer einen Process
#[repr(C)]
#[derive(Debug)]
pub struct Process {
    pub pid: usize,
    pub file_name: String,
    vmas: linked_list::LinkedList<Box<VMA>>, // List von allen auf die CPU wartenden Threads
}

impl Process {

    // Neuen Prozess anlegen
    pub fn new(fname:String) -> Box<Process> {
        Box::new(Process {
            pid: NEXT_PID.fetch_add(1, Ordering::SeqCst), 
            file_name: fname,
            vmas: linked_list::LinkedList::new(),
        })
    }

    // VMA hinzufuegen
    // Rueckgabewert: true -> Erfolg
    //                false -> Fehler, VMA ueberlappt
    pub fn add_vma(&mut self, vma: Box<VMA>) -> bool {

        /*
         * Hier muss Code eingefuegt werden
         */
        return true;
    }

}
