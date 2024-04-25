
use core::alloc::Allocator;
use core::alloc::Layout;

use crate::devices::cga as cga;  
use crate::devices::cga_print;       
use crate::devices::key as key;     
use crate::devices::keyboard as keyboard;  
use crate::kernel::allocator as allocator;  
use alloc::string::ToString;
use alloc::{boxed::Box, vec::Vec};

#[derive(Debug)]
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

    //let layout: Layout = Layout::new::<VectorStruct>();
    //let mut heap_vector:Box<VectorStruct> = Box::new(VectorStruct{x: 5, y: 4});

    let s = "Moooin".to_string();

    println!("- - Pointer allocated");

    println!("{}", s);

    //println!("Struct: {:#?}", heap_vector);

    println!("Struct wird verändert");
    

    //heap_vector.x = 555;

    //println!("Struct neu: {:#?}", heap_vector);

    println!("Struct wird verändert");
    
    allocator::dump_free_list();

}



pub fn run () {

    demo();

    /* Hier muss Code eingefuegt werden */

}
