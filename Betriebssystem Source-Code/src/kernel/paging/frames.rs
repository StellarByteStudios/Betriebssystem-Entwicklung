/*****************************************************************************
 *                                                                           *
 *                  F R A M E S                                              *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Verwaltung der Page-Frames in zwei Listen:               *
 *                     - Kernel-Page-Frames: 0 .. 64 MiB - 1                 *
 *                     - User-Page-Frames:   >= 64 MiB                       *
 *                  Der Code ist eine angepasste Version des ListAllocators. *
 *                  Wir verwalten hier auch Speicherbloecke, deren Start-    *
 *                  Adresse aber immer 4 KB aliginiert sind und deren Groesse*
 *                  immer 4 KB oder ein Vielfaches davon sind. Zudem werden  *
 *                  die Metadaten direkt in dem freien Page-Frame gespeichert*
 *                  und die Liste ist aufsteigend sortiert nach den          *
 *                  Startadressen der Bloecke. Durch die Sortierung ist eine *
 *                  Verschmelzung bei der Freigabe einfach moeglich.         *
 *                                                                           *
 * Autor:           Michael Schoettner, 21.1.2024                            *
 *****************************************************************************/

use core::borrow::Borrow;
use core::num;
use core::ops::Add;
use core::slice;
use core::sync::atomic::AtomicUsize;
use core::{mem, ptr};

use alloc::alloc::Layout;
use alloc::vec::Vec;
use spin::Mutex;

use crate::boot::multiboot::PhysRegion;
use crate::consts::KERNEL_PHYS_SIZE;
use crate::consts::PAGE_FRAME_SIZE;
use crate::devices::kprint;

use super::physlistallocator::PfListAllocator;

// letzte nutzbare physikalische Adresse
// (notwendig fuer das 1:1 mapping des Kernels in den Page-Tables)
static mut MAX_PHYS_ADDR: PhysAddr = PhysAddr(0);

// Page-Frames > KERNEL_VM_SIZE
static mut FREE_USER_PAGE_FRAMES: PfListAllocator = PfListAllocator::new();

// Page-Frames 0 .. KERNEL_VM_SIZE - 1
static mut FREE_KERNEL_PAGE_FRAMES: PfListAllocator = PfListAllocator::new();

// Grenze für KernalFrames
static KERNAL_SPACE_END_ADDRESS: AtomicUsize = AtomicUsize::new(0);

// Eine physikalische Adresse
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct PhysAddr(pub u64);

impl PhysAddr {
    pub fn new(addr: u64) -> PhysAddr {
        Self(addr)
    }

    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.0 as *mut T
    }

    pub fn as_ptr<T>(&self) -> *const T {
        self.0 as *const T
    }

    pub fn raw(&self) -> u64 {
        self.0
    }

    pub fn get_max_phys_addr() -> PhysAddr {
        unsafe { MAX_PHYS_ADDR }
    }
}

impl core::fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Phys(0x{:x})", self.0)
    }
}

impl From<PhysAddr> for u64 {
    fn from(addr: PhysAddr) -> Self {
        addr.0
    }
}

impl Add<PhysAddr> for PhysAddr {
    type Output = PhysAddr;

    fn add(self, rhs: PhysAddr) -> Self::Output {
        let res = (self.0.checked_add(rhs.0).unwrap()) as u64;
        PhysAddr(res)
    }
}

fn clean_frame(start_addr: *mut u64, size: usize) {
    unsafe {
        for i in 0..size / 8 {
            let pointer = start_addr.add(i);
            ptr::write(pointer, 0);
        }
    }
}

// Initialisiert die Page-Frame-Liste anhand der uebergebenen freien Memory-Regionen
// Bei Bedarf werden die Memory-Regionen angepasst, sodass die Startadresse
// 4 KB aliginiert ist und auch die Grösse 4 KB oder ein Vielfaches davon ist
pub fn pf_init(free: Vec<PhysRegion>) {
    // Wie viele blöcke sind in dem Vektor drin
    let blocks_count: usize = free.len();

    // Speichere wie voll der Kernal-space schon ist
    let mut kernal_size: usize = 0;

    // Speichere die letzte Adresse
    let mut last_addres: u64 = 0;

    // Temporäre Allokatoren basteln

    let mut free_user_page_frames: PfListAllocator = PfListAllocator::new();
    let mut free_kernal_page_frames: PfListAllocator = PfListAllocator::new();

    // Durch alle Blöcke Durchlaufen
    for i in 0..blocks_count {
        // Bestimme die Größe
        let block_size: usize = (free[i].end - free[i].start) as usize;

        // Aktuallisieren der letzten Adresse
        if last_addres < free[i].end {
            last_addres = free[i].end;
        }

        // Wenn kernal_size + block_size kleiner als KERNEL_PHYS_SIZE
        if kernal_size + block_size < KERNEL_PHYS_SIZE {
            // Dann den Block komplett zu den Kernalframes hinzufügen
            unsafe { free_kernal_page_frames.init_free_block(free[i].start as usize, block_size) };

            // kernal_size anpassen
            kernal_size = kernal_size + block_size;

            continue;
        }

        // Wenn kernal_size <= KERNEL_PHYS_SIZE aber kernal_size + block_size größer KERNEL_PHYS_SIZE
        if kernal_size <= KERNEL_PHYS_SIZE && kernal_size + block_size > KERNEL_PHYS_SIZE {
            // Teile den Block in zwei
            let kernal_block_size: usize = KERNEL_PHYS_SIZE - kernal_size;
            let user_block_size: usize = block_size - kernal_block_size;
            let border_address: usize = free[i].start as usize + kernal_block_size;

            // Füge den unteren Block komplett zu den Kernalframs hinzufügen
            unsafe {
                free_kernal_page_frames.init_free_block(free[i].start as usize, kernal_block_size)
            };
            // Füge den oberen Block komplett zu den Userframes hinzufügen
            unsafe { free_user_page_frames.init_free_block(border_address, user_block_size) }

            // kernal_size anpassen
            kernal_size = kernal_size + kernal_block_size;

            // Kernal Spade Ende abspeichern
            KERNAL_SPACE_END_ADDRESS.store(border_address, core::sync::atomic::Ordering::SeqCst);

            continue;
        }

        // Wenn kernal_size größer KERNEL_PHYS_SIZE (Kernal ist abgedeckt)
        // Dann den Block komplett zu den Userframs hinzufügen
        unsafe { free_user_page_frames.init_free_block(free[i].start as usize, block_size) };
    }

    // Nun alles in den statischen Variablen speichern
    unsafe { MAX_PHYS_ADDR = PhysAddr::new(last_addres) };
    unsafe { FREE_KERNEL_PAGE_FRAMES = free_kernal_page_frames };
    unsafe { FREE_USER_PAGE_FRAMES = free_user_page_frames };
}

// Alloziere 'pf_count' aufeinanderfolgende Page-Frames
// Vom Kernel-Space, falls 'in_kernel_space' = true
// Oder User-Space, falls 'in_kernel_space' = false
pub fn pf_alloc(pf_count: usize, in_kernel_space: bool) -> PhysAddr {
    // Fall es ist im Kernel Space
    if in_kernel_space {
        unsafe {
            let alloc_adress: *mut u64 = FREE_KERNEL_PAGE_FRAMES.alloc(
                Layout::from_size_align_unchecked(pf_count * PAGE_FRAME_SIZE, PAGE_FRAME_SIZE),
            );
            // angeforderten Speicher nullen
            clean_frame(alloc_adress, pf_count * PAGE_FRAME_SIZE);
            return PhysAddr::new(alloc_adress as u64);
        }
    }

    unsafe {
        let alloc_adress: *mut u64 = FREE_USER_PAGE_FRAMES.alloc(
            Layout::from_size_align_unchecked(pf_count * PAGE_FRAME_SIZE, PAGE_FRAME_SIZE),
        );
        // angeforderten Speicher nullen
        clean_frame(alloc_adress, pf_count * PAGE_FRAME_SIZE);
        return PhysAddr::new(alloc_adress as u64);
    }
}

// Gebe 'pf_count' aufeinanderfolgende Page-Frames frei
// Zuordnung User- oder Kernel-Space ergibt sich anhand der Adresse
pub fn pf_free(pf_addr: PhysAddr, pf_count: usize) {
    if pf_addr
        < PhysAddr::new(KERNAL_SPACE_END_ADDRESS.load(core::sync::atomic::Ordering::SeqCst) as u64)
    {
        unsafe {
            FREE_KERNEL_PAGE_FRAMES.dealloc(
                pf_addr.as_mut_ptr(),
                Layout::from_size_align_unchecked(pf_count * PAGE_FRAME_SIZE, PAGE_FRAME_SIZE),
            );
        }
        return;
    }

    unsafe {
        FREE_USER_PAGE_FRAMES.dealloc(
            pf_addr.as_mut_ptr(),
            Layout::from_size_align_unchecked(pf_count * PAGE_FRAME_SIZE, PAGE_FRAME_SIZE),
        );
    }
}

pub fn dump_kernal_frames() {
    unsafe { FREE_KERNEL_PAGE_FRAMES.dump_free_list() };
}

pub fn dump_user_frames() {
    unsafe { FREE_USER_PAGE_FRAMES.dump_free_list() };
}
