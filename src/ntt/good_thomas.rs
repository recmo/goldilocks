//! The Good-Thomas or Prime Factor FFT algorithm.
//!
//! If the length of the input is smooth then this method won't be used
//! directly, but only as part of the codelets, as there wil allways be a factor
//! 2 in the split.
use crate::{
    divisors::split,
    permute::transpose,
    utils::{gcd, modinv},
    Field,
};

// TODO: Use a mapping that avoids modulo operations.

pub fn ntt(values: &mut [Field]) {
    let n1 = split(values.len());
    debug_assert!(n1 > 1, "Good-Thomas requires length to be composite");
    let n2 = values.len() / n1;
    debug_assert_eq!(
        gcd(n1, n2),
        1,
        "Good-Thomas requires n1 and n2 to be coprime"
    );
    recurse(values, super::ntt, (n1, n2));
}

pub fn recurse(values: &mut [Field], inner: impl Fn(&mut [Field]), (n1, n2): (usize, usize)) {
    let n = values.len();
    debug_assert_eq!(n1 * n2, n);

    // Compute permutations
    let (k1, k2, k3, k4) = parameters(n1, n2);
    let permute_i = |i| {
        let (i1, i2) = (i / n2, i % n2);
        (i1 * k1 + i2 * k2) % n
    };
    let permute_k = |i| {
        let (i1, i2) = (i % n1, i / n1);
        (i1 * k3 + i2 * k4) % n
    };

    // Input permutation.
    let mut buffer = vec![Field::new(0); n];
    for (i, b) in buffer.iter_mut().enumerate() {
        *b = values[permute_i(i)];
    }

    buffer.chunks_exact_mut(n2).for_each(&inner);
    transpose(&mut buffer, (n2, n1));
    buffer.chunks_exact_mut(n1).for_each(&inner);

    // Output permutation.
    for (i, v) in buffer.iter().enumerate() {
        values[permute_k(i)] = *v;
    }
}

#[must_use]
pub fn parameters(n1: usize, n2: usize) -> (usize, usize, usize, usize) {
    let n = n1 * n2;
    debug_assert_eq!(gcd(n1, n2), 1);

    // Find parameters
    // See C.S. Burrus (2018) eq. (10.5).
    let (k1, k2) = (n2, n1);
    let (k3, k4) = (modinv(n2, n1) * n2, modinv(n1, n2) * n1);

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

    (k1, k2, k3, k4)
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt_fn, *};

    #[test]
    fn test_generic_6() {
        test_ntt_fn(ntt, 6);
    }

    #[test]
    fn test_generic_12() {
        test_ntt_fn(ntt, 12);
    }

    #[test]
    fn test_generic_15() {
        test_ntt_fn(ntt, 15);
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
