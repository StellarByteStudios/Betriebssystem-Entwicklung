// Verknüpfung auf den Syscall um ein Bild zu malen

use crate::kernel::syscall::user_api::usr_paint_picture_on_pos;

pub fn draw_picture(x: u32, y: u32, width: u32, height: u32, data: &[u8], bpp: u32){
    /* Später über Syscall
    vga::draw_bitmap(
        x,
        y,
        frames[i].width,
        frames[i].height,
        frames[i].data.as_slice(),
        frames[i].bpp,
    );*/
    usr_paint_picture_on_pos(x as u64, y as u64, width as u64, height as u64, bpp as u64, data.as_ptr())
}