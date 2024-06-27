use super::complex_numbers::Complex;
// Constants for the Mandelbrot calculation
const MAX_ITER: u32 = 100;
//const WIDTH: u32 = 800;
//const HEIGHT: u32 = 600;
const SCALE: i32 = 1000; // This scale is used to convert coordinates to fixed-point

// =========== Funktionen die im Core-Crate nicht drin sind =========== //

const LN2: f32 = 0.69314718056;
// Approximate the natural logarithm using a series expansion
fn ln(x: f32) -> f32 {
    if x <= 0.0 {
        return f32::NAN; // Not a Number
    }
    if x == 1.0 {
        return 0.0;
    }

    // Reduce the range of x to [1, 2)
    let mut y = x;
    let mut k = 0;
    while y > 1.5 {
        y /= 2.0;
        k += 1;
    }
    while y < 0.5 {
        y *= 2.0;
        k -= 1;
    }

    // Use the series expansion to approximate ln(1 + z) where z = y - 1
    let z = y - 1.0;
    let mut term = z;
    let mut sum = 0.0;
    let mut n = 1.0;

    for _ in 0..10 {
        sum += term / n;
        term *= -z;
        n += 1.0;
    }

    sum + (k as f32) * LN2
}

// Calculate log2 using the change of base formula
fn log2(x: f32) -> f32 {
    ln(x) / LN2 as f32
}

// Function to compute factorial
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (1..=n).product()
    }
}

// Function to compute power
fn pow(x: f32, n: u32) -> f64 {
    let mut result = 1.0;
    for _ in 0..n {
        result *= x;
    }
    result as f64
}

// Function to compute cosine using Taylor series expansion
fn cos(x: f32) -> f64 {
    let mut sum = 0.0;
    let terms = 10; // Number of terms in the series

    for n in 0..terms {
        let numerator = pow(-1.0, n as u32) * pow(x, 2 * n as u32);
        let denominator = factorial(2 * n as u64) as f64;
        sum += numerator / denominator;
    }

    sum
}

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
    let r = b.0 * (6.28318 * cos(c.0 * t + d.0)) + a.0;
    let g = b.1 * (6.28318 * cos(c.1 * t + d.1)) + a.1;
    let b = b.2 * (6.28318 * cos(c.2 * t + d.2)) + a.2;
    let col = [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8];
    concat_u8_to_u32(col[0], col[1], col[2])
}

// Übersetzen von Algorithmusfarbe zu unserer
fn concat_u8_to_u32(a: u8, b: u8, c: u8) -> u32 {
    ((a as u32) << 16) | ((b as u32) << 8) | (c as u32)
}
