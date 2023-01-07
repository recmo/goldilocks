use core::mem::swap;
use std::ops::Range;

// Force the compiler to generate a branch instruction instead of conditionals.
// Trick from [plonky2].
//
// [plonky2]: https://github.com/mir-protocol/plonky2/blob/d90a0559297366e1e2390cff9e3d1d5cf53875b7/util/src/lib.rs#L263-L287
#[inline(always)]
pub fn branch_hint() {
    unsafe {
        core::arch::asm!("", options(nomem, nostack, preserves_flags));
    }
}

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

/// Split a slice into two non-overlapping ranges.
fn split_at_mut<T>(slice: &mut [T], a: Range<usize>, b: Range<usize>) -> (&mut [T], &mut [T]) {
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
