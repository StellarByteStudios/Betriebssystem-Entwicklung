/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: stack                                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Allocating and deallocation memory for a stack.                 ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 15.05.2023                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::{alloc::Layout, boxed::Box};
use core::{fmt, ptr::null_mut};

use crate::{
    consts,
    consts::{PAGE_SIZE, STACK_SIZE, USER_STACK_VM_START},
    kernel::{
        cpu,
        paging::{frames, pages, physical_addres::PhysAddr},
        processes::process_handler::get_exited_pml4_address_by_pid,
        systemallocator::allocator,
    },
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Stack {
    data: *mut u8,
    size: usize,
    pid: usize,
}

impl Stack {
    pub fn new(size: usize, pid: usize) -> Box<Stack> {
        // 64 bit alignment for stack
        let layout = unsafe { Layout::from_size_align_unchecked(size, consts::STACK_ALIGNMENT) };

        // alloc memory for stack and set ptr. to end of block - consts::STACK_ENTRY_SIZE
        let start = allocator::alloc(layout);
        let data = ((start as usize) + (size as usize) - consts::STACK_ENTRY_SIZE) as *mut u8;
        if data.is_null() {
            println!("Panic: failed in 'Stack::new'");
            cpu::halt();
        }

        Box::new(Stack { data, size, pid })
    }

    pub fn new_mapped_stack(
        pid: usize,
        size: usize,
        kernel_stack: bool,
        pml4_addr: PhysAddr,
    ) -> Box<Stack> {
        // Wenn es ein Kernal Stack ist, nix anders machen (Alten Konstruktor)
        if kernel_stack {
            return Stack::new(size, pid);
        }

        // Ansonsten Methode zum Mappen in pages aufrufen
        // Mapping anlegen
        let start_pointer = pages::pg_mmap_user_stack(pid, pml4_addr);
        if start_pointer.is_null() {
            kprintln!("Panic: failed in 'new_mapped_stack'");
        }

        // Datapointer schieben (Stack wächst von oben nach unten)
        let data =
            ((start_pointer as usize) + (size as usize) - consts::STACK_ENTRY_SIZE) as *mut u8;

        if data.is_null() {
            kprintln!("Panic: failed in 'Stack::new_mapped_stack'");
            cpu::halt();
        }

        return Box::new(Stack { data, size, pid });
    }

    pub fn stack_end(&self) -> *mut u64 {
        self.data as *mut u64
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe {
            // get physical address of virtual Stack and free memory
            if self.data as usize >= USER_STACK_VM_START {
                // get pml4 address to to convert virtual address in physical address
                let pml4_addr_opt = get_exited_pml4_address_by_pid(self.pid as u64);

                // drop pages individually
                for i in 0..STACK_SIZE / PAGE_SIZE {
                    // calculate virtual address of each page
                    let virt_addr = USER_STACK_VM_START + i * PAGE_SIZE;
                    let layout =
                        Layout::from_size_align_unchecked(PAGE_SIZE, consts::STACK_ALIGNMENT);
                    if let Some(pml4_addr) = pml4_addr_opt {
                        // convert virtual address in physical address
                        let phys_addr = pages::get_physical_address(pml4_addr, virt_addr);
                        // add memory to page frame allocator
                        frames::pf_free(phys_addr, 1);
                    }
                }
            } else {
                let layout = Layout::from_size_align_unchecked(self.size, consts::STACK_ALIGNMENT);

                // get pointer to the starting address of the kernel stack (self.data is the current stack pointer and not the starting address of the stack)
                let stack_ref = self;
                let raw_ptr = stack_ref as *mut Stack as *mut u8;
                let ptr = unsafe { raw_ptr.sub(stack_ref.size) };

                // add memory to kernel heap
                allocator::dealloc(ptr, layout);
            }
        }
    }
}

/* Alter Stack drop (probleme mit Virtual Mem)
impl Drop for Stack {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.size, consts::STACK_ALIGNMENT);
            allocator::dealloc(self.data, layout);
        }
    }
}*/

impl Default for Stack {
    fn default() -> Self {
        Self {
            data: 0 as *mut u8,
            size: 0,
            pid: 0,
        }
    }
}
