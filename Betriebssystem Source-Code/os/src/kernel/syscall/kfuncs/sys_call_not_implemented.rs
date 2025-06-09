pub extern "C" fn sys_call_not_implemented() -> u64 {
    kprintln!("Syscall called which is not implemented");

    loop {}
    return 0;
}
