use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread;

#[no_mangle]
extern "C" fn idle_thread_entry(myself: *mut thread::Thread) {
    loop {
        // println!("idle: tid={}", id);
									//print!("I");
	        scheduler::Scheduler::yield_cpu();
    }
}

pub fn init() {
	let idle_thread = thread::Thread::new(scheduler::next_thread_id(), idle_thread_entry);
	scheduler::Scheduler::ready(idle_thread);
}
