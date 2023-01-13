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
    let (a, b) = (3, 5);

    let mut expected = values.to_vec();
    crate::ntt::naive::ntt(&mut expected);

    // Input permutation.
    let mut buffer = [
        values[0], values[6], values[12], values[3], values[9],
        values[10], values[1], values[7], values[13], values[4],
        values[5], values[11], values[2], values[8], values[14],
    ];
    // buffer[0] = values[0];
    // buffer[6] = values[1];
    // buffer[12] = values[2];
    // buffer[3] = values[3];
    // buffer[9] = values[4];
    // buffer[10] = values[5];
    // buffer[1] = values[6];
    // buffer[7] = values[7];
    // buffer[13] = values[8];
    // buffer[4] = values[9];
    // buffer[5] = values[10];
    // buffer[11] = values[11];
    // buffer[2] = values[12];
    // buffer[8] = values[13];
    // buffer[14] = values[14];

    transpose(&mut buffer, (a, b));
    buffer.chunks_exact_mut(a).for_each(small::ntt_3);
    transpose(&mut buffer, (b, a));
    buffer.chunks_exact_mut(b).for_each(small::ntt_5);
    transpose(&mut buffer, (a, b));

    // Output permutation.
    values.copy_from_slice(&[
        buffer[0], buffer[3], buffer[6], buffer[9], buffer[12],
        buffer[5], buffer[8], buffer[11], buffer[14], buffer[2],
        buffer[10], buffer[13], buffer[1], buffer[4], buffer[7],
    ]);

    assert_eq!(values, &expected[..]);
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
