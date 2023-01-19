//! Platform agnostic implementations.
//!
//! Also serve as reference implementations for tests.

//! TODO: Square root: https://zcash.github.io/halo2/design/implementation/fields.html#sarkar-square-root-algorithm-table-based-variant

// We do a lot of intentional casting with truncation in this file.
#![allow(clippy::cast_possible_truncation, clippy::cast_lossless)]

use super::MODULUS;
use crate::{field::algo::MONT_R1, utils::branch_hint};
use core::mem::swap;

/// Adds two field elements.
///
/// Requires `a` and `b` to be reduced.
#[inline(always)]
pub fn add(a: u64, b: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    let b = MODULUS.wrapping_sub(b);
    let (r, c) = a.overflowing_sub(b);
    let adj = 0u32.wrapping_sub(c as u32);
    r.wrapping_sub(adj as u64)
}

/// Subtracts two field elements.
///
/// Requires `a` and `b` to be reduced.
#[inline(always)]
pub fn sub(a: u64, b: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    let (r, c) = a.overflowing_sub(b);
    let adj = 0u32.wrapping_sub(c as u32);
    r.wrapping_sub(adj as u64)
}

/// Multiplies two field elements.
#[inline(always)]
pub fn mul(a: u64, b: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    reduce_128((a as u128) * (b as u128))
}

#[inline(always)]
pub fn mont_mul(a: u64, b: u64) -> u64 {
    mont_reduce_128((a as u128) * (b as u128))
}

/// Reduce a `u64`
#[inline(always)]
pub const fn reduce_64(mut n: u64) -> u64 {
    if n > MODULUS {
        n -= MODULUS;
    }
    n
}

/// Reduce a 128 bit number
#[inline(always)]
pub fn reduce_128(n: u128) -> u64 {
    reduce_159(n as u64, (n >> 64) as u32, (n >> 96) as u64)
}

/// Reduce a 159 bit number
/// See <https://cp4space.hatsya.com/2021/09/01/an-efficient-prime-for-number-theoretic-transforms/>
/// See <https://github.com/mir-protocol/plonky2/blob/3a6d693f3ffe5aa1636e0066a4ea4885a10b5cdf/field/src/goldilocks_field.rs#L340-L356>
#[inline(always)]
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

/// See <https://github.com/facebook/winterfell/blob/d0f4373da34569dc39fe8b978e785ebd83bdbeb5/math/src/field/f64/mod.rs#L576-L588>
#[inline(always)]
pub fn mont_reduce_128(x: u128) -> u64 {
    debug_assert!(x < (MODULUS as u128) << 64);
    let (x0, x1) = (x as u64, (x >> 64) as u64);
    let (a, e) = x0.overflowing_add(x0 << 32);
    let b = a.wrapping_sub(a >> 32).wrapping_sub(e as u64);

    // sub(x1, b)
    let (r, c) = x1.overflowing_sub(b);
    let adj = 0u32.wrapping_sub(c as u32);
    r.wrapping_sub(adj as u64)
}

/// Compute a^e
pub fn mont_pow(mut a: u64, mut e: u64) -> u64 {
    debug_assert!(a < MODULUS);

    let mut r = MONT_R1;
    while e > 0 {
        if e & 1 == 1 {
            r = mont_mul(r, a);
        }
        a = mont_mul(a, a);
        e >>= 1;
    }
    r
}

#[inline(always)]
pub fn shift_48(a: u64) -> u64 {
    let mut b = (a >> 16) << 32;
    b -= b >> 32;
    let c = super::sub(a << 48, a >> 48);
    super::add(b, c)
}

#[inline(always)]
pub fn shift_32(a: u64) -> u64 {
    let mut b = (a >> 32) << 32;
    b -= b >> 32;
    super::add(b, a << 32)
}

#[inline(always)]
pub fn shift_64(a: u64) -> u64 {
    let mut b = a << 32;
    b -= b >> 32;
    // TODO: Are the arguments correctly sized?
    super::sub(b, a >> 32)
}

/// Compute a ⋅ 2^n
#[inline(always)]
pub fn shift(mut a: u64, n: u64) -> u64 {
    debug_assert!(a < MODULUS);

    match n {
        32 => return shift_32(a),
        // 48 => return shift_48(a),
        64 => return shift_64(a),
        _ => {}
    }

    // OPT: Avoid div-rem
    let (q, r) = (n / 96, n % 96);
    if q & 1 == 1 {
        a = sub(0, a);
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
#[inline(always)]
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

#[inline(always)]
pub fn inv(a: u64) -> u64 {
    inv_ebgcd(a)
}

/// Modular inverse using extended binary gcd.
///
/// Returns 0 for 0.
#[allow(dead_code)]
fn inv_ebgcd(a: u64) -> u64 {
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

/// Modular inverse using an addition chain.
///
/// Returns 0 for 0.
#[allow(dead_code)]
fn inv_addchain(a: u64) -> u64 {
    debug_assert!(a < MODULUS);

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
