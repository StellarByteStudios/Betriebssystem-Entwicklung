use core::slice;

use crate::{
    devices::graphical::{
        graphic_console_printer::{print_string, print_string_on_position},
        vga,
    },
    kernel::cpu,
};

#[no_mangle]
pub extern "C" fn sys_draw_pixel(x: usize, y: usize, color: usize) {
    vga::draw_pixel(x as u32, y as u32, color as u32);
}

#[no_mangle]
pub extern "C" fn sys_paint_picture_on_pos(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    bbp: usize,
    bitmapbuff: *const u8,
) -> usize {
    // Fehlerabfrage
    if bitmapbuff.is_null() {
        return 1;
    }

    // Länge der Bitmap berechnen
    let lenght = width * height * bbp;

    // Bitmap wieder umwandeln
    unsafe {
        // Aus dem Buffer ein Slice machen
        let bitmap = slice::from_raw_parts(bitmapbuff, lenght);

        // Bitmap ausgeben
        vga::draw_bitmap(
            x as u32,
            y as u32,
            width as u32,
            height as u32,
            bitmap,
            bbp as u32,
        );
    }

    return 0;
}
