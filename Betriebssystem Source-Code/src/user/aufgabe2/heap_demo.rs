
use core::alloc::Allocator;
use core::alloc::Layout;

use crate::devices::cga as cga;  
use crate::devices::cga_print;       
use crate::devices::key as key;     
use crate::devices::keyboard as keyboard;  
use crate::kernel::allocator as allocator;  
use alloc::{boxed::Box, vec::Vec};

struct VectorStruct{
    x: u32,
    y: u32
}

// Hilfsfunktion: Auf Return-Taste warten
fn wait_for_return() {
	
	println!("");
  println!("Weiter mit <ENTER>");

   loop {
      let mut key: key::Key = keyboard::key_hit();
        
      if key.valid() == true {
		     if key.get_ascii() == 13 { break; }
      }
   }
}


fn demo() {

    allocator::dump_free_list();
    println!("");

    let layout: Layout = Layout::new::<VectorStruct>();
    let ptr: *mut VectorStruct = allocator::alloc(layout) as *mut VectorStruct;

    
    unsafe {
        (*ptr).x = 42;
        (*ptr).y = 69;
    }
    println!("- - Pointer allocated");
    
    unsafe{
        println!("- - Content: Vector x = {}, y = {}", (*ptr).x, (*ptr).y);
    }
    
    
    //unsafe {
    //    let bx = Box::from_raw(ptr);
    //}
    
    allocator::dump_free_list();

}



pub fn run () {

    demo();

    /* Hier muss Code eingefuegt werden */

}
