/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: allocator                                                       ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Imnplementing functions for the heap allocator used by the rust ║
   ║         compiler. The function 'init' must be called early in 'startup'.║
   ║                                                                         ║
   ║         Memory-Laylout                                                  ║
   ║            0x0        real mode & bios stuff       	                 ║
   ║            0x100000   our OS image, including global variables          ║
   ║            0x500000   Start address of our heap                         ║
   ║                                                                         ║
   ║         Remarks                                                         ║
   ║            - Requires a PC with at least 8 MB RAM                       ║
   ║            - Lowest loading address for grub is 1 MB                    ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Philipp Oppermann                                               ║
   ║         https://os.phil-opp.com/allocator-designs/                      ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::alloc::Layout;
use core::ptr;

use crate::{
    gprintln,
    kernel::{allocator::list::LinkedListAllocator, syscall::user_api::usr_mmap_heap_space},
};

pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MB heap size

// defining the Allocator (which implements the 'GlobalAlloc' trait)
#[global_allocator]
//static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

/**
 Description: Initialization of the allocator. Must be called early in 'startup'.
*/
pub fn init(pid: usize, heap_size: usize) {
    // Erst speicher anfordern
    let heap_start: usize = usr_mmap_heap_space(pid, heap_size as u64) as usize;

    // Fehlerprüfung
    if heap_start == 0 {
        return;
    }

    // Allokator initialisieren
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

/**
 Description: Allocates memory from the heap.
             Compiler generates code calling this function.
*/
pub fn alloc(layout: Layout) -> *mut u8 {
    unsafe { ALLOCATOR.lock().alloc(layout) }
}

/**
 Description: Deallocates memory from the heap.
             Compiler generates code calling this function.
*/
pub fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe { ALLOCATOR.lock().dealloc(ptr, layout) }
}

/**
 Description: A wrapper around spin::Mutex to permit trait implementations
              Required for implementing `GlobalAlloc` in `bump.rs` and
             `list.rs`. Can be used for debugging the heap allocator.
*/
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

/**
 Description: Helper function used in in `bump.rs` and `list.rs`.
              Rust requires pointers to be aligned.
*/
pub fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr // addr already aligned
    } else {
        addr - remainder + align
    }
}

// Größe Runterschneiden
pub fn align_down(size: usize, align: usize) -> usize {
    let remainder = size % align;
    return size - remainder;
}
