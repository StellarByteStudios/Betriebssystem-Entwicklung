use core::{fmt, slice};

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use tar_no_std::TarArchiveRef;
use super::multiboot::MultibootInfo;

// Beschreibt eine App, die separat vom Kernel compiliert wurde
pub struct AppRegion {
    pub start: u64,
    pub end: u64,
    pub file_name: String,
}

impl fmt::Debug for AppRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppRegion [0x{:x}, 0x{:x}, file_name = {:?}]",
            self.start, self.end, self.file_name
        )
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct ModEntry {
    pub start: u32,
    pub end: u32,
    pub string: u32,
    pub reserved: u32,
}

impl fmt::Debug for ModEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.start;
        let e = self.end;

        write!(f, "Module [0x{:x}, 0x{:x}]", s, e)
    }
}

// Sucht ein Boot-Modul -> ist bei uns eine App
pub fn get_app(mbi_ptr: u64) -> Option<AppRegion> {
    // Erstmal Multiboot auslesen
    let multiboot_info: &MultibootInfo = unsafe { MultibootInfo::read(mbi_ptr) };

    // Infos holen
    let app_count: u32 = multiboot_info.mods_count;
    let app_mod_entry: ModEntry =
        unsafe { *((multiboot_info.mods_addr as *const usize) as *const ModEntry) };
    let app_start: u64 = app_mod_entry.start as u64;
    let app_end: u64 = app_mod_entry.end as u64;

    return Some(AppRegion {
        start: app_start,
        end: app_end,
        file_name: String::new(),
    });
}

// Hier extrahieren wir alle Apps aus initrd.tar
// Achtung: der Heap muss bereits initialisert sein!
pub fn get_apps_from_tar(mbi_ptr: u64) -> Option<Vec<AppRegion>> {
    // Erstmal Multiboot auslesen
    let multiboot_info: &MultibootInfo = unsafe { MultibootInfo::read(mbi_ptr) };

    // Infos holen
    let app_count: u32 = multiboot_info.mods_count;
    let tar_mod_entry: ModEntry =
        unsafe { *((multiboot_info.mods_addr as *const usize) as *const ModEntry) };
    let tar_start: u64 = tar_mod_entry.start as u64;
    let tar_end: u64 = tar_mod_entry.end as u64;
    
    kprintln!("!MULTIBOOT INFO !");
    kprintln!("Mod-Count: {}, Tar-Start {:#x}, Tar-End: {:#x}", app_count, tar_start, tar_end);

    // Daten aus dem Archiv als rohen Memory-Slice schreiben
    let tar_data: &[u8] = unsafe { slice::from_raw_parts(tar_start as *const u8, (tar_end - tar_start) as usize) };
    
    // Archiv aus dem Slice erstellen
    let archive: TarArchiveRef = TarArchiveRef::new(tar_data);

    // Alle Entries aus dem Archiv holen
    let mut entries = archive.entries().collect::<Vec<_>>();
    kprintln!("Number of apps loaded: {}", entries.len());
    // Nichts drin?
    if entries.len() < 1 {
        return None;
    }
    
    // Entries in Apps verwandeln
    let mut apps: Vec<AppRegion> = Vec::new();
    for entry in entries {
        let filename = entry.filename().as_str().to_string();
        let app_start_address = entry.data().as_ptr() as u64;
        let app_end_address = entry.data().as_ptr() as u64 + entry.data().len() as u64;
        
        apps.push(AppRegion {start: app_start_address, end: app_end_address, file_name: filename});
    }

    return Some(apps);
}
