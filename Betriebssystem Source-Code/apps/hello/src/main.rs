#![no_std]
mod kernel;
use crate::kernel::syscall::user_api;
use core::panic::PanicInfo;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    user_api::usr_hello_world_print(99999);
}

/*
* Panic Handler
*/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //kprintln!("Panic: {}", info);
    //kprintln!("{:?}", Backtrace::new());
    loop {}
}
