// OPT: Use a super optimizer to find optimal sequences for add, mul, neg, sub,
// and shift.

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
