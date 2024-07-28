/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: stack                                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Allocating and deallocation memory for a stack.                 ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 15.05.2023                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::alloc::Layout;
use core::fmt;

use crate::consts;
use crate::kernel::allocator;
use crate::kernel::cpu;

#[repr(C)]
#[derive(Clone)]
pub struct Stack {
    data: *mut u8,
    size: usize,
}

impl Stack {
    /**
    Description: Alloc memory for a new stack
    */
    pub fn new(size: usize) -> Stack {
        // 64 bit alignment for stack
        let layout = unsafe { Layout::from_size_align_unchecked(size, consts::STACK_ALIGNMENT) };

        // alloc memory for stack
        let data = allocator::alloc(layout);
        if data.is_null() {
            println!("Panic: failed in 'Stack::new'");
        }
        Stack { data, size }
    }

    /**
    Description: Get last useable address of stack
    */
    pub fn end_of_stack(&self) -> usize {
        self.data as usize + self.size - consts::STACK_ENTRY_SIZE
    }
}

/**
Description: Deallocated memory for a stack
*/
impl Drop for Stack {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.size, consts::STACK_ALIGNMENT);
            allocator::dealloc(self.data, layout);
        }
    }
}

/**
Description: Dump stack [first usable address, last usable address]
*/
impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Stack [0x{:x}, 0x{:x}]",
            self.data as usize,
            self.data as usize + self.size - consts::STACK_ENTRY_SIZE
        )
    }
}
