//! The Good-Thomas or Prime Factor FFT algorithm.
//! 
//! * <https://eng.libretexts.org/Bookshelves/Electrical_Engineering/Signal_Processing_and_Modeling/Fast_Fourier_Transforms_(Burrus)>
//! * <https://www.youtube.com/watch?v=8cjDKirNIko>
use super::Ntt;
use crate::{
    permute::{self, cycles, Permute},
    utils::{gcd, modinv},
    Field,
};
use rayon::prelude::*;
use std::sync::Arc;

pub struct GoodThomas {
    split:     (usize, usize),
    inner_n1:  Arc<dyn Ntt>,
    inner_n2:  Arc<dyn Ntt>,
    permute_i: Arc<dyn Permute<Field>>,
    transpose: Arc<dyn Permute<Field>>,
    permute_k: Arc<dyn Permute<Field>>,
    parallel:  bool,
}

impl GoodThomas {
    pub fn new(n1: usize, n2: usize) -> Self {
        let n = n1 * n2;
        assert_eq!(
            gcd(n1, n2),
            1,
            "Good-Thomas factors ({n1}×{n2}) must be coprime."
        );

        // Inner NTTs
        let inner_n1 = super::strategy(n1);
        let inner_n2 = super::strategy(n2);

        // Find permutation parameters
        // See C.S. Burrus (2018) eq. (10.5).
        let (k1, k2) = (n2, n1);
        let (k3, k4) = (modinv(k1, n1) * n2, modinv(k2, n2) * n1);

        // IDEA: At least for 3×5 this permutation can be a swap (k1=10, k2=6)!
        // Swaps are efficient to do in place. Can we always find a valid solution
        // that is a swap?
        // Can we find and alternative permutation for the transpose that is a swap?
        // Does this generalize to Cooley-Tukey?

        // Necessary and sufficient conditions for the choice of parameters to work.
        // See C.S. Burrus (2018) eq. (10.1) and (10.2).
        debug_assert_eq!(n1 * n2, n);
        debug_assert_eq!(gcd(n1, n2), 1);
        debug_assert_eq!(k1 % n2, 0);
        debug_assert_eq!(k2 % n1, 0);
        debug_assert_eq!(gcd(k1, n1), 1);
        debug_assert_eq!(gcd(k2, n2), 1);
        debug_assert_eq!(k3 % n2, 0);
        debug_assert_eq!(k4 % n1, 0);
        debug_assert_eq!(gcd(k3, n1), 1);
        debug_assert_eq!(gcd(k4, n2), 1);
        debug_assert_eq!((k1 * k4) % n, 0);
        debug_assert_eq!((k2 * k3) % n, 0);
        debug_assert_eq!((k1 * k3) % n, n2);
        debug_assert_eq!((k2 * k4) % n, n1);

        // Create permutations
        // TODO: For large splits (say 8192×65537) Cycles takes a lot of memory.
        let mut permute_i = cycles::Cycles::<u32>::from_fn(n, |i| {
            let (i1, i2) = (i / n2, i % n2);
            (i1 * k1 + i2 * k2) % n
        });
        let transpose = permute::transpose_strategy((n1, n2));
        let mut permute_k = cycles::Cycles::<u32>::from_fn(n, |i| {
            let (i1, i2) = (i % n1, i / n1);
            (i1 * k3 + i2 * k4) % n
        });

        permute_i.invert();

        Self {
            split: (n1, n2),
            inner_n1,
            inner_n2,
            permute_i: Arc::new(permute_i),
            transpose,
            permute_k: Arc::new(permute_k),
            parallel: n >= 1 << 15,
        }
    }
}

impl Ntt for GoodThomas {
    fn len(&self) -> usize {
        self.split.0 * self.split.1
    }

    fn ntt(&self, values: &mut [Field]) {
        assert_eq!(
            values.len(),
            self.len(),
            "Input length must match NTT length"
        );
        let (n1, n2) = self.split;

        // Input permutation.
        self.permute_i.permute(values);

        // First inner NTT: n1 parallel n2 sized NTT.
        if !self.parallel {
            values
                .chunks_exact_mut(n2)
                .for_each(|values| self.inner_n2.ntt(values));
        } else {
            values
                .par_chunks_exact_mut(n2)
                .for_each(|values| self.inner_n2.ntt(values));
        }

        // Transpose.
        self.transpose.permute(values);

        // Second inner NTT.
        if !self.parallel {
            values
                .chunks_exact_mut(n1)
                .for_each(|values| self.inner_n1.ntt(values));
        } else {
            values
                .par_chunks_exact_mut(n1)
                .for_each(|values| self.inner_n1.ntt(values));
        }

        // Output permutation.
        self.permute_k.permute(values);
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt, *};

    #[test]
    fn test_2x3() {
        test_ntt(GoodThomas::new(2, 3));
    }

    #[test]
    fn test_3x4() {
        test_ntt(GoodThomas::new(3, 4));
    }

    #[test]
    fn test_3x5() {
        test_ntt(GoodThomas::new(3, 5));
    }

    #[test]
    fn test_5x2() {
        test_ntt(GoodThomas::new(5, 3));
    }

    #[test]
    fn test_15x16() {
        test_ntt(GoodThomas::new(15, 16));
    }

    #[test]
    fn test_16x17() {
        test_ntt(GoodThomas::new(16, 17));
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::{super::bench::bench_ntt, *};
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        bench_ntt(criterion, "good_thomas", ntt, 15);
    }
}
