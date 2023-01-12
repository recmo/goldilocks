use super::small;
use crate::Field;

const RADER_5: [Field; 4] = [
    Field::new(18446744069414584320),
    Field::new(8395509897500808986),
    Field::new(4828663060389951155),
    Field::new(9387724148476942401),
];

pub fn ntt(values: &mut [Field]) -> bool {
    match values.len() {
        ..=1 => {}
        2 => ntt_2(values),
        3 => ntt_3(values),
        17 => ntt_17(values),
        257 => ntt_257(values),
        65537 => ntt_65537(values),
        _ => return false,
    }
    true
}

/// Rader NTT of size 3.
///
/// It's basically the same as the naive NTT.
pub fn ntt_2(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 3);
    let a = values[0];
    let b = values[1];
    let (a, b) = (a + b, a - b);
    values[0] = a;
    values[1] = b;
}

/// Rader NTT of size 3.
///
/// Compared to the naive implementation this saves one bit-shift.
/// But in the native version the bitshifts are `64` and `32` which are
/// more efficient than the `95` and `63 + 31` in this version. It also requires
/// one more addition/subtraction.
pub fn ntt_3(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 3);
    let a = values[0];
    let b = values[1];
    let c = values[2];

    let t = a;

    // Transform [b, c] for cyclic convolution.
    let (b, c) = (b + c, b - c);

    // Take care of the ω⁰ terms. This is easiest after the 2-NTT because the
    // constant term `b` is the sum of the other terms.
    // let (a, b) = (a + b, a + b);

    let a = a + b;

    // Point multiply by the transform of [ω, ω²],
    // multiplied by ½ to scale for the inverse transform.
    let b = b << 95;
    let c = (c << 63) + (c << 31);

    let b = b + t;

    // Transform back to complete the cyclic convolution.
    let (b, c) = (b + c, b - c);

    values[0] = a;
    values[1] = b;
    values[2] = c;
}

pub fn rader_ntt(values: &mut [Field]) {
    let n = values.len();
    let generator = 2_usize;
    let inv_generator = 3_usize;

    let permutation = |i: usize| generator.pow(i as u32) % n;
    let inv_permutation = |i: usize| inv_generator.pow(i as u32) % n;

    for i in 0..n - 1 {
        dbg!(i, permutation(i), inv_permutation(i));
        assert_eq!(inv_permutation(permutation(i)), i);
    }
}

/// Rader NTT of size 5.
pub fn ntt_5(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 5);
    let a = values[0];
    let b = values[1];
    let c = values[2];
    let d = values[3];
    let e = values[4];
    let t = a;

    // Permute [b, c, d, e] to make the remaining DFT matrix cyclic.
    let (b, c, d, e) = (b, d, e, c);
    // let (b, d, e, c) = (b, c, d, e);

    // Transform [b, c, d, e] for cyclic convolution.
    let (b, d) = (b + d, b - d);
    let (c, e) = (c + e, c - e);
    let e = e << 48;
    let (b, c) = (b + c, b - c);
    let (d, e) = (d + e, d - e);

    let a = a + b;

    let scale = Field::from(1);
    // let mut coeffs = [1, 3, 4, 2]
    let mut coeffs = [1, 4, 2, 3]
        .iter()
        .map(|i| Field::root(5).unwrap().pow(*i) * scale)
        .collect::<Vec<_>>();
    crate::ntt::ntt_naive(coeffs.as_mut_slice());

    let b = b * coeffs[0];
    let c = c * coeffs[1];
    let d = d * coeffs[2];
    let e = e * coeffs[3];

    // Point multiply by the transform of [ω ω³ ω⁴ ω²],
    // let b = b * RADER_5[0];
    // let c = c * RADER_5[1];
    // let d = d * RADER_5[2];
    // let e = e * RADER_5[3];

    // multiplied by ¼ to scale for the inverse transform.
    let b = b >> 2;
    let c = c >> 2;
    let d = d >> 2;
    let e = e >> 2;

    let b = b + t;

    // Transform back to complete the cyclic convolution.
    let (b, c) = (b + c, b - c);
    let (d, e) = (d + e, d - e);
    let e = -(e << 48);
    let (b, d) = (b + d, b - d);
    let (c, e) = (c + e, c - e);

    // Permute [b, c, d, e] back to original order.
    let (b, c, d, e) = (b, d, e, c);
    // let (b, d, e, c) = (b, c, d, e);

    values[0] = a;
    values[1] = b;
    values[2] = c;
    values[3] = d;
    values[4] = e;
}

pub fn ntt_17(values: &mut [Field]) {
    todo!();
}

pub fn ntt_257(values: &mut [Field]) {
    todo!();
}

pub fn ntt_65537(values: &mut [Field]) {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::{super::ntt_naive, *};

    #[test]
    fn test_ntt_3() {
        let size = 3;
        let input = (43..43 + size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_3(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_5_coeff() {
        let scale = Field::from(4).inv();
        let scale = Field::from(1);
        // let mut coeffs = [1, 3, 4, 2]
        let mut coeffs = [2, 4, 3, 1]
            .iter()
            .map(|i| Field::root(5).unwrap().pow(*i) * scale)
            .collect::<Vec<_>>();
        ntt_naive(coeffs.as_mut_slice());
        assert_eq!(RADER_5, coeffs[..]);
    }

    #[test]
    fn test_ntt_5() {
        let size = 5;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        rader_ntt(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }
}
