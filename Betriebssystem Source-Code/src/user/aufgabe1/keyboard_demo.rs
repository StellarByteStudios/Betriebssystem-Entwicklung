
use crate::devices::cga::print_byte;
use crate::devices::cga as cga;  // shortcut for cga
use crate::devices::cga_print;   // used to import code needed by println! 
use crate::devices::key as key;      use crate::devices::keyboard::key_hit;
// shortcut for key
use crate::devices::keyboard as keyboard;  // shortcut for keyboard


pub fn run() {

   loop {
      // Warten bis ein Valid Key da ist
      let mut key: key::Key;
      
      loop {
         key = key_hit();

         if key.valid(){
            break;
         }
      }

      // Das Symbol auslesen
      let ascii_byte: u8 = key.get_ascii();

      // Sonderfälle für bestimmte Tasten
      if ascii_byte == 0b1101 {
         print_byte(0b1010 as u8);
         continue;
      }

      //kprintln!("Das gelesene Byte der Tastatur war: {:4b}", ascii_byte);
      //kprintln!("Newline: {:4b}", '\n' as u8);

      // Symbol auf die Konsole schreiben
      print_byte(ascii_byte);
   }

}
