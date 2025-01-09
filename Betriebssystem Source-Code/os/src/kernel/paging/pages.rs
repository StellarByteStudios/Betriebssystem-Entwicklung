/*****************************************************************************
 *                                                                           *
 *                  P A G E S                                                *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Hier sind die Funktionnen fuer die Page-Tables.          *
 *                                                                           *
 * Autor:           Michael Schoettner, 13.9.2023                            *
 *****************************************************************************/

use core::sync::atomic::AtomicUsize;

use crate::boot::appregion::AppRegion;
use crate::boot::multiboot;
use crate::boot::multiboot::MultibootFramebuffer;
use crate::boot::multiboot::MultibootInfo;
use crate::consts;
use crate::consts::KERNEL_VM_SIZE;
use crate::consts::PAGE_FRAME_SIZE;
use crate::consts::PAGE_SIZE;
use crate::consts::STACK_SIZE;
use crate::consts::USER_STACK_VM_END;
use crate::consts::USER_STACK_VM_START;
use crate::kernel::paging::frames;
use crate::kernel::paging::pagetable_flags::PTEflags;
use crate::mylib::mathadditions::math::pow_usize;

use super::frames::pf_alloc;
use super::pagetable_entry::PageTableEntry;
use super::physical_addres::PhysAddr;

// Anzahl Eintraege in einer Seitentabelle
const PAGE_TABLE_ENTRIES: usize = 512;

static TABLES_IN_PHYSICAL_MEMORY: AtomicUsize = AtomicUsize::new(0);
static VIDEO_START_ADDRESS: AtomicUsize = AtomicUsize::new(0);
static NUMBER_OF_VIDEO_TABLES: AtomicUsize = AtomicUsize::new(0);

// Funktionen fuer die Page-Tables
#[repr(transparent)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

// Index zerhacken für die 4 Stufen der Page Tables
fn get_index_in_table(vm_addr: usize, level: usize) -> usize {
    const SHIFTS: [usize; 4] = [12, 21, 30, 39]; // Bit shifts for PTE, PDE, PDPE, PML4
    let index = (vm_addr >> SHIFTS[level]) & 0x1FF; // Extract 9 bits for the index
    return index;
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

    //

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
        app_mapping: bool,
        app_start_address: usize,
    ) {
        fn get_index(vm_addr: usize, level: usize) -> usize {
            return super::pages::get_index_in_table(vm_addr, level);
        }

        // Recursive helper function to map pages
        fn map_recursive(
            table: &mut PageTable,
            mut vm_addr: usize,
            mut nr_of_pages: usize,
            level: usize,
            directmap: bool,
            kernalflags: bool,
            app_mapping: bool,
            mut app_current_phys_address: usize,
        ) {
            if level == 0 {
                // Base case: Physikalische Adresse Mappen aber nicht anfordern
                for _ in 0..nr_of_pages {
                    // Index für in die Pagetable bereichen
                    let table_index = get_index(vm_addr, level);

                    // Entry mit physikalischer Adresse beschreiben
                    let entry: &mut PageTableEntry = &mut table.entries[table_index];
                    // Was für ein Mapping machen wir
                    if directmap {
                        entry.set_addr(PhysAddr::new(vm_addr as u64));
                    } else if app_mapping {
                        entry.set_addr(PhysAddr::new(app_current_phys_address as u64));
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
                    app_current_phys_address += PAGE_SIZE;
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
                    app_mapping,
                    app_current_phys_address,
                );

                vm_addr += pages_in_this_index * PAGE_SIZE;
                app_current_phys_address += pages_in_this_index * PAGE_SIZE;
                nr_of_pages -= pages_in_this_index;
            }
        }

        // Start the recursion from the top-level table (PML4)
        map_recursive(
            self,
            vm_addr,
            nr_of_pages,
            3,
            directmap,
            kernalflags,
            app_mapping,
            app_start_address,
        );
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
    pml4_table.mmap_general(0, nr_of_pages, true, true, false, 0);

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
    
    pml4_table.mmap_general(page_video_adress, how_many_pages, true, true, false, 0);

    return pml4_addr;
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
        false,
        0,
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
        false,
        0,
    );

    // Mapping für den Framebuffer für den Grafikmodus
    pml4_thread_table.mmap_general(
        VIDEO_START_ADDRESS.load(core::sync::atomic::Ordering::SeqCst),
        NUMBER_OF_VIDEO_TABLES.load(core::sync::atomic::Ordering::SeqCst),
        true,
        true,
        false,
        0,
    );
    return pml4_addr;
}



// Setze das CR3 Register
pub fn pg_set_cr3(pml4_addr: PhysAddr) {
    PageTable::set_cr3(pml4_addr);
}



// Diese Funktion richtet ein Mapping fuer ein App-Image ein
pub fn pg_mmap_user_app(pml4_addr: PhysAddr, app: AppRegion) {
    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_thread_table;
    unsafe { pml4_thread_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // Einmappen der Anwendung (Code)
    pml4_thread_table.mmap_general(
        consts::USER_CODE_VM_START,
        ((app.end - app.start) as usize / PAGE_SIZE) + 1,
        false,
        false,
        true,
        app.start as usize,
    );
}

/*
* Debug-Methode: Übersetzt eine virtuelle Adresse und gibt die ersten paar Einträge
* auf Lvl. 1 aus (enthält echte physische Addressen)
*/
pub fn where_physical_address(pml4_addr: PhysAddr, virtual_address: usize) {
    let pml4_table;
    unsafe { pml4_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    kprintln!("--------- mapping virtual Address ---------");
    kprintln!("Virtual Address: 0x{:x}", virtual_address);
    kprintln!(
        "Index in Page lvl 4: {}",
        get_index_in_table(virtual_address, 3)
    );
    kprintln!(
        "Index in Page lvl 3: {}",
        get_index_in_table(virtual_address, 2)
    );
    kprintln!(
        "Index in Page lvl 2: {}",
        get_index_in_table(virtual_address, 1)
    );
    kprintln!(
        "Index in Page lvl 1: {}",
        get_index_in_table(virtual_address, 0)
    );

    kprintln!("------- now step through -------");

    kprintln!("Address of PageTable lvl 4: 0x{:x}", pml4_addr.raw());

    let page_table_4_entry: PageTableEntry =
        pml4_table.entries[get_index_in_table(virtual_address, 3)];
    if !page_table_4_entry.is_present() {
        kprintln!("....This page is not present. (lvl. 3)");
        return;
    }
    kprintln!(
        "Address of PageTable lvl 3: 0x{:x}, Flags: {:#b}",
        page_table_4_entry.get_addr().raw(),
        page_table_4_entry.get_flags().bits()
    );
    let page_table_3: &mut PageTable =
        unsafe { &mut *(page_table_4_entry.get_addr().as_mut_ptr::<PageTable>()) };

    let page_table_3_entry: PageTableEntry =
        page_table_3.entries[get_index_in_table(virtual_address, 2)];
    if !page_table_3_entry.is_present() {
        kprintln!("....This page is not present. (lvl. 2)");
        return;
    }
    kprintln!(
        "Address of PageTable lvl 2: 0x{:x}, Flags: {:#b}",
        page_table_3_entry.get_addr().raw(),
        page_table_3_entry.get_flags().bits()
    );
    let page_table_2: &mut PageTable =
        unsafe { &mut *(page_table_3_entry.get_addr().as_mut_ptr::<PageTable>()) };

    let page_table_2_entry: PageTableEntry =
        page_table_2.entries[get_index_in_table(virtual_address, 1)];
    if !page_table_2_entry.is_present() {
        kprintln!("....This page is not present. (lvl. 1)");
        return;
    }
    kprintln!(
        "Address of PageTable lvl 1: 0x{:x}, Flags: {:#b}",
        page_table_2_entry.get_addr().raw(),
        page_table_2_entry.get_flags().bits()
    );
    let page_table_1: &mut PageTable =
        unsafe { &mut *(page_table_2_entry.get_addr().as_mut_ptr::<PageTable>()) };

    kprintln!("Erste Einträge der PageTable auf lvl 1:");
    for i in 0..16 {
        kprintln!("{}: Address=0x{:x}, Flags={:#b}", i, page_table_1.entries[i].get_addr().raw(), page_table_1.entries[i].get_flags().bits());
    }
}
