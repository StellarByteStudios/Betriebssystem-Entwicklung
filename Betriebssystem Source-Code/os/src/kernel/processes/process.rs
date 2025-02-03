use crate::kernel::paging::pages;
use crate::kernel::paging::physical_addres::PhysAddr;
use crate::kernel::processes::process;
use crate::kernel::processes::vma::{VmaType, VMA};
use crate::kernel::threads::thread;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::{btree_map, linked_list};
use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicUsize, Ordering};

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
    unsafe {
        /*
        let process1 = PROCESSES.as_mut();
        let process2 = process1.unwrap();
        let process3 = process2.get_mut(&pid);
        let process4 = process3.unwrap(); 
        return process4; */
        
    }
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
    let pml4_adress = unsafe { PROCESSES.as_ref().unwrap().get(&pid).unwrap().pml4_addr };
    return pml4_adress;
}

/* * * * Prozessobject * * * */
static NEXT_PID: AtomicUsize = AtomicUsize::new(0);

// Verwaltungsstruktur fuer einen Process
#[repr(C)]
#[derive(Debug)]
pub struct Process {
    pub pid: usize,
    pub file_name: String,
    pml4_addr: PhysAddr,
    vmas: linked_list::LinkedList<Box<VMA>>, // List der Virtual Memory Areas des Prozesses
}

impl Process {
    // Neuen Prozess anlegen
    pub fn new(fname: String) -> Box<Process> {
        // Neue pml4 Table anlegen
        // Oberste Page-Table anlegen (mit Kernel initialisiert)
        let new_pml4_addr = pages::pg_init_user_tables();

        Box::new(Process {
            pid: NEXT_PID.fetch_add(1, Ordering::SeqCst),
            file_name: fname,
            pml4_addr: new_pml4_addr,
            vmas: linked_list::LinkedList::new(),
        })
    }

    // VMA hinzufuegen
    // Rueckgabewert: true -> Erfolg
    //                false -> Fehler, VMA ueberlappt
    pub fn add_vma(&mut self, vma_to_safe: Box<VMA>) -> bool {
        // Für jeden Eintrag in der Liste die Grenzen checken
        for vma in self.vmas.iter() {
            if vma.does_overlap(vma_to_safe.as_ref()) {
                return false;
            }
        }

        // TODO: Aus irgendwelchen gründen gibt es ein Alignment Error, wenn die VMA ein Heap ist
        if vma_to_safe.get_type() == VmaType::Heap {
            return true;
        }

        // VMA einspeisen
        self.vmas.push_back(vma_to_safe);

        // Erfolg zurückgeben
        return true;
    }

    pub fn is_address_neighbour_page_of_stack(&self, address: usize) -> bool {
        // Alle VMAs durchgehen
        for vma in self.vmas.iter() {
            kprintln!("--- Prüfe VMA");
            // Ist das ein Stack
            if vma.get_type() == VmaType::Stack {
                kprintln!("--- Beim Prüfen eine Stack VMA gefunden");
                // Ist sie Nachbar von der Stack-VMA
                if vma.is_address_on_neighbour_page(address) {
                    return true;
                }
            }
        }

        // Wenn kein Nachbar gefunden wurde, wars keine Stacküberschreitung
        return false;
    }

    pub fn dump_vmas(&self) {
        for vma in self.vmas.iter() {
            kprintln!("{:?}", vma);
        }
    }
}
