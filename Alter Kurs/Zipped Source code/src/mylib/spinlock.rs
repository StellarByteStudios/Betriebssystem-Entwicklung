/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: spinlock                                                        ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Basic generic spinlock using atomics.                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 13.6.2024                 ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use core::arch::asm;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/**
 Description: Spinlock
*/
pub struct Spinlock<T: ?Sized> {
    lock: AtomicBool,    // false = lock is not set, true = lock is set
    data: UnsafeCell<T>, // unsafe to allow mutable access through non-mutable ref (see SpinlockGuard)
}

// required for sharing access to Spinlock between threads
unsafe impl<T> Sync for Spinlock<T> where T: Send {}
unsafe impl<T> Send for Spinlock<T> where T: Send {}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Spinlock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /**
     Description: Spin until we successfully acquire the lock
    */
    pub fn lock(&self) -> SpinlockGuard<T> {
        loop {
            let res = self
                .lock
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
            if res.is_ok() {
                break;
            }
            // Hint to the processor to reduce power consumption
            unsafe {
                asm!("pause", options(nomem, nostack));
            }
        }

        // Return a SpinlockGuard which will allow mutable access to 'data'
        // and call 'unlock' if it is dropped.
        SpinlockGuard { lock: &self }
    }

    /**
     Description: Free the spinlock. Called from `drop` in the `SpinlockGuard`
    */
    fn unlock(&self) {
        self.lock.store(false, Ordering::SeqCst);
    }
}

/**
Description: Spinlock guard used by Spinlock to automatically call `unlock`
             for the spinlock in case the guard is dropped. And it also
             provides mutable and non-mutable to data protected by the lock.
*/
pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

/**
Description: Implementation for `as_ref()`
*/
impl<'a, T> Deref for SpinlockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

/**
Description: Implementation for `as_mut()`
*/
impl<'a, T> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

/**
Description: Implementation for `drop()` which will call `unlock` on the spinlock
*/
impl<'a, T> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}
