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

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_fontchange(myself: *mut thread::Thread) {
    // Argumente von Thread holen
    let args = Thread::get_args(myself);

    // Gibt es überhaut ein Lied?
    if args.get(1).is_none() {
        vprintln!("No mode given... :(");
        Scheduler::exit();
    }

    // wenn nichts angegeben wieder Standard
    if args.len() < 3 {
        vprintln!("reset {} color", args.get(1).unwrap().as_str());
        match args.get(1).unwrap().as_str() {
            "letters" | "letters" | "front" => graphic_console_printer::reset_font_color(),
            "bg" | "background" => graphic_console_printer::reset_bg_color(),
            _ => vprintln!("only can use letters/front or bg/background"),
        }
        Scheduler::exit();
    }

    // Wenn Farbe angegeben wurde
    // Gibt es genug Argumente?
    if args.len() < 5 {
        vprintln!("not enougth arguments given for r g b values");
        Scheduler::exit();
    }
    // Alle Zahlen überprüfen
    let r = args[2].parse::<u8>();
    let g = args[3].parse::<u8>();
    let b = args[4].parse::<u8>();

    if r.is_err() || g.is_err() || b.is_err() {
        vprintln!(
            "Not vaiable rgb values [{},{},{}]",
            args.get(2).unwrap().as_str(),
            args.get(3).unwrap().as_str(),
            args.get(4).unwrap().as_str()
        );
        Scheduler::exit();
    }

    // Alles ist gut gelaufen
    match args.get(1).unwrap().as_str() {
        "letters" | "front" => {
            graphic_console_printer::set_font_color(r.unwrap(), g.unwrap(), b.unwrap())
        }
        "bg" | "background" => {
            graphic_console_printer::set_bg_color(r.unwrap(), g.unwrap(), b.unwrap())
        }
        _ => {
            vprintln!("only can use letters/front or bg/background");
            Scheduler::exit()
        }
    }
    vprintln!("Color changed successfully");

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init(args: Vec<String>) {
    let graphic_thread = thread::Thread::new_with_args(
        scheduler::next_thread_id(),
        args[0].clone(),
        graphic_console_fontchange,
        args,
    );
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("fontchange [letter/background] [r g b]");
    vprintln!("    first option decides between letter color and background");
    vprintln!("    second option are the rgb values from 0 to 255");
}
