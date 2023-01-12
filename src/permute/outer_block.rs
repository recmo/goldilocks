//! Transpose of non-square matrices

use crate::utils::gcd;
use std::ptr;
use super::permutation;
use super::square::transpose_square;

pub fn transpose<T: Copy>(a: &mut [T], (rows, cols): (usize, usize)) {
    debug_assert_eq!(a.len(), rows * cols);
    if a.is_empty() {
        return;
    }

    // Compute the GCD, which is a power of two.
    let twos = (rows | cols).trailing_zeros();
    debug_assert_eq!(1 << twos, gcd(rows, cols));



}