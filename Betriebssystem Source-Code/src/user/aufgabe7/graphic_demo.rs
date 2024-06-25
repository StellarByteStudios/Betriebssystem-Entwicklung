use alloc::boxed::Box;

use crate::devices::cga;
use crate::devices::cga_print;
use crate::devices::fonts::font_8x8;
use crate::devices::vga;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread;
use crate::user::aufgabe7::bmp_hhu;
use crate::user::aufgabe7::draw_mandelbrot::color;
use crate::user::aufgabe7::draw_mandelbrot::mandelbrot;

use super::draw_mandelbrot;

/**
 Description: Calculate a color value interpolated in one dimensions
*/
fn lin_inter_pol_1d(x: u32, xr: u32, l: u32, r: u32) -> u32 {
    return ((((l >> 16) * (xr - x) + (r >> 16) * x) / xr) << 16)
        | (((((l >> 8) & 0xFF) * (xr - x) + ((r >> 8) & 0xFF) * x) / xr) << 8)
        | (((l & 0xFF) * (xr - x) + (r & 0xFF) * x) / xr);
}

/**
 Description: Calculate a color value interpolated in two dimensions
*/
fn lin_inter_pol_2d(
    x: u32,
    y: u32,
    xres: u32,
    yres: u32,
    lt: u32,
    rt: u32,
    lb: u32,
    rb: u32,
) -> u32 {
    return lin_inter_pol_1d(
        y,
        yres,
        lin_inter_pol_1d(x, xres, lt, rt),
        lin_inter_pol_1d(x, xres, lb, rb),
    );
}

/**
 Description: Draw colours
*/
fn draw_colors() {
    let (xres, yres) = vga::get_res();

    for y in 0..yres {
        for x in 0..xres {
            let pix = lin_inter_pol_2d(x, y, xres, yres, 0x0000FF, 0x00FF00, 0xFF0000, 0xFFFF00);
            vga::draw_pixel(x, y, pix);
        }
    }
}

fn draw_mandelbrot() {
    // Mandelbrotset berechnen
    //let mandelbrot_data = draw_mandelbrot::mandelbrot();

    kprintln!("Mandelbrot erfolgreich berechnet");

    // Startpixel wählen
    let startpixel: (u32, u32) = (10, 100);

    // Punkte Malen
    let image_width = 800;
    let image_height = 600;

    for y in 0..image_height {
        for x in 0..image_width {
            // Pixel holen
            let u = x as f32 / image_height as f32;
            let v = y as f32 / image_height as f32;
            let t = mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
            // Zu Farbe umrechnen
            let pix = color((2.0 * t + 0.5) % 1.0);
            vga::draw_pixel(x + startpixel.0, y + startpixel.1, pix);
            // pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0))
        }
    }
}

/**
 Description: Entry function of the graphic demo thread
*/
#[no_mangle]
extern "C" fn graphic_thread_entry(myself: *mut thread::Thread) {
    let text_h = font_8x8::CHAR_HEIGHT;

    draw_colors();

    vga::draw_string(0, 0, vga::rgb_24(0, 255, 0), "hhuTOS 0.7");
    vga::draw_string(0, text_h, vga::rgb_24(0, 255, 0), "==========");
    vga::draw_string(
        0,
        3 * text_h,
        vga::rgb_24(0, 255, 0),
        "Wir sind jetzt im Grafikmodus!",
    );

    /*
    vga::draw_bitmap(
        10,
        100,
        bmp_hhu::WIDTH,
        bmp_hhu::HEIGHT,
        bmp_hhu::DATA,
        bmp_hhu::BPP,
    );*/

    draw_mandelbrot();
}

/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread = thread::Thread::new(scheduler::next_thread_id(), graphic_thread_entry);
    scheduler::Scheduler::ready(graphic_thread);
}
