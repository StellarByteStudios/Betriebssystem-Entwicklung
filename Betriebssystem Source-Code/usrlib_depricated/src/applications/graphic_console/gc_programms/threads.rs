use alloc::boxed::Box;

use crate::{
    devices::vga,
    kernel::{
        cpu,
        threads::{
            scheduler::{self, Scheduler},
            thread::{self, Thread},
        },
    },
    mylib::queue::Queue,
    user::applications::graphic_console::{graphic_console_logic, graphic_console_printer},
};

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_show_threads(myself: *mut thread::Thread) {
    vprintln!("List of Threads which are currently running:");

    // Liste an Threads holen
    let ie = cpu::disable_int_nested();

    // Funktioniert nicht mit vorgegebenem Scheduler
    //scheduler::print_ready_queue();

    cpu::enable_int_nested(ie);

    // Queue ausgeben
    //vprintln!("Ready Queue: {}", thread_queue);

    Scheduler::exit()
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread =
        thread::Thread::new(scheduler::next_thread_id(), graphic_console_show_threads);
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("Shows all currently running Threads !(BUGGY)!");
}
