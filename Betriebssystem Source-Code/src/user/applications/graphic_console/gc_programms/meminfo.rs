use alloc::string::ToString;

use crate::{
    kernel::{
        allocator, cpu,
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
    let ie = cpu::disable_int_nested();

    allocator::dump_free_list_graphic();

    cpu::enable_int_nested(ie);

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread: alloc::boxed::Box<thread::Thread> = thread::Thread::new_name(
        scheduler::next_thread_id(),
        "meminfo".to_string(),
        graphic_console_meminfo,
    );
    scheduler::Scheduler::ready(graphic_thread);

    //kprintln!("{}", allocator::free_list_string().as_str());
}

pub fn print_help() {
    vprintln!("prints the current state of the heap");
}
