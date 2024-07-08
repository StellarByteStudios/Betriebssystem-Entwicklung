use crate::{
    kernel::threads::{scheduler, thread},
    user::applications::graphic_console::{graphic_console_logic, graphic_console_printer},
};

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_thread(myself: *mut thread::Thread) {
    kprintln!("Zeichnen jetzt auf die Grafische Konsole");
    /*
    super::graphic_demo::draw_colors();

    //graphic_console::clear_screen();

    graphic_console_printer::print_char('H');
    graphic_console_printer::print_char('a');
    graphic_console_printer::print_char('l');
    graphic_console_printer::print_char('l');
    graphic_console_printer::print_char('o');

    graphic_console_printer::print_string(" Welt String\nNewline\n");

    graphic_console_logic::init_keyboardhandler();

    loop {}
     */
    graphic_console_logic::init_keyboardhandler();
    graphic_console_printer::clear_screen_rainbow();
    loop {}
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread = thread::Thread::new(scheduler::next_thread_id(), graphic_console_thread);
    scheduler::Scheduler::ready(graphic_thread);
}
