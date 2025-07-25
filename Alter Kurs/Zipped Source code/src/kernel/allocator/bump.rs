/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: bump                                                            ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Imnplementing a basic heap allocator which cannot use           ║
   ║         deallocated memory. Thus it is only for learning and testing.   ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Philipp Oppermann                                               ║
   ║         https://os.phil-opp.com/allocator-designs/                      ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use super::{align_up, Locked};
use alloc::{
    alloc::{GlobalAlloc, Layout},
    format,
    string::String,
};
use core::ptr::{self, null, null_mut};

/**
 Description: Metadata of the bump allocator
*/
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    // Creates a new empty bump allocator.
    pub const fn new() -> Self {
        let allocator: BumpAllocator = BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        };

        return allocator;
    }

    // Initialize the allocator with the given heap bounds.
    //
    // This function is unsafe because the caller must guarantee that
    // the given heap bounds are valid. This method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        /* Hier muss Code eingefuegt werden */
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = 0;
        self.allocations = 0;
    }

    // Dump free list
    pub fn dump_free_list(&mut self) {
        println!("Memory-Dump of Bump-Allocator:");
        println!(
            "- - Heap Start:               {:#8x};    Heap End:         {:#x}",
            self.heap_start, self.heap_end
        );
        println!(
            "- - Heap Next-Pointer offset: {:#8x};    Next free memory: {:#x}",
            self.next,
            self.heap_start + self.next
        );
        println!("- - Allocated Blocks:         {:8}\n", self.allocations);
    }

    pub fn dump_free_list_graphic(&mut self) {
        vprintln!("Memory-Dump of Bump-Allocator:");
        vprintln!(
            "- - Heap Start:               {:#8x};    Heap End:         {:#x}",
            self.heap_start,
            self.heap_end
        );
        vprintln!(
            "- - Heap Next-Pointer offset: {:#8x};    Next free memory: {:#x}",
            self.next,
            self.heap_start + self.next
        );
        vprintln!("- - Allocated Blocks:         {:8}\n", self.allocations);
    }

    pub fn free_list_string(&mut self) -> String {
        let mut output_string = String::new();

        output_string.push_str("Memory-Dump of Bump-Allocator:\n");
        output_string.push_str(
            format!(
                "- - Heap Start:               {:#8x};    Heap End:         {:#x}\n",
                self.heap_start, self.heap_end
            )
            .as_str(),
        );
        output_string.push_str(
            format!(
                "- - Heap Next-Pointer offset: {:#8x};    Next free memory: {:#x}\n",
                self.next,
                self.heap_start + self.next
            )
            .as_str(),
        );
        output_string
            .push_str(format!("- - Allocated Blocks:         {:8}\n", self.allocations).as_str());

        return output_string;
    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        // Neuen anfang zum zurückgeben ausrechnen
        let memory_pointer: *mut u8 = (self.heap_start + self.next) as *mut u8;

        // Pointer vom "verbrauchten" Speicher weg schieben
        self.next = self.next + layout.size();

        // Mitzählen
        self.allocations = self.allocations + 1;

        // Speicher
        return memory_pointer;
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        /*kprintln!(
            "bump-dealloc: size={}, align={}; not supported",
            layout.size(),
            layout.align()
        );*/
    }
}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout);
    }
}
