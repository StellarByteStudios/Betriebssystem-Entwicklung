use alloc::boxed::Box;
use alloc::string::ToString;

use crate::kernel::syscall::kernal_test_buffer::BUFFER;
use crate::kernel::syscall::user_api::{usr_getlastkey, usr_hello_world_print, usr_write};
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread::Thread;

pub extern "C" fn write_in_buffer_thread_entry() {
    // Writing into Buffer
    const TEXT: &str = "Hallo im Buffer";
    const LENGTH: usize = TEXT.len();
    let mut buffer = [0u8; LENGTH];
    buffer.copy_from_slice(TEXT.as_bytes());

    let pointer_buffer = buffer.as_mut_ptr();
    vprintln!(
        "Syscall write: {}",
        usr_write(pointer_buffer, LENGTH as u64)
    );

    // Prüfen ob das jetzt im Buffer steht
    vprintln!("Was im Buffer steht: {:?}", &BUFFER.lock()[0..LENGTH])
}

pub fn init() {
    let idle_thread: Box<Thread> = Thread::new_name(
        scheduler::next_thread_id(),
        write_in_buffer_thread_entry,
        false,
        "write in buffer-Thread".to_string(),
    );
    scheduler::Scheduler::ready(idle_thread);
}
