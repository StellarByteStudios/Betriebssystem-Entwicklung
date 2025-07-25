use usrlib::utility::mathadditions::{complex_numbers::Complex, math::log2};
// Constants for the Mandelbrot calculation

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
