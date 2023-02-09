use super::{cycles::Cycles, square, Permute};
use crate::utils::gcd;
use rayon::prelude::*;
use std::sync::atomic::{AtomicPtr, Ordering};

/// Transpose a matrix in place.
pub struct Gcd {
    rows:     usize,
    cols:     usize,
    square:   usize,
    cycles:   Cycles<u32>,
    parallel: bool,
}

impl Gcd {
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows >= 1 && cols >= 1);
        let size = rows * cols;
        let square = gcd(rows, cols);
        let (rows, cols) = (rows / square, cols / square);

        // Compute final row permutation
        // This is probably not the most efficient way to construct the cycles,
        // but it works and is only done once.
        let mut permutation = vec![0; rows * cols * square];
        for i in 0..rows {
            for j in 0..cols {
                for k in 0..square {
                    permutation[j * square * rows + i + k * rows] =
                        i * square * cols + j + k * cols;
                }
            }
        }
        let cycles = Cycles::<u32>::from_list(&permutation);

        Self {
            rows,
            cols,
            square,
            cycles,
            parallel: size > 1 << 17,
        }
    }
}

impl<T: 'static + Copy + Send + Sync> Permute<T> for Gcd {
    fn len(&self) -> usize {
        self.rows * self.cols * self.square * self.square
    }

    fn permute(&self, values: &mut [T]) {
        assert_eq!(values.len(), <Gcd as Permute<T>>::len(self));

        // Transpose squares
        let stride = self.cols * self.square;
        if !self.parallel {
            for i in 0..self.rows {
                for j in 0..self.cols {
                    unsafe {
                        square::transpose_square(
                            values
                                .as_mut_ptr()
                                .add(i * stride * self.square + j * self.square),
                            stride,
                            self.square,
                        );
                    }
                }
            }
        } else {
            // Cast to AtomicPtr to allow sharing between threads.
            let values = AtomicPtr::new(values.as_mut_ptr());

            (0..(self.rows * self.cols)).into_par_iter().for_each(|i| {
                let values = values.load(Ordering::Relaxed);
                let i = i / self.cols;
                let j = i % self.cols;
                unsafe {
                    square::transpose_square(
                        values.add(i * stride * self.square + j * self.square),
                        stride,
                        self.square,
                    );
                }
            });
        }

        // Cycle rows
        if !self.parallel {
            self.cycles.apply_chunks(values, self.square);
        } else {
            self.cycles.par_apply_chunks(values, self.square);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::copy, *};
    use crate::divisors::{divisors, split};

    #[test]
    fn test_transpose_4_6() {
        test_size(4, 6);
    }

    #[test]
    fn test_transpose_32_16() {
        test_size(32, 16);
    }

    #[test]
    fn test_transpose_16_32() {
        test_size(16, 32);
    }

    #[test]
    fn test_transpose_17_20() {
        test_size(17, 20);
    }

    #[test]
    fn test_transpose() {
        for size in divisors()
            .iter()
            .filter(|n| **n <= (1 << 12))
            .map(|n| *n as usize)
        {
            let a = split(size);
            let b = size / a;
            test_size(a, b);
            test_size(b, a);
        }
    }

    fn test_size(rows: usize, cols: usize) {
        let size = rows * cols;
        let mut matrix = (0_u64..size as u64).collect::<Vec<_>>();
        let mut expected = matrix.clone();
        Gcd::new(rows, cols).permute(&mut matrix);
        copy::transpose(&mut expected, (rows, cols));
        assert_eq!(matrix, expected);
    }
}
