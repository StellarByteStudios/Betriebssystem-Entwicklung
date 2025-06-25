use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::{btree_map, linked_list},
    string::{String, ToString},
    vec::Vec,
};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::{
    devices::pcspk,
    kernel::{
        cpu::{disable_int_nested, enable_int_nested},
        paging::{pages, physical_addres::PhysAddr},
        processes::{
            process::Process,
            process_handler,
            vma::{VmaType, VMA},
        },
        threads::{
            scheduler,
            scheduler::{Scheduler, SCHEDULER},
            thread,
        },
    },
};

pub static mut PROCESSES: Option<btree_map::BTreeMap<usize, Box<Process>>> = None;

pub static mut EXITED_PROCESSES: Option<btree_map::BTreeMap<u64, Box<Process>>> = None;

/* * * * Statische Prozessverwaltung * * * */
// Prozessverwaltung anlegen; wird nur 1x aufgerufen
pub fn init() {
    unsafe {
        PROCESSES = Some(btree_map::BTreeMap::new());
    }
}

pub fn init_exited() {
    unsafe {
        EXITED_PROCESSES = Some(btree_map::BTreeMap::new());
    }
}

// Prozess löschen
pub fn remove_process_by_pid(pid: u64) {
    unsafe {
        if EXITED_PROCESSES.is_none() {
            init_exited();
        }

        // remove active process of active tree and insert in exited tree
        if let Some(ref mut processes) = PROCESSES {
            let removed_proc = processes.remove(&(pid as usize));
            if let Some(proc) = removed_proc {
                EXITED_PROCESSES
                    .as_mut()
                    .unwrap()
                    .insert(proc.pid as u64, proc);
            }
        }
    }
}

// drop all processes in exited list
pub fn cleanup() {
    unsafe {
        let irq = disable_int_nested();
        if let Some(ref mut map) = EXITED_PROCESSES {
            // get all pids from the tree
            let keys: Vec<u64> = map.keys().copied().collect();

            // drop Threads and Stacks
            for (&pid, process) in map.iter() {
                Scheduler::kill_thread_with_pid(process.pid);
            }

            // drop processes
            for pid in keys {
                if let Some(proc) = map.remove(&pid) {
                    drop(proc);
                }
            }
        }
        enable_int_nested(irq);
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
    let process = unsafe { PROCESSES.as_mut().unwrap().get_mut(&pid).unwrap() };

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
    let pml4_adress = unsafe {
        PROCESSES
            .as_ref()
            .unwrap()
            .get(&pid)
            .unwrap()
            .get_pml4_addr()
    };
    return pml4_adress;
}

pub fn get_exited_pml4_address_by_pid(pid: u64) -> Option<PhysAddr> {
    unsafe {
        Some(
            EXITED_PROCESSES
                .as_ref()
                .unwrap()
                .get(&pid)
                .unwrap()
                .get_pml4_addr(),
        )
    }
}
