use alloc::boxed::Box;

use crate::devices::cga; 
use crate::devices::cga_print; 
use crate::devices::fonts::font_8x8;
use crate::devices::vga;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::thread;
use crate::user::aufgabe7::bmp_hhu;


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

    vga::draw_bitmap(
        10,
        100,
        bmp_hhu::WIDTH,
        bmp_hhu::HEIGHT,
        bmp_hhu::DATA,
        bmp_hhu::BPP,
    );
}


/**
 Description: Create and add the graphic demo thread
*/
pub fn init() {
    let graphic_thread = thread::Thread::new(scheduler::next_thread_id(), graphic_thread_entry);
    scheduler::Scheduler::ready(graphic_thread);
}
