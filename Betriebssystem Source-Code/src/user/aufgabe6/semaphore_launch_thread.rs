use crate::{
    kernel::threads::{
        scheduler::{self, Scheduler},
        thread::Thread,
    },
    user::aufgabe5::music_thread,
};
use alloc::boxed::Box;

use super::semaphore_demo;

pub fn init() {
    // ID holen
    let thread_id: usize = scheduler::next_thread_id();
    // Thread anlegen
    let demo_thread: Box<Thread> = Thread::new(thread_id, semaphore_launch_thread);
    // Thread beim Scheduler registrieren
    scheduler::Scheduler::ready(demo_thread);
}

#[no_mangle]
extern "C" fn semaphore_launch_thread(myself: *mut Thread) {
    // Loopthreads anlegen
    // Thread anlegen
    let loop_thread1: usize = semaphore_demo::init();
    let loop_thread2: usize = semaphore_demo::init();
    let loop_thread3: usize = semaphore_demo::init();
    let music_thread: usize = music_thread::init();

    /*
    // Thread der nach 1000 Iterationen gekillt werden soll
    let victim_id: usize = loop_thread1;

    //Scheduler::yield_cpu();

    // Counter parallelen Hochzählen
    let mut counter: usize = 0;
    loop {

        // Schauen ob lebenszeit Abgelaufen ist
        if counter >= 1000 {
            // Anderen Thread mitnehmen
            Scheduler::kill(victim_id);

            // Sich selbst beenden
            Scheduler::exit();
        }

        // Hochzählen des Counters
        counter += 1;

        // Ansonsten Weitergeben an nächsten Thread
        //kprintln!("Koordinator-Thread ist durchgelaufen {}", counter);
        Scheduler::yield_cpu();
    } */
}
