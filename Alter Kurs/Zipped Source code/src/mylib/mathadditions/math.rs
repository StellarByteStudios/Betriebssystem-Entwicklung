// =========== Logarithmus zu Basis 2  =========== //

const LN2: f32 = 0.69314718056;
// Approximate the natural logarithm using a series expansion
pub fn ln(x: f32) -> f32 {
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
pub fn log2(x: f32) -> f32 {
    ln(x) / LN2 as f32
}

// =========== Fakultät =========== //

// Function to compute factorial
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (1..=n).product()
    }
}

// =========== Expotentialfunktion  =========== //

pub fn pow(x: f32, n: u32) -> f64 {
    let mut result = 1.0;
    for _ in 0..n {
        result *= x;
    }
    result as f64
}

// =========== Cosinus  =========== //

pub fn cos(x: f32) -> f64 {
    let mut sum = 0.0;
    let terms = 10; // Number of terms in the series

    for n in 0..terms {
        let numerator = pow(-1.0, n as u32) * pow(x, 2 * n as u32);
        let denominator = factorial(2 * n as u64) as f64;
        sum += numerator / denominator;
    }

    sum
}

// =========== Betrag  =========== //

pub fn abs(x: f32) -> f32 {
    if x < 0.0 {
        return -x;
    }

    return x;
}

// =========== Wurzel  =========== //

// Method to calculate the square root of a f32 using the Newton-Raphson method
const ROOT_TOLERANCE: f32 = 1e-6;
pub fn sqrt(value: f32) -> f32 {
    if value == 0.0 {
        return 0.0;
    }

    let mut x = value;
    let mut last_x = 0.0;

    // Iteratively improve the approximation
    while abs(x - last_x) > ROOT_TOLERANCE {
        last_x = x;
        x = 0.5 * (x + value / x);
    }

    x
}
