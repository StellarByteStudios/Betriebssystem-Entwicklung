use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread::Thread;

pub fn init() {
    let hello_world_thread = Thread::new(scheduler::next_thread_id(), hello_world_thread_entry);
    scheduler::Scheduler::ready(hello_world_thread);
}

#[no_mangle]
extern "C" fn hello_world_thread_entry() {
    println!("Hallo Welt von einem Thread!");
}
