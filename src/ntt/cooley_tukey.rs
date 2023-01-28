use super::Ntt;
use crate::{
    permute::{self, Permute},
    Field,
};
use rayon::prelude::*;
use std::sync::Arc;

pub struct CooleyTukey {
    size:         usize,
    split:        (usize, usize),
    inner_a:      Arc<dyn Ntt>,
    inner_b:      Arc<dyn Ntt>,
    transpose_ab: Arc<dyn Permute<Field>>,
    transpose_ba: Arc<dyn Permute<Field>>,
    root:         Field,
    twiddles:     Vec<Field>,
    parallel:     bool,
}

impl CooleyTukey {
    pub fn new(a: usize, b: usize) -> Self {
        let size = a * b;
        let inner_a = super::strategy(a);
        let inner_b = super::strategy(b);
        let transpose_ab = permute::transpose_strategy((a, b));
        let transpose_ba = permute::transpose_strategy((b, a));

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

#[cfg(test)]
mod tests {
    use crate::ntt::good_thomas::GoodThomas;

    use super::{
        super::{naive, tests::test_ntt},
        *,
    };

    #[test]
    fn test_4x4() {
        test_ntt(GoodThomas::new(4, 4));
    }

    #[test]
    fn test_16x16() {
        test_ntt(GoodThomas::new(16, 16));
    }

    #[test]
    fn test_32x32() {
        test_ntt(GoodThomas::new(32, 32));
    }

    #[test]
    fn test_16x24() {
        test_ntt(GoodThomas::new(16, 24));
    }

    #[test]
    fn test_4x64() {
        test_ntt(GoodThomas::new(16, 24));
    }
}
