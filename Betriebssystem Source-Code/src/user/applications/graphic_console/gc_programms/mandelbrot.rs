use alloc::{string::String, vec::Vec};

use crate::{
    devices::vga,
    kernel::threads::{
        scheduler::{self, Scheduler},
        thread::{self, Thread},
    },
    user::{
        applications::graphic_console::{graphic_console_logic, graphic_console_printer},
        aufgabe7::draw_mandelbrot::{color, mandelbrot},
    },
};

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_console_mandelbrot(myself: *mut thread::Thread) {
    /* (falls später benötigt)
    // Argumente von Thread holen
    let mut args = Thread::get_args(myself);

    // Befehl vorne löschen
    args.remove(0);
     */
    // Größe des Bildes festlegen
    let image_width = 400;
    let image_height = 300;

    // Wo wird es hin gezeichnet
    let startpixel: (u32, u32) = (500, 50);

    // Zeilenende
    graphic_console_printer::print_string("Printing Mandelbrot\n");

    draw_mandelbrot(image_width, image_height, startpixel);

    Scheduler::exit();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init(args: Vec<String>) {
    let graphic_thread = thread::Thread::new_with_args(
        scheduler::next_thread_id(),
        args[0].clone(),
        graphic_console_mandelbrot,
        args,
    );
    scheduler::Scheduler::ready(graphic_thread);
}

pub fn print_help() {
    vprintln!("Calculates a definitive set of mandelbrot and prints it on screen");
}

// = = = Berechnung für Mandelbrotmenge = = = //

fn draw_mandelbrot(image_width: u32, image_height: u32, startpixel: (u32, u32)) {
    // Mandelbrotset berechnen
    //kprintln!("Mandelbrot erfolgreich berechnet");

    // Startpixel wählen
    //let startpixel: (u32, u32) = (10, 50);

    //let mut pixel_gesetzt: u32 = 0;

    // Punkte Malen
    //let image_width = 800;
    //let image_height = 600;

    for y in 0..image_height {
        for x in 0..image_width {
            // Pixel holen
            let u = x as f32 / image_height as f32;
            let v = y as f32 / image_height as f32;
            let t = mandelbrot(1.8 * (u - 0.2) - 1.4, 1.8 * (v - 0.5));
            // Zu Farbe umrechnen
            let pix = color((2.0 * t + 0.5) % 1.0);
            //kprintln!("Pixelfarbe berechnet:t = {:10}; col = {:#x}", t, pix);
            vga::draw_pixel(x + startpixel.0, y + startpixel.1, pix);
        }
    }

    kprintln!("calculates a definitive set of mandelbrot and prints it on screen");
}
