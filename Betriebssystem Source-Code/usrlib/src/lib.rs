#![no_std]
#![allow(dead_code)] // avoid warnings
#![allow(unused_variables)] // avoid warnings

extern crate alloc;
pub mod kernel;
//pub mod applications;
pub mod utility;

#[macro_use] // import macros, too
pub mod graphix;

fn main() {
    //gprintln!("Hello, world!");
}
