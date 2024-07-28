use crate::mylib::mathadditions::{
    complex_numbers::Complex,
    math::{cos, log2},
};

// Constants for the Mandelbrot calculation
const MAX_ITER: u32 = 100;
//const WIDTH: u32 = 800;
//const HEIGHT: u32 = 600;
const SCALE: i32 = 1000; // This scale is used to convert coordinates to fixed-point

// =========== Eingentliche Berechnung =========== //

// Calculate the Mandelbrot set and write it to a PPM image file
pub fn mandelbrot(x: f32, y: f32) -> f32 {
    let mut z = Complex { a: 0.0, b: 0.0 };
    let c = Complex { a: x, b: y };
    let max = 256;
    let mut i = 0;
    while i < max && z.arg_sq() < 32.0 {
        z = z * z + c;
        i += 1;
    }
    return (i as f32 - log2(log2(z.arg_sq()))) / (max as f32);
}

pub fn color(t: f32) -> u32 {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * cos(c.0 * t + d.0)) + a.0 - 2.2;
    let g = b.1 * (6.28318 * cos(c.1 * t + d.1)) + a.1 - 2.5;
    let b = b.2 * (6.28318 * cos(c.2 * t + d.2)) + a.2 - 2.1;
    let col = [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8];
    //kprintln!("berechnete Farbe: {}, {}, {}", col[0], col[1], col[2]);
    //kprintln!("berechnete Farbe: {:.4}, {:.4}, {:.4}", r, g, b);
    concat_u8_to_u32(col[0], col[1], col[2])
}

// Übersetzen von Algorithmusfarbe zu unserer
fn concat_u8_to_u32(a: u8, b: u8, c: u8) -> u32 {
    ((a as u32) << 16) | ((b as u32) << 8) | (c as u32)
}
