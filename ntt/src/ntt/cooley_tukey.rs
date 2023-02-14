use super::{Ntt, MIN_WORK_SIZE};
use crate::{
    permute::{self, Permute},
    utils::div_round_up,
    Field,
};
use rayon::prelude::*;
use std::{cmp, sync::Arc};

pub struct CooleyTukey {
    size:         usize,
    split:        (usize, usize),
    inner_a:      Arc<dyn Ntt>,
    inner_b:      Arc<dyn Ntt>,
    transpose_ab: Arc<dyn Permute<Field>>,
    transpose_ba: Arc<dyn Permute<Field>>,
    root:         Field,
    twiddles:     Vec<Field>,
    a_work_size:  usize,
    b_work_size:  usize,
}

impl CooleyTukey {
    #[allow(clippy::similar_names)] // TODO
    pub fn new(a: usize, b: usize) -> Self {
        // eprintln!("CooleyTukey({a} x {b})");
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

        // Compute (parallel) work sizes
        // If these are larger than `n` `rayon` will not parallelize and run
        // on the current thread, which is what we want.
        let a_work_size = cmp::min(size, div_round_up(MIN_WORK_SIZE, a) * a);
        let b_work_size = cmp::min(size, div_round_up(MIN_WORK_SIZE, b) * b);

        Self {
            size,
            split: (a, b),
            inner_a,
            inner_b,
            transpose_ab,
            transpose_ba,
            root,
            twiddles,
            a_work_size,
            b_work_size,
        }
    }
}

impl Ntt for CooleyTukey {
    fn len(&self) -> usize {
        self.size
    }

    fn ntt(&self, values: &mut [Field]) {
        debug_assert_eq!(values.len() % self.size, 0);
        for values in values.chunks_exact_mut(self.size) {
            let (a, _) = self.split;

            // Interpret `values` as an (a, b) matrix and convert to a (b, a) matrix
            self.transpose_ab.permute(values);

            // Run `b` NTTs of size `a` and apply twiddles
            if !self.twiddles.is_empty() {
                values
                    .par_chunks_mut(self.a_work_size)
                    .zip(self.twiddles.par_chunks(self.a_work_size))
                    .enumerate()
                    .for_each(
                        |(i, (mut block, mut twiddles)): (usize, (&mut [Field], &[Field]))| {
                            self.inner_a.ntt(block);

                            // Apply twiddles to the block
                            if i == 0 {
                                // Skip first row
                                block = &mut block[a..];
                                twiddles = &twiddles[a..];
                            }
                            for (row, twiddles) in
                                block.chunks_exact_mut(a).zip(twiddles.chunks_exact(a))
                            {
                                for (value, &twiddle) in row.iter_mut().zip(twiddles).skip(1) {
                                    *value *= twiddle;
                                }
                            }
                        },
                    );
            } else {
                values
                    .par_chunks_mut(self.a_work_size)
                    .enumerate()
                    .for_each(|(mut i, mut block)| {
                        self.inner_a.ntt(block);

                        // Compute and apply twiddles.
                        if i == 0 {
                            // Skip first row
                            block = &mut block[a..];
                            i += 1;
                        }
                        let first_row = i * self.a_work_size / a;
                        let mut omega_col = self.root.pow(first_row as u64);
                        for row in block.chunks_exact_mut(a) {
                            let mut omega_row = omega_col;
                            for value in row[1..a - 1].iter_mut() {
                                *value *= omega_row;
                                omega_row *= omega_col;
                            }
                            row[a - 1] *= omega_row;
                            omega_col *= self.root;
                        }
                    });
            }

            self.transpose_ba.permute(values);

            values
                .par_chunks_mut(self.b_work_size)
                .for_each(|block| self.inner_b.ntt(block));

            self.transpose_ab.permute(values);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt, *};

    #[test]
    fn test_4x4() {
        test_ntt(CooleyTukey::new(4, 4));
    }

    #[test]
    fn test_16x16() {
        test_ntt(CooleyTukey::new(16, 16));
    }

    #[test]
    fn test_32x32() {
        test_ntt(CooleyTukey::new(32, 32));
    }

    #[test]
    fn test_16x24() {
        test_ntt(CooleyTukey::new(16, 24));
    }

    #[test]
    fn test_4x64() {
        test_ntt(CooleyTukey::new(16, 24));
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::{super::bench::bench_ntt, *};
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        bench_ntt(criterion, "cooley_tukey", CooleyTukey::new(16, 16));
        bench_ntt(criterion, "cooley_tukey", CooleyTukey::new(256, 257));
    }
}
