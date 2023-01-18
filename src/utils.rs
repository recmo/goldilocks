use core::mem::swap;
use std::ops::Range;

/// Force the compiler to generate a branch instruction instead of conditionals.
/// Trick from [plonky2].
///
/// [plonky2]: https://github.com/mir-protocol/plonky2/blob/d90a0559297366e1e2390cff9e3d1d5cf53875b7/util/src/lib.rs#L263-L287
#[inline(always)]
pub fn branch_hint() {
    unsafe {
        core::arch::asm!("", options(nomem, nostack, preserves_flags));
    }
}

/// Force the compiler to assume a predicate is true.
/// Trick from [plonky2].
///
/// [plonky2]: https://github.com/mir-protocol/plonky2/blob/3a6d693f3ffe5aa1636e0066a4ea4885a10b5cdf/util/src/lib.rs#L253-L261
#[inline(always)]
pub fn assume(p: bool) {
    debug_assert!(p);
    if !p {
        unsafe {
            core::hint::unreachable_unchecked();
        }
    }
}

pub fn modexp(mut a: usize, mut e: usize, m: usize) -> usize {
    assert!(m > 0);
    assert!(m < 1 << (usize::BITS / 2), "modulus is too large");
    a %= m;

    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = (r * a) % m;
        }
        a = (a * a) % m;
        e = e >> 1;
    }
    r
}

/// Modular inverse.
///
/// Note that `m` is not necessarily prime.
pub fn modinv(a: usize, m: usize) -> usize {
    let (g, x, _) = egcd(a, m);
    assert_eq!(g, 1);
    let x = if x < 0 {
        (x + m as isize) as usize
    } else {
        x as usize
    };
    debug_assert_eq!(a * x % m, 1);
    x
}

/// Greatest common divisor.
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            break;
        }
    }
    a << shift
}

pub fn egcd(a: usize, b: usize) -> (usize, isize, isize) {
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);
    while r1 != 0 {
        let q = r0 / r1;
        r0 -= q * r1;
        s0 -= (q as isize) * s1;
        t0 -= (q as isize) * t1;
        swap(&mut r0, &mut r1);
        swap(&mut s0, &mut s1);
        swap(&mut t0, &mut t1);
    }
    (r0 as usize, s0, t0)
}

/// Split a slice into two non-overlapping ranges.
pub fn split_at_mut<T>(slice: &mut [T], a: Range<usize>, b: Range<usize>) -> (&mut [T], &mut [T]) {
    if a.start > b.start {
        let (b, a) = split_at_mut(slice, b, a);
        (a, b)
    } else {
        let (_left, remainder) = slice.split_at_mut(a.start);
        let (ra, remainder) = remainder.split_at_mut(a.end - a.start);
        let (_middle, remainder) = remainder.split_at_mut(b.start - a.end);
        let (rb, _right) = remainder.split_at_mut(b.end - b.start);
        (ra, rb)
    }
}
