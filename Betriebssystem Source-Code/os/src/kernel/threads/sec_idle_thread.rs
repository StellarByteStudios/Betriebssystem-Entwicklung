use crate::kernel::cpu;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread;
use crate::kernel::threads::thread::Thread;

use alloc::boxed::Box;
use alloc::string::ToString;
use usrlib::utility::delay;

pub extern "C" fn idle_thread_entry() {
    scheduler::set_initialized();
    loop {
        vprint!("I ");
        kprintln!("Idling...");

        delay::delay(100);
    }
}

pub fn init() {
    /*
    let idle_thread: Box<Thread> = thread::Thread::new_name(
        scheduler::next_thread_id(),
        idle_thread_entry,
        true,
        "sec_Idle-Thread".to_string(),
    );*/
    //scheduler::Scheduler::ready(idle_thread);
}
