#![cfg(target_arch = "aarch64")]
// OPT: Use a super optimizer to find optimal sequences for add, mul, neg, sub,
// and shift.
use std::arch::aarch64::*;

#[inline(always)]
#[must_use]
pub fn add(a: u64, b: u64) -> u64 {
    let r: u64;
    let c: u64;
    unsafe {
        core::arch::asm!("
        adds {r}, {a}, {b}
        csetm {c:w}, cs
        ",
            a = in(reg) a,
            b = in(reg) b,
            r = lateout(reg) r,
            c = lateout(reg) c,
            options(pure, nomem, nostack),
        )
    }
    r.wrapping_add(c)
}

#[inline(always)]
#[must_use]
pub fn sub(a: u64, b: u64) -> u64 {
    let r: u64;
    let c: u64;
    unsafe {
        core::arch::asm!("
        subs {r}, {a}, {b}
        csetm {c:w}, cc
        ",
            a = in(reg) a,
            b = in(reg) b,
            r = lateout(reg) r,
            c = lateout(reg) c,
            options(pure, nomem, nostack),
        )
    }
    r.wrapping_sub(c)
}

/// NEON implementation of [`mont_reduce_128`].
pub fn mont_reduce_128(x0: uint64x2_t, x1: uint64x2_t) -> uint64x2_t {
    unsafe {
        dbg!(x0, x1);
        // let (a, e) = x0.overflowing_add(x0 << 32);
        let x0s = vshlq_n_u64(x0, 32);
        let a = vaddq_u64(x0, x0s);
        let e = vcltq_u64(a, x0s); // All ones iff overflow

        // let b = a.wrapping_sub(a >> 32).wrapping_sub(e as u64);
        let a1 = vshrq_n_u64(a, 32);
        let b = vsubq_u64(a, a1);
        let e = vshrq_n_u64(e, 63);
        let b = vsubq_u64(b, e);

        // sub(x1, b)
        // let (r, c) = x1.overflowing_sub(b);
        let r = vsubq_u64(x1, b);
        let c = vcgtq_u64(r, x1); // All ones iff overflow

        // let adj = 0u32.wrapping_sub(c as u32);
        let adj = vshrq_n_u64(c, 32);

        // r.wrapping_sub(adj as u64)
        vsubq_u64(r, adj)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::{generic, MODULUS};
    use proptest::{prop_assume, proptest};

    #[test]
    fn test_mont_reduce_128() {
       proptest!(|(x0: [u64; 2], x1: [u64; 2])| {
            prop_assume!(x1[0] < MODULUS);
            prop_assume!(x1[1] < MODULUS);
            let expected = [
                generic::mont_reduce_128((x0[0] as u128) + ((x1[0] as u128) << 64)),
                generic::mont_reduce_128((x0[1] as u128) + ((x1[1] as u128) << 64)),
            ];
            let x0 = unsafe { std::mem::transmute(x0) };
            let x1 = unsafe { std::mem::transmute(x1) };
            let value = mont_reduce_128(x0, x1);
            let value: [u64;2] = unsafe { std::mem::transmute(value) };
            assert_eq!(value, expected);
        });
     }
}
