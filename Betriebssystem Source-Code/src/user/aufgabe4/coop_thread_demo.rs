use crate::kernel::threads::scheduler::{self, Scheduler};
use crate::kernel::threads::thread::Thread;
use alloc::boxed::Box;

use super::coop_thread_loop;


pub fn init(){
    // ID holen
    let thread_id: usize = scheduler::next_thread_id();
    // Thread anlegen
    let coop_demo_thread: Box<Thread> = Thread::new(thread_id, coop_demo_thread_entry);
    // Thread beim Scheduler registrieren
    scheduler::Scheduler::ready(coop_demo_thread);
}


#[no_mangle]
extern "C" fn coop_demo_thread_entry(myself: *mut Thread) {

    // Loopthreads anlegen
    // Thread anlegen
    let loop_thread1: usize = coop_thread_loop::init();
    let loop_thread2: usize = coop_thread_loop::init();
    let loop_thread3: usize = coop_thread_loop::init();



    // Thread der nach 1000 Iterationen gekillt werden soll
    let victim_id: usize = loop_thread2;

    // Counter parallelen Hochzählen
    let mut counter: usize = 0;
    loop {
        // Hochzählen des Counters
        counter += 1;

        // Schauen ob lebenszeit Abgelaufen ist
        if counter >= 1000 {
            // Anderen Thread mitnehmen
            Scheduler::kill(victim_id);

            // Sich selbst beenden
            Scheduler::exit();
        }

        // Ansonsten Weitergeben an nächsten Thread 
        //kprintln!("Koordinator-Thread ist durchgelaufen {}", counter);
        Scheduler::yield_cpu();
    }
}
