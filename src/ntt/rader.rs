use super::Ntt;
use crate::{
    permute::{cycles, Permute},
    utils::modexp,
    Field,
};
use std::sync::Arc;

pub struct Rader {
    size:      usize,
    permute_i: Arc<dyn Permute<Field>>,
    permute_k: Arc<dyn Permute<Field>>,
    inner:     Arc<dyn Ntt>,
    twiddles:  Vec<Field>,
}

impl Rader {
    pub fn new(size: usize) -> Self {
        // Hardcoded parameters for the prime factors.
        let (gi, gk): (usize, usize) = match size {
            2 => (1, 1), // No-op
            3 => (2, 2), // No-op
            5 => (2, 3),
            17 => (3, 6),
            257 => (3, 86),
            65537 => (3, 21846),
            _ => panic!("Size {size} not supported by Rader NTT"),
        };
        debug_assert_eq!((gi * gk) % size, 1);
        debug_assert_eq!(modexp(gi, size / 2, size), size - 1);
        debug_assert_eq!(modexp(gk, size / 2, size), size - 1);

        // Permutations
        let permute_i = cycles::Cycles::<u16>::from_fn(size - 1, |i| modexp(gi, i, size) - 1);
        let mut permute_k = cycles::Cycles::<u16>::from_fn(size - 1, |i| modexp(gk, i, size) - 1);
        permute_k.invert();
        let inner = super::strategy(size - 1);

        // Twiddles
        let mut twiddles = vec![Field::new(0); size - 1];
        let root = Field::root(size as u64).unwrap();
        let scale = Field::from((size - 1) as u64).inv();
        for i in 0..size - 1 {
            let pi = modexp(gi, i, size);
            twiddles[i] = root.pow(pi as u64) * scale;
        }
        inner.ntt(twiddles.as_mut_slice());

        Self {
            size,
            permute_i: Arc::new(permute_i) as Arc<dyn Permute<Field>>,
            permute_k: Arc::new(permute_k) as Arc<dyn Permute<Field>>,
            twiddles,
            inner,
        }
    }
}

impl Ntt for Rader {
    fn len(&self) -> usize {
        self.size
    }

    fn ntt(&self, values: &mut [Field]) {
        assert_eq!(values.len(), self.size);
        let (first, convolve) = values.split_first_mut().unwrap();

        // Input permutation.
        self.permute_k.permute(convolve);

        // Inner NTT
        self.inner.ntt(convolve);

        // Apply constants, twiddles and scale factor.
        let x0 = *first;
        *first += convolve[0];
        convolve
            .iter_mut()
            .zip(self.twiddles.iter())
            .for_each(|(b, &t)| *b *= t);
        convolve[0] += x0;

        // Transform back (scale factor already applied)
        convolve[1..].reverse();
        self.inner.ntt(convolve);

        // Output permutation
        self.permute_i.permute(convolve);
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt, *};

    #[test]
    fn test_2() {
        test_ntt(Rader::new(2));
    }

    #[test]
    fn test_3() {
        test_ntt(Rader::new(3));
    }

    #[test]
    fn test_5() {
        test_ntt(Rader::new(5));
    }

    #[test]
    fn test_17() {
        test_ntt(Rader::new(17));
    }

    #[test]
    fn test_257() {
        test_ntt(Rader::new(257));
    }

    #[test]
    #[ignore] // The naive reference implementation takes 4 minutes.
    fn test_65537() {
        test_ntt(Rader::new(65537));
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::{super::bench::bench_ntt, *};
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        bench_ntt(criterion, "rader", Rader::new(2));
        bench_ntt(criterion, "rader", Rader::new(3));
        bench_ntt(criterion, "rader", Rader::new(5));
        bench_ntt(criterion, "rader", Rader::new(17));
        bench_ntt(criterion, "rader", Rader::new(257));
        bench_ntt(criterion, "rader", Rader::new(65537));
    }
}
