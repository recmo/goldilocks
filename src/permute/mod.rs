mod copy;
mod cyclic;
pub mod permutation;
mod square;

pub use self::{copy::*, cyclic::*, square::*};

/// Transpose a matrix in place.
///
/// Optimized for large `gcd(width, height)`.
///
/// # Panics
///
/// Panics if `matrix.len()` does not equal `width * height`.
pub fn transpose<T: Copy>(matrix: &mut [T], (rows, cols): (usize, usize)) {
    assert_eq!(matrix.len(), rows * cols);

    if rows == cols {
        assert_eq!(std::mem::size_of::<T>(), 8);
        transpose_square_pub(unsafe { std::mem::transmute(matrix) }, cols, rows);
    } else {
        // TODO: Better algorithm.
        transpose_copy(matrix, (rows, cols));
    }
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
