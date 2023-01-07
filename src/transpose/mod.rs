mod cyclic;
mod permute;
mod square;

pub use self::{cyclic::*, permute::*, square::*};
use crate::utils::gcd;
use std::ops::Range;

/// Transpose a matrix in place.
///
/// Optimized for large `gcd(width, height)`.
pub fn transpose(matrix: &mut [u64], width: usize, height: usize) {
    assert_eq!(matrix.len(), width * height);
    let g = gcd(width, height);

    // Transpose the square blocks
    for j in (0..height).step_by(g) {
        for i in (0..width).step_by(g) {
            todo!();
            // transpose_square(&mut matrix[i + j * width..i + g + j * width],
            // block* width, g);
        }
    }

    // Transpose the blocks themselves
    transpose_cyclic(matrix, g, width / g, height / g);
}

/// Transpose a matrix in place using a buffer.
pub fn transpose_copy(matrix: &mut [u64], width: usize, height: usize) {
    let copy = matrix.to_vec();
    // Process in write order.
    for i in 0..width {
        for j in 0..height {
            matrix[i * height + j] = copy[j * width + i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_copy() {
        let mut matrix = [
            0, 1, 2, 
            3, 4, 5
        ];
        transpose_copy(&mut matrix, 3, 2);
        assert_eq!(matrix, [
            0, 3, 
            1, 4, 
            2, 5
        ]);
    }
}
