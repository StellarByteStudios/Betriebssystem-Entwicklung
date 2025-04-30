use core::slice;

use crate::{
    devices::graphical::{
        graphic_console_printer::{print_string, print_string_on_position},
        vga,
    },
    kernel::cpu,
};

#[no_mangle]
pub extern "C" fn sys_paint_picture_on_pos(
    x: u64,
    y: u64,
    width: u64,
    height: u64,
    bbp: u64,
    bitmapbuff: *const u8,
) -> i64 {
    // Fehlerabfrage
    if bitmapbuff.is_null() {
        return -1;
    }

    // Länge der Bitmap berechnen
    let lenght = width * height * bbp;

    // Bitmap wieder umwandeln
    unsafe {
        // Aus dem Buffer ein Slice machen
        let bitmap = slice::from_raw_parts(bitmapbuff, lenght as usize);

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
