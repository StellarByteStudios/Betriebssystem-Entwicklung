use alloc::boxed::Box;
use alloc::string::ToString;
use usrlib::kernel::syscall::user_api::usr_getlastkey;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread::Thread;

pub extern "C" fn get_last_key_thread_entry() {
    vprintln!(
        "Syscall get_last_key: {}",
        (char::from_u32(usr_getlastkey() as u32).unwrap_or('*'))
    );
    return;
}

pub fn init() {
    let idle_thread: Box<Thread> = Thread::new_name(
        scheduler::next_thread_id(),
        get_last_key_thread_entry,
        false,
        "get-last-key-Thread".to_string(),
    );
    scheduler::Scheduler::ready(idle_thread);
}
