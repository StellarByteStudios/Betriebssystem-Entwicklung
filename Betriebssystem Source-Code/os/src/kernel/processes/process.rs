
use alloc::boxed::Box;
use alloc::collections::linked_list;
use alloc::string::String;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::kernel::paging::pages;
use crate::kernel::paging::physical_addres::PhysAddr;
use crate::kernel::processes::vma::{VmaType, VMA};

static NEXT_PID: AtomicUsize = AtomicUsize::new(0);

/* * * * Prozessobject * * * */
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

    pub fn get_pml4_addr(&self) -> PhysAddr {
        return self.pml4_addr.clone();
    }
}