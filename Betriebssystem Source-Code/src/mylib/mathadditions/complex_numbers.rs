use core::ops::{Add, Div, Mul, Sub};

use super::math::sqrt;

#[derive(Clone, Copy)]
pub struct Complex {
    pub a: f32,
    pub b: f32,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Complex {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex {
            a: self.a * rhs.a - self.b * rhs.b,
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}

impl Mul<f32> for Complex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Complex {
            a: self.a * rhs,
            b: self.a * rhs,
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Complex {
        let denominator = rhs.a * rhs.a + rhs.b * rhs.b;
        Complex {
            a: (self.a * rhs.a + self.b * rhs.b) / denominator,
            b: (self.b * rhs.a - self.a * rhs.b) / denominator,
        }
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        if self.a != other.a {
            return false;
        }

        if self.b != other.b {
            return false;
        }

        return true;
    }
}

impl Complex {
    pub fn arg_sq(self) -> f32 {
        self.a * self.a + self.b * self.b
    }
}

impl Complex {
    fn abs(self) -> Self {
        // Funktion abs fehlt, deswegen umständlich
        let mut a_abs = self.a;
        let mut b_abs = self.b;

        if a_abs < 0.0 {
            a_abs = -a_abs
        }

        if b_abs < 0.0 {
            b_abs = -b_abs
        }

        return Complex { a: a_abs, b: b_abs };
    }
}

impl Complex {
    pub fn re(self) -> f32 {
        self.a.clone()
    }

    pub fn im(self) -> f32 {
        self.b.clone()
    }
}

impl Complex {
    pub fn norm(&self) -> f32 {
        sqrt(self.a * self.a + self.b * self.b)
    }
}
