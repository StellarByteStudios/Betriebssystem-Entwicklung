use alloc::{string::String, vec::Vec};

use crate::{
    devices::vga,
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
extern "C" fn graphic_console_clear(myself: *mut thread::Thread) {
    // Argumente von Thread holen
    let args = Thread::get_args(myself);

    // Gibt es überhaut welche
    if args.get(1).is_none() {
        // Normaler Clear
        graphic_console_printer::clear_screen();
        Scheduler::exit();
    }

    // Ist es der Regenbogen Clear?
    match args.get(1).unwrap().as_str() {
        "rainbow" | "Rainbow" | "color" | "Color" | "colorful" | "Colorful" => {
            graphic_console_printer::clear_screen_rainbow()
        } // Regenbogen Hintegrund
        _ => graphic_console_printer::clear_screen(), // Normaler Clear
    }

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init(args: Vec<String>) {
    let graphic_thread =
        thread::Thread::new_with_args(scheduler::next_thread_id(), graphic_console_clear, args);
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("clear [optional color]");
    vprintln!("    clears the screen, sets cursor on top");
    vprintln!("    alternative with rainbow as option");
}
