use core::alloc::Allocator;
use core::alloc::Layout;

use crate::devices::cga;
use crate::devices::cga_print;
use crate::devices::key;
use crate::devices::keyboard;
use crate::kernel::allocator;
use crate::kernel::allocator::dump_free_list;
use alloc::string::ToString;
use alloc::{boxed::Box, vec::Vec};

#[derive(Debug)]
struct VectorStruct {
    x: u32,
    y: u32,
}

// Hilfsfunktion: Auf Return-Taste warten
fn wait_for_return() {
    println!("");
    println!("Weiter mit <ENTER>");

    loop {
        let mut key: key::Key = keyboard::key_hit();

        if key.valid() == true {
            if key.get_ascii() == 13 {
                break;
            }
        }
    }
}

fn demo1() {
    println!("Demo 1/4: 2 Structs Dynamisch allozieren");
    println!("=========================================\n");

    allocator::dump_free_list();
    println!("\nStructs anlegen");

    // Structs anlegen
    let s1: Box<VectorStruct> = Box::new(VectorStruct { x: 5, y: 4 });
    let s2: Box<VectorStruct> = Box::new(VectorStruct { x: 1, y: 2 });

    // Structs ausgeben
    println!("Struct: {:?}", s1);
    println!("Struct: {:?}", s2);
    println!("");

    allocator::dump_free_list();
    println!("\nEine Taste Drücken um fortzufahren");

    wait_for_return();
}

fn demo2() {
    println!("Demo 2/4: 2 Structs wieder freigegeben");
    println!("=========================================\n");

    // Wurde automatisch beim Funktionswechsel freigegeben
    allocator::dump_free_list();

    println!("\nEine Taste Drücken um fortzufahren");

    wait_for_return();
}
fn demo3() {}
fn demo4() {}

pub fn run() {
    //demo();
    demo1();
    cga::clear();

    demo2();

    cga::clear();
    demo3();

    cga::clear();
    demo4();
}
