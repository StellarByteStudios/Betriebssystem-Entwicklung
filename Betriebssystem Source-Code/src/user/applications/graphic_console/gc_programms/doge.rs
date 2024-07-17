use alloc::string::ToString;

use crate::{
    devices::vga,
    kernel::threads::{
        scheduler::{self, Scheduler},
        thread,
    },
    mylib,
    user::applications::graphic_console::{graphic_console_logic, graphic_console_printer},
};

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_doge(myself: *mut thread::Thread) {
    vprintln!("Painting Doge");
    vga::draw_bitmap(
        500,
        20,
        mylib::picturepainting::pictures::doge::WIDTH,
        mylib::picturepainting::pictures::doge::HEIGHT,
        mylib::picturepainting::pictures::doge::DATA,
        mylib::picturepainting::pictures::doge::BPP,
    );

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread = thread::Thread::new_name(
        scheduler::next_thread_id(),
        "doge".to_string(),
        graphic_console_doge,
    );
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("Prints a doge on screen");
}
