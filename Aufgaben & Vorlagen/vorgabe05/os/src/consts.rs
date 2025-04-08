#![allow(dead_code)]          // avoid warnings 


// Speicher pro Stack = 64 KB
pub const STACK_SIZE: usize = 0x1_0000;

// 1 MB Heap für das Einrichten des Systems (siehe 'kmain')
pub const TEMP_HEAP_SIZE: usize =  0x10_0000;

// Seitengroesse = 4 KB
pub const PAGE_SIZE: usize = 0x1000;


pub const KERNEL_PHYS_START: usize = 0;
pub const KERNEL_PHYS_END: usize = KERNEL_PHYS_SIZE - 1;
pub const KERNEL_PHYS_SIZE: usize = 0x400_0000;   // 64 MiB

// Kernel Pages werden 1:1 abgebildet, fuer den kompletten phys. Adressraum
// (virt. Adresse = phys. Adresse)
pub const KERNEL_VM_START: usize = 0;
pub const KERNEL_VM_END: usize = KERNEL_VM_SIZE - 1;
pub const KERNEL_VM_SIZE: usize = 0x100_0000_0000;  // 1 TiB

// User Pages werden *nicht* 1:1 abgebildet
// x86-64 unterstützt i.d.R. nur 48 Bit virtuelle Adressen, siehe Intel Vol. 1, 3.3.7.1
// (manche moderne CPUs auch mehr, aber dann benoetigt man 5 stufiges Paging)
// Wir verwenden "nur" 48 Bits
pub const USER_VM_START: usize = KERNEL_VM_SIZE;
pub const USER_VM_END: usize = 0xFFFF_FFFF_FFFF;
pub const USER_VM_SIZE: usize = USER_VM_END - USER_VM_START + 1;

// Adresse an der Apps gelinkt werden (muss in den Apps beachtet werden!)
pub const USER_CODE_VM_START: usize = 0x100_0000_0000;  // 1 TiB

// Adresse fuer den Stack eines User-Mode Threads (64 TiB - 1)
pub const USER_STACK_VM_START:usize = 0x4000_0000_0000;
pub const USER_STACK_VM_END: usize = USER_STACK_VM_START + STACK_SIZE - 1;
