#![allow(dead_code)]          // avoid warnings 

// Stack size for each new thread
pub const STACK_SIZE: usize = 0x4000;
pub const STACK_ALIGNMENT: usize = 8;
pub const STACK_ENTRY_SIZE: usize = 8;
