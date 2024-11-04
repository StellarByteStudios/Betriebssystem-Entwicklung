use alloc::boxed::Box;
use alloc::string::ToString;

use crate::kernel::cpu;
use crate::kernel::syscall::user_api::{usr_hello_world, usr_hello_world_print};
use crate::kernel::threads::thread::Thread;
use crate::kernel::threads::{scheduler, thread};
use crate::mylib::delay;

pub extern "C" fn hello_world_thread_entry() {
    //    let tid = scheduler::get_active_tid();
    //    println!("Hello World! thread-id = {}", tid);
    //kprintln!("Hello World");

    let mut i: u64 = 1;

    loop {
        vprintln!("Syscall Helloworld mit Argumenten");
        // Funktioniert nicht im Usermode
        //let val: u8 = cpu::inb(0);
        // Funktioniert nicht im Usermode
        //kprintln!("Nicht ausführbar!");

        // Serielle Ausgabe über Syscall
        usr_hello_world();
        usr_hello_world_print(i);
        i = i + 1;

        delay::delay(250);
    }
}

pub fn init() {
    let idle_thread: Box<Thread> = thread::Thread::new_name(
        scheduler::next_thread_id(),
        hello_world_thread_entry,
        false,
        "user-hello-Thread".to_string(),
    );
    scheduler::Scheduler::ready(idle_thread);
}
