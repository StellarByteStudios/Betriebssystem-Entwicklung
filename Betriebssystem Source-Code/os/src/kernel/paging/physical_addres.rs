use core::ops::Add;

// letzte nutzbare physikalische Adresse
// (notwendig fuer das 1:1 mapping des Kernels in den Page-Tables)
pub static mut MAX_PHYS_ADDR: PhysAddr = PhysAddr(0);

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
