#![no_std]
use usrlib;
use usrlib::kernel::syscall::user_api;
use core::panic::PanicInfo;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    //let temp: u64 = 0xc0ffee;
    let temp: u64 = 0x6F6C6C6168;


    loop {
        user_api::usr_hello_world_print(133713371337);

        for i in 0..999999 {
            let temp = 5;
            let value = 7;
            let conclusion = temp + value + i;
        }

    }
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
