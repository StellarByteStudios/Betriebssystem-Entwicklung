use crate::kernel::{processes::process_handler, threads::scheduler::get_active_pid};

#[no_mangle]
pub extern "C" fn sys_dump_vmas() {
    // Prozess ID holen
    let pid = get_active_pid();
    kprintln!("VMAs von Prozess {:} werden gedumpt", pid);

    process_handler::dump_vma_of_process(pid);
}
