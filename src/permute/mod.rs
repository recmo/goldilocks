mod copy;
pub mod cycles;
pub mod gcd;
pub mod gw18;
pub mod permutation;
mod square;

pub use self::{copy::*, square::*};
use std::sync::Arc;

pub trait Permute<T: 'static + Copy + Send + Sync>: Sync + Send {
    fn len(&self) -> usize;

    fn permute(&self, values: &mut [T]);
}

impl<T: 'static + Copy + Send + Sync> Permute<T> for Arc<dyn Permute<T>> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn permute(&self, values: &mut [T]) {
        self.as_ref().permute(values)
    }
}

/// Generate a strategy for transposing matrices of the given size.
pub fn transpose_strategy<T: 'static + Copy + Send + Sync>(
    (rows, cols): (usize, usize),
) -> Arc<dyn Permute<T> + 'static> {
    let size = rows * cols;
    let gcd = crate::utils::gcd(rows, cols);

    if rows == cols {
        Arc::new(SquareTranspose::new(rows)) as Arc<dyn Permute<T>>
    } else if size <= 1 << 20 {
        let permute = permutation::transpose(rows, cols);
        cycles::from_fn(size, permute)
    } else {
        Arc::new(gcd::Gcd::new(rows, cols)) as Arc<dyn Permute<T>>
        // Arc::new(gw18::Gw18::new((rows, cols))) as Arc<dyn Permute<T>>
    }
}

/// Transpose a matrix in place.
///
/// Optimized for large `gcd(width, height)`.
///
/// # Panics
///
/// Panics if `matrix.len()` does not equal `width * height`.
pub fn transpose<T: Copy + Send + Sync>(matrix: &mut [T], (rows, cols): (usize, usize)) {
    assert_eq!(matrix.len(), rows * cols);

    gw18::transpose(matrix, (rows, cols));
}

/// Transpose a matrix in place using a buffer.
pub fn transpose_copy<T: Copy>(matrix: &mut [T], (rows, cols): (usize, usize)) {
    let copy = matrix.to_vec();
    // Process in write order.
    for i in 0..cols {
        for j in 0..rows {
            matrix[i * rows + j] = copy[j * cols + i];
        }
    }
}

/// Bit-reverse permute an array in place.
pub fn bit_reverse<T: Copy + Send + Sync>(slice: &mut [T]) {
    let copy = slice.to_vec();
    permute_wo_oop(&copy, slice, 1, permutation::bit_reverse(slice.len()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_copy_2_3() {
        let mut matrix = [
            0, 1, 2, 
            3, 4, 5
        ];
        transpose_copy(&mut matrix, (2, 3));
        assert_eq!(matrix, [
            0, 3, 
            1, 4, 
            2, 5
        ]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_copy_3_3() {
        let mut matrix = [
            0, 1, 2, 
            3, 4, 5,
            6, 7, 8
        ];
        transpose_copy(&mut matrix, (3, 3));
        assert_eq!(matrix, [
            0, 3, 6,
            1, 4, 7,
            2, 5, 8
        ]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_copy_2_4() {
        let mut matrix = [
            0, 1, 2, 3, 
            4, 5, 6, 7
        ];
        transpose_copy(&mut matrix, (2, 4));
        assert_eq!(matrix, [
            0, 4, 
            1, 5, 
            2, 6,
            3, 7
        ]);
    }
}
