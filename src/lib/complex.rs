use std::f64::consts::PI;
use std::fmt;
use std::fmt::{Formatter};
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use crate::c;

#[derive(Copy, Clone, PartialEq)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    pub fn new_euler(r: f64, phi: f64) -> Complex {
        Complex {
            re: r * phi.cos(),
            im: r * phi.sin(),
        }
    }

    /// Construct a new primitive nth root of unity.
    pub fn nth_root_of_unity(n: u32) -> Complex {
        if 0 == n {
            Complex::one()
        } else {
            let angle = (2f64 * PI) / (n as f64);
            Complex::new_euler(1f64, angle)
        }
    }

    pub fn one() -> Complex {
        Complex::new(1f64, 0f64)
    }

    pub fn zero() -> Complex {
        Complex::new(0f64, 0f64)
    }

    /// The imaginary unit.
    pub fn i() -> Complex {
        Complex::new(0f64, 1f64)
    }

    /// Compute the square of norm/absolute value.
    pub fn norm_sqrt(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Compute an integer power of this number efficiently with repeated squaring
    pub fn pow(&self, n: u32) -> Complex {
        let optimization = 5;

        if 0 == n {
            Complex::one()
        } else if n < optimization {
            let mut x = Complex::one();

            for _ in 0..n {
                x *= *self;
            }

            x
        } else {
            /// l = floor(log_2(n))
            /// r = n - 2^1
            let (l, r) = if n.is_power_of_two() {
                (n.trailing_zeros(), 0)
            } else {
                let p = n.checked_next_power_of_two().unwrap().trailing_zeros() - 1;
                (p, n - 2u32.pow(p))
            };

            let mut x = *self;
            let mut x = *self;

            for _ in 0..l {
                x *= x;
            }

            self.pow(r) * x
        }
    }

    /// real part
    pub fn re(&self) -> f64 {
        self.re
    }

    /// imaginary part
    pub fn im(&self) -> f64 {
        self.im
    }

    pub fn approx_eq(&self, other: &Complex) -> bool {
        let threshold = 0.000000000001;

        let d1 = (self.re() - other.re()).abs();
        let d2 = (self.im() - other.im()).abs();


        d1 < threshold && d2 < threshold
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}


impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex::new(self.re * rhs.re - self.im * rhs.im,
                     self.re * rhs.im + self.im * rhs.re)
    }
}


impl AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        *self = *self + rhs
    }
}

impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        *self = *self * rhs
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        c![-self.re, -self.im]
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:+.3} + {:+3}i", self.re, self.im)
    }
}


#[test]
fn complex_test() {
    assert_eq!(c![4f64, 6f64], c![1f64, 2f64] + c![3f64, 4f64]);
    assert_eq!(c![-5f64, 10f64], c![1f64, 2f64] * c![3f64, 4f64]);
    assert_eq!(5f64, c![1f64, 2f64].norm_sqrt());

    let mut z = c![1f64, 2f64];
    z += c![3f64, 4f64];
    assert_eq!(z, c![4f64, 6f64]);

    let x = Complex::nth_root_of_unity(15);
    assert!(Complex::one().approx_eq(&x.pow(15)));

    assert_eq!(Complex::one(), c![7f64, 8f64].pow(0));
}
