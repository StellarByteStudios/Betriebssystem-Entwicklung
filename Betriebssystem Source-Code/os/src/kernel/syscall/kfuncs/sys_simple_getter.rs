use crate::{
    devices::{graphical::vga, pit},
    kernel::threads::scheduler,
};

#[no_mangle]
pub extern "C" fn sys_getpid() -> u64 {
    let tid = scheduler::get_active_pid();
    return tid as u64;
}

#[no_mangle]
pub extern "C" fn sys_gettid() -> u64 {
    let tid = scheduler::get_active_tid();
    return tid as u64;
}

#[no_mangle]
pub extern "C" fn sys_get_systime() -> u64 {
    return pit::get_systime();
}

#[no_mangle]
pub extern "C" fn sys_get_screen_witdh() -> u64 {
    return vga::get_res().0 as u64;
}
