use core::ptr;

use crate::kernel::syscall::kernal_test_buffer::{
    BUFFER, BUFFERLENGTH, CURRENTLENGHT, MAINBUFFERLOCK,
};

#[no_mangle]
pub extern "C" fn sys_write(buff: *mut u8, len: u64) -> i64 {
    // Länge prüfen
    if len > BUFFERLENGTH as u64 {
        return -1;
    }

    // Holen der Locks
    let mainlock = MAINBUFFERLOCK.lock();
    let mut buffer = BUFFER.lock();

    // Schreiben in den Kernel-Buffer
    unsafe {
        // Only write up to the minimum of len or buffer's length
        for i in 0..len {
            buffer[i as usize] = ptr::read(buff.add(i as usize));
        }
    }
    // Buffer wieder freigeben
    drop(buffer);

    // Länge anpassen
    let mut bufferlenght = CURRENTLENGHT.lock();

    *bufferlenght = len;

    drop(bufferlenght);

    drop(mainlock);

    return 0;
}
