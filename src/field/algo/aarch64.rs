// OPT: Use a super optimizer to find optimal sequences for add, mul, neg, sub,
// and shift.

pub fn add(a: u64, b: u64) -> u64 {
    let r: u64;
    let c: u64;
    unsafe { core::arch::asm!("
        adds {r}, {a}, {b}
        csetm {c:w}, cs
        ",
        a = in(reg) a,
        b = in(reg) b,
        r = lateout(reg) r,
        c = lateout(reg) c,
        options(pure, nomem, nostack),
    )}
    r.wrapping_add(c)
}

pub fn sub(a: u64, b: u64) -> u64 {
    let r: u64;
    let c: u64;
    unsafe { core::arch::asm!("
        subs {r}, {a}, {b}
        csetm {c:w}, cc
        ",
        a = in(reg) a,
        b = in(reg) b,
        r = lateout(reg) r,
        c = lateout(reg) c,
        options(pure, nomem, nostack),
    )}
    r.wrapping_sub(c)
}



#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;

    pub fn group(criterion: &mut Criterion) {
        bench_binary(criterion, "aarch64/add", add);
        bench_binary(criterion, "aarch64/sub", sub);
    }
}
