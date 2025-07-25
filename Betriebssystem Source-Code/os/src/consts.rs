#![allow(dead_code)] // avoid warnings

use crate::devices::cga;
// Speicher pro Stack = 64 KB
pub const STACK_SIZE: usize = 0x1_0000;
// nur eine Seite Stack
//pub const STACK_SIZE: usize = PAGE_SIZE*2;// * 0x10;
pub const STACK_ALIGNMENT: usize = 8;
pub const STACK_ENTRY_SIZE: usize = 8;

//
// Konstanten fuer den virtuellen Adresseraum des User-Modes
// Vorerst nur fuer den Stack des User-Mode-Threads, beginnt ab 64 TiB - 1.
//
pub const USER_STACK_VM_START: usize = 0x4000_0000_0000;
pub const USER_STACK_VM_END: usize = USER_STACK_VM_START + STACK_SIZE - 1;

pub const USER_CODE_HEAP_START: usize = 0x7000_0000_0000;

// Adresse an der Apps gelinkt werden (muss in den Apps beachtet werden!)
pub const USER_CODE_VM_START: usize = 0x100_0000_0000; // 1 TiB

pub const CLOCK_POS: (u32, u32) = (cga::get_screen_width() - 1, 0);

// Nur für RTC
pub const GRAPHIC_CLOCK_POS: (u32, u32) = ((1280 / 10) - 20, 0);
pub const GRAPHIC_BYTE_CLOCK_POS: (u32, u32) = ((1280 / 10) - 1, 0);

// Kachelgroesse = 4 KB
pub const PAGE_FRAME_SIZE: usize = 0x1000;

// Seitengroesse = 4 KB
pub const PAGE_SIZE: usize = PAGE_FRAME_SIZE;

//
// Konstanten fuer den physikalischen Adresseraum des Kernels
//
pub const KERNEL_PHYS_SIZE: usize = 0x400_0000; // 64 MiB DRAM fuer den Kernel

// (1 MB) 64 KB Heap für das Einrichten des Systems (siehe 'kmain')
pub const TEMP_HEAP_SIZE: usize = 0x10_0000;

// 1 MB Heap für das Einrichten des Systems (siehe 'kmain')
pub const KERNEL_HEAP_SIZE: usize = 0x10_0000;
