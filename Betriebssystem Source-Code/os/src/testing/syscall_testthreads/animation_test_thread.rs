use alloc::boxed::Box;
use alloc::string::ToString;
//use usrlib::graphix::picturepainting::animate;
use crate::kernel::cpu;
use crate::kernel::threads::scheduler::Scheduler;
use crate::kernel::threads::thread::Thread;
use crate::kernel::threads::{scheduler, thread};
use crate::devices::graphical::graphic_console_printer;

pub extern "C" fn animation_test_thread_entry() {
    // Irgendwann noch variabel?
    let animation = "Charmander";

    //kprintln!("Die Animation die jetzt gespielt wird: {}", "Charmander");

    graphic_console_printer::print_string("Now Playing: ");
    graphic_console_printer::print_string(animation);
    graphic_console_printer::print_string("\n");

    // Raussuchen welche Animation gemeint wird
    /* !!!!!! Funktioniert grade noch nicht in externer library
    match animation {
        "blink" | "blinking" => animate::animate_blink(500, 20),
        "charmander" | "Charmander" | "pokemon" | "Pokemon" => animate::animate_charmander(500, 20),
        "ghost" | "gilbert" | "Gilbert" => animate::animate_ghost(500, 20),
        _ => vprintln!("Animation not avaiable... :("), // nicht registriert
    }*/

    Scheduler::exit();
}

pub fn init() {
    let idle_thread: Box<Thread> = thread::Thread::new_name(
        scheduler::next_thread_id(),
        animation_test_thread_entry,
        false,
        "animation-Thread".to_string(),
    );
    scheduler::Scheduler::ready(idle_thread);
}
