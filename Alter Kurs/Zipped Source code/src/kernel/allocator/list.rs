/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: list                                                            ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Imnplementing a list heap allocator.                            ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Philipp Oppermann                                               ║
   ║         https://os.phil-opp.com/allocator-designs/                      ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use super::{align_up, Locked};
use crate::kernel::cpu;
use alloc::{
    alloc::{GlobalAlloc, Layout},
    format,
    string::String,
};
use core::{
    borrow::{Borrow, BorrowMut},
    mem,
    ptr::{self, null, null_mut},
};
use x86_64::registers::debug;

/**
 Description: Metadata of a free memory block in the list allocator
*/
#[derive(Debug)]
struct ListNode {
    // size of the memory block
    size: usize,

    // &'static mut type semantically describes an owned object behind
    // a pointer. Basically, it’s a Box without a destructor that frees
    // the object at the end of the scope.
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    // Create new ListMode on Stack
    // (must be 'const')
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    // return start address of memory block
    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    // return end address of memory block
    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }

    fn print(&self) {
        // Ausgabe der eingenen Daten
        print!(
            "Node: Size = Dec:{0:7} | Hex:{0:#8x}; Addr = {1:#8x};  Next = ",
            self.size,
            self.start_addr()
        );

        // Ausgabe der Next Referenz wenn sie existiert
        if self.next.is_some() {
            println!("{:#8x}", self.next.as_ref().unwrap().start_addr());
            return;
        }

        println!("None");
    }

    fn k_print(&self) {
        kprint!(
            "Node: Size = Dec: {0:} | Hex: {0:#x}; Addr = {1:#8x};  Next = ",
            self.size,
            self.start_addr()
        );

        // Ausgabe der Next Referenz wenn sie existiert
        if self.next.is_some() {
            //kprintln!("{:#8x}", self.next.as_ref().unwrap().start_addr());
            return;
        }

        //kprintln!("None");
    }

    fn v_print(&self) {
        // Ausgabe der eingenen Daten
        vprint!(
            "Node: Size = Dec:{0:12} | Hex:{0:#10x}; Addr = {1:#10x};  Next = ",
            self.size,
            self.start_addr()
        );

        // Ausgabe der Next Referenz wenn sie existiert
        if self.next.is_some() {
            vprintln!("{:#8x}", self.next.as_ref().unwrap().start_addr());
            return;
        }

        vprintln!("None");
    }

    fn to_string(&self) -> String {
        // Ausgabe der eingenen Daten
        let mut output_string = String::new();

        kprintln!("to_string (Node) angefangen");

        output_string.push_str(
            format!(
                "Node: Size = Dec:{0:12} | Hex:{0:#10x}; Addr = {1:#10x};  Next = ",
                self.size,
                self.start_addr()
            )
            .as_str(),
        );

        kprintln!("Head von to_string (list) zusammengepackt");

        // Ausgabe der Next Referenz wenn sie existiert
        if self.next.is_some() {
            output_string
                .push_str(format!("{:#8x}", self.next.as_ref().unwrap().start_addr()).as_str());
            return output_string;
        }

        output_string.push_str("None");

        return output_string;
    }
}

/**
 Description: Metadata of the list allocator
*/
pub struct LinkedListAllocator {
    head: ListNode,
    heap_start: usize,
    heap_end: usize,
}

impl LinkedListAllocator {
    // Creates an empty LinkedListAllocator.
    //
    // Must be const because needs to be evaluated at compile time
    // because it will be used for initializing the ALLOCATOR static
    // see 'allocator.rs'
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
            heap_start: 0,
            heap_end: 0,
        }
    }

    // Initialize the allocator with the given heap bounds.
    //
    // This function is unsafe because the caller must guarantee that
    // the given heap bounds are valid. This method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_block(heap_start, heap_size);

        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
    }

    // Adds the given free memory block 'addr' to the front of the free list.
    unsafe fn add_free_block(&mut self, addr: usize, size: usize) {
        // ensure that the freed block is capable of holding ListNode
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        //kprintln!("Lege neue Node mit Größe {:} Byte (Hex: {:#x}) an", size, size);
        // create a new ListNode (on stack)
        let mut node = ListNode::new(size);

        //kprintln!(" * * Vor dem Write * *");
        //node.print();

        // set next ptr of new ListNode to existing 1st block
        node.next = self.head.next.take();

        // create a pointer to 'addr' of Type ListNode
        let node_ptr = addr as *mut ListNode;

        // copy content of new ListeNode to 'addr'
        node_ptr.write(node);

        //kprintln!(" * * Nach dem Write * *");
        //node_ptr.as_ref().unwrap().k_print();

        // update ptr. to 1st block in global variable 'head'
        self.head.next = Some(&mut *node_ptr);
    }

    // Search a free block with the given size and alignment and remove
    // it from the free list.
    //
    // Return: 'ListNode' or 'None'
    fn find_free_block(&mut self, size: usize, align: usize) -> Option<&'static mut ListNode> {
        // Anfang der Liste holen
        let mut current_node: &mut ListNode = self.head.borrow_mut();

        // Solange es noch Listenelemente gibt
        while current_node.next.is_some() {
            // Checken ob Platz groß genug ist
            if Self::check_block_for_alloc(current_node.next.as_mut().unwrap(), size, align)
                .is_err()
            {
                // Falls nicht einfach weitersuchen
                current_node = current_node.next.as_mut().unwrap();
                continue;
            }

            // Freien Block speichern
            let mut free_block = current_node.next.take();

            // Block rauslöschen
            current_node.next = free_block.as_mut().unwrap().next.take();

            // Freien Block zurückgeben
            return free_block;
        }

        // no suitable block found
        return None;
    }

    // Check if the given 'block' is large enough for an allocation with
    // 'size' and alignment 'align'
    //
    // Return: OK(allocation start address) or Err
    fn check_block_for_alloc(block: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        if block.size < size {
            return Err(());
        }

        return Ok(block.start_addr());
    }

    // Adjust the given layout so that the resulting allocated memory
    // block is also capable of storing a `ListNode`.
    //
    // Returns the adjusted size and alignment as a (size, align) tuple.
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<ListNode>());
        (size, layout.align())
    }

    // Dump free list
    pub fn dump_free_list(&mut self) {
        //kprintln!("= = = List Dump Started = = =");
        println!("Freispeicherliste (mit Dummy-Element)");
        //print!("    Kopf: ");
        //self.head.print();
        //println!("Heap Start: {:#8x};    Heap End {:#8x};", self.heap_start, self.heap_end);
        //println!("Alle Elemente in der Liste ausgeben:");

        // Anfang der Liste holen
        let mut current_node: &mut ListNode = self.head.borrow_mut();

        // Solange es noch Listenelemente gibt
        while current_node.next.is_some() {
            // Einrückung
            print!("    ");
            // Element ausgeben
            current_node.print();
            //current_node.k_print();

            // Element weitergehen
            current_node = current_node.next.as_mut().unwrap();
        }
        // Letztes Element ausgeben
        // Einrückung
        print!("    ");
        current_node.print();
        //current_node.k_print();

        //kprintln!("= = = List Dump Ended = = =");
    }
    // Dump free list
    pub fn dump_free_list_graphic(&mut self) {
        //kprintln!("= = = List Dump Started = = =");
        vprintln!("Freispeicherliste (mit Dummy-Element)");
        //kprintln!("Nach erster ausgabe");
        //print!("    Kopf: ");
        //self.head.print();
        //println!("Heap Start: {:#8x};    Heap End {:#8x};", self.heap_start, self.heap_end);
        //println!("Alle Elemente in der Liste ausgeben:");

        // Anfang der Liste holen
        let mut current_node: &mut ListNode = self.head.borrow_mut();

        //kprintln!("Head geholt");

        // Solange es noch Listenelemente gibt
        while current_node.next.is_some() {
            // Einrückung
            vprint!(" *  ");
            // Element ausgeben
            current_node.v_print();
            //current_node.k_print();

            // Element weitergehen
            current_node = current_node.next.as_mut().unwrap();
        }
        // Letztes Element ausgeben
        // Einrückung
        vprint!(" *  ");
        current_node.v_print();
        //current_node.k_print();

        //kprintln!("= = = List Dump Ended = = =");
    }

    // Funktioniert noch nicht wegen Konflikte der Stringklasse
    pub fn free_list_string(&mut self) -> String {
        kprintln!("Vor dem String anlegen");

        let mut output_string = String::new();

        kprintln!("Nach dem String anlegen");

        output_string.push_str("Freispeicherliste (mit Dummy-Element)\n");
        output_string.push_str("    Kopf: ");
        kprintln!("Nach ersten pushes");
        output_string.push_str(self.head.to_string().as_str());
        output_string.push_str("\n");
        output_string.push_str(
            format!(
                "Heap Start: {:#8x};    Heap End {:#8x};",
                self.heap_start, self.heap_end
            )
            .as_str(),
        );
        //println!("Alle Elemente in der Liste ausgeben:");

        // Anfang der Liste holen
        let mut current_node: &mut ListNode = self.head.borrow_mut();

        // Solange es noch Listenelemente gibt
        while current_node.next.is_some() {
            // Einrückung
            output_string.push_str("    ");
            // Element ausgeben
            output_string.push_str(current_node.to_string().as_str());
            output_string.push_str("\n");
            //current_node.k_print();

            // Element weitergehen
            current_node = current_node.next.as_mut().unwrap();
        }
        // Letztes Element ausgeben
        // Einrückung
        output_string.push_str("    ");
        output_string.push_str(current_node.to_string().as_str());
        output_string.push_str("\n");

        return output_string;
    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        /*
        kprintln!(
            "list-alloc: size={}, align={}",
            layout.size(),
            layout.align()
        );*/

        // Ist der Angeforderte Platz mind. 16 Byte? Wichtig für Node
        // Wenn nicht, dann stretchen
        //let mut good_size: usize = layout.size();
        let (mut good_size, align) = LinkedListAllocator::size_align(layout);

        if good_size < 0x10 {
            //kprintln!("Streched Size in Allocation");
            good_size = 0x10;
        }

        // Freien Block suchen
        let free_block_option: Option<&mut ListNode> = self.find_free_block(good_size, align);

        // Kurze Abfrage ob das funktioniert hat
        if free_block_option.is_none() {
            //kprintln!("ERROR: Allocation failed, no free block");
            return null_mut();
        }

        // Gefundenen Block auspacken
        let free_block = free_block_option.unwrap();

        // Kann man die Reste noch verwerten (groß genug)
        if free_block.size - good_size >= 0x10 {
            // Reste als neuen Block speichern
            self.add_free_block(
                free_block.start_addr() + good_size,
                free_block.size - good_size,
            );
        }

        // Freien Block zurückgeben
        return free_block.start_addr() as *mut u8;
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        /*
        kprintln!(
            "list-dealloc: size={}, align={}",
            layout.size(),
            layout.align()
        );*/

        let (size, _) = LinkedListAllocator::size_align(layout);

        // Ist der Angeforderte Platz mind. 16 Byte Wichtig für Node
        // Wenn nicht, dann stretchen
        let mut good_size: usize = size;

        if good_size < 0x10 {
            //kprintln!("Streched Size in Dealloc");
            good_size = 0x10;
        }

        /*
        kprintln!(
            "\nDeallocating block of Size {0:#x} on addr {1:#x}\n",
            good_size,
            ptr as usize
        ); */

        self.add_free_block(ptr as usize, good_size)
    }
}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout);
    }
}
