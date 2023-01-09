mod copy;
mod cyclic;
pub mod permutation;
mod square;

pub use self::{copy::*, cyclic::*, square::*};
use crate::utils::gcd;

/// Transpose a matrix in place.
///
/// Optimized for large `gcd(width, height)`.
///
/// # Panics
///
/// Panics if `matrix.len()` does not equal `width * height`.
pub fn transpose(matrix: &mut [u64], width: usize, height: usize) {
    assert_eq!(matrix.len(), width * height);
    let g = gcd(width, height);

    // Transpose the square blocks
    for _j in (0..height).step_by(g) {
        for _i in (0..width).step_by(g) {
            todo!();
            // transpose_square(&mut matrix[i + j * width..i + g + j * width],
            // block* width, g);
        }
    }

    // Transpose the blocks themselves
    transpose_cyclic(matrix, g, width / g, height / g);
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
        transpose_copy(&mut matrix, (2, 2));
        assert_eq!(matrix, [
            0, 4, 
            1, 5, 
            2, 6,
            3, 7
        ]);
    }
}
