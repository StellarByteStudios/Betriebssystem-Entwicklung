

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

        write!(f, "Module [0x{:x}, 0x{:x}]",s, e)
    }
}


// Sucht ein Boot-Modul -> ist bei uns eine App
pub fn get_app(mbi_ptr: u64) -> Option<AppRegion> {

   /*
    * Hier muss Code eingefuegt werden
    */

}
    
