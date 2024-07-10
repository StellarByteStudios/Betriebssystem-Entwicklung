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
extern "C" fn graphic_console_cat(myself: *mut thread::Thread) {
    vprintln!("Painting Crumpyyy");
    vga::draw_bitmap(
        500,
        20,
        mylib::picturepainting::pictures::crumpy_cat::WIDTH,
        mylib::picturepainting::pictures::crumpy_cat::HEIGHT,
        mylib::picturepainting::pictures::crumpy_cat::DATA,
        mylib::picturepainting::pictures::crumpy_cat::BPP,
    );

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread = thread::Thread::new(scheduler::next_thread_id(), graphic_console_cat);
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("Prints a cat on screen");
}
