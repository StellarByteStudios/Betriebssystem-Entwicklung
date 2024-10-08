use alloc::boxed::Box;
use alloc::string::ToString;

use crate::kernel::cpu;
use crate::kernel::threads::thread::Thread;
use crate::kernel::threads::{scheduler, thread};
use crate::mylib::delay;

pub extern "C" fn hello_world_thread_entry() {
    //    let tid = scheduler::get_active_tid();
    //    println!("Hello World! thread-id = {}", tid);
    // kprintln!("Hello World");
    //    let val = cpu::inb(1);
    loop {
        vprint!("U");

        /*
        let mut x: u64 = 0;
        loop {
            x = x + 1;
            if x > 100000000 {
                break;
            }
        } */
        delay::delay(100);
    }
}

pub fn init() {
    let idle_thread: Box<Thread> = thread::Thread::new_name(
        scheduler::next_thread_id(),
        hello_world_thread_entry,
        true,
        "user-hello-Thread".to_string(),
    );
    scheduler::Scheduler::ready(idle_thread);
}
