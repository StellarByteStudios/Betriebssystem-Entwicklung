/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: stack                                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Allocating and deallocation memory for a stack.                 ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 15.05.2023                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::alloc::Layout;
use alloc::boxed::Box;
use core::fmt;
use core::ptr::null_mut;

use crate::consts;
use crate::kernel::allocator;
use crate::kernel::cpu;
use crate::kernel::paging::pages;
use crate::kernel::paging::physical_addres::PhysAddr;

#[repr(C)]
pub struct Stack {
    data: *mut u8,
    size: usize,
}

impl Stack {
    pub fn new(size: usize) -> Box<Stack> {
        // 64 bit alignment for stack
        let layout = unsafe { Layout::from_size_align_unchecked(size, consts::STACK_ALIGNMENT) };

        // alloc memory for stack and set ptr. to end of block - consts::STACK_ENTRY_SIZE
        let start = allocator::alloc(layout);
        let data = ((start as usize) + (size as usize) - consts::STACK_ENTRY_SIZE) as *mut u8;
        if data.is_null() {
            println!("Panic: failed in 'Stack::new'");
            cpu::halt();
        }

        /*
        kprintln!(
            "Stack::new, memory block = [0x{:x}; 0x{:x}]",
            start as usize,
            (data as usize + consts::STACK_ENTRY_SIZE)
        );*/

        Box::new(Stack { data, size })
    }

    pub fn new_mapped_stack(size: usize, kernel_stack: bool, pml4_addr: PhysAddr) -> Box<Stack> {
        // Wenn es ein Kernal Stack ist, nix anders machen (Alten Konstruktor)
        if kernel_stack {
            return Stack::new(size);
        }

        // Ansonsten Methode zum Mappen in pages aufrufen
        // Mapping anlegen
        let start_pointer = pages::pg_mmap_user_stack(pml4_addr);
        // Datapointer schieben (Stack wächst von oben nach unten)
        let data =
            ((start_pointer as usize) + (size as usize) - consts::STACK_ENTRY_SIZE) as *mut u8;

        if data.is_null() {
            println!("Panic: failed in 'Stack::new_mapped_stack'");
            cpu::halt();
        }

        /*
        kprintln!(
            "Stack::new_mapped_stack, memory block = [0x{:x}; 0x{:x}]",
            start_pointer as usize,
            (data as usize + consts::STACK_ENTRY_SIZE)
        );*/

        return Box::new(Stack { data, size });
        //return Stack::new(size);
    }

    pub fn stack_end(&self) -> *mut u64 {
        self.data as *mut u64
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.size, consts::STACK_ALIGNMENT);
            allocator::dealloc(self.data, layout);
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            data: 0 as *mut u8,
            size: 0,
        }
    }
}
