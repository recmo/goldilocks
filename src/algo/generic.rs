/// ! Platform agnostic implementations.
/// !
/// ! Also serve as reference implementations for tests.

/// p = φ² - φ + 1 = 2⁶⁴ - 2³² + 1
const MODULUS: u64 = 0xffff_ffff_0000_0001;

/// The preferred generator for the field.
const GENERATOR: u64 = 2717;

/// Order of the multiplicative group
const ORDER: u64 = MODULUS - 1;

/// φ = 2³²
const PHI: u64 = 0x1_0000_0000;

/// φ² mod p = 2⁶⁴ mod p = 2³² - 1
const PHI2: u64 = 0xffff_ffff;

/// Reduce a `u64`
pub(crate) const fn reduce_1(mut n: u64) -> u64 {
    if n > MODULUS {
        n -= MODULUS;
    }
    n
}

/// Reduce a 128 bit number
fn reduce_2(low: u64, high: u64) -> u64 {
    let (mid, high) = (high as u32, high >> 32);
    reduce_3(low, mid, high)
}

/// Reduce a 159 bit number
/// See <https://cp4space.hatsya.com/2021/09/01/an-efficient-prime-for-number-theoretic-transforms/>
fn reduce_3(low: u64, mid: u32, high: u64) -> u64 {
    debug_assert!(high <= u64::MAX >> 1);
    let (mut low2, carry) = low.overflowing_sub(high);
    if carry {
        low2 = low2.wrapping_add(MODULUS);
    }

    let mut product = ((mid as u64) << 32);
    product -= (product >> 32);

    let (mut result, carry) = product.overflowing_add(low2);
    if carry || result >= MODULUS {
        result = result.wrapping_sub(MODULUS);
    }
    debug_assert!(result < MODULUS);
    result
}

/// Adds two field elements.
///
/// Requires `a` and `b` to be reduced.
pub(crate) const fn add(a: u64, b: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    let (mut result, carry) = a.overflowing_add(b);
    if carry || result >= MODULUS {
        result = result.wrapping_sub(MODULUS);
    }
    debug_assert!(result < MODULUS);
    result
}

/// Multiplies two field elements.
pub(crate) fn mul(a: u64, b: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    let r = (a as u128) * (b as u128);
    let (low, high) = (r as u64, (r >> 64) as u64);
    reduce_2(low, high)
}

// OPT: Dedicated `square` fn

/// Compute a^e
pub(crate) fn pow(mut a: u64, mut e: u64) -> u64 {
    debug_assert!(a < MODULUS);

    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mul(r, a);
        }
        a = mul(a, a);
        e = e >> 1;
    }
    r
}

/// Compute the preferred n-th root of unity.
pub(crate) fn omega(n: u64) -> u64 {
    assert_eq!(ORDER % n, 0);
    pow(GENERATOR, ORDER / n)
}

/// Compute a ⋅ 2^n
pub(crate) fn shift(mut a: u64, n: u64) -> u64 {
    debug_assert!(a < MODULUS);
    // OPT: Avoid div-rem
    let (q, r) = (n / 96, n % 96);
    if q & 1 == 1 {
        a = MODULUS - a;
    }
    match r {
        0 => a,
        1..=32 => reduce_3(a << r, (a >> (64 - r)) as u32, 0),
        33..=63 => reduce_3(a << r, (a >> (64 - r)) as u32, a >> (96 - r)),
        64..=95 => reduce_3(0, (a << (r - 64)) as u32, a >> (96 - r)),
        96.. => unreachable!(),
    }
}

/// Compute a ⋅ ω₃₈₄ⁱ
pub(crate) fn omega_384(a: u64, i: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(i < 384);

    if i & 1 == 0 {
        shift(a, i >> 1)
    } else {
        let i = i >> 1;
        add(shift(a, i + 24), shift(a, i + 168))
    }
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
        assert_eq!(omega(384), add(omega(8), pow(omega(8), 7)));
    }

    #[test]
    fn test_shift() {
        proptest!(|(a: u64, s: u64)| {
            prop_assume!(a < MODULUS);
            assert_eq!(shift(a, s), mul(a, pow(2, s)));
        });
    }

    #[test]
    fn test_omega_192() {
        let root = omega(192);
        proptest!(|(a: u64, i in 0_u64..192)| {
            assert_eq!(omega_384(a, 2 * i), mul(a, pow(root, i)));
        });
    }

    #[test]
    fn test_omega_384() {
        let root = omega(384);
        proptest!(|(a: u64, i in 0_u64..384)| {
            assert_eq!(omega_384(a, i), mul(a, pow(root, i)));
        });
    }
}
