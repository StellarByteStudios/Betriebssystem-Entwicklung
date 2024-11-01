use crate::kernel::syscall::user_api::syscall0;
use crate::kernel::syscall::user_api::SYSNO_HELLO_WORLD;
use crate::kernel::threads::scheduler;

#[no_mangle]
pub extern "C" fn sys_hello_world_print(arg0: u64) {
    kprintln!("Hello World with Argument={}", arg0);
}
