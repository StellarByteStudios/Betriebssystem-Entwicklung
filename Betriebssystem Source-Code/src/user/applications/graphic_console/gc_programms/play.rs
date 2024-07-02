use alloc::{string::String, vec::Vec};

use crate::{
    devices::pcspk,
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
extern "C" fn graphic_console_play(myself: *mut thread::Thread) {
    // Argumente von Thread holen
    let args = Thread::get_args(myself);

    // Gibt es überhaut ein Lied?
    if args.get(1).is_none() {
        Scheduler::exit();
    }

    kprintln!(
        "Der Song der jetzt gespielt wird: {}",
        args.get(1).unwrap().as_str()
    );

    // Raussuchen welches Lied gemeint wird
    match args.get(1).unwrap().as_str() {
        "mario" | "Mario" => pcspk::super_mario(),
        "tetris" | "Tetris" => pcspk::tetris(),
        "aero" | "Aero" | "aerodynamic" | "Aerodynamic" => pcspk::aerodynamic(),
        "starwars" | "Starwars" | "imperial" | "Imperial" => pcspk::starwars_imperial(),
        _ => Scheduler::exit(), // kein registrierter Song
    }

    //graphic_console_printer("Now Playing: ");
    //graphic_console_printer(args.get(1).unwrap().as_str());

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init(args: Vec<String>) {
    let graphic_thread =
        thread::Thread::new_with_args(scheduler::next_thread_id(), graphic_console_play, args);
    scheduler::Scheduler::ready(graphic_thread);
}
