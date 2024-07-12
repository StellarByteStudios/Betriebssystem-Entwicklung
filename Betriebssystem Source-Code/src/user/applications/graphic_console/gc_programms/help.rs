use alloc::{string::String, vec::Vec};

use crate::{
    kernel::threads::{
        scheduler::{self, Scheduler},
        thread::{self, Thread},
    },
    user::applications::graphic_console::{
        gc_programms, graphic_console_logic, graphic_console_printer,
    },
};

// Liste von befehlen
const COMMANDS: &'static [&'static str] = &[
    "scream",
    "greet",
    "clear [color]",
    "echo [message to echo]",
    "play [song to play]",
    "mandelbrot",
    "testprint",
    "sysinfo",
    "help [program]",
    "threads",
    "kill [threadID]",
    "silence",
    "cat",
    "meminfo",
    "scrollup",
];

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_help(myself: *mut thread::Thread) {
    // Argumente von Thread holen
    let args = Thread::get_args(myself);

    // Befehl vorne löschen
    //args.remove(0);

    // Wurde ein spezieller Befehl übergeben?
    if args.len() < 2 {
        vprintln!("Hier eine Liste an Befehlen die es gibt");
        // Alle Befehle einfach ausgeben
        for i in 0..COMMANDS.len() {
            vprintln!("  - {}", COMMANDS[i]);
        }
    }

    // Hilfe für die einzelnen Programme
    if args.len() >= 2 {
        vprintln!("Hilfe fuer den Befehl \"{}\"", args[1]);
        // Hinweis für den Befehl ausgeben
        match args[1].as_str() {
            "scream" => gc_programms::scream::print_help(),
            "greet" => gc_programms::greet::print_help(),
            "clear" => gc_programms::clear::print_help(),
            "echo" => gc_programms::echo::print_help(),
            "play" => gc_programms::play::print_help(),
            "mandelbrot" => gc_programms::mandelbrot::print_help(),
            "testprint" => gc_programms::macrotest::print_help(),
            "sysinfo" => gc_programms::sysinfo::print_help(),
            "help" => gc_programms::help::print_help(),
            "threads" => gc_programms::threads::print_help(),
            "kill" => gc_programms::kill::print_help(),
            "silence" => gc_programms::silence::print_help(),
            "cat" => gc_programms::cat::print_help(),
            "meminfo" => gc_programms::meminfo::print_help(),
            "scrollup" => gc_programms::scrollup::print_help(),
            _ => vprintln!("No Program called \"{}\"", args[1]),
        }
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
        thread::Thread::new_with_args(scheduler::next_thread_id(), graphic_console_help, args);
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("help [options]");
    vprintln!(" - without options -> List of all Programs");
    vprintln!(" - option -> prints help of program")
}
