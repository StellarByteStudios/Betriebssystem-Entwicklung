/*****************************************************************************
 *                                                                           *
 *                  v m a                                                    *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Implementierung von Virtual Memory Areas (VMAs), um den  *
 *                  Adressraum eines Prozesses zu beschreiben. Jeder Prozess *
 *                  hat eine Liste von VMAs.                                 *
 *                                                                           *
 * Autor:           Michael Schoettner, 5.1.2024                             *
 *****************************************************************************/
use alloc::boxed::Box;
use core::fmt;

#[derive(Debug)]
pub enum VmaType {
    Code,
    Heap,
    Stack,
}

// Verwaltungsstruktur fuer eine VMA
#[repr(C)]
pub struct VMA {
    start: u64,
    end: u64,
    typ: VmaType,
}

impl VMA {
    // Neue VMA anlegen
    pub fn new(s: u64, e: u64, t: VmaType) -> Box<VMA> {
        Box::new(VMA {
            start: s,
            end: e,
            typ: t,
        })
    }

    // Pruefe ob zwei VMAs ueberlappen
    pub fn does_overlap(&self, other: &VMA) -> bool {

        /* 
         * Hier muss Code eingefuegt werden
         */
        
        return true;

    }
}

impl fmt::Debug for VMA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VMA [0x{:x}, 0x{:x}], type = {:?}",
            self.start, self.end, self.typ
        )
    }
}
