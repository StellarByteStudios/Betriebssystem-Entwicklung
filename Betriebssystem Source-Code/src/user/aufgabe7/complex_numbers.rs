use core::ops::{Add, Mul};

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

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex {
            a: self.a * rhs.a - self.b * rhs.b,
            b: self.a * rhs.b + self.b * rhs.a,
        }
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
