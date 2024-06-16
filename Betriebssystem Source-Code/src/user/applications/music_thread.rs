use crate::kernel::threads::scheduler::{self, Scheduler};
use crate::kernel::threads::thread::Thread;
use alloc::boxed::Box;
use crate::devices::{cga, pcspk};


pub fn init() -> usize {
    // ID holen
    let thread_id: usize = scheduler::next_thread_id();
    // Thread anlegen
    let coop_thread_loop: Box<Thread> = Thread::new(thread_id, music_thread);
    // Thread beim Scheduler registrieren
    scheduler::Scheduler::ready(coop_thread_loop);
    // ID zurückgeben
    return thread_id;
}


#[no_mangle]
extern "C" fn music_thread(myself: *mut Thread) {
    // Musik abspielen
    pcspk::starwars_imperial();

    // Thread beendent
    Scheduler::exit();
}
