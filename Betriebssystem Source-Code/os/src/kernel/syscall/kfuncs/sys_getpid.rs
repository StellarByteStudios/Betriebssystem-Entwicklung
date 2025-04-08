use crate::{kernel::threads::scheduler};

#[no_mangle]
pub extern "C" fn sys_getpid() -> u64 {
    let tid = scheduler::get_active_pid();
    return tid as u64;
}
