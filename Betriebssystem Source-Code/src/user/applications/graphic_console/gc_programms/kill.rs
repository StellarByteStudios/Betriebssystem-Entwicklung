use alloc::{string::String, vec::Vec};

use crate::{
    devices::pcspk,
    kernel::threads::{
        scheduler::{self, Scheduler},
        thread::{self, Thread},
    },
    user::applications::graphic_console::{graphic_console_logic, graphic_console_printer},
};

// Liste von Lieder
const SONGS: &'static [&'static str] = &[
    "mario",
    "tetris",
    "aerodynamic",
    "imperial",
    "entchen",
    "intro",
    "nyancat",
];

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_kill(myself: *mut thread::Thread) {
    // Argumente von Thread holen
    let args = Thread::get_args(myself);

    // Gibt es überhaut eine Nummer?
    if args.get(1).is_none() {
        vprintln!("No Vailid ThreadID");
        Scheduler::exit();
    }

    // Versuche Zahl zu Parsen
    let int_result = args[1].parse::<usize>();

    // War das erfolgreich
    if int_result.is_err() {
        vprintln!("{} is no Vailid ThreadID", args[1]);
        Scheduler::exit();
    }

    // Thread killein
    let success: bool = Scheduler::kill(int_result.unwrap());

    if success {
        vprintln!("Thread {} killed successfully", args[1]);
    } else {
        vprintln!("ThreadID {} not found", args[1]);
    }

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init(args: Vec<String>) {
    let graphic_thread =
        thread::Thread::new_with_args(scheduler::next_thread_id(), graphic_console_kill, args);
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("kill [option]");
    vprintln!("    ends thread of given ID if avaiable");
}
