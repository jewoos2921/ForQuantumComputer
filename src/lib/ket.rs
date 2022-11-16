use float_cmp::ApproxEqUlps;

use crate::lib::{complex, matrix};
use matrix::MAX_SIZE;
use complex::Complex;

/// A ket describe the state of a quantum register.
///
///
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ket {
    size: usize,
    /// The ket's element's w.r.t. the computational basis.
    pub elements: [Complex; MAX_SIZE],

}

impl Ket {
    /// Construct a new, zero-initialized ket of given size.
    pub fn new(size: usize) -> Ket {
        Ket {
            size,
            elements: [Complex::zero(); MAX_SIZE],
        }
    }


    /// 
}