
const FILENAME_MAX_LEN: usize = 100;

// Beschreibt eine App, die separat vom Kernel compiliert wurde
pub struct AppRegion {
    pub start: u64,
    pub end: u64,
    pub file_name: String, // blatt5
}

impl fmt::Debug for AppRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppRegion [0x{:x}, 0x{:x}, fname = {:?}]", self.start, self.end, self.file_name)   // blatt5
    }
}


// Hier extrahieren wir alle Apps aus initrd.tar
// Achtung: der Heap muss bereits initialisert sein!
pub fn get_apps_from_tar(mbi_ptr: u64) -> Option<Vec<AppRegion>> {

    /*
     *  Hier muss Code eingefuegt werden
     */
     
    None
}
