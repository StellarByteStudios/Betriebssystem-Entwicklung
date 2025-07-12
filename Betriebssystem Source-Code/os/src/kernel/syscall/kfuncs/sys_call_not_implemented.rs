pub extern "C" fn sys_call_not_implemented() {
    kprintln!("Syscall called which is not implemented");

    loop {}
}
