
use core::sync::atomic::{AtomicU8,Ordering};


// called from mylib/input.rs
pub fn get_lastkey() -> u8 {
    LAST_KEY.load(Ordering::SeqCst)
}

// accessed by ISR, storing last read ASCII code
// and by get_lastkey, see above
static LAST_KEY: AtomicU8 = AtomicU8::new(0);


impl Keyboard {
   


    /*****************************************************************************
     * Funktion:        plugin                                                   *
     *---------------------------------------------------------------------------*
     * Beschreibung:    Unterbrechungen fuer die Tastatur erlauben. Ab sofort    *
     *                  wird bei einem Tastendruck die Methode 'trigger'         *
     *                  aufgerufen.                                              *
     *****************************************************************************/
    pub fn plugin() { 
			
       /* Hier muss Code eingefuegt werden */
       
    }
}


/*****************************************************************************
 * Implementierung: ISR                                                      *
 *****************************************************************************/
struct KeyboardISR;

impl isr::ISR for KeyboardISR {
    /*****************************************************************************
     * Funktion:        trigger                                                  *
     *---------------------------------------------------------------------------*
     * Beschreibung:    ISR fuer die Tastatur. Wird aufgerufen, wenn die Tastatur*
     *                  eine Unterbrechung ausloest.                             *
     *****************************************************************************/
    fn trigger(&self) {
			
 	   /* Hier muss Code eingefuegt werden */
 	   
    }
}
