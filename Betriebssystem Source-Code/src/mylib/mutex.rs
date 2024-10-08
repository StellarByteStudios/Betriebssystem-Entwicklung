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
use core::ptr::{null, null_mut};
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Spin;

use crate::kernel::cpu;
use crate::kernel::threads::scheduler::{self, Scheduler, SCHEDULER};
use crate::kernel::threads::thread::Thread;
use crate::mylib::queue::Queue;
use crate::mylib::spinlock::Spinlock;

use super::spinlock::SpinlockGuard;

/**
 Description: Mutex
*/
pub struct Mutex {
    lock: AtomicBool,
    wait_queue: Spinlock<Queue<Box<Thread>>>, // blockierte Threads
}

// Gleiche unsafe Implementierung wie in 'std::sync::Mutex'
unsafe impl Sync for Mutex {}
unsafe impl Send for Mutex {}

impl Mutex {
    pub const fn new() -> Mutex {
        Mutex {
            lock: AtomicBool::new(false),
            wait_queue: Spinlock::new(Queue::new()),
        }
    }

    /**
     Description: Get the mutex.
    */
    pub fn lock(&self) -> MutexGuard {
        loop {
            // Ist bereits geblockt
            let allready_locked: Result<bool, bool> =
                self.lock
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);

            // Ist noch frei?
            if allready_locked.is_ok() {
                return MutexGuard { lock: &self };
            }

            // switch vorbereiten
            // Funktioniert nicht mit Vorgegebenem Allokator
            //let threads2switch: (*mut Thread, *mut Thread) = scheduler::prepare_block();
            let threads2switch: (*mut Thread, *mut Thread) = (null_mut(), null_mut());

            // Geblockten Thread in die Warteschlange machen
            self.wait_queue
                .lock()
                .enqueue(unsafe { Box::from_raw(threads2switch.0) });

            // Thread zu neuem Wechseln
            Thread::switch(threads2switch.0, threads2switch.1);
        }
    }

    /**
     Description: Free the mutex. Called from `drop` in the `MutexGuard`
    */
    fn unlock(&self) {
        // Queue als lock holen
        let mut queue: SpinlockGuard<Queue<Box<Thread>>> = self.wait_queue.lock();

        // Nachschauen, ob die Queue leer ist
        if queue.is_empty() {
            self.lock.store(false, Ordering::SeqCst);
            return;
        }

        // Ist das nächste auch valid
        let next: Option<Box<Thread>> = queue.dequeue();

        if next.is_none() {
            self.lock.store(false, Ordering::SeqCst);
            return;
        }

        // Nächsten Thread aus wait_queue in die queue des Schedulers packen
        // Funktioniert nicht mit vorgegebenem Scheduler
        //scheduler::deblock(Box::into_raw(next.unwrap()));

        // Queue freigeben
        drop(queue);

        // Noch freigeben
        self.lock.store(false, Ordering::SeqCst);
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
