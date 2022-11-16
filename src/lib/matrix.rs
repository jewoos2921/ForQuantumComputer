use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Mul;

pub use crate::lib::complex::Complex;

/// max size of matrix and therefore ket.
pub const MAX_SIZE: usize = 32;
const MAX_ELEMENTS: usize = MAX_SIZE * MAX_SIZE;

/// Efficient array of complex Number
pub type Vector = [Complex; MAX_SIZE];

/// Represents a square matrix over C of maximum size 'MAX_SIZE'
///
/// Each element is an instance of 'Complex', and we store the elements
/// internally in an array of size MAX_SIZE * MAX_SIZE * sizeof(Complex);
///
/// In practice, this means each matrix occupies around '16kib'.
///
#[allow(missing_copy_implementations)]
pub struct Matrix {
    size: usize,
    elements: [Complex; MAX_ELEMENTS],
}


impl Matrix {
    /// Construct a new zero-initialized matrix of given size.
    ///
    /// # Panics
    ///
    /// We panic if the given size exceeds 'MAX_SIZE'
    pub fn new(size: usize) -> Matrix {
        assert!(size <= MAX_SIZE);

        Matrix {
            size,
            elements: [Complex::zero(); MAX_ELEMENTS],
        }
    }

    /// Construct a new  matrix of given size from elements
    ///
    /// # Panics
    ///
    /// We panic if the given size exceeds 'MAX_SIZE'
    pub fn new_from_elements(size: usize, elements: Vec<Complex>) -> Matrix {
        assert!(size <= MAX_SIZE);
        assert_eq!(size * size, elements.len());

        let mut m = Matrix::new(size);

        for (i, elem) in elements.iter().enumerate() {
            m.set(i / size, i % size, *elem)
        }

        m
    }

    /// Construct a new  identity matrix of given size.
    ///
    /// # Panics
    ///
    /// We panic if the given size exceeds 'MAX_SIZE'
    pub fn identity(size: usize) -> Matrix {
        assert!(size <= MAX_SIZE);

        let mut elements = [Complex::zero(); MAX_ELEMENTS];

        for i in 0..size {
            elements[i * MAX_SIZE + i] = Complex::one();
        }

        Matrix {
            size,
            elements,
        }
    }

    /// Embed another matrix into this one. over rising elements
    ///
    /// Embed with top-left position at (i, j)
    ///
    /// # Panics
    ///
    /// We panic if this matrix isn't large enough.
    pub fn embed(&mut self, other: &Matrix, i: usize, j: usize) {
        assert!(i + other.size <= self.size);
        assert!(j + other.size <= self.size);


        for x in 0..other.size {
            for y in 0..other.size {
                let value = other.get(x, y);
                self.set(i + x, i + y, value)
            }
        }
    }

    /// Permute the rows to generate a new matrix.
    ///
    /// Row _i_ goes to row _permutation[i]_.
    ///
    /// # Panics
    ///
    /// We panic if set(permutation) != {0, ..., self.size - 1}
    pub fn permute_rows(&self, permutation: Vec<usize>) -> Matrix {
        assert_eq!(self.size, permutation.len());
        assert!(Matrix::permutation_valid(&permutation));

        let mut m = Matrix::new(self.size);

        for (source_i, target_i) in permutation.iter().enumerate() {
            for j in 0..self.size {
                m.set(*target_i, j, self.get(source_i, j));
            }
        }

        m
    }

    /// Permute the columns to generate a new matrix.
    ///
    /// Column _i_ goes to column _permutation[i]_.
    ///
    /// # Panics
    ///
    /// We panic if set(permutation) != {0, ..., self.size - 1}
    pub fn permute_columns(&self, permutation: Vec<usize>) -> Matrix {
        assert_eq!(self.size, permutation.len());
        assert!(Matrix::permutation_valid(&permutation));


        let mut m = Matrix::new(self.size);

        for (source_j, target_j) in permutation.iter().enumerate() {
            for i in 0..self.size {
                m.set(i, *target_j, self.get(i, source_j));
            }
        }

        m
    }

    /// Tests whether the permutation is valid.
    fn permutation_valid(permutation: &Vec<usize>) -> bool {
        let mut sorted = permutation.clone();

        sorted.sort();
        for (i, val) in sorted.iter().enumerate() {
            if i != *val {
                return false;
            }
        }
        return true;
    }

    /// Size of the matrix
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the element in position (i, j)
    pub fn get(&self, i: usize, j: usize) -> Complex {
        self.elements[i * MAX_SIZE + j]
    }

    /// Set the element in position (i, j) to value.
    pub fn set(&mut self, i: usize, j: usize, value: Complex) {
        self.elements[i * MAX_SIZE + j] = value;
    }

    /// Approximately equal test.
    pub fn approx_eq(&self, other: &Matrix) -> bool {
        if self.size != other.size {
            return false;
        }

        for i in 0..self.size {
            for j in 0..self.size {
                if !self.get(i, j).approx_eq(&other.get(i, j)) {
                    return false;
                }
            }
        }
        true
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(size={}, elements=[", self.size).ok();
        for i in 0..self.size {
            write!(f, "\n").ok();

            for j in 0..self.size {
                write!(f, "[{:?}] ", self.get(i, j)).ok();
            }
        }
        write!(f, "]")
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.size, other.size);

        for i in 0..MAX_ELEMENTS {
            if self.elements[i] != other.elements[i] {
                return false;
            }
        }
        true
    }
}

impl<'a> Add<&'a Matrix> for &'a Matrix {
    type Output = Matrix;

    fn add(self, rhs: &'a Matrix) -> Self::Output {
        assert_eq!(self.size, rhs.size);

        let mut m = Matrix::new(self.size);

        for i in 0..self.size {
            for j in 0..self.size {
                m.set(i, j, self.get(i, j) + rhs.get(i, j));
            }
        }
        m
    }
}

impl<'a> Mul<&'a Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'a Matrix) -> Self::Output {
        assert_eq!(self.size, rhs.size);

        let mut m = Matrix::new(self.size);

        for i in 0..self.size {
            for j in 0..self.size {
                let mut val = Complex::zero();

                for k in 0..self.size {
                    val += self.get(i, k) * rhs.get(k, j);
                }

                m.set(i, j, val)
            }
        }
        m
    }
}

/// Permute the columns to generate a new matrix.
/// Column i goes to column permutationi.
/// # Panics
/// We panic if set(permutation) != {0, ..., self.size - 1}
impl<'a> Mul<&'a Vector> for &'a Matrix {
    type Output = Vector;

    fn mul(self, rhs: &'a Vector) -> Self::Output {
        let mut output = [Complex::zero(); MAX_SIZE];

        // Check that vector tail is zero.
        for i in self.size..MAX_SIZE {
            assert_eq!(Complex::zero(), rhs[i]);
        }

        for i in 0..self.size {
            let mut val = Complex::zero();
            for k in 0..self.size {
                val += self.get(i, k) * rhs[k];
            }

            output[i] = val;
        }
        output
    }
}


#[test]
fn matrix_test() {
    let m = m_real![1, 2; 3, 4];

    let mut v: Vector = [Complex::zero(); MAX_SIZE];
    v[0] = c!(10f64, 0f64);
    v[1] = c!(20f64, 0f64);

    let mut expected: Vector = [Complex::zero(); MAX_SIZE];
    expected[0] = c!(50f64, 0f64);
    expected[1] = c!(110f64, 0f64);

    let added = m_real![2, 4; 6, 8];

    let squared = m_real![7, 10; 15, 22];

    assert_eq!(added, &m + &m);
    assert_eq!(squared, &m * &m);
    assert_eq!(expected, &m * &v);
}

#[test]
fn embed_test() {
    let mut m = m_real![1, 2; 3, 4];
    let n = m_real![5];

    m.embed(&n, 1, 1);

    assert_eq!(m_real![1, 2; 3, 5], m);
}

#[test]
fn permutation_test() {
    let m = m_real![1, 2; 3, 4];

    assert_eq!(m_real![1, 2; 3, 4], m.permute_rows(vec![0, 1]));
    assert_eq!(m_real![3, 4; 1, 2], m.permute_rows(vec![1, 0]));

    assert_eq!(m_real![1, 2; 3, 4], m.permute_columns(vec![0, 1]));
    assert_eq!(m_real![2, 1; 4, 3], m.permute_columns(vec![1, 0]));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn bad_row_permutation_test() {
    let m = m_real![1, 2; 3, 4];

    m.permute_rows(vec![0, 0]);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn bad_column_permutation_test() {
    let m = m_real![1, 2; 3, 4];

    m.permute_columns(vec![0, 0]);
}