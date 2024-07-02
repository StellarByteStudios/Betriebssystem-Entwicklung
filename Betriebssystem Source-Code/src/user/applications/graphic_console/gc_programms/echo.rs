use alloc::{string::String, vec::Vec};

use crate::{
    kernel::threads::{
        scheduler::{self, Scheduler},
        thread::{self, Thread},
    },
    user::applications::graphic_console::{graphic_console_logic, graphic_console_printer},
};

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_echo(myself: *mut thread::Thread) {
    // Argumente von Thread holen
    let mut args = Thread::get_args(myself);

    // Befehl vorne löschen
    args.remove(0);

    // Alle argumente wiedergeben
    for arg in args {
        graphic_console_printer::print_string(arg.as_str());
        graphic_console_printer::print_string(" ");
    }

    // Zeilenende
    graphic_console_printer::print_string("\n");

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init(args: Vec<String>) {
    let graphic_thread =
        thread::Thread::new_with_args(scheduler::next_thread_id(), graphic_console_echo, args);
    scheduler::Scheduler::ready(graphic_thread);
}
