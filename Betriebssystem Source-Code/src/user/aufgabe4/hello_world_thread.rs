use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread::Thread;
use alloc::boxed::Box;

pub fn init() {
    // ID holen
    let thread_id: usize = scheduler::next_thread_id();
    // Thread anlegen
    let hello_world_thread: Box<Thread> = Thread::new(thread_id, hello_world_thread_entry);
    // Thread beim Scheduler registrieren
    scheduler::Scheduler::ready(hello_world_thread);
     
}

#[no_mangle]
extern "C" fn hello_world_thread_entry(myself: *mut Thread) {
    println!("Hallo Welt von einem Thread!");
}


