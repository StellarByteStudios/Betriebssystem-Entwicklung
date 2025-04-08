use crate::kernel::threads::scheduler;

#[no_mangle]
pub extern "C" fn sys_hello_world() {
    kprintln!(
        "Hello World from user thread tid={}",
        scheduler::get_active_tid()
    );
}
