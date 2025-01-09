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
    pub fn flags_for_kernel_pages() -> Self {
        let kernel_flags = PTEflags {
            bits: 0b0000_0000_0000_0011,
        };
        return kernel_flags;
        //return Self::flags_for_user_pages();
    }

    pub fn flags_for_user_pages() -> Self {
        let user_flags = PTEflags {
            bits: 0b0000_0000_0000_0111,
        };
        return user_flags;
    }
}
