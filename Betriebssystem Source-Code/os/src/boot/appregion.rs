use core::fmt;

use super::multiboot::MultibootInfo;

// Beschreibt eine App, die separat vom Kernel compiliert wurde
pub struct AppRegion {
    pub start: u64,
    pub end: u64,
}

impl fmt::Debug for AppRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppRegion [0x{:x}, 0x{:x}]", self.start, self.end)
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
    });
}
