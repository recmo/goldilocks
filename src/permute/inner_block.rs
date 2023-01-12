//! Transpose of non-square matrices

use crate::utils::gcd;
use std::ptr;

use super::permutation;

pub fn transpose<T: Copy>(a: &mut [T], (rows, cols): (usize, usize)) {
    debug_assert_eq!(a.len(), rows * cols);
    if a.is_empty() {
        return;
    }

    // Compute the GCD, which is a power of two.
    let twos = (rows | cols).trailing_zeros();
    debug_assert_eq!(1 << twos, gcd(rows, cols));
    unsafe {
        inplace(
            a.as_mut_ptr(),
            cols,
            (rows >> twos, cols >> twos),
            1_usize << twos,
        );
    }
}

unsafe fn inplace<T: Copy>(a: *mut T, stride: usize, block: (usize, usize), size: usize) {
    if size == 1 {
        inplace_block(a, stride, block);
    } else {
        let size = size >> 1;
        let right = size * block.1;
        let down = size * stride * block.0;

        inplace(a, stride, block, size);
        inplace(a.add(right + down), stride, block, size);
        swap(a.add(right), a.add(down), stride, block, size);
    }
}

unsafe fn swap<T: Copy>(a: *mut T, b: *mut T, stride: usize, block: (usize, usize), size: usize) {
    if size == 1 {
        block_swap(a, b, stride, block);
    } else {
        let size = size >> 1;
        let right = size * block.1;
        let down = size * stride * block.0;
        swap(a, b, stride, block, size);
        swap(a.add(right), b.add(down), stride, block, size);
        swap(a.add(down), b.add(right), stride, block, size);
        swap(
            a.add(right + down),
            b.add(right + down),
            stride,
            block,
            size,
        );
    }
}

unsafe fn inplace_block<T: Copy>(a: *mut T, stride: usize, (rows, cols): (usize, usize)) {
    // TODO: Re-use buffer in thread.
    let mut buffer = Vec::with_capacity(rows * cols);
    for i in 0..rows {
        for j in 0..cols {
            buffer.push(*a.add(i * stride + j));
        }
    }
    let permute = permutation::transpose(cols, rows);
    for i in 0..rows {
        for j in 0..cols {
            *a.add(i * stride + j) = buffer[permute(i * cols + j)];
        }
    }
}

unsafe fn block_swap<T: Copy>(a: *mut T, b: *mut T, stride: usize, (rows, cols): (usize, usize)) {
    let (mut a, mut b) = (a, b);
    let permute = permutation::transpose(cols, rows);
    for i in 0..rows {
        for j in 0..cols {
            let other = permute(i * cols + j);
            let (k, l) = (other / cols, other % cols);
            ptr::swap_nonoverlapping(a.add(i * stride + j), b.add(k * stride + l), 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::transpose_copy, *};

    #[test]
    fn test_transpose_4x6() {
        let mut matrix = (0..24 as u64).collect::<Vec<_>>();
        let mut reference = matrix.clone();
        transpose(&mut matrix, (4, 6));
        transpose_copy(&mut reference, (4, 6));
        assert_eq!(matrix, reference);
    }
}
