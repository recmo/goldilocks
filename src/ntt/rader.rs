use crate::{ntt::small, utils::modexp, Field};
use std::sync::Once;

// (pi, ki, twiddle) values for the Rader NTT of size 257.
static mut RADER_257: ([usize; 256], [usize; 256], [Field; 256]) =
    ([0; 256], [0; 256], [Field::new(0); 256]);
static RADER_257_INIT: Once = Once::new();

// (pi, ki, twiddle) values for the Rader NTT of size 65537.
static mut RADER_65537: ([usize; 65536], [usize; 65536], [Field; 65536]) =
    ([0; 65536], [0; 65536], [Field::new(0); 65536]);
static RADER_65537_INIT: Once = Once::new();

/// Rader NTT of prime size.
///
/// Reduces a NTT of size `n` to a cyclic convolution of size `n - 1`, which
/// is computed with two NTTs of size `n - 1`. Since the prime factors of the
/// Goldilocks multiplicative order are the Fermat primes, the NTTs will be
/// 2^2^k sized, which is very efficient to compute.
pub fn ntt(values: &mut [Field]) {
    match values.len() {
        ..=1 => return,
        2 => ntt_2(values),
        3 => ntt_3(values),
        5 => ntt_5(values),
        17 => ntt_17(values),
        257 => ntt_257(values),
        65537 => ntt_65537(values),
        n => panic!("Size {n} not supported by Rader NTT"),
    }
}

/// Parameters for the Rader permutation.
pub fn parameters(n: usize) -> (usize, usize) {
    let (gi, gk): (usize, usize) = match n {
        2 => (1, 1),
        3 => (2, 2),
        5 => (2, 3),
        17 => (3, 6),
        257 => (3, 86),
        65537 => (3, 21846),
        _ => panic!("Size {n} not supported by Rader NTT"),
    };
    debug_assert_eq!((gi * gk) % n, 1);
    debug_assert_eq!(modexp(gi, n / 2, n), n - 1);
    debug_assert_eq!(modexp(gk, n / 2, n), n - 1);
    (gi, gk)
}

/// Twiddle factors for the Rader NTT.
pub fn twiddles(n: usize, p: impl Fn(usize) -> usize) -> Vec<Field> {
    let root = Field::root(n as u64).unwrap();
    let scale = Field::new((n - 1) as u64).inv();
    let mut twiddles = (0..n - 1)
        .map(|i| root.pow(p(i) as u64) * scale)
        .collect::<Vec<_>>();
    super::ntt(&mut twiddles);
    twiddles
}

pub fn generic(values: &mut [Field]) {
    let n = values.len();

    // Lookup generator pairs for all factors.
    let (gi, gk) = parameters(n);

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
    let twiddles = twiddles(n, pi);

    for i in 0..n - 1 {
        buffer[i] *= twiddles[i];
    }

    // Add x0 to all `values[1..]` terms by adding to the constant term before INTT.
    buffer[0] += x0;

    // Transform back
    buffer[1..].reverse();
    super::ntt(&mut buffer);

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
///
/// This method is also used by the codegen for the small NTTs.
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

pub fn ntt_17(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 17);

    // Input permutation and NTT
    #[rustfmt::skip]
    let mut buffer = [
        values[ 1], values[ 6], values[ 2], values[12],
        values[ 4], values[ 7], values[ 8], values[14],
        values[16], values[11], values[15], values[ 5],
        values[13], values[10], values[ 9], values[ 3],
    ];
    small::ntt_16(&mut buffer);

    // Apply constants, twiddles and scaling factors.
    let a = values[0];
    values[0] += buffer[0];
    buffer[0] *= Field::new(1152921504338411520);
    buffer[0] += a;
    buffer[1] *= Field::new(6259776822214049175);
    buffer[2] *= Field::new(9380094172986290191);
    buffer[3] *= Field::new(891943630314919127);
    buffer[4] *= Field::new(17228171707553225791);
    buffer[5] *= Field::new(12855743360534130886);
    buffer[6] *= Field::new(6167687396920564837);
    buffer[7] *= Field::new(17201834061724655524);
    buffer[8] *= Field::new(15308299771656910737);
    buffer[9] *= Field::new(18186005861103657533);
    buffer[10] *= Field::new(53595491891823545);
    buffer[11] *= Field::new(1906638201581172103);
    buffer[12] *= Field::new(18303651001328874822);
    buffer[13] *= Field::new(3077466521755967626);
    buffer[14] *= Field::new(12423593102987598328);
    buffer[15] *= Field::new(18361513053649472048);

    // inverse NTT to complete the convolution.
    buffer[1..].reverse();
    small::ntt_16(&mut buffer);

    // Output permutation, plus reversal for inverse NTT.
    values[1] = buffer[0];
    values[3] = buffer[1];
    values[9] = buffer[2];
    values[10] = buffer[3];
    values[13] = buffer[4];
    values[5] = buffer[5];
    values[15] = buffer[6];
    values[11] = buffer[7];
    values[16] = buffer[8];
    values[14] = buffer[9];
    values[8] = buffer[10];
    values[7] = buffer[11];
    values[4] = buffer[12];
    values[12] = buffer[13];
    values[2] = buffer[14];
    values[6] = buffer[15];
}

pub fn ntt_257(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 257);

    // Initialize parameters
    let (pi, ki, twiddles) = unsafe {
        // Safety: We initialize once and then never mutate.
        RADER_257_INIT.call_once(|| {
            let (pi, ki, twiddles) = &mut RADER_257;
            let root = Field::root(257).unwrap();
            let scale = Field::from(256).inv();
            for i in 0..256 {
                pi[i] = modexp(3, i, 257);
                ki[i] = modexp(86, i, 257);
                twiddles[i] = root.pow(pi[i] as u64) * scale;
            }
            super::ntt(twiddles);
        });
        &RADER_257
    };

    // Input permutation and NTT
    let mut buffer = Vec::with_capacity(256);
    buffer.extend(ki.iter().map(|&i| values[i]));
    super::ntt(&mut buffer);

    // Apply constants, twiddles and scale factor.
    let x0 = values[0];
    values[0] += buffer[0];
    buffer
        .iter_mut()
        .zip(twiddles.iter())
        .for_each(|(b, &t)| *b *= t);
    buffer[0] += x0;

    // Transform back
    buffer[1..].reverse();
    super::ntt(&mut buffer);

    // Permute into results
    pi.iter()
        .zip(buffer.iter())
        .for_each(|(&i, &b)| values[i] = b);
}

pub fn ntt_65537(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 65537);

    // Initialize parameters
    let (pi, ki, twiddles) = unsafe {
        // Safety: We initialize once and then never mutate.
        RADER_65537_INIT.call_once(|| {
            let (pi, ki, twiddles) = &mut RADER_65537;
            let root = Field::root(65537).unwrap();
            let scale = Field::from(65536).inv();
            for i in 0..65536 {
                pi[i] = modexp(3, i, 65537);
                ki[i] = modexp(21846, i, 65537);
                twiddles[i] = root.pow(pi[i] as u64) * scale;
            }
            super::ntt(twiddles);
        });
        &RADER_65537
    };

    // Input permutation and NTT
    let mut buffer = Vec::with_capacity(65536);
    buffer.extend(ki.iter().map(|&i| values[i]));
    super::ntt(&mut buffer);

    // Apply constants, twiddles and scale factor.
    let x0 = values[0];
    values[0] += buffer[0];
    buffer
        .iter_mut()
        .zip(twiddles.iter())
        .for_each(|(b, &t)| *b *= t);
    buffer[0] += x0;

    // Transform back
    buffer[1..].reverse();
    super::ntt(&mut buffer);

    // Permute into results
    pi.iter()
        .zip(buffer.iter())
        .for_each(|(&i, &b)| values[i] = b);
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt_fn, *};
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn test_ntt_small() {
        test_ntt_fn(ntt, 0);
        test_ntt_fn(ntt, 1);
    }

    #[test]
    fn test_generic_3() {
        test_ntt_fn(generic, 3);
    }

    #[test]
    fn test_generic_5() {
        test_ntt_fn(generic, 5);
    }

    #[test]
    fn test_generic_17() {
        test_ntt_fn(generic, 17);
    }

    #[test]
    fn test_generic_257() {
        test_ntt_fn(generic, 257);
    }

    #[test]
    #[ignore] // The naive reference implementation takes 4 minutes.
    fn test_generic_65537() {
        test_ntt_fn(generic, 65537);
    }

    #[test]
    fn test_ntt_2() {
        test_ntt_fn(ntt_2, 2);
    }

    #[test]
    fn test_ntt_3() {
        test_ntt_fn(ntt_3, 3);
    }

    #[test]
    fn test_ntt_5() {
        test_ntt_fn(ntt_5, 5);
    }

    #[test]
    fn test_ntt_17() {
        test_ntt_fn(ntt_17, 17);
    }

    #[test]
    fn test_ntt_257() {
        test_ntt_fn(ntt_257, 257);
    }

    #[test]
    #[ignore] // The naive reference implementation takes 4 minutes.
    fn test_ntt_65537() {
        test_ntt_fn(ntt_65537, 65537);
    }

    // Test specific and generic against each other for 65537.
    #[test]
    fn test_ntt_65537_generic() {
        let mut rng = StdRng::seed_from_u64(Field::MODULUS);
        let mut values = (0..65537).map(|_| rng.gen()).collect::<Vec<_>>();
        let mut expected = values.clone();
        generic(&mut expected);
        ntt_65537(&mut values);
        assert_eq!(values[..10], expected[..10]);
        for (&value, &expected) in values.iter().zip(expected.iter()) {
            assert_eq!(value, expected);
        }
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
