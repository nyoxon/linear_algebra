#![allow(dead_code, unused_imports, unused_variables, 
         unused_mut, non_snake_case, deprecated)]

use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

pub struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Self {
        Self {
            real: r,
            imaginary: i,
        }
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.real, -self.imaginary)
    }

    pub fn norm(&self) -> f64 {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }

    pub fn arg(&self) -> f64 {
        (self.imaginary / self.real).atan()
    }

    pub fn pow_real(&self, p: f64) -> Self {
        let zp = self.norm().powf(p);
        let arg = self.arg();
        let cosp = (p * arg).cos();
        let sinp = (p * arg).sin();

        Self::new(zp * cosp, zp * sinp)
    }

    pub fn pow_complex(&self, p: &Complex) -> Self {
        let (a, b) = (self.real, self.imaginary);
        let arg = self.arg();
        let zae = self.norm().powf(a) * (-b * arg).exp();
        let inner = b * self.norm().ln() + a * arg;
        let real = inner.cos() * zae;
        let imag = inner.sin() * zae;

        Self::new(real, imag)
    }

}

impl Add<&Complex> for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Self::Output {
        Complex::new(self.real + other.real, self.imaginary + other.imaginary)
    }
}

impl Sub<&Complex> for &Complex {
    type Output = Complex;

    fn sub(self, other: &Complex) -> Self::Output {
        Complex::new(self.real - other.real, self.imaginary - other.imaginary)
    }
}

impl Mul<f64> for &Complex {
    type Output = Complex;

    fn mul(self, other: f64) -> Self::Output {
        Complex::new(other * self.real, other * self.imaginary)
    }
}

impl Mul<&Complex> for f64 {
    type Output = Complex;

    fn mul(self, other: &Complex) -> Self::Output {
        Complex::new(self * other.real, self * other.imaginary)
    }
}

impl Mul<&Complex> for &Complex {
    type Output = Complex;

    fn mul(self, other: &Complex) -> Self::Output {
        Complex::new(self.real * other.real - self.imaginary * other.imaginary, 
        self.real * other.imaginary + other.real * self.imaginary)
    }
}

impl Div<&Complex> for &Complex {
    type Output = Complex;

    fn div(self, other: &Complex) -> Self::Output {
        let norm = other.real * other.real + other.imaginary * other.imaginary;
        Complex::new((self.real * other.real + self.imaginary * other.imaginary) /
                     norm,
                     (self.imaginary * other.real - self.real * other.imaginary) /
                     norm)
    }
}


impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")?;
        if self.imaginary == 0.0 {
            write!(f, "{}", self.real)?;
        } else {
            write!(f, "{:?}", (self.real, self.imaginary))?;
        }
        write!(f, "")
    }
}