//! The Good-Thomas or Prime Factor FFT algorithm.
//!
//! See <https://amir.sdsu.edu/Bhagat18High.pdf>
use crate::Field;
use crate::ntt::small;
use crate::permute::gw18::transpose;

pub fn recurse(value: &mut [Field], inner: impl Fn(&mut [Field]), (a, b): (usize, usize)) {

    todo!()
}

pub fn ntt_15(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 15);
    let n = 15;
    let (a, b) = (3, 5);
    let (n1, n2) = (5, 3);
    let (m1, m2) = (10, 6);

    // Input permutation.
    let mut buffer = [Field::new(0); 15];
    for i1 in 0..a {
        for i2 in 0..b {
            let i = (i1 * n1 + i2 * n2) % n;
            buffer[i1 * b + i2] = values[i];
        }
    }

    buffer.chunks_exact_mut(b).for_each(small::ntt_5);
    transpose(&mut buffer, (b, a));
    buffer.chunks_exact_mut(a).for_each(small::ntt_3);

    // Output permutation.
    for i1 in 0..a {
        for i2 in 0..b {
            let k = (i1 * m1 + i2 * m2) % n;
            values[k] = buffer[i1 + i2 * a];
        }
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
