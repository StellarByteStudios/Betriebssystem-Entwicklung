use alloc::boxed::Box;

use crate::{
    devices::cga,
    kernel::threads::{
        scheduler,
        thread::{self, Thread},
    },
    mylib::{
        delay,
        mutex::{Mutex, MutexGuard},
        spinlock::{Spinlock, SpinlockGuard},
    },
};

//static SPINLOCK: Spinlock<bool> = Spinlock::new(false);
static MUTEX: Mutex = Mutex::new();

pub fn init() -> usize {
    // ID holen
    let thread_id: usize = scheduler::next_thread_id();
    // Thread anlegen
    let coop_thread_loop: Box<Thread> = Thread::new(thread_id, synced_loop_thread_entry);
    // Thread beim Scheduler registrieren
    scheduler::Scheduler::ready(coop_thread_loop);
    // ID zurückgeben
    return thread_id;
}

/*

#[no_mangle]
extern "C" fn synced_loop_thread_entry(myself: *mut thread::Thread) {

    // Thread ID holen
    let my_tid: usize = thread::Thread::get_tid(myself);

    // Wie viele Threads laufen vorher? Müssen aus rechnung raus
    let prev_thread_count: usize = 2;

    // Counter parallelen Hochzählen
    let mut counter: usize = 0;

    loop {

        // Locken für die Synchronisierung
        let lock: SpinlockGuard<bool> = SPINLOCK.lock();

        // Ausgabe auf dem Bildschirm
        cga::setpos((5 + (my_tid-prev_thread_count) * 20) as u32, 10);
        //delay::delay(1);
        println!("Loop [{}] : {}", my_tid, counter);

        // Freigeben für die Synchronisierung
        drop(lock);

        // Hochzählen des Counters
        counter += 1;

        delay::delay(10);
    }
}
 */

#[no_mangle]
extern "C" fn synced_loop_thread_entry(myself: *mut thread::Thread) {
    // Thread ID holen
    let my_tid: usize = thread::Thread::get_tid(myself);

    // Wie viele Threads laufen vorher? Müssen aus rechnung raus
    let prev_thread_count: usize = 2;

    // Counter parallelen Hochzählen
    let mut counter: usize = 0;

    loop {
        // Locken für die Synchronisierung
        let lock: MutexGuard = MUTEX.lock();

        // Ausgabe auf dem Bildschirm
        cga::setpos((5 + (my_tid - prev_thread_count) * 20) as u32, 10);
        delay::delay(1);
        println!("Loop [{}] : {}", my_tid, counter);

        // Freigeben für die Synchronisierung
        drop(lock);

        // Hochzählen des Counters
        counter += 1;

        delay::delay(10);
    }
}
