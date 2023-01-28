use super::Ntt;
use crate::{
    permute::{cycles, Permute},
    utils::modexp,
    Field,
};
use std::sync::Arc;

pub struct Rader {
    size:       usize,
    permute_pi: Arc<dyn Permute<Field>>,
    permute_ki: Arc<dyn Permute<Field>>,
    inner:      Arc<dyn Ntt>,
    twiddles:   Vec<Field>,
}

impl Rader {
    pub fn new(size: usize) -> Self {
        // Hardcoded parameters for the prime factors.
        let (gi, gk): (usize, usize) = match size {
            2 => (1, 1),
            3 => (2, 2),
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
        let permute_pi = Arc::new(cycles::from_fn(size, |i| {
            if i == 0 {
                0
            } else {
                modexp(gi, i - 1, size)
            }
        }));
        let permute_ki = Arc::new(cycles::from_fn(size, |i| {
            if i == 0 {
                0
            } else {
                modexp(gk, i - 1, size)
            }
        }));
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
            permute_pi,
            permute_ki,
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
        // Input permutation.
        self.permute_pi.permute(values);

        // Inner NTT
        self.inner.ntt(&mut values[1..]);

        // Apply constants, twiddles and scale factor.
        let x0 = values[0];
        values[0] += values[1];
        values[1..]
            .iter_mut()
            .zip(self.twiddles.iter())
            .for_each(|(b, &t)| *b *= t);
        values[1] += x0;

        // Transform back (scale factor already applied)
        values[1..].reverse();
        self.inner.ntt(&mut values[1..]);

        // Output permutation
        self.permute_ki.permute(values);
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
        bench_ntt(criterion, "rader", ntt, 2);
        bench_ntt(criterion, "rader", ntt, 3);
        bench_ntt(criterion, "rader", ntt, 5);
        bench_ntt(criterion, "rader", ntt, 17);
        bench_ntt(criterion, "rader", ntt, 257);
        bench_ntt(criterion, "rader", ntt, 65537);
    }
}
