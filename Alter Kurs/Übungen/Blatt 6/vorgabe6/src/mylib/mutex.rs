/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: mutex                                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Mutex with wait_queue. It will block threads calling 'lock', if ║
   ║         the lock is already held by another thread. When the lock is    ║
   ║         freed a waiting thread is deblocked (put into ready queue).     ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 13.6.2024                 ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use alloc::boxed::Box;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::kernel::cpu;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread::Thread;
use crate::mylib::queue::Queue;
use crate::mylib::spinlock::Spinlock;


/**
 Description: Mutex
*/pub struct Mutex {
    lock: AtomicBool,
    wait_queue: Spinlock<Queue<Box<Thread>>>, // blockierte Threads
}

// Gleiche unsafe Implementierung wie in 'std::sync::Mutex'
unsafe impl Sync for Mutex {}
unsafe impl Send for Mutex {}

impl Mutex {
    pub const fn new() -> Mutex {
			
			  /* Hier muss Code eingefuegt werden */

    }

    /**
     Description: Get the mutex.
    */
    pub fn lock(&self) -> MutexGuard {

			  /* Hier muss Code eingefuegt werden */

    }

    /**
     Description: Free the mutex. Called from `drop` in the `MutexGuard`
    */
    fn unlock(&self) {

			  /* Hier muss Code eingefuegt werden */

    }

}

/**
Description: Mutex guard used by Mutex to automatically call `unlock`
             for the mutex in case the guard is dropped.
*/
pub struct MutexGuard<'a> {
    lock: &'a Mutex,
}


/**
Description: Implementation for `drop()` which will call `unlock` on the mutex
*/
impl<'a> Drop for MutexGuard<'a> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}
