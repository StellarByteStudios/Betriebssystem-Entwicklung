
// Verwaltungsstruktur fuer einen Process
#[repr(C)]
#[derive(Debug)]
pub struct Process {
    pub pid: u64,
    pub file_name: String,
    vmas: linked_list::LinkedList<Box<VMA>>, // List von allen auf die CPU wartenden Threads
}

impl Process {

    // VMA hinzufuegen
    // Rueckgabewert: true -> Erfolg
    //                false -> Fehler, VMA ueberlappt
    pub fn add_vma(&mut self, vma: Box<VMA>) -> bool {
    
       /*
        * Hier muss Code eingefuegt werden
        */
    }

}
