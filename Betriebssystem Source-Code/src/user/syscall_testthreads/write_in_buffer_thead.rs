use alloc::boxed::Box;
use alloc::string::ToString;

use crate::kernel::syscall::kernal_test_buffer::BUFFER;
use crate::kernel::syscall::user_api::{
    usr_getlastkey, usr_hello_world_print, usr_read, usr_write,
};
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread::Thread;

pub extern "C" fn write_in_buffer_thread_entry() {
    // Writing into Buffer
    // Text zerflücken in Byte-Array
    const TEXT: &str = "Hallo im Buffer";
    const LENGTH: usize = TEXT.len();

    let mut buffer: [u8; 15] = [0u8; LENGTH];
    buffer.copy_from_slice(TEXT.as_bytes());
    let pointer_buffer: *const u8 = buffer.as_ptr();

    // Text in Kernel-Buffer schreiben
    vprintln!(
        "Syscall write: {}",
        usr_write(pointer_buffer, LENGTH as u64)
    );

    // Prüfen ob das jetzt im Buffer steht
    let buffer_read: [u8; 15] = [0u8; LENGTH];
    let pointer_buffer_read: *mut u8 = buffer.as_mut_ptr();
    vprintln!(
        "Syscall write: {}",
        usr_read(pointer_buffer_read, LENGTH as u64)
    );

    // Ausgeben des gelesenem
    vprint!("Aus dem Kernel gelesen: ");
    unsafe {
        for i in 0..LENGTH {
            let value: u8 = *pointer_buffer_read.add(i);
            vprint!("{}", value as char);
        }
    }
    vprintln!("");
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
