use crate::{
    devices::vga,
    kernel::threads::{
        scheduler::{self, Scheduler},
        thread,
    },
    user::applications::graphic_console::{graphic_console_logic, graphic_console_printer},
};

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_sysinfo(myself: *mut thread::Thread) {
    vprintln!("Byte OS: 1.0");
    vprintln!("------------------------------------\n");
    vprintln!("Aktuelle Funktionalitaeten:");
    vprint!("    Bildschirmausgabe ");
    graphic_console_printer::set_font_color(150, 30, 200);
    graphic_console_printer::set_bg_color(255, 255, 255);
    vprintln!("(auch bunt)");
    graphic_console_printer::reset_font_color();
    graphic_console_printer::reset_bg_color();
    vprintln!("    - Heapverwaltung (mit Freispeicherliste)");
    vprintln!("    - Interrupts");
    vprintln!("    - Tastatureingabe (Ueber Interrupts)");
    vprintln!("    - Koroutinen (Kooperativ - verkettet)");
    vprintln!("    - Queue (Für die Threads)");
    vprintln!("    - Scheduler (Kooperativ)");
    vprintln!("    - Threads (Kooperativ)");
    vprintln!("    - Musik");
    vprintln!("    - Shellbefehle");
    vprintln!("        * Ein und Ausgabe von Text");
    vprintln!("        * Auswahl von Musik");
    vprintln!("        * Fraktalberechnung");
    vprintln!("        * und vieles mehr..");
    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread = thread::Thread::new(scheduler::next_thread_id(), graphic_console_sysinfo);
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("Prints the information, what the OS is capable of");
}
