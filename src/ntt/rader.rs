use crate::Field;
use crate::utils::modexp;

pub fn ntt(values: &mut [Field]) {
    let n = values.len();

    // Lookup generator pairs for all factors.
    let (gi, gk): (usize, usize) = match n {
        ..=1 => return,
        2 => (1, 1),
        3 => (2, 2),
        5 => (2, 3),
        17 => (3, 6),
        257 => (3, 86),
        65537 => (3, 21846),
        _ => panic!("Size {n} not supported by Rader NTT"),
    };
    debug_assert_eq!((gi * gk) % n, 1);

    // Construct permutations.
    let pi = |i| modexp(gi, i, n);
    let pk = |i| modexp(gk, i, n);

    // Permute
    let mut buffer = vec![Field::default(); n - 1];
    for i in 0..n - 1 {
        buffer[i] = values[pk(i)];
    }

    // Transform using n-1 sized transform
    super::ntt(&mut buffer);

    // Add `values[1..].sum()` to `value[0]`. `buffer[0]` conveniently contains
    // this sum after the NTT.
    let x0 = values[0];
    values[0] += buffer[0];

    // Apply twiddles
    let mut twiddles = (0..n - 1).map(|i| Field::root(n as u64).unwrap().pow(pi(i) as u64)).collect::<Vec<_>>();
    super::ntt(&mut twiddles);

    for i in 0..n - 1 {
        buffer[i] *= twiddles[i];
    }

    // Add x0 to all `values[1..]` terms by adding to the constant term before INTT.
    buffer[0] += x0 * Field::from(n as u64 - 1);

    // Transform back
    super::intt(&mut buffer);

    // Permute into results
    for i in 0..n - 1 {
        values[pi(i)] = buffer[i];
    }
}

/// Rader NTT of size 2.
///
/// It's basically the same as the naive NTT.
pub fn ntt_2(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 2);
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

/// Rader NTT of size 5.
///
/// Uses 4 multiplications, 2 shifts and 18 additions/subtractions. Compare this
/// to the naive implementation which uses 16 multiplications and 20
/// additions/subtractions.
pub fn ntt_5(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 5);
    let a = values[0];
    let b = values[1];
    let c = values[2];
    let d = values[3];
    let e = values[4];

    // Permute [b, c, d, e] to make the remaining DFT matrix cyclic.
    let (b, c, d, e) = (b, d, e, c);

    // Transform [b, c, d, e] for cyclic convolution.
    let (b, d) = (b + d, b - d);
    let (c, e) = (c + e, c - e);
    let e = e << 48;
    let (b, c) = (b + c, b - c);
    let (d, e) = (d + e, d - e);
    let (b, c, d, e) = (b, d, c, e);

    // At this point `b` sums all the other terms.
    let t = a;
    let a = a + b;

    // Point multiply by the NTT transform of [ω ω² ω⁴ ω³],
    // Also includes 1/4 scaling factor for the inverse transform.
    let b = b * Field::new(4611686017353646080);
    let c = c * Field::new(16181989089180173841);
    let d = d * Field::new(5818851782451133869);
    let e = e * Field::new(11322249509082494407);

    // We add `x₀` to the constant term (unscaled).
    let b = b + t;

    // Transform back to complete the cyclic convolution.
    let (b, c, d, e) = (b, e, d, c);
    let (b, d) = (b + d, b - d);
    let (c, e) = (c + e, c - e);
    let e = e << 48;
    let (b, c) = (b + c, b - c);
    let (d, e) = (d + e, d - e);
    let (b, c, d, e) = (b, d, c, e);

    // Permute [b, c, d, e] back to original order.
    let (b, c, d, e) = (b, c, e, d);

    values[0] = a;
    values[1] = b;
    values[2] = c;
    values[3] = d;
    values[4] = e;
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt_fn, *};

    #[test]
    fn test_ntt_specific() {
        test_ntt_fn(ntt_2, 2);
        test_ntt_fn(ntt_3, 3);
        test_ntt_fn(ntt_5, 5);
    }

    #[test]
    fn test_ntt_small() {
        test_ntt_fn(ntt, 0);
        test_ntt_fn(ntt, 1);
        test_ntt_fn(ntt, 2);
    }

    #[test]
    fn test_ntt_3() {
        test_ntt_fn(ntt, 3);
    }

    #[test]
    fn test_ntt_5() {
        test_ntt_fn(ntt, 5);
    }

    #[test]
    fn test_ntt_17() {
        test_ntt_fn(ntt, 17);
    }

    #[test]
    fn test_ntt_257() {
        test_ntt_fn(ntt, 257);
    }

    #[test]
    #[ignore] // The naive reference implementation is expensive.
    fn test_ntt_65537() {
        test_ntt_fn(ntt, 65537);
    }

}
