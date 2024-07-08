use crate::{
    kernel::{
        allocator,
        threads::{
            scheduler::{self, Scheduler},
            thread,
        },
    },
    user::applications::graphic_console::{graphic_console_logic, graphic_console_printer},
};

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_meminfo(myself: *mut thread::Thread) {
    // Infos ausprinten
    graphic_console_printer::print_string(allocator::free_list_string().as_str());
    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    /*
    let graphic_thread: alloc::boxed::Box<thread::Thread> =
        thread::Thread::new(scheduler::next_thread_id(), graphic_console_meminfo);
    scheduler::Scheduler::ready(graphic_thread);
    */
    kprintln!("{}", allocator::free_list_string().as_str());
}

pub fn print_help() {
    vprintln!("No Help Implemented");
}
