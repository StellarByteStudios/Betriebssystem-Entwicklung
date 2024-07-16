use crate::kernel::cpu;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread;
use crate::kernel::threads::thread::Thread;
use alloc::boxed::Box;

pub fn init() {
    let idle_thread: Box<Thread> =
        thread::Thread::new(scheduler::next_thread_id(), idle_thread_entry);
    scheduler::Scheduler::ready(idle_thread);
}

#[no_mangle]
extern "C" fn idle_thread_entry(myself: *mut thread::Thread) {
    scheduler::set_initialized();

    loop {
        // verursacht Zeitweise Abstürze?
        cpu::halt();
    }
}
