#[no_mangle]
pub extern "C" fn sys_hello_world_print(arg0: u64) {
    kprintln!("Hello World with Argument={}", arg0);
}
