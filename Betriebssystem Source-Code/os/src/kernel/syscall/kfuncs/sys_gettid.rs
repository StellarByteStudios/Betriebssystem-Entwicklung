use crate::{kernel::threads::scheduler, mylib};

#[no_mangle]
pub extern "C" fn sys_gettid() -> u64 {
    let tid = scheduler::get_active_tid();
    return tid as u64;
}
