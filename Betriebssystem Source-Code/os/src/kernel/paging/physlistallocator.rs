/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: list                                                            ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Implementing a list heap allocator.                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Philipp Oppermann                                               ║
   ║         https://os.phil-opp.com/allocator-designs/                      ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use super::super::allocator::{align_up, Locked};
use crate::{
    consts::PAGE_FRAME_SIZE,
    kernel::{allocator::align_down, cpu},
};
use alloc::{
    alloc::{GlobalAlloc, Layout},
    string::String,
};
use core::{
    borrow::{Borrow, BorrowMut},
    mem,
    ops::Deref,
    ptr::{self, null, null_mut},
};

/**
 Description: Metadata of a free memory block in the list allocator
*/
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
}

/**
 Description: Metadata of the list allocator
*/
pub struct PfListAllocator {
    head: ListNode,
    heap_start: usize,
    heap_end: usize,
}

impl PfListAllocator {
    // Creates an empty PfListAllocator.
    //
    // Must be const because needs to be evaluated at compile time
    // because it will be used for initializing the ALLOCATOR static
    // see 'allocator.rs'
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
            heap_start: usize::MAX,
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
        self.heap_end = heap_start + heap_size - 1;
    }

    // Funktion um bei der erstellung die Neuen Blocks inzuzufügen
    pub unsafe fn init_free_block(&mut self, addr: usize, size: usize) {
        // Heap_start anpassen
        if self.heap_start > addr {
            self.heap_start = addr;
        }

        // Block hinzufügen
        let new_end_addr: usize = self.add_free_block(addr, size);

        // Heap_end anpassen
        if self.heap_end < new_end_addr {
            self.heap_end = new_end_addr;
        }
    }

    // Richtige Speicherstelle für den Block suchen
    // Passt alle Listnodes direkt richtig an
    fn find_pred_in_sorted_list(&mut self, addr: usize, node: &ListNode) -> &mut ListNode {
        // Vorne in der Liste anfangen
        let mut current = &mut self.head;

        // Sind wir schon vor dem ersten element?
        if addr < current.start_addr() {
            // Dann Head zurückgeben
            return current;
        }

        // Druch die Liste durchgehen
        while let Some(next) = current.next.borrow() {
            // Prüfen, ob die eigene Startadresse nach current aber vor next liegt
            if current.start_addr() < addr && next.start_addr() > addr {
                // Node zurückgeben
                return current;
            } else {
                // falscher Slot? -> continue with next block
                current = current.next.as_mut().unwrap();
            }
        }
        return current;
    }

    // Adds the given free memory block 'addr' to the front of the free list.
    unsafe fn add_free_block(&mut self, addr: usize, size: usize) -> usize {
        // ensure that the freed block is capable of holding ListNode
        assert!(size >= PAGE_FRAME_SIZE);

        // Adresse alignen
        let aligned_address = align_up(addr, PAGE_FRAME_SIZE);
        
        let cuted_size_unaligned: usize = size - (aligned_address - addr);
        let cuted_size: usize = align_down(cuted_size_unaligned, PAGE_FRAME_SIZE);

        // create a new ListNode (on stack)
        let mut node = ListNode::new(cuted_size);

        // Vorgänger finden
        let pred = self.find_pred_in_sorted_list(aligned_address, &node);

        // Nachfolger sichern
        let succ = pred.next.take();

        // !!!!! FÜR DIE VERSCHMELZUNG !!!!!
        // Gucken ob mit succ verschmolzen werden kann
        // Adress + size = succ.startadress-1
        if let Some(succ_ref) = succ.as_ref() {
            if addr + size >= succ_ref.start_addr() - 1 {
                // Größe von Node vergrößern
                node.size += succ_ref.size;

                // Den neuen Nachfolger holen
                let new_succ = succ.unwrap().next.take();

                // Neuen Nachfolger an Node anhängen
                node.next = new_succ;
            } else {
                // Ansonsten einfach Liste an node anhängen
                node.next = succ;
            }
        } else {
            // Ansonsten einfach Liste an node anhängen
            node.next = succ;
        }

        // Gucken ob mit pred verschmolzen werden muss
        // Adress = succ.endadress +1
        if pred.end_addr() + 1 >= addr {
            // Größe von Pred anpassen
            pred.size = pred.size + node.size;

            // neuen Nachfolger für Pred an diesen Anhängen
            pred.next = node.next.take();

            // Können wieder zurück gehen, weil die Änderungen jetzt schon in Pred stehen
            // Neue Endadresse zurückgeben
            return aligned_address + cuted_size;
        }

        // Pointer auf 'addr' of Type ListNode
        let node_ptr = aligned_address as *mut ListNode;
        
        // copy content of new ListeNode to 'addr'
        node_ptr.write(node);

        // update pred mit der Addresse der neuen Node
        pred.next = Some(&mut *node_ptr);

        // Neue Endadresse zurückgeben
        return aligned_address + cuted_size;
    }

    // Search a free block with the given size and alignment and remove
    // it from the free list.
    //
    // Return: 'ListNode' or 'None'
    fn find_free_block(&mut self, size: usize, align: usize) -> Option<&'static mut ListNode> {
        // reference to current list node, updated for each iteration
        let mut current = &mut self.head;

        // search for a large enough memory block in the linked list
        // save next block in 'block' (may be 'None' -> use 'Some')
        while let Some(ref mut block) = current.next {
            // check if current 'block' is large enough
            if let Ok(alloc_start) = Self::check_block_for_alloc(&block, size, align) {
                let next = block.next.take(); // save successor of 'block'
                let ret = Some(current.next.take().unwrap()); // take 'block'
                current.next = next; // set 'next' to successor of 'block'
                return ret;
            } else {
                // block too small -> continue with next block
                current = current.next.as_mut().unwrap();
            }
        }
        // no suitable block found
        None
    }

    // Check if the given 'block' is large enough for an allocation with
    // 'size' and alignment 'align'
    //
    // Return: OK(allocation start address) or Err
    fn check_block_for_alloc(block: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(block.start_addr(), align);

        let alloc_end = match alloc_start.checked_add(size) {
            Some(end) => end, // unused but required by compiler
            None => return Err(()),
        };

        // block too small?
        if alloc_end > block.end_addr() {
            return Err(());
        }

        // rest of block too small to hold a ListNode (required because the
        // allocation splits the block in a used and a free part)
        let remaining_block_size = block.end_addr() - alloc_end;
        if remaining_block_size > 0 && remaining_block_size < PAGE_FRAME_SIZE {
            return Err(());
        }

        // block suitable for allocation
        Ok(alloc_start)
    }

    // Adjust the given layout so that the resulting allocated memory
    // block is also capable of storing a `ListNode`.
    //
    // Returns the adjusted size and alignment as a (size, align) tuple.
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(PAGE_FRAME_SIZE)
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(PAGE_FRAME_SIZE);
        (size, layout.align())
    }

    // Dump free list
    pub fn dump_free_list(&mut self) {
        vprintln!("Dumping free memory list (including dummy element)");
        vprintln!(
            "   Heap start:   0x{:x}, heap end:  0x{:x}",
            self.heap_start,
            self.heap_end
        );

        // reference to current list node, updated for each iteration
        let mut current = &mut self.head;

        // Walk through linked list
        while let Some(ref mut block) = current.next {
            vprintln!(
                "   Block start:  0x{:x}, block end: 0x{:x}, block size: {:x}",
                block.start_addr(),
                block.start_addr() + block.size - 1,
                block.size
            );

            // continue with next block
            current = current.next.as_mut().unwrap();
        }
    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u64 {

        // perform layout adjustments
        let (size, align) = PfListAllocator::size_align(layout);
        let ret_ptr: *mut u64;

        if let Some(block) = self.find_free_block(size, align) {
            let alloc_end = block.start_addr().checked_add(size).expect("overflow");

            // the remaining memory will be inserted as new block
            // the size is large enough to store metadata; this is
            // checked in 'check_block_for_alloc' called by 'find_free_block'
            let remaining_block_size = block.end_addr() - alloc_end;
            if remaining_block_size > 0 {
                self.add_free_block(alloc_end, remaining_block_size);
            }
            ret_ptr = block.start_addr() as *mut u64;
        } else {
            ret_ptr = ptr::null_mut(); // out of memory
        }
        ret_ptr
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let (size, _) = PfListAllocator::size_align(layout);
        self.add_free_block(ptr as usize, size);
    }
}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<PfListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout);
    }
}
