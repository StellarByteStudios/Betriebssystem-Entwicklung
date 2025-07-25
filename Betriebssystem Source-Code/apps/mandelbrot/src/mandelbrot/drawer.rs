// = = = Berechnung für Mandelbrotmenge = = = //

use usrlib::kernel::syscall::user_api::usr_draw_pixel;

use crate::mandelbrot::{calculator::mandelbrot, color::color};

pub fn draw_mandelbrot(image_width: u32, image_height: u32, startpixel: (u32, u32)) {
    for y in 0..image_height {
        for x in 0..image_width {
            // Pixel holen
            let u = x as f32 / image_height as f32;
            let v = y as f32 / image_height as f32;
            let t = mandelbrot(1.8 * (u - 0.2) - 1.4, 1.8 * (v - 0.5));
            // Zu Farbe umrechnen
            let pix = color((2.0 * t + 0.5) % 1.0);
            usr_draw_pixel(
                (x + startpixel.0) as usize,
                (y + startpixel.1) as usize,
                pix as usize,
            );
        }
    }
}
