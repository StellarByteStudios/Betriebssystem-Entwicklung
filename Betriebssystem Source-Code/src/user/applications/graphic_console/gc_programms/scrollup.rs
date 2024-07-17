use alloc::string::ToString;

use crate::{
    kernel::threads::{
        scheduler::{self, Scheduler},
        thread,
    },
    user::applications::graphic_console::{graphic_console_logic, graphic_console_printer},
};

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_scrollup(myself: *mut thread::Thread) {
    graphic_console_printer::scrollup();

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread = thread::Thread::new_name(
        scheduler::next_thread_id(),
        "scrollup".to_string(),
        graphic_console_scrollup,
    );
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("Scrolls up screen");
}
