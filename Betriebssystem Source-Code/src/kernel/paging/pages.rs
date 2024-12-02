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
use core::sync::atomic::AtomicUsize;
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
use crate::mylib::mathadditions::math::pow_usize;

use super::frames::pf_alloc;

// Anzahl Eintraege in einer Seitentabelle
const PAGE_TABLE_ENTRIES: usize = 512;

static TABLES_IN_PHYSICAL_MEMORY: AtomicUsize = AtomicUsize::new(0);
static VIDEO_START_ADDRESS: AtomicUsize = AtomicUsize::new(0);
static NUMBER_OF_VIDEO_TABLES: AtomicUsize = AtomicUsize::new(0);

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

    // Index zerhacken für die 4 Stufen der Page Tables
    fn get_index_in_table(vm_addr: usize, level: usize) -> usize {
        const SHIFTS: [usize; 4] = [12, 21, 30, 39]; // Bit shifts for PTE, PDE, PDPE, PML4
        let index = (vm_addr >> SHIFTS[level]) & 0x1FF; // Extract 9 bits for the index
                                                        //kprintln!("= = = Index für Page-Table lvl {:} ist {:}", level, index);
        return index;
    }

    // Allgemeine Mapping funktion die alle Mappings übernehmen können soll
    // 'vm_addr':     virtuelle Startaddresse des Mappings
    // 'nr_of_pages': Anzahl der Seiten, die ab 'vm_addr' gemappt werden sollen
    // 'directmap':   wenn true: wird ein 1:1 mapping gemacht ohne das Speicher alloziert wird
    //                  wenn false: es wird speicher über den pf-allokator angefordert und die Adressen in die Entries geschrieben
    // kernalflags:   gibt an, ob bei den Pagetables die Kernalflags (true) oder die Userflags (false) gesetzt werden
    fn mmap_general(
        &mut self,
        vm_addr: usize,
        nr_of_pages: usize,
        directmap: bool,
        kernalflags: bool,
    ) {
        fn get_index(vm_addr: usize, level: usize) -> usize {
            return super::pages::PageTable::get_index_in_table(vm_addr, level);
        }

        // Recursive helper function to map pages
        fn map_recursive(
            table: &mut PageTable,
            mut vm_addr: usize,
            mut nr_of_pages: usize,
            level: usize,
            directmap: bool,
            kernalflags: bool,
        ) {
            if level == 0 {
                // Base case: Physikalische Adresse Mappen aber nicht anfordern
                for _ in 0..nr_of_pages {
                    //let phys_addr = pf_alloc(1, true); // Allocate a physical frame
                    //entry.set_addr(phys_addr); // Set the physical address

                    // Index für in die Pagetable bereichen
                    let table_index = get_index(vm_addr, level);

                    // Entry mit physikalischer Adresse beschreiben
                    let entry: &mut PageTableEntry = &mut table.entries[table_index];
                    // Was für ein Mapping machen wir
                    if directmap {
                        entry.set_addr(PhysAddr::new(vm_addr as u64));
                    } else {
                        // Speicher anfordern
                        let phys_addr = pf_alloc(1, kernalflags);
                        // Diesen Frame der Adresse zuweisen
                        entry.set_addr(phys_addr);
                    }

                    // Flags für kernal oder usermode?
                    if kernalflags {
                        entry.set_flags(PTEflags::flags_for_kernel_pages());
                    } else {
                        entry.set_flags(PTEflags::flags_for_user_pages())
                    }

                    // Sonderfall, die erste Page soll wegen null-pointer auf nicht Present gesetzt werden
                    if vm_addr == 0 {
                        entry.set_flags(entry.get_flags() ^ PTEflags::PRESENT);
                    }

                    vm_addr += PAGE_SIZE;
                }

                // Rekursion wieder zurück kehren
                return;
            }

            // Recursive case: traverse or allocate next-level tables
            let start_index = get_index(vm_addr, level);
            let end_index = get_index(vm_addr + nr_of_pages * PAGE_SIZE - 1, level);

            for idx in start_index..=end_index {
                // Den Entry aus der Table laden
                let entry = &mut table.entries[idx];

                // Falls die Page untendrunter noch nie angefordert wurde muss sie neu angelegt werden
                if !entry.is_present() {
                    // Speicher für Pagetable allozieren
                    let next_table_addr = pf_alloc(1, true);

                    // Entry mit physikalischer Adresse beschreiben und Flags setzen
                    entry.set_addr(next_table_addr);
                    // Flags für kernal oder usermode?
                    if kernalflags {
                        entry.set_flags(PTEflags::flags_for_kernel_pages());
                    } else {
                        entry.set_flags(PTEflags::flags_for_user_pages())
                    }
                }

                // Table laden, welche in dem Entry steht
                let next_table = unsafe { &mut *(entry.get_addr().as_mut_ptr::<PageTable>()) };
                // Berechnen wie viele Pages jetzt in der Ebene untendrunter angefordert werden müssebn
                let pages_in_this_index = (pow_usize(PAGE_TABLE_ENTRIES, level)).min(nr_of_pages);

                // Rekursiver Aufruf für die Pagetables untendrunter
                map_recursive(
                    next_table,
                    vm_addr,
                    pages_in_this_index,
                    level - 1,
                    directmap,
                    kernalflags,
                );

                vm_addr += pages_in_this_index * PAGE_SIZE;
                nr_of_pages -= pages_in_this_index;
            }
        }

        // Start the recursion from the top-level table (PML4)
        map_recursive(self, vm_addr, nr_of_pages, 3, directmap, kernalflags);
    }
    /*
    // Diese Funktion richtet ein neues Mapping ein
    // 'vm_addr':     virtuelle Startaddresse des Mappings
    // 'nr_of_pages': Anzahl der Seiten, die ab 'vm_addr' gemappt werden sollen
    fn mmap_kernel_one2one(&mut self, mut vm_addr: usize, nr_of_pages: usize) {
        //const ADDR_MASK: usize = 0x0000_FFFF_FFFF_F000; // Address mask for page-aligned addresses
        let physical_address_counter: usize = vm_addr;

        fn get_index(vm_addr: usize, level: usize) -> usize {
            return super::pages::PageTable::get_index_in_table(vm_addr, level);
        }

        // Recursive helper function to map pages
        fn map_recursive_one2one(
            table: &mut PageTable,
            mut vm_addr: usize,
            mut nr_of_pages: usize,
            level: usize,
        ) {
            if level == 0 {
                // Base case: Physikalische Adresse Mappen aber nicht anfordern
                for _ in 0..nr_of_pages {
                    //let phys_addr = pf_alloc(1, true); // Allocate a physical frame
                    //entry.set_addr(phys_addr); // Set the physical address

                    // Index für in die Pagetable bereichen
                    let table_index = get_index(vm_addr, level);

                    // Entry mit physikalischer Adresse beschreiben und Flags setzen
                    let entry = &mut table.entries[table_index];
                    entry.set_addr(PhysAddr::new(vm_addr as u64));
                    entry.set_flags(PTEflags::flags_for_kernel_pages());

                    // Sonderfall, die erste Page soll wegen null-pointer auf nicht Present gesetzt werden
                    if vm_addr == 0 {
                        entry.set_flags(PTEflags::flags_for_kernel_pages() ^ PTEflags::PRESENT);
                    }

                    vm_addr += PAGE_SIZE;
                }

                // Rekursion wieder zurück kehren
                return;
            }

            // Recursive case: traverse or allocate next-level tables
            let start_index = get_index(vm_addr, level);
            let end_index = get_index(vm_addr + nr_of_pages * PAGE_SIZE - 1, level);

            for idx in start_index..=end_index {
                // Den Entry aus der Table laden
                let entry = &mut table.entries[idx];

                // Falls die Page untendrunter noch nie angefordert wurde muss sie neu angelegt werden
                if !entry.is_present() {
                    // Speicher für Pagetable allozieren
                    let next_table_addr = pf_alloc(1, true);

                    // Entry mit physikalischer Adresse beschreiben und Flags setzen
                    entry.set_addr(next_table_addr);
                    entry.set_flags(PTEflags::flags_for_kernel_pages());
                }

                // Table laden, welche in dem Entry steht
                let next_table = unsafe { &mut *(entry.get_addr().as_mut_ptr::<PageTable>()) };
                // Berechnen wie viele Pages jetzt in der Ebene untendrunter angefordert werden müssebn
                let pages_in_this_index = (pow_usize(PAGE_TABLE_ENTRIES, level)).min(nr_of_pages);

                // Rekursiver Aufruf für die Pagetables untendrunter
                map_recursive_one2one(next_table, vm_addr, pages_in_this_index, level - 1);

                vm_addr += pages_in_this_index * PAGE_SIZE;
                nr_of_pages -= pages_in_this_index;
            }
        }

        // Start the recursion from the top-level table (PML4)
        map_recursive_one2one(self, vm_addr, nr_of_pages, 3);
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
    } */
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

    // Speichern wie viele Pages für das 1:1 mapping benötigt werden (später für Thread-Pages wichtig)
    TABLES_IN_PHYSICAL_MEMORY.store(nr_of_pages, core::sync::atomic::Ordering::SeqCst);

    // Alloziere eine Tabelle fuer Page Map Level 4 (PML4) -> 4 KB
    let pml4_addr = frames::pf_alloc(1, true);
    assert!(pml4_addr != PhysAddr(0));

    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_table;
    unsafe { pml4_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // Aufruf von "mmap"
    // Mappen von allen Physikalischen Adressen
    //pml4_table.mmap_kernel_one2one(0, nr_of_pages);
    pml4_table.mmap_general(0, nr_of_pages, true, true);

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

    // Speichern, wo und welche Addressen für den Videospeicher brauchen
    VIDEO_START_ADDRESS.store(page_video_adress, core::sync::atomic::Ordering::SeqCst);
    NUMBER_OF_VIDEO_TABLES.store(how_many_pages, core::sync::atomic::Ordering::SeqCst);

    //pml4_table.mmap_kernel_one2one(video_addr, how_many_pages);
    pml4_table.mmap_general(page_video_adress, how_many_pages, true, true);

    // CR3 setzen
    //pg_set_cr3(pml4_addr); //Auskommentiert, weil wir das erst später setzten wollen
    pml4_addr
}

// Diese Funktion richtet ein Mapping fuer den User-Mode Stack ein
pub fn pg_mmap_user_stack(pml4_addr: PhysAddr) -> *mut u8 {
    // Erstmal Kernelmapping kopieren
    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_thread_table;
    unsafe { pml4_thread_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // Stack mappen
    pml4_thread_table.mmap_general(
        USER_STACK_VM_START,
        STACK_SIZE / PAGE_FRAME_SIZE,
        false,
        false,
    );

    // Startadresse zurück geben
    return USER_STACK_VM_START as *mut u8;
}

pub fn pg_init_user_tables() -> PhysAddr {
    // Alloziere eine Tabelle fuer Page Map Level 4 (PML4) -> 4 KB
    let pml4_addr = frames::pf_alloc(1, true);
    assert!(pml4_addr != PhysAddr(0));

    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_thread_table;
    unsafe { pml4_thread_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }
    // 1:1 Mapping mit Kernel-Rechten
    pml4_thread_table.mmap_general(
        0,
        TABLES_IN_PHYSICAL_MEMORY.load(core::sync::atomic::Ordering::SeqCst),
        true,
        true,
    );

    // Mapping für den Framebuffer für den Grafikmodus
    pml4_thread_table.mmap_general(
        VIDEO_START_ADDRESS.load(core::sync::atomic::Ordering::SeqCst),
        NUMBER_OF_VIDEO_TABLES.load(core::sync::atomic::Ordering::SeqCst),
        true,
        true,
    );
    return pml4_addr;
}

// Setze das CR3 Register
pub fn pg_set_cr3(pml4_addr: PhysAddr) {
    PageTable::set_cr3(pml4_addr);
}
