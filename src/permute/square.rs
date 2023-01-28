//! Transpose square matrices.
use std::{
    ptr::swap_nonoverlapping,
    sync::atomic::{AtomicPtr, Ordering},
};

use super::Permute;

pub struct SquareTranspose(usize);

impl SquareTranspose {
    pub fn new(size: usize) -> Self {
        Self(size)
    }
}

impl<T: 'static + Copy + Send + Sync> Permute<T> for SquareTranspose {
    fn len(&self) -> usize {
        self.0 * self.0
    }

    fn permute(&self, values: &mut [T]) {
        assert_eq!(values.len(), self.0 * self.0);
        transpose(values, self.0);
    }
}

pub fn transpose<T: Copy>(values: &mut [T], size: usize) {
    // eprintln!("square::transpose({size})");
    assert_eq!(values.len(), size * size);
    if size <= 1 {
        return;
    }

    unsafe {
        transpose_square(values.as_mut_ptr(), size, size);
    }
}

unsafe fn transpose_square<T: Copy>(values: *mut T, stride: usize, size: usize) {
    // eprintln!("square::transpose({size})");
    const REC_THRESHOLD: usize = 1 << 4;
    const PAR_THRESHOLD: usize = 1 << 10;

    if size < REC_THRESHOLD {
        // TODO: Try diagonal order to reduce cache collision.
        for i in 0..size {
            for j in 0..i {
                unsafe {
                    // Safety: pointers are non-overlapping and point to valid elements.
                    swap_nonoverlapping(values.add(i * stride + j), values.add(j * stride + i), 1);
                }
            }
        }
    } else if size < PAR_THRESHOLD {
        // Recurse by splitting into four quadrants.
        // +-----+-----+
        // | a×a | a×b |
        // +-----+-----+
        // | b×a | b×b |
        // +-----+-----+
        let a = size / 2;
        let b = size - a;

        transpose_square(values, stride, a);
        transpose_swap(values.add(a), values.add(a * stride), stride, (a, b));
        transpose_square(values.add(a * stride + a), stride, b);
    } else {
        // Cast to AtomicPtr to allow sharing between threads.
        let values = AtomicPtr::new(values);

        let a = size / 2;
        let b = size - a;
        rayon::join(
            || {
                let values = values.load(Ordering::Relaxed);
                transpose_swap(values.add(a), values.add(a * stride), stride, (a, b));
            },
            || {
                rayon::join(
                    || transpose_square(values.load(Ordering::Relaxed), stride, a),
                    || {
                        transpose_square(
                            values.load(Ordering::Relaxed).add(a * stride + a),
                            stride,
                            b,
                        )
                    },
                );
            },
        );
    }
}

unsafe fn transpose_swap<T: Copy>(
    a: *mut T,
    b: *mut T,
    stride: usize,
    (rows, cols): (usize, usize),
) {
    // eprintln!("square::transpose_swap({rows}, {cols})");
    const REC_THRESHOLD: usize = 1 << 8;
    const PAR_THRESHOLD: usize = 1 << 20;
    let size = rows * cols;

    if size < REC_THRESHOLD {
        // TODO: Run down diagonals to reduce cache collision.
        for i in 0..rows {
            for j in 0..cols {
                unsafe {
                    // Safety: a and b are non-overlapping and point to valid elements.
                    let ai = i * stride + j;
                    let bi = j * stride + i;
                    swap_nonoverlapping(a.add(ai), b.add(bi), 1);
                }
            }
        }
    } else if size < PAR_THRESHOLD {
        // Recurse by splitting in half along longest axis.
        if rows > cols {
            let top = rows / 2;
            let bottom = rows - top;
            transpose_swap(a, b, stride, (top, cols));
            transpose_swap(a.add(top * stride), b.add(top), stride, (bottom, cols));
        } else {
            let left = cols / 2;
            let right = cols - left;
            transpose_swap(a, b, stride, (rows, left));
            transpose_swap(a.add(left), b.add(left * stride), stride, (rows, right));
        }
    } else {
        // Cast to AtomicPtr to allow sharing between threads.
        let a = AtomicPtr::new(a);
        let b = AtomicPtr::new(b);

        // Recurse by splitting in half along longest axis.
        if rows > cols {
            let top = rows / 2;
            let bottom = rows - top;
            rayon::join(
                || {
                    let a = a.load(Ordering::Relaxed);
                    let b = b.load(Ordering::Relaxed);
                    transpose_swap(a, b, stride, (top, cols));
                },
                || {
                    let a = a.load(Ordering::Relaxed);
                    let b = b.load(Ordering::Relaxed);
                    transpose_swap(a.add(top * stride), b.add(top), stride, (bottom, cols));
                },
            );
        } else {
            let left = cols / 2;
            let right = cols - left;
            rayon::join(
                || {
                    let a = a.load(Ordering::Relaxed);
                    let b = b.load(Ordering::Relaxed);
                    transpose_swap(a, b, stride, (rows, left));
                },
                || {
                    let a = a.load(Ordering::Relaxed);
                    let b = b.load(Ordering::Relaxed);
                    transpose_swap(a.add(left), b.add(left * stride), stride, (rows, right));
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::copy, *};

    #[test]
    fn test_transpose() {
        for size in 0_usize..100 {
            let n = size * size;
            let mut matrix = (0_u64..n as u64).collect::<Vec<_>>();
            let mut reference = matrix.clone();
            transpose(&mut matrix, size);
            copy::transpose(&mut reference, (size, size));
            assert_eq!(matrix, reference);
        }
    }
}
