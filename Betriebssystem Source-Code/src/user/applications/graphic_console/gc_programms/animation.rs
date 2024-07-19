use alloc::{string::String, vec::Vec};

use crate::{
    devices::pcspk,
    kernel::threads::{
        scheduler::{self, Scheduler},
        thread::{self, Thread},
    },
    mylib::picturepainting::animate,
    user::applications::graphic_console::{graphic_console_logic, graphic_console_printer},
};

// Liste von Lieder
const ANIMATIONS: &'static [&'static str] = &["blink"];

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_animate(myself: *mut thread::Thread) {
    // Argumente von Thread holen
    let args = Thread::get_args(myself);

    // Gibt es überhaut ein Lied?
    if args.get(1).is_none() {
        vprintln!("No name given... :(");
        Scheduler::exit();
    }

    kprintln!(
        "Die Animation die jetzt gespielt wird: {}",
        args.get(1).unwrap().as_str()
    );

    graphic_console_printer::print_string("Now Playing: ");
    graphic_console_printer::print_string(args.get(1).unwrap().as_str());
    graphic_console_printer::print_string("\n");

    // Raussuchen welches Lied gemeint wird
    match args.get(1).unwrap().as_str() {
        "blinking" => animate::animate_blink(500, 20),
        _ => vprintln!("Animation not avaiable... :("), // kein registrierter Song
    }

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init(args: Vec<String>) {
    let graphic_thread = thread::Thread::new_with_args(
        scheduler::next_thread_id(),
        args[0].clone(),
        graphic_console_animate,
        args,
    );
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("animation [option]");
    vprintln!("    plays the animation given in option");
    vprintln!("    Hier eine Liste an Befehlen die es gibt");
    // Alle Befehle einfach ausgeben
    for i in 0..ANIMATIONS.len() {
        vprintln!("      - {}", ANIMATIONS[i]);
    }
}
