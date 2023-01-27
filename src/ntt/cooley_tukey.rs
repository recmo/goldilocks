use std::sync::Arc;

use super::Ntt;
use crate::{divisors::split, permute::{transpose, Permute}, Field};
use rayon::prelude::*;

pub struct CooleyTukey {
    size:     usize,
    split:    (usize, usize),
    inner_a:  Arc<dyn Ntt>,
    inner_b:  Arc<dyn Ntt>,
    transpose_ab: Arc<dyn Permute<Field>>,
    transpose_ba: Arc<dyn Permute<Field>>,
    root:     Field,
    twiddles: Vec<Field>,
    parallel: bool,
}

impl CooleyTukey {
    pub fn new(size: usize) -> Self {
        let split = split(size);
        assert!(
            split > 1,
            "Unimplemented Cooley-Tukey split for for {size} (split = {split}"
        );
        assert!(size % split == 0);
        let (a, b) = (split, size / split);
        let inner_a = super::strategy(a);
        let inner_b = super::strategy(b);
        let transpose_ab = crate::permute::transpose_strategy((a, b));
        let transpose_ba = crate::permute::transpose_strategy((b, a));

        // Compute twiddles
        let root = Field::root(size as u64).unwrap();

        // Precompute twiddles for medium sized NTTs.
        // (For large ones we just compute them on the fly.)
        let twiddles = if size <= 1 << 15 {
            let mut twiddles = Vec::with_capacity(size);
            let mut omega_b = Field::new(1);
            for _ in 0..b {
                let mut omega_ab = Field::new(1);
                for _ in 0..a {
                    twiddles.push(omega_ab);
                    omega_ab *= omega_b;
                }
                omega_b *= root;
            }
            twiddles
        } else {
            Vec::new()
        };

        Self {
            size,
            split: (a, b),
            inner_a,
            inner_b,
            transpose_ab,
            transpose_ba,
            root,
            twiddles,
            parallel: size >= 1 << 15,
        }
    }
}

impl Ntt for CooleyTukey {
    fn len(&self) -> usize {
        self.size
    }

    fn ntt(&self, values: &mut [Field]) {
        let (a, b) = self.split;

        // Interpret `values` as an (a, b) matrix and convert to a (b, a) matrix
        self.transpose_ab.permute(values);

        // Run `b` NTTs of size `a` and apply twiddles
        if !self.twiddles.is_empty() {
            let operation = |(i, (row, twiddles)): (usize, (&mut [Field], &[Field]))| {
                self.inner_a.ntt(row);

                // Apply twiddles
                if i > 0 {
                    for (value, &twiddle) in row.iter_mut().zip(twiddles).skip(1) {
                        *value *= twiddle;
                    }
                }
            };
            if !self.parallel {
                values
                    .chunks_exact_mut(a)
                    .zip(self.twiddles.chunks_exact(a))
                    .enumerate()
                    .for_each(operation);
            } else {
                values
                    .par_chunks_exact_mut(a)
                    .zip(self.twiddles.par_chunks_exact(a))
                    .enumerate()
                    .for_each(operation);
            }
        } else {
            if !self.parallel {
                let mut omega_col = self.root;
                values.chunks_exact_mut(a).enumerate().for_each(|(i, row)| {
                    self.inner_a.ntt(row);

                    // Compute and apply twiddles.
                    if i > 0 {
                        let mut omega_row = omega_col;
                        for value in row.iter_mut().skip(1) {
                            *value *= omega_row;
                            omega_row *= omega_col;
                        }
                        omega_col *= self.root;
                    }
                });
            } else {
                values.par_chunks_exact_mut(a).enumerate().for_each_with(
                    None,
                    |omega_col: &mut Option<Field>, (row, values)| {
                        self.inner_a.ntt(values);

                        // Compute and apply twiddles.
                        if row > 0 {
                            let omega_col =
                                omega_col.get_or_insert_with(|| self.root.pow(row as u64));
                            let mut omega_row = *omega_col;
                            for value in values.iter_mut().skip(1) {
                                *value *= omega_row;
                                omega_row *= *omega_col;
                            }
                            *omega_col *= self.root;
                        }
                    },
                );
            }
        }

        self.transpose_ba.permute(values);

        if !self.parallel {
            values.chunks_exact_mut(b).for_each(|row| {
                self.inner_b.ntt(row);
            });
        } else {
            values
                .par_chunks_mut(b)
                .for_each(|row| self.inner_b.ntt(row));
        }

        self.transpose_ab.permute(values);
    }
}

/// Cooley–Tukey $\sqrt{n}$ recursive NTT.
pub fn ntt(values: &mut [Field]) {
    const PAR_THRESHOLD: usize = 1 << 15;
    let n = values.len();

    // Interpret `values` as an approximately square matrix.
    let a = split(n);
    assert!(a > 1, "Unimplemented Cooley-Tukey split for for {n}");
    let b = n / a;

    if n < PAR_THRESHOLD {
        recurse(values, super::ntt, (a, b));
    } else {
        par_recurse(values, super::ntt, (a, b));
    }
}

/// Cooley-Tukey six-step NTT.
pub fn recurse(value: &mut [Field], inner: impl Fn(&mut [Field]), (a, b): (usize, usize)) {
    debug_assert_eq!(value.len(), a * b);
    let root = Field::root(value.len() as u64)
        .expect("Vector length does not divide multiplicative group order.");

    transpose(value, (a, b));
    let mut omega_col = root;
    value.chunks_exact_mut(a).enumerate().for_each(|(i, row)| {
        inner(row);
        if i > 0 {
            let mut omega_row = omega_col;
            for value in row.iter_mut().skip(1) {
                *value *= omega_row;
                omega_row *= omega_col;
            }
            omega_col *= root;
        }
    });
    transpose(value, (b, a));
    value.chunks_exact_mut(b).for_each(&inner);
    transpose(value, (a, b));
}

/// Parallel Cooley-Tukey six-step NTT.
pub fn par_recurse(
    value: &mut [Field],
    inner: impl Fn(&mut [Field]) + Sync,
    (a, b): (usize, usize),
) {
    debug_assert_eq!(value.len(), a * b);
    let root = Field::root(value.len() as u64)
        .expect("Vector length does not divide multiplicative group order.");

    transpose(value, (a, b));

    value.par_chunks_exact_mut(a).enumerate().for_each_with(
        None,
        |omega_col: &mut Option<Field>, (row, values)| {
            inner(values);
            if row > 0 {
                let omega_col = omega_col.get_or_insert_with(|| root.pow(row as u64));
                let mut omega_row = *omega_col;
                for value in values.iter_mut().skip(1) {
                    *value *= omega_row;
                    omega_row *= *omega_col;
                }
                *omega_col *= root;
            }
        },
    );
    transpose(value, (b, a));
    value.par_chunks_exact_mut(b).for_each(&inner);
    transpose(value, (a, b));
}

pub fn twiddle(value: &mut [Field], (rows, cols): (usize, usize)) {
    assert_eq!(value.len(), rows * cols);
    let root = Field::root(value.len() as u64)
        .expect("Vector length does not divide multiplicative group order.");

    debug_assert_eq!(root.pow((value.len() / 2) as u64), -Field::from(1));
    debug_assert_eq!(root.pow(value.len() as u64), Field::from(1));

    let mut omega_col = root;
    for j in 1..rows {
        let mut omega_row = omega_col;
        for i in 1..cols {
            value[j * cols + i] *= omega_row;
            omega_row *= omega_col;
        }
        omega_col *= root;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{naive, tests::test_ntt_fn},
        *,
    };

    #[test]
    fn test_ntt_16() {
        test_ntt_fn(ntt, 16);
    }

    #[test]
    fn test_ntt_256() {
        test_ntt_fn(ntt, 256);
    }

    #[test]
    #[ignore]
    fn test_ntt_65536() {
        test_ntt_fn(ntt, 65536);
    }

    #[test]
    fn test_ntt_1024() {
        test_ntt_fn(ntt, 1024);
    }

    #[test]
    fn test_ntt_384() {
        test_ntt_fn(ntt, 384);
    }

    #[test]
    fn test_recurse_256_16_16() {
        test_ntt_fn(|values| recurse(values, naive::ntt, (16, 16)), 256);
    }

    #[test]
    fn test_recurse_256_4_64() {
        test_ntt_fn(|values| recurse(values, naive::ntt, (4, 64)), 256);
    }

    #[test]
    fn test_par_recurse_256_16_16() {
        test_ntt_fn(|values| par_recurse(values, naive::ntt, (16, 16)), 256);
    }

    #[test]
    fn test_par_recurse_256_4_64() {
        test_ntt_fn(|values| par_recurse(values, naive::ntt, (4, 64)), 256);
    }
}
