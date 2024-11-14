#![allow(dead_code)] // avoid warnings

use crate::devices::{cga, vga};

// Stack size for each new thread
//pub const STACK_SIZE: usize = 0x4000 * 64 * 4;
pub const STACK_ALIGNMENT: usize = 8;
pub const STACK_ENTRY_SIZE: usize = 8;
pub const HEAP_SIZE: usize = 1024 * 1024 * 64 * 4;
pub const CLOCK_POS: (u32, u32) = (cga::get_screen_width() - 1, 0);
// Nur für RTC
pub const GRAPHIC_CLOCK_POS: (u32, u32) = ((1280 / 10) - 20, 0);
pub const GRAPHIC_BYTE_CLOCK_POS: (u32, u32) = ((1280 / 10) - 1, 0);

// Speicher pro Stack = 64 KB
pub const STACK_SIZE: usize = 0x1_0000;

// 1 MB Heap für das Einrichten des Systems (siehe 'kmain')
pub const TEMP_HEAP_SIZE: usize = 0x1_0000;

// 1 MB Heap für das Einrichten des Systems (siehe 'kmain')
pub const KERNEL_HEAP_SIZE: usize = 0x10_0000;

// Kachelgroesse = 4 KB
pub const PAGE_FRAME_SIZE: usize = 0x1000;

//
// Konstanten fuer den physikalischen Adresseraum des Kernels
//
pub const KERNEL_PHYS_SIZE: usize = 0x400_0000; // 64 MiB DRAM fuer den Kernel
pub const KERNEL_PHYS_START: usize = 0;
pub const KERNEL_PHYS_END: usize = KERNEL_PHYS_SIZE - 1;
