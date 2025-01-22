
// Diese Funktion richtet ein Mapping fuer den User-Mode Stack ein
pub fn pg_mmap_user_stack(pid: u64, pml4_addr: PhysAddr) -> *mut u8 {

    // VMA fuer den Code hinzufuegen
    
    /*
     * Hier muss Code eingefuegt werden
     */

}

// Diese Funktion richtet ein Mapping fuer ein App-Image ein
// Hierbei werden fuer alle Pages die Page-Frames alloziert
pub fn pg_mmap_user_app(pid: u64, pml4_addr: PhysAddr, start: usize, end: usize) { 
   
   // VMA fuer den Code hinzufuegen

    /*
     * Hier muss Code eingefuegt werden
     */
    
}

// Diese Funktion richtet ein Mapping fuer den User-Mode Stack ein
pub fn pg_mmap_user_heap(pid: u64, addr: u64, len: u64) -> u64 {

    /*
     * Hier muss Code eingefuegt werden
     */

}
