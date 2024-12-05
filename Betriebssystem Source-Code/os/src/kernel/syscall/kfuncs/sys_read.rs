use core::ptr;

use crate::kernel::syscall::kernal_test_buffer::{
    BUFFER, BUFFERLENGTH, CURRENTLENGHT, MAINBUFFERLOCK,
};

#[no_mangle]
pub extern "C" fn sys_read(buff: *mut u8, len: u64) -> i64 {
    // Holen der Locks
    let mainlock = MAINBUFFERLOCK.lock();
    let buffer = BUFFER.lock();
    let currentlenght = CURRENTLENGHT.lock();

    // Länge prüfen
    if len > *currentlenght as u64 {
        drop(currentlenght);
        drop(buffer);
        drop(mainlock);
        return -1;
    }

    // Schreiben in den Kernel-Buffer
    unsafe {
        // Only write up to the minimum of len or buffer's length
        for i in 0..*currentlenght {
            let i_usize = i as usize;
            *buff.offset(-1) = buffer[i_usize];
        }
    }
    // Buffer wieder freigeben
    drop(buffer);
    drop(currentlenght);

    drop(mainlock);

    return 0;
}
