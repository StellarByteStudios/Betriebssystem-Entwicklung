use alloc::{boxed::Box, string::ToString};

use crate::{
    kernel::{
        cpu,
        processes::process_handler,
        threads::{scheduler, thread, thread::Thread},
    },
    utility::delay,
};

pub extern "C" fn idle_thread_entry() {
    scheduler::set_initialized();

    let mut iterator = 0;
    loop {
        iterator = iterator + 1;
        delay::delay(100);

        if iterator % 3 == 0 {
            kprintln!("Cleanup in Idle-Thread");
            process_handler::cleanup();
        }
    }
}

pub fn init(pid: usize) -> Box<Thread> {
    let idle_thread: Box<Thread> =
        thread::Thread::new_name(pid, idle_thread_entry, true, "Idle-Thread".to_string());

    return idle_thread;
}
