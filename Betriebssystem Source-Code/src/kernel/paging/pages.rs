/*****************************************************************************
 *                                                                           *
 *                  P A G E S                                                *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Hier sind die Funktionnen fuer die Page-Tables.          *
 *                                                                           *
 * Autor:           Michael Schoettner, 13.9.2023                            *
 *****************************************************************************/

use alloc::borrow::ToOwned;
use bitflags::bitflags;
use core::fmt;
use core::ops::Add;
use core::ops::BitOr;
use core::ptr;
use core::ptr::null_mut;
use x86;

use crate::boot::multiboot;
use crate::boot::multiboot::MultibootFramebuffer;
use crate::boot::multiboot::MultibootInfo;
use crate::consts::KERNEL_VM_SIZE;
use crate::consts::PAGE_FRAME_SIZE;
use crate::consts::PAGE_SIZE;
use crate::consts::STACK_SIZE;
use crate::consts::USER_STACK_VM_END;
use crate::consts::USER_STACK_VM_START;
use crate::kernel::paging::frames;
use crate::kernel::paging::frames::PhysAddr;
use crate::mylib::mathadditions::math::pow;

use super::frames::pf_alloc;

// Anzahl Eintraege in einer Seitentabelle
const PAGE_TABLE_ENTRIES: usize = 512;

// Flags eines Eintrages in der Seitentabelle
bitflags::bitflags! {
    pub struct PTEflags: u64 {
        const PRESENT = 1 << 0;
        const WRITEABLE = 1 << 1;
        const USER = 1 << 2;
        const WRITE_THROUGH = 1 << 3;
        const CACHE_DISABLE = 1 << 4;
        const ACCESSED = 1 << 5;
        const DIRTY = 1 << 6;
        const HUGE_PAGE = 1 << 7;
        const GLOBAL = 1 << 8;
        const FREE = 1 << 9;          // Page-Entry free = 1, used = 0
    }
}

impl PTEflags {
    fn flags_for_kernel_pages() -> Self {
        /*
         * Hier muss Code eingefuegt werden
         *
         */

        // Kernel_flags später ohne Userbit gesetzt
        //let kernel_flags = PTEflags {
        //    bits: 0b0000_0001_1000_0011,
        //};
        let kernel_flags = PTEflags {
            bits: 0b0000_0000_0000_0111,
        };
        return kernel_flags;
    }

    fn flags_for_user_pages() -> Self {
        /*
         * Hier muss Code eingefuegt werden
         *
         */

        let user_flags = PTEflags {
            bits: 0b0000_0000_0000_0111,
        };
        return user_flags;
    }
}

// Page-Table-Eintrag
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    // Neuen Page-Table-Eintrag anlegen
    pub fn new(addr: PhysAddr, flags: PTEflags) -> Self {
        Self::new_internal(addr, flags | PTEflags::PRESENT)
    }

    fn new_internal(addr: PhysAddr, flags: PTEflags) -> Self {
        let addr: u64 = addr.into();
        Self(addr | flags.bits())
    }

    // Flags lesen
    pub fn get_flags(&self) -> PTEflags {
        PTEflags::from_bits_truncate(self.0)
    }

    // Flags schreiben
    pub fn set_flags(&mut self, flags: PTEflags) {
        *self = PageTableEntry::new_internal(self.get_addr(), flags);
        self.update();
    }

    // Adresse lesen
    pub fn get_addr(&self) -> PhysAddr {
        PhysAddr::new(self.0 & 0x000f_ffff_ffff_f000)
    }

    // Setze die Adresse im Page-Table-Eintrag
    pub fn set_addr(&mut self, addr: PhysAddr) {
        *self = PageTableEntry::new_internal(addr, self.get_flags());
        self.update();
    }

    // Seite present?
    pub fn is_present(&self) -> bool {
        self.get_flags().contains(PTEflags::PRESENT)
    }

    // Free-Bit lesen
    pub(super) fn get_free(&self) -> bool {
        self.get_flags().contains(PTEflags::FREE)
    }

    // Free-Bit schreiben
    pub(super) fn set_free(&mut self, value: bool) {
        let mut flags = self.get_flags();
        if value {
            flags.insert(PTEflags::FREE);
        } else {
            flags.remove(PTEflags::FREE);
        }
        self.set_flags(flags);
        self.update();
    }

    // Änderungen in den Speicher durchschreiben
    fn update(&mut self) {
        let pe: *mut PageTableEntry = self;
        unsafe {
            pe.write(*pe);
        }
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[addr={:?}, flags={:?}]",
            self.get_addr(),
            self.get_flags()
        )
    }
}

// Funktionen fuer die Page-Tables
#[repr(transparent)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

impl PageTable {
    // Aktuelle Root-Tabelle auslesen
    pub fn get_cr3() -> PhysAddr {
        let cr3 = unsafe { x86::controlregs::cr3() };
        PhysAddr::new(cr3)
    }

    // Setze Root-Tabelle
    pub fn set_cr3(addr: PhysAddr) {
        unsafe {
            x86::controlregs::cr3_write(addr.into());
        }
    }

    // Diese Funktion richtet ein neues Mapping ein
    // 'vm_addr':     virtuelle Startaddresse des Mappings
    // 'nr_of_pages': Anzahl der Seiten, die ab 'vm_addr' gemappt werden sollen
    fn mmap_kernel_one2one(&mut self, mut vm_addr: usize, nr_of_pages: usize) {
        const ENTRIES_PER_TABLE: usize = 512; // For x86_64 4-level paging
                                              //const ADDR_MASK: usize = 0x0000_FFFF_FFFF_F000; // Address mask for page-aligned addresses
        let physical_address_counter: usize = vm_addr;

        // Extract indices for each level of the page table
        fn get_index(vm_addr: usize, level: usize) -> usize {
            const SHIFTS: [usize; 4] = [12, 21, 30, 39]; // Bit shifts for PTE, PDE, PDPE, PML4
            (vm_addr >> SHIFTS[level]) & 0x1FF // Extract 9 bits for the index
        }

        // Recursive helper function to map pages
        fn map_recursive(
            table: &mut PageTable,
            mut vm_addr: usize,
            mut nr_of_pages: usize,
            level: usize,
            mut physical_address_counter: usize,
        ) {
            if level == 0 {
                // Base case: map the pages at level 0
                for _ in 0..nr_of_pages {
                    //let phys_addr = pf_alloc(1, true); // Allocate a physical frame
                    //entry.set_addr(phys_addr); // Set the physical address

                    // Index für in die Pagetable bereichen
                    let table_index = get_index(vm_addr, level);

                    // Entry mit physikalischer Adresse beschreiben und Flags setzen
                    let entry = &mut table.entries[table_index];
                    entry.set_addr(PhysAddr::new(physical_address_counter as u64));
                    entry.set_flags(PTEflags::flags_for_kernel_pages());

                    /*
                    kprintln!(
                        "!=!=!=! Gerade folgende Adresse gemapped: 0x{:x}; für Virtual Adress: 0x{:x}",
                        physical_address_counter, vm_addr,
                    ); */

                    // Physikalische Adresse hochzählen
                    physical_address_counter += PAGE_FRAME_SIZE;

                    vm_addr += PAGE_SIZE;
                }
            } else {
                // Recursive case: traverse or allocate next-level tables
                let start_index = get_index(vm_addr, level);
                let end_index = get_index(vm_addr + nr_of_pages * PAGE_SIZE - 1, level);

                for idx in start_index..=end_index {
                    let entry = &mut table.entries[idx];
                    if !entry.is_present() {
                        // Allocate the next level table if not present
                        let next_table_addr = pf_alloc(1, true);

                        // Entry mit physikalischer Adresse beschreiben und Flags setzen
                        entry.set_addr(next_table_addr);
                        entry.set_flags(PTEflags::flags_for_kernel_pages());
                    }
                    let next_table = unsafe { &mut *(entry.get_addr().as_mut_ptr::<PageTable>()) };

                    // Determine how many pages to map within this index
                    //let aligned_vm_addr = vm_addr & ADDR_MASK;
                    //let pages_in_this_index =
                    //    (aligned_vm_addr + (ENTRIES_PER_TABLE - idx) * PAGE_SIZE).min(nr_of_pages);
                    let pages_in_this_index =
                        (pow(ENTRIES_PER_TABLE as f32, level as u32) as usize).min(nr_of_pages);

                    map_recursive(
                        next_table,
                        vm_addr,
                        pages_in_this_index,
                        level - 1,
                        physical_address_counter,
                    );
                    // Physikalische Adresse für diese Table hochrechnen
                    physical_address_counter += pages_in_this_index * PAGE_FRAME_SIZE;

                    vm_addr += pages_in_this_index * PAGE_SIZE;
                    nr_of_pages -= pages_in_this_index;
                }
            }
        }

        // Start the recursion from the top-level table (PML4)
        map_recursive(self, vm_addr, nr_of_pages, 3, physical_address_counter);
    }

    // Iterative Testversion für Kernel 1:1 Mapping
    #[deprecated]
    fn mmap_kernel_iterative(&mut self, mut vm_addr: usize, nr_of_pages: usize) {
        // Self ist bereits eine Page die Angelegt wurde (lvl 4)

        // Speicher für Page lvl 3 anfordern
        let page_table_lvl_3_adress = pf_alloc(1, true);
        let page_table_lvl_3 = unsafe { &mut *(page_table_lvl_3_adress.as_mut_ptr::<PageTable>()) };

        // Page lvl 3 auf lvl 4 registrieren
        self.entries[0] =
            PageTableEntry::new(page_table_lvl_3_adress, PTEflags::flags_for_kernel_pages());
        self.entries[0].update();

        // Speicher für Page lvl 2 anfordern
        let page_table_lvl_2_adress = pf_alloc(1, true);
        let page_table_lvl_2 = unsafe { &mut *(page_table_lvl_2_adress.as_mut_ptr::<PageTable>()) };

        // Page lvl 2 auf lvl 3 registrieren
        page_table_lvl_3.entries[0] =
            PageTableEntry::new(page_table_lvl_2_adress, PTEflags::flags_for_kernel_pages());
        page_table_lvl_3.entries[0].update();

        // zum testen
        let mut pages_count: u64 = 0;

        // lvl 1 Pages chronologisch die Physischen Adressen einteilen
        // Speicher für Page lvl 1 anfordern (Das werden viele sein (64 mal für 128 MB))
        for i in 0..64 {
            // Pagetable Speicher holen
            let page_table_lvl_1_adress = pf_alloc(1, true);
            let page_table_lvl_1 =
                unsafe { &mut *(page_table_lvl_1_adress.as_mut_ptr::<PageTable>()) };

            // Pagetable füllen
            for k in 0..512 {
                // Physische Adresse Ausrechnen (Pagesize * 512 * i) + k * Pagesize
                let adress: u64 = ((PAGE_SIZE * 512 * i) + (k * PAGE_SIZE)) as u64;
                let phys_address = PhysAddr::new(adress);

                page_table_lvl_1.entries[k] =
                    PageTableEntry::new(phys_address, PTEflags::flags_for_kernel_pages());
                page_table_lvl_1.entries[k].update();

                // Neue Table einetragen
                pages_count = pages_count + 1;
            }

            // Pagetable in Tabelle obendrüber registrieren
            page_table_lvl_2.entries[i] =
                PageTableEntry::new(page_table_lvl_1_adress, PTEflags::flags_for_kernel_pages());
            page_table_lvl_2.entries[i].update();
        }

        kprintln!("Wirklich erstellte Seiten: {}", pages_count);
    }
}

// Hier richten wir Paging-Tabellen ein, um den Kernel von 0 - KERNEL_SPACE 1:1 zu mappen
// Fuer die Page-Tables werden bei Bedarf Page-Frames alloziert
// CR3 wird am Ende gesetzt
pub fn pg_init_kernel_tables(mbi_ptr: u64) -> PhysAddr {
    kprintln!("pg_init_kernel_tables");

    // Ausrechnen wie viel Seiten "gemappt" werden muessen
    let max_phys_addr: usize = PhysAddr::get_max_phys_addr().raw() as usize;
    let nr_of_pages = (max_phys_addr + 1) / PAGE_SIZE;
    kprintln!("   nr_of_pages = {}", nr_of_pages);
    kprintln!("   max_phys_addr = 0x{:x}", max_phys_addr);

    // Alloziere eine Tabelle fuer Page Map Level 4 (PML4) -> 4 KB
    let pml4_addr = frames::pf_alloc(1, true);
    assert!(pml4_addr != PhysAddr(0));
    //kprintln!("Adresse der Page Lvl 4 = {:?}", pml4_addr);

    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_table;
    unsafe { pml4_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // Aufruf von "mmap"
    // Mappen von allen Physikalischen Adressen
    pml4_table.mmap_kernel_one2one(0, nr_of_pages);

    // Mappen des VGA Outputs
    // Hole Multiboot infos
    let mb_info = unsafe { MultibootInfo::read(mbi_ptr) };
    let mb_fb: MultibootFramebuffer = mb_info.framebuffer;

    // Größe des Speichers berechnen pitch * height sind alle Bytes
    let video_addr = mb_fb.addr as usize;
    let video_pitch = mb_fb.pitch;
    let video_height = mb_fb.height;
    // addr: 4244635648, pitch: 5120, width: 1280, height: 720, bpp: 32 (bisher immer gleich)
    // Auf die Pages runden
    let page_video_adress = video_addr & 0xFFFF_FFFF_FFFF_F000;
    let how_many_pages = (((video_pitch * video_height) / PAGE_FRAME_SIZE as u32) + 1) as usize;

    pml4_table.mmap_kernel_one2one(video_addr, how_many_pages);

    kprintln!(
        "========= Mappen hat funktioniert; Framebuffer mit {:} Pages",
        how_many_pages
    );

    // CR3 setzen
    //pg_set_cr3(pml4_addr); //Auskommentiert, weil wir das erst später setzten wollen
    pml4_addr
}

// Diese Funktion richtet ein Mapping fuer den User-Mode Stack ein
pub fn pg_mmap_user_stack(pml4_addr: PhysAddr) -> *mut u8 {
    /*
     * Hier muss Code eingefuegt werden
     *
     */
    return null_mut();
}

// Setze das CR3 Register
pub fn pg_set_cr3(pml4_addr: PhysAddr) {
    PageTable::set_cr3(pml4_addr);
}
