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
extern "C" fn graphic_console_clear(myself: *mut thread::Thread) {
    graphic_console_printer::clear_screen();

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread = thread::Thread::new(scheduler::next_thread_id(), graphic_console_clear);
    scheduler::Scheduler::ready(graphic_thread);
}
