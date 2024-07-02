#![allow(dead_code)] // avoid warnings

use crate::devices::cga;

// Stack size for each new thread
pub const STACK_SIZE: usize = 0x4000 * 64 * 4;
pub const STACK_ALIGNMENT: usize = 8;
pub const STACK_ENTRY_SIZE: usize = 8;
pub const HEAP_SIZE: usize = 1024 * 1024 * 64 * 4;
pub const CLOCK_POS: (u32, u32) = (cga::get_screen_width() - 1, 0);

// Ersetzt durch funktionsabfrage in vga
//pub const SCREEN_WIDTH: u32 = 1280;
//pub const SCREEN_HEIGHT: u32 = 720;
