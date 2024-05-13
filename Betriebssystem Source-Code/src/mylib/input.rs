

use crate::devices::keyboard;


pub fn getch() -> u8 {
   let mut k: u8;
   
   loop {
      k = keyboard::get_lastkey();
      if k != 0 {
		  break;
      }
   }
   return k;
}

pub fn wait_for_return() {
   //kprintln!("Called wait_for_return");
   loop {
      // Vorlage abgeändert von 10 auf '\n' as u8
      let ch = keyboard::get_lastkey();
      //kprintln!("Got key {:#x}; looking for {:#x}", ch, '\n' as u8);
      if ch == '\n' as u8 {
		  break;
      }
   }
   //kprintln!("Ended wait_for_return");
}
