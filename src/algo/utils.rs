use core::mem::swap;

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

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
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
