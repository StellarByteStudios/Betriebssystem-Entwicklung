use crate::devices::pit;
use crate::kernel::syscall::kernal_test_buffer::{BUFFER, CURRENTLENGHT, MAINBUFFERLOCK};

#[no_mangle]
pub extern "C" fn sys_get_systime() -> u64 {
    return pit::get_systime();
}