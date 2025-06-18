use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::{btree_map, linked_list},
    string::{String, ToString},
};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::kernel::{
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
};

pub static mut PROCESSES: Option<btree_map::BTreeMap<usize, Box<Process>>> = None;

/* * * * Statische Prozessverwaltung * * * */
// Prozessverwaltung anlegen; wird nur 1x aufgerufen
pub fn init() {
    unsafe {
        PROCESSES = Some(btree_map::BTreeMap::new());
    }
}

pub fn remove_process_by_pid(pid: u64) -> Option<Box<Process>> {
    /*
    let process = unsafe { PROCESSES.as_mut() };

    let unwraped = process.unwrap();

    let removed = unwraped.remove(&(pid as usize));

    return removed;
*/
    return unsafe { PROCESSES.as_mut().and_then(|btree_map: &mut btree_map::BTreeMap<usize, Box<Process>>| btree_map.remove(&(pid as usize))) };
}

pub fn kill_process(pid: usize) {
    //let proc = remove_process_by_pid(pid as u64);

    //drop(proc);

    let int_disable = disable_int_nested();

    // Droppe alle Threads
    Scheduler::kill_thread_with_pid(pid);

    enable_int_nested(int_disable);

    Scheduler::exit();

    /*
    // TODO: Hier gibts irgendwie noch speicherfehler
    kprintln!("Bevor allem anderem");


    // Einzigen Thread holen
    let tid = scheduler::get_active_tid();

    Scheduler::exit();
    loop {

    }

    kprintln!("Nach get tid");

    // Einzelnen Thread beenden
    //Scheduler::kill(tid);
    //Scheduler::exit();

    kprintln!("Nach exit");
    loop {}
    // TODO: Beendet hier alles? Wird Prozess noch abgeräumt

    // Threads zum Prozess suchen
    /*
    // TODO: Hier gibts irgendwie noch speicherfehler
    let threads_to_kill = Scheduler::get_thread_ids_with_pid(pid);


    // Alle Threads killen
    for id in threads_to_kill {
        Scheduler::kill(id);
    }*/

    let process;
    // Prozess aus der Liste nehmen
    unsafe {
        process = PROCESSES.as_mut().unwrap().remove(&pid).unwrap();
    }

    // TODO: Alle VMAs freigeben
    for vma in process.vmas {
        // TODO: VMA Freigeben
    }*/
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
