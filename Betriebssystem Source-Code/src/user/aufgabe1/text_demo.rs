

use crate::devices::cga;        // shortcut for cga
use crate::devices::cga_print;  // used to import code needed by println! 


pub fn run () {

   // Überschrift ausgeben
   println!("Testen der Zahlenausgabefunktion:\n");

   // Tabellenkopf
   println!("  | dec | hex | bin   |");
   println!("  ---------------------");

   // Tabellenkontent
   for i in 0..17 {
      println!("  | {:2}  | {:#4x}| {:>5b} |", i, i, i);
   }
   

}
