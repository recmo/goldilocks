//! Platform agnostic implementations.
//!
//! Also serve as reference implementations for tests.

//! TODO: Square root: https://zcash.github.io/halo2/design/implementation/fields.html#sarkar-square-root-algorithm-table-based-variant

// https://github.com/facebook/winterfell/blob/main/math/src/field/f64/mod.rs#L121

// We do a lot of intentional casting with truncation in this file.
#![allow(clippy::cast_possible_truncation, clippy::cast_lossless)]

use crate::utils::branch_hint;
use core::mem::swap;

/// p = φ² - φ + 1 = 2⁶⁴ - 2³² + 1
const MODULUS: u64 = 0xffff_ffff_0000_0001;

/// The preferred generator for the field.
const GENERATOR: u64 = 2717;

/// Order of the multiplicative group
const ORDER: u64 = MODULUS - 1;

/// Reduce a `u64`
pub const fn reduce_64(mut n: u64) -> u64 {
    if n > MODULUS {
        n -= MODULUS;
    }
    n
}

/// Reduce a 128 bit number
pub fn reduce_128(n: u128) -> u64 {
    reduce_159(n as u64, (n >> 64) as u32, (n >> 96) as u64)
}

/// Reduce a 159 bit number
/// See <https://cp4space.hatsya.com/2021/09/01/an-efficient-prime-for-number-theoretic-transforms/>
fn reduce_159(low: u64, mid: u32, high: u64) -> u64 {
    debug_assert!(high <= u64::MAX >> 1);
    let (mut low2, carry) = low.overflowing_sub(high);
    if carry {
        branch_hint(); // A borrow is exceedingly rare. It is faster to branch.
        low2 = low2.wrapping_add(MODULUS);
    }

    let mut product = (mid as u64) << 32;
    product -= product >> 32;

    let (mut result, carry) = product.overflowing_add(low2);
    if carry {
        // This branch is likely to happen. It should compile to a use
        // branchless conditional operations.
        result = result.wrapping_sub(MODULUS);
    }

    // Final reduction.
    // At this point the value is already correct mod MODULUS, but it may be
    // larger than MODULUS. Doing this extra reduction does not have a measurable
    // impact on performance.
    if result >= MODULUS {
        // This branch is unlikely to happen. It is guaranteed not to be taken
        // if the above branch was taken. (But merging the two branches is
        // slower.)
        branch_hint();
        result -= MODULUS;
    }
    debug_assert!(result < MODULUS);
    result
}

/// Adds two field elements.
///
/// Requires `a` and `b` to be reduced.
pub fn add(a: u64, b: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    let (mut result, carry) = a.overflowing_add(b);
    if carry {
        result = result.wrapping_sub(MODULUS);
    }
    if result >= MODULUS {
        // This branch is unlikely to happen. It is guaranteed not to be taken
        // if the above branch was taken. (But merging the two branches is
        // slower.)
        branch_hint();
        result -= MODULUS;
    }
    debug_assert!(result < MODULUS);
    result
}

/// Multiplies two field elements.
pub fn mul(a: u64, b: u64) -> u64 {
    // return mul_2(a, b);
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    reduce_128((a as u128) * (b as u128))
}

// OPT: Dedicated `square` fn

/// Compute a^e
pub fn pow(mut a: u64, mut e: u64) -> u64 {
    debug_assert!(a < MODULUS);

    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mul(r, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    r
}

/// Compute the preferred n-th root of unity.
pub fn root(n: u64) -> u64 {
    assert_eq!(ORDER % n, 0);
    pow(GENERATOR, ORDER / n)
}

/// Compute a ⋅ 2^n
pub fn shift(mut a: u64, n: u64) -> u64 {
    debug_assert!(a < MODULUS);
    // OPT: Avoid div-rem
    let (q, r) = (n / 96, n % 96);
    if q & 1 == 1 {
        a = MODULUS - a;
    }
    match r {
        0 => a,
        1..=32 => reduce_159(a << r, (a >> (64 - r)) as u32, 0),
        33..=63 => reduce_159(a << r, (a >> (64 - r)) as u32, a >> (96 - r)),
        64..=95 => reduce_159(0, (a << (r - 64)) as u32, a >> (96 - r)),
        96.. => unreachable!(),
    }
}

/// Compute a ⋅ ω₃₈₄ⁱ
pub fn root_384(a: u64, i: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(i < 384);

    if i & 1 == 0 {
        shift(a, i >> 1)
    } else {
        let i = i >> 1;
        add(shift(a, i + 24), shift(a, i + 168))
    }
}

/// Modular inverse.
///
/// Returns 0 for 0.
pub fn inv(a: u64) -> u64 {
    debug_assert!(a < MODULUS);
    if a == 0 {
        return 0;
    }

    // Initial values for the half-extended GCD with MODULUS
    let (mut u, mut v) = (MODULUS, a);
    let (mut t0, mut t1) = (0_u64, 1_u64);

    // Make sure `v` is odd
    let mut twos = v.trailing_zeros();
    v >>= twos;

    // Initial negative sign, shifting left or right by 96 bits is the same
    // as negating because 2^96 = -1 in Goldilocks.
    twos += 96;

    // Binary half-extended GCD
    loop {
        debug_assert!(u > v);
        debug_assert_eq!(u & 1, 1);
        debug_assert_eq!(v & 1, 1);
        debug_assert_eq!(t1 * u + t0 * v, MODULUS);

        u -= v;
        t0 += t1;

        let count = u.trailing_zeros();
        u >>= count;
        t1 <<= count;
        twos += count;

        if u < v {
            swap(&mut u, &mut v);
            swap(&mut t0, &mut t1);
            // We need to flip the sign, which is the same as shifting right by 96
            twos += 96;
        }
        if u == v {
            break;
        }
    }

    // Dividing by 2^shift is equivalent to multiplying by 2^(191 * shift)
    // because 2 is a 192th primitive root.
    twos *= 191;
    t0 = shift(t0, twos as u64);
    debug_assert!(t0 < MODULUS);
    debug_assert_eq!(mul(t0, a), 1);
    t0
}

// TODO: <https://github.com/facebook/winterfell/blob/21173bdf3e552ca7662c7aa2d34515b084ae21b0/math/src/field/f64/mod.rs#L121>
#[allow(dead_code)]
fn inv_addchain(a: u64) -> u64 {
    debug_assert!(a < MODULUS);

    // OPT: Benchmark against GCD based methods.

    // Invert using Fermat's little theorem.
    // Method from Adam P. Goucher <https://twitter.com/apgox>
    // This uses 72 multiplications, which is one less than the chain returned
    // by the BBBD algorithm in <https://github.com/str4d/addchain>.
    // a^-1 = a^(p-2) mod p
    // p - 2 = 2(2^31 - 1)(2^32 + 1) + 1
    // 2^31 - 1 = 2(2^15 - 1)(2^15 + 1) + 1  (optimal)
    // 2^15 - 1 has an addition chain in Archim Flammenkamp's database.

    // Compute b = a^(2^15 - 1) = a^32767 using an optimal addition chain.
    // See <http://wwwhomes.uni-bielefeld.de/achim/addition_chain.html>
    let a2 = mul(a, a);
    let a3 = mul(a2, a);
    let a6 = mul(a3, a3);
    let a7 = mul(a6, a);
    let a14 = mul(a7, a7);
    let a28 = mul(a14, a14);
    let a56 = mul(a28, a28);
    let a63 = mul(a56, a7);
    let a126 = mul(a63, a63);
    let a252 = mul(a126, a126);
    let a504 = mul(a252, a252);
    let a1008 = mul(a504, a504);
    let a2016 = mul(a1008, a1008);
    let a4032 = mul(a2016, a2016);
    let a4095 = mul(a4032, a63);
    let a8190 = mul(a4095, a4095);
    let a16380 = mul(a8190, a8190);
    let a32760 = mul(a16380, a16380);
    let b = mul(a32760, a7);
    // 19 multiplications (optimal)

    // Compute c = a^(2^31 - 1)
    // 19 + 18 = 37 multiplications (optimal)
    let mut c = b;
    for _ in 0..15 {
        c = mul(c, c);
    } // b^(2^15)
    c = mul(c, b); // b^(2^15 + 1) = a^(2^30 - 1)
    c = mul(c, c); // b^(2^16 + 2) = a^(2^31 - 2)
    c = mul(c, a); // 2^31 - 1

    // Compute d = a^(p - 2)
    // 37 + 35 = 72 multiplications
    let mut d = c;
    for _ in 0..32 {
        d = mul(d, d);
    }
    d = mul(d, c); // c^(2^31 + 1) = a^((2^31 - 1)(2^32 + 1))
    d = mul(d, d); // c^2(2^31 + 1) a^(2(2^31 - 1)(2^32 + 1))
    d = mul(d, a); // a^(2(2^32 - 2)(2^32 + 1) + 1)
    d
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::{prop_assume, proptest};

    #[test]
    fn test_add_identity() {
        proptest!(|(a: u64)| {
            prop_assume!(a < MODULUS);
            let b = add(a, 0);
            assert_eq!(a, b);
        });
    }

    #[test]
    fn test_add_commutative() {
        proptest!(|(a: u64, b: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let ab = add(a, b);
            let ba = add(b, a);
            assert_eq!(ab, ba);
        });
    }

    #[test]
    fn test_add_associative() {
        proptest!(|(a: u64, b: u64, c: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let a_bc = add(a, add(b, c));
            let ab_c = add(add(a, b), c);
            assert_eq!(a_bc, ab_c);
        });
    }

    #[test]
    fn test_mul_identity() {
        proptest!(|(a: u64)| {
            prop_assume!(a < MODULUS);
            let b = mul(a, 1);
            assert_eq!(a, b);
        });
    }

    #[test]
    fn test_mul_commutative() {
        proptest!(|(a: u64, b: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let ab = mul(a, b);
            let ba = mul(b, a);
            assert_eq!(ab, ba);
        });
    }

    #[test]
    fn test_mul_associative() {
        proptest!(|(a: u64, b: u64, c: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let a_bc = mul(a, mul(b, c));
            let ab_c = mul(mul(a, b), c);
            assert_eq!(a_bc, ab_c);
        });
    }

    #[test]
    fn test_distributive() {
        proptest!(|(a: u64, b: u64, c: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let a_bc = mul(a, add(b, c));
            let ab_ac = add(mul(a, b), mul(a, c));
            assert_eq!(a_bc, ab_ac);
        });
    }

    #[test]
    fn test_pow() {
        proptest!(|(a: u64, b in 0_u64..100, c in 0_u64..100)| {
            prop_assume!(a < MODULUS);
            let ab = pow(a, b);
            let ac = pow(a, c);
            let a_bc = pow(a, b + c);
            let ab_ac = mul(ab, ac);
            assert_eq!(a_bc, ab_ac);
        });
    }

    #[test]
    fn test_sqrt_two() {
        // Verify that we take the positive square root of two
        assert_eq!(root(384), add(root(8), pow(root(8), 7)));
    }

    #[test]
    fn test_shift() {
        assert_eq!(shift(1, 192), 1);
        assert_eq!(shift(1, 96), MODULUS - 1);
        proptest!(|(a: u64, s: u64)| {
            prop_assume!(a < MODULUS);
            assert_eq!(shift(a, s), mul(a, pow(2, s)));
        });
    }

    #[test]
    fn test_root_192() {
        let root = root(192);
        proptest!(|(a: u64, i in 0_u64..192)| {
            assert_eq!(root_384(a, 2 * i), mul(a, pow(root, i)));
        });
    }

    #[test]
    fn test_root_384() {
        let root = root(384);
        proptest!(|(a: u64, i in 0_u64..384)| {
            assert_eq!(root_384(a, i), mul(a, pow(root, i)));
        });
    }

    #[test]
    fn test_inv() {
        proptest!(|(a: u64)| {
            prop_assume!(a < MODULUS);
            let b = inv(a);
            if a == 0 {
                assert_eq!(b, 0);
            } else {
                assert_eq!(mul(a, b), 1);
            }
        });
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use core::hint::black_box;
    use criterion::{BatchSize, Criterion};
    use rand::{thread_rng, Rng};

    pub fn group(criterion: &mut Criterion) {
        bench_nop(criterion);
        bench_reduce_128(criterion);
        bench_add(criterion);
        bench_mul(criterion);
        bench_inv(criterion);
        bench_omega(criterion);
    }

    // Helper to get an idea of the benchmark overhead
    fn bench_nop(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("field/nop", move |bencher| {
            bencher.iter_batched(
                || (rng.gen::<u64>() % MODULUS, rng.gen::<u64>() % MODULUS),
                |(a, b)| black_box(a.wrapping_add(b)),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_reduce_128(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("field/reduce_128", move |bencher| {
            bencher.iter_batched(
                || rng.gen(),
                |a: u128| black_box(reduce_128(a)),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_add(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("field/add", move |bencher| {
            bencher.iter_batched(
                || (rng.gen::<u64>() % MODULUS, rng.gen::<u64>() % MODULUS),
                |(a, b)| black_box(add(a, b)),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_mul(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("field/mul", move |bencher| {
            bencher.iter_batched(
                || (rng.gen::<u64>() % MODULUS, rng.gen::<u64>() % MODULUS),
                |(a, b)| black_box(mul(a, b)),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_inv(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("field/inv", move |bencher| {
            bencher.iter_batched(
                || rng.gen::<u64>() % MODULUS,
                |a| black_box(inv(a)),
                BatchSize::SmallInput,
            );
        });
    }

    fn bench_omega(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("field/root_384", move |bencher| {
            bencher.iter_batched(
                || rng.gen::<u64>() % MODULUS,
                |a| black_box(root_384(a, 1)),
                BatchSize::SmallInput,
            );
        });
    }
}
