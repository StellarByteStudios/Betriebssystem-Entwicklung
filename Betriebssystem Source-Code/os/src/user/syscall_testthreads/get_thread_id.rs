use alloc::boxed::Box;
use alloc::string::ToString;

use crate::kernel::syscall::user_api::{usr_getlastkey, usr_gettid};
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread::Thread;

pub extern "C" fn get_threadid_thread_entry() {
    vprintln!("Syscall get-threadid: {}", usr_gettid() as u64);
    return;
}

pub fn init() {
    let idle_thread: Box<Thread> = Thread::new_name(
        scheduler::next_thread_id(),
        get_threadid_thread_entry,
        false,
        "get-threadID-Thread".to_string(),
    );
    scheduler::Scheduler::ready(idle_thread);
}
