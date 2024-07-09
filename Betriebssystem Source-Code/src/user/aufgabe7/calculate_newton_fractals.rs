use crate::{devices::vga, mylib::mathadditions::complex_numbers::Complex};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const MAX_ITER: u32 = 100;
const TOLERANCE: f32 = 1e-6;

pub fn draw_newton() {
    let offset_x: u32 = 200;
    let offset_y: u32 = 200;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = calculate_newton_color(x, y);
            vga::draw_pixel(x + offset_x, y + offset_y, color);
        }
    }
}

pub fn calculate_newton_color(x: u32, y: u32) -> u32 {
    let cx = (x as f32 / WIDTH as f32) * 4.0 - 2.0;
    let cy = (y as f32 / HEIGHT as f32) * 4.0 - 2.0;
    let c = Complex { a: cx, b: cy };

    let (root, iterations) = newton_method(c);

    let color = match root {
        Some(root) => {
            // Color based on the root and number of iterations
            let r = (iterations * 10) as u8;
            let g = ((root.re() * 127.0) + 128.0) as u8;
            let b = ((root.im() * 127.0) + 128.0) as u8;
            vga::rgb_24(r, g, b)
        }
        None => vga::rgb_24(0, 0, 0),
    };
    return color;
}

fn newton_method(z: Complex) -> (Option<Complex>, u32) {
    let mut z = z;
    for i in 0..MAX_ITER {
        let f = polynomial(z);
        let f_prime = polynomial_derivative(z);

        let nullpunkt = Complex { a: 0.0, b: 0.0 };

        if f_prime == nullpunkt {
            return (None, i);
        }

        let z_next = z - f / f_prime;
        if (z_next - z).norm() < TOLERANCE {
            return (Some(z_next), i);
        }

        z = z_next;
    }

    (None, MAX_ITER)
}

fn polynomial(z: Complex) -> Complex {
    // Example polynomial: 5z^3 - 1
    z * z * z * 5.0 - Complex { a: 1.0, b: 0.0 }
}

fn polynomial_derivative(z: Complex) -> Complex {
    // Derivative of the example polynomial: 15*z^2
    z * z * 15.0
}
