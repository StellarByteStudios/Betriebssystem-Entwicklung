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

use crate::consts::PAGE_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmaType {
    Code,
    Heap,
    Stack,
    Environment,
}

// Verwaltungsstruktur fuer eine VMA
#[repr(C)]
pub struct VMA {
    start: usize,
    end: usize,
    typ: VmaType,
}

impl VMA {
    // Neue VMA anlegen
    pub fn new(s: usize, e: usize, t: VmaType) -> Box<VMA> {
        Box::new(VMA {
            start: s,
            end: e,
            typ: t,
        })
    }

    // Pruefe ob zwei VMAs ueberlappen
    pub fn does_overlap(&self, other: &VMA) -> bool {
        // Fall: Eigenes Ende ist noch vor anderem Start
        if self.end < other.start {
            return false;
        }

        // Fall: Eigener Start ist schon nach anderem Ende
        if self.start > other.end {
            return false;
        }

        // Wenn beide Bedingungen nicht gegeben sind, dann gibt es Überschneidungen
        return true;
    }

    pub fn is_address_on_neighbour_page(&self, addr: usize) -> bool {
        // Adresse ist in VMA schon drin
        if addr >= self.start && addr <= self.end {
            return false;
        }

        // Eine Page neben dran
        if addr >= self.start - PAGE_SIZE && addr < self.end + PAGE_SIZE {
            return true;
        }

        // Wenn außerhalb
        return false;
    }

    // Getter zum Testen
    pub fn get_start(&self) -> usize {
        return self.start;
    }
    pub fn get_end(&self) -> usize {
        return self.end;
    }
    pub fn get_type(&self) -> VmaType {
        return self.typ;
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
