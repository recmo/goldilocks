//! The Good-Thomas or Prime Factor FFT algorithm.
//!
//! See <https://amir.sdsu.edu/Bhagat18High.pdf>
use crate::Field;
use crate::ntt::small;
use crate::permute::gw18::transpose;
use crate::utils::gcd;

pub fn recurse(value: &mut [Field], inner: impl Fn(&mut [Field]), (a, b): (usize, usize)) {

    todo!()
}

pub fn ntt_15(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 15);

    // Parameters
    let n = 15;
    let (a, b) = (3, 5);
    let (n1, n2) = (5, 3);
    let (m1, m2) = (10, 6);

    debug_assert_eq!(a * b, n);
    debug_assert_eq!(gcd(a, b), 1);
    debug_assert_eq!((n1 * m2) % n, 0);
    debug_assert_eq!((n2 * m1) % n, 0);

    let permute_i = |i| {
        let (i1, i2) = (i / b, i % b);
        (i1 * n1 + i2 * n2) % n
    };
    let permute_k = |i| {
        let (i1, i2) = (i % a, i / a);
        (i1 * m1 + i2 * m2) % n
    };

    // Input permutation.
    let mut buffer = [Field::new(0); 15];
    for (i, b) in buffer.iter_mut().enumerate() {
        *b = values[permute_i(i)];
    }

    buffer.chunks_exact_mut(b).for_each(small::ntt_5);
    transpose(&mut buffer, (b, a));
    buffer.chunks_exact_mut(a).for_each(small::ntt_3);

    // Output permutation.
    for (i, v) in buffer.iter().enumerate() {
        values[permute_k(i)] = *v;
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt_fn, *};

    #[test]
    fn test_generic_15() {
        test_ntt_fn(ntt_15, 15);
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::{super::bench::bench_ntt, *};
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        bench_ntt(criterion, "good_thomas", ntt_15, 15);
    }
}
