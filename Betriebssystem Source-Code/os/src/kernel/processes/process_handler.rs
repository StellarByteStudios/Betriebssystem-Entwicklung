use crate::kernel::paging::pages;
use crate::kernel::paging::physical_addres::PhysAddr;
use crate::kernel::processes::process_handler;
use crate::kernel::processes::vma::{VmaType, VMA};
use crate::kernel::threads::thread;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::{btree_map, linked_list};
use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::kernel::processes::process::Process;

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
pub fn get_process_by_id(pid: usize) -> &'static Process {
    return unsafe { PROCESSES.as_mut().unwrap().get_mut(&pid).unwrap() };
}

// App-Name abfragen
pub fn get_app_name(pid: usize) -> Option<String> {
    unsafe {
        if PROCESSES.is_none() {
            return None;
        }
    }

    return unsafe {
        Some(
            PROCESSES
                .as_ref()
                .unwrap()
                .get(&pid)
                .unwrap()
                .file_name
                .clone(),
        )
    };
}

pub fn add_vma_to_process(pid: usize, vma: Box<VMA>) -> bool {
    // Prozess holen
    let process =unsafe { PROCESSES.as_mut().unwrap().get_mut(&pid).unwrap() };

    // VMA abspeichern
    let success = process.add_vma(vma);

    // Erfolg zurückgeben
    return success;
}

pub fn dump_vma_of_process(pid: usize) {
    let process = get_process_by_id(pid);
    process.dump_vmas();
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

pub fn get_pml4_address_by_pid(pid: usize) -> PhysAddr {
    let pml4_adress = unsafe { PROCESSES.as_ref().unwrap().get(&pid).unwrap().get_pml4_addr() };
    return pml4_adress;
}
