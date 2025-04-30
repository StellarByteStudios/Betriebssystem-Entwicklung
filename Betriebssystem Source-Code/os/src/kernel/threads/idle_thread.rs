use alloc::{boxed::Box, string::ToString};

use crate::{
    kernel::{
        cpu,
        threads::{scheduler, thread, thread::Thread},
    },
    utility::delay,
};

pub extern "C" fn idle_thread_entry() {
    scheduler::set_initialized();
    loop {
        delay::delay(100);
    }
}

pub fn init(pid: usize) -> Box<Thread> {
    let idle_thread: Box<Thread> =
        thread::Thread::new_name(pid, idle_thread_entry, true, "Idle-Thread".to_string());

    return idle_thread;
}
