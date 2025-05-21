/*****************************************************************************
 *                                                                           *
 *                  P A G E S                                                *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Hier sind die Funktionnen fuer die Page-Tables.          *
 *                                                                           *
 * Autor:           Michael Schoettner, 13.9.2023                            *
 *****************************************************************************/
use core::{ptr, ptr::null_mut, slice, sync::atomic::AtomicUsize};

use usrlib::utility::mathadditions::math::pow_usize;
use x86_64::VirtAddr;

use super::{frames::pf_alloc, pagetable_entry::PageTableEntry, physical_addres::PhysAddr};
use crate::{
    boot::{
        appregion::AppRegion,
        multiboot,
        multiboot::{MultibootFramebuffer, MultibootInfo},
    },
    consts,
    consts::{PAGE_SIZE, STACK_SIZE, USER_STACK_VM_END, USER_STACK_VM_START},
    kernel::{
        interrupts::intdispatcher,
        paging::{frames, pagetable_flags::PTEflags},
        processes::{process_handler, vma},
    },
};

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
    // Ausrechnen wie viel Seiten "gemappt" werden muessen
    let max_phys_addr: usize = PhysAddr::get_max_phys_addr().raw() as usize;
    let nr_of_pages = (max_phys_addr + 1) / PAGE_SIZE;

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
    let how_many_pages = (((video_pitch * video_height) / PAGE_SIZE as u32) + 1) as usize;

    // Speichern, wo und welche Addressen für den Videospeicher brauchen
    VIDEO_START_ADDRESS.store(page_video_adress, core::sync::atomic::Ordering::SeqCst);
    NUMBER_OF_VIDEO_TABLES.store(how_many_pages, core::sync::atomic::Ordering::SeqCst);

    pml4_table.mmap_general(page_video_adress, how_many_pages, true, true, false, 0);

    return pml4_addr;
}

// Diese Funktion richtet ein Mapping fuer den User-Mode Stack ein
pub fn pg_mmap_user_stack(pid: usize, pml4_addr: PhysAddr) -> *mut u8 {
    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_thread_table;
    unsafe { pml4_thread_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // VMA berechnen und anlegen
    let new_vma = vma::VMA::new(
        USER_STACK_VM_START,
        USER_STACK_VM_START + STACK_SIZE,
        vma::VmaType::Stack,
    );

    // Passt diese VMA noch?
    let success = process_handler::add_vma_to_process(pid, new_vma);

    if !success {
        return null_mut();
    }

    // Stack mappen
    pml4_thread_table.mmap_general(
        USER_STACK_VM_START,
        STACK_SIZE / PAGE_SIZE,
        false,
        false,
        false,
        0,
    );

    // Startadresse zurück geben
    return USER_STACK_VM_START as *mut u8;
}

// Returned True wenn alles funktioniert hat; False bei fehler
pub fn pg_mmap_extend_user_stack(pid: usize, pml4_addr: PhysAddr, address_to_map: usize) -> bool {
    // Page der Adresse herausfinden
    let start_address = address_to_map & 0xFFFF_FFFF_FFFF_F000; // Unterste 12 Bit abschneiden
    let end_address = start_address + PAGE_SIZE - 1;

    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_thread_table;
    unsafe { pml4_thread_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // VMA berechnen und anlegen
    let new_vma = vma::VMA::new(start_address, end_address, vma::VmaType::Stack);

    // Passt diese VMA noch?
    let success = process_handler::add_vma_to_process(pid, new_vma);

    if !success {
        return false;
    }
    let number_of_bytes = end_address - start_address + 1;
    let number_of_pages = number_of_bytes / PAGE_SIZE;

    // Stack mappen
    pml4_thread_table.mmap_general(start_address, 1, false, false, false, 0);

    return true;
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
// Bei fehler Return false, true sonst
pub fn pg_mmap_user_app(pid: usize, pml4_addr: PhysAddr, app: AppRegion) -> bool {
    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_thread_table;
    unsafe { pml4_thread_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // Größe der App berechnen
    let app_lenght: usize = (app.end - app.start) as usize;
    let app_pages = (app_lenght / PAGE_SIZE) + 1;

    kprintln!(
        "Lege Mapping für App \"{}\" an. Sie brauch {} Pages, denn sie ist {:#x} groß",
        app.file_name.as_str(),
        app_pages,
        app_lenght
    );

    // Physische Speicherzellen anfordern
    let app_phys_start_address = pf_alloc(app_pages, false);

    // App kopieren
    unsafe {
        let app_data: &[u8] = slice::from_raw_parts(app.start as *const u8, app_lenght);
        ptr::copy_nonoverlapping(
            app_data.as_ptr(),
            app_phys_start_address.as_mut_ptr(),
            app_lenght,
        );
    }

    // VMA berechnen und anlegen
    let vma_end =
        consts::USER_CODE_VM_START + (((app.end - app.start) as usize / PAGE_SIZE) + 1) * PAGE_SIZE;
    let new_vma = vma::VMA::new(consts::USER_CODE_VM_START, vma_end, vma::VmaType::Code);

    // Past diese VMA noch?
    let success = process_handler::add_vma_to_process(pid, new_vma);

    if !success {
        return false;
    }

    // Einmappen der Anwendung (Code)
    pml4_thread_table.mmap_general(
        consts::USER_CODE_VM_START,
        ((app.end - app.start) as usize / PAGE_SIZE) + 1,
        false,
        false,
        true,
        app_phys_start_address.raw() as usize,
    );

    return true;
}

// Diese Funktion richtet ein Mapping fuer den User-Mode Stack ein
pub fn pg_mmap_user_heap(pid: usize, addr: usize, len: usize) -> u64 {
    // PageTable holen
    let pml4_addr = process_handler::get_pml4_address_by_pid(pid);

    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_thread_table;
    unsafe { pml4_thread_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // VMA berechnen und anlegen
    let vma_end = addr + ((len / PAGE_SIZE) + 1) * PAGE_SIZE;
    let new_vma = vma::VMA::new(addr, vma_end, vma::VmaType::Heap);

    // Past diese VMA noch?
    let success = process_handler::add_vma_to_process(pid, new_vma);

    if !success {
        return 1;
    }

    // Einmappen des Speichers
    pml4_thread_table.mmap_general(addr, (len / PAGE_SIZE) + 1, false, false, false, 0);

    return 0;
}

pub fn pg_mmap_user_environment(pid: usize, start_address: usize, len: usize) -> PhysAddr {
    // PageTable holen
    let pml4_addr = process_handler::get_pml4_address_by_pid(pid);

    // Type-Cast der pml4-Tabllenadresse auf "PageTable"
    let pml4_thread_table;
    unsafe { pml4_thread_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // VMA berechnen und anlegen
    let vma_end = start_address + ((len / PAGE_SIZE) + 1) * PAGE_SIZE;
    let new_vma = vma::VMA::new(start_address, vma_end, vma::VmaType::Environment);
    // Past diese VMA noch?
    let success = process_handler::add_vma_to_process(pid, new_vma);
    kprintln!("VMA für Environment angelegt");
    if !success {
        return PhysAddr::new(0);
    }

    // Wie viele Pages brauche ich für meine Argumente
    let env_page_count = (len / PAGE_SIZE) + 1;

    // mappen der Environment Pages
    pml4_thread_table.mmap_general(start_address, env_page_count, false, false, false, 0);

    // Holen der physischen Startadresse
    let raw_phys_address = get_physical_address(pml4_addr, start_address);

    return raw_phys_address;
}

fn get_physical_address(pml4_addr: PhysAddr, virtual_address: usize) -> PhysAddr {
    // Table Adresse zum Pointer
    let pml4_table;
    unsafe { pml4_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    // Durch alle Tabellen durchsteppen
    let page_table_4_entry: PageTableEntry =
        pml4_table.entries[get_index_in_table(virtual_address, 3)];

    let page_table_3: &mut PageTable =
        unsafe { &mut *(page_table_4_entry.get_addr().as_mut_ptr::<PageTable>()) };

    let page_table_3_entry: PageTableEntry =
        page_table_3.entries[get_index_in_table(virtual_address, 2)];

    let page_table_2: &mut PageTable =
        unsafe { &mut *(page_table_3_entry.get_addr().as_mut_ptr::<PageTable>()) };

    let page_table_2_entry: PageTableEntry =
        page_table_2.entries[get_index_in_table(virtual_address, 1)];

    let page_table_1: &mut PageTable =
        unsafe { &mut *(page_table_2_entry.get_addr().as_mut_ptr::<PageTable>()) };

    let page_table_1_entry: PageTableEntry =
        page_table_1.entries[get_index_in_table(virtual_address, 0)];

    // Index auf die Physische Adresse
    let right_index = get_index_in_table(virtual_address, 0);

    // Physische Adresse holen
    return page_table_1.entries[right_index].get_addr();
}

/*
* Debug-Methode: Übersetzt eine virtuelle Adresse und gibt die ersten paar Einträge
* auf Lvl. 1 aus (enthält echte physische Addressen)
*/
pub fn where_physical_address(pml4_addr: PhysAddr, virtual_address: usize) {
    let pml4_table;
    unsafe { pml4_table = &mut *(pml4_addr.as_mut_ptr::<PageTable>()) }

    kprintln!("\n\n= = = = = = = = = mapping of virtual Address = = = = = = = = =");
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

    kprintln!("Eintrag den wir suchen in der Table lvl 1: ");
    let page_table_1_entry: PageTableEntry =
        page_table_1.entries[get_index_in_table(virtual_address, 0)];

    kprintln!(
        "Physical Address of Entry: 0x{:x}, Flags: {:#b}",
        page_table_1_entry.get_addr().raw(),
        page_table_1_entry.get_flags().bits()
    );

    kprintln!("Erste Einträge der PageTable auf lvl 1:");
    for i in 0..16 {
        kprintln!(
            "{}: Address=0x{:x}, Flags={:#b}",
            i,
            page_table_1.entries[i].get_addr().raw(),
            page_table_1.entries[i].get_flags().bits()
        );
    }
    let right_index = get_index_in_table(virtual_address, 0);
    kprintln!(
        "Der gesuchte Eintrag ist:\n\t{:}: Address=0x{:x}, Flags={:#b}",
        right_index,
        page_table_1.entries[right_index].get_addr().raw(),
        page_table_1.entries[right_index].get_flags().bits()
    );
    kprintln!("= = = = = = = = = = = = = = = = = = = =\n\n")
}
