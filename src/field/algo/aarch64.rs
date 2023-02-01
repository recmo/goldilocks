#![cfg(target_arch = "aarch64")]
// OPT: Use a super optimizer to find optimal sequences for add, mul, neg, sub,
// and shift.
use std::arch::{aarch64::*, asm};

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

/// Aarch64 vector implementation of [`mont_reduce_128`].
#[inline(always)]
#[must_use]
pub fn mont_reduce_128(x0: uint64x2_t, x1: uint64x2_t) -> uint64x2_t {
    unsafe {
        // let (a, e) = x0.overflowing_add(x0 << 32);
        let x0s = vshlq_n_u64(x0, 32);
        // Using an `asm!` here because the compiler is not able to generate the right opcodes.
        // let a = vaddq_u64(x0, x0s);
        let a = inst::add(x0, x0s);
        let e = vcltq_u64(a, x0s); // -1 iff overflow

        // let b = a.wrapping_sub(a >> 32).wrapping_sub(e as u64);
        let a1 = vshrq_n_u64(a, 32);
        let b = vsubq_u64(a, a1);
        let b = vaddq_u64(b, e);

        // let (r, c) = x1.overflowing_sub(b);
        let r = vsubq_u64(x1, b);
        let c = vcgtq_u64(r, x1); // All ones iff overflow

        // let adj = 0u32.wrapping_sub(c as u32);
        // Using an `asm!` here because the compiler is not able to generate the right opcodes.
        // let adj = vshrq_n_u64(c, 32);
        let adj = inst::shr32(c);

        // r.wrapping_sub(adj as u64)
        vsubq_u64(r, adj)
    }
}

/// Hand written assembly version of [`mont_reduce_128`].
/// 
/// It is slower than the mixed-intrinsic one because it is more opaque to the compiler.
#[inline(always)]
#[must_use]
pub fn mont_reduce_128_asm(x0: uint64x2_t, x1: uint64x2_t) -> uint64x2_t {
    // let (a, e) = x0.overflowing_add(x0 << 32);
    let t = inst::shl32(x0);
    let a = inst::add(x0, t);
    let e = inst::gt(t, a);

    // let b = a.wrapping_sub(a >> 32).wrapping_sub(e as u64);
    let a1 = inst::shr32(a);
    let b = inst::sub(a, a1);
    let b = inst::add(b, e);

    // let (r, c) = x1.overflowing_sub(b);
    let r = inst::sub(x1, b);
    let c = inst::gt(r, x1);

    // let adj = 0u32.wrapping_sub(c as u32);
    let adj = inst::shr32(c);

    // r.wrapping_sub(adj as u64)
    inst::sub(r, adj)
}

/// Hand written intrinsics. For some reason, the compiler is not able to
/// generate the right opcodes for the built in intrinsics.
/// 
/// Small assembly blocks are preferred because they allow better register allocation.
mod inst {
    use core::arch::{aarch64::uint64x2_t, asm};

    #[inline(always)]
    pub fn shl32(a: uint64x2_t) -> uint64x2_t {
        let r: uint64x2_t;
        unsafe {
            asm!("shl {r:v}.2d, {a:v}.2d, #32",
                a = in(vreg) a,
                r = lateout(vreg) r,
                options(pure, nomem, nostack),
            );
        }
        r
    }

    #[inline(always)]
    pub fn shr32(a: uint64x2_t) -> uint64x2_t {
        let r: uint64x2_t;
        unsafe {
            asm!("ushr {r:v}.2d, {a:v}.2d, #32",
                a = in(vreg) a,
                r = lateout(vreg) r,
                options(pure, nomem, nostack),
            );
        }
        r
    }

    #[inline(always)]
    pub fn add(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        let r: uint64x2_t;
        unsafe {
            asm!("add {r:v}.2d, {a:v}.2d, {b:v}.2d",
                a = in(vreg) a,
                b = in(vreg) b,
                r = lateout(vreg) r,
                options(pure, nomem, nostack),
            );
        }
        r
    }

    #[inline(always)]
    pub fn sub(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        let r: uint64x2_t;
        unsafe {
            asm!("sub {r:v}.2d, {a:v}.2d, {b:v}.2d",
                a = in(vreg) a,
                b = in(vreg) b,
                r = lateout(vreg) r,
                options(pure, nomem, nostack),
            );
        }
        r
    }

    #[inline(always)]
    pub fn gt(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
        let r: uint64x2_t;
        unsafe {
            asm!("cmhi {r:v}.2d, {a:v}.2d, {b:v}.2d",
                a = in(vreg) a,
                b = in(vreg) b,
                r = lateout(vreg) r,
                options(pure, nomem, nostack),
            );
        }
        r
    }
}

#[cfg(test)]
mod test {
    use super::{
        super::{generic, MODULUS},
        *,
    };
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

    #[test]
    fn test_mont_reduce_128_asm() {
        proptest!(|(x0: [u64; 2], x1: [u64; 2])| {
            prop_assume!(x1[0] < MODULUS);
            prop_assume!(x1[1] < MODULUS);
            let expected = [
                generic::mont_reduce_128((x0[0] as u128) + ((x1[0] as u128) << 64)),
                generic::mont_reduce_128((x0[1] as u128) + ((x1[1] as u128) << 64)),
            ];
            let x0 = unsafe { std::mem::transmute(x0) };
            let x1 = unsafe { std::mem::transmute(x1) };
            let value = mont_reduce_128_asm(x0, x1);
            let value: [u64;2] = unsafe { std::mem::transmute(value) };
            assert_eq!(value, expected);
        });
    }
}

fn pair(a: u64, b: u64) -> uint64x2_t {
    unsafe { std::mem::transmute([a, b]) }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::{super::MODULUS, *};
    use core::hint::black_box;
    use criterion::{measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion, Throughput};
    use rand::{thread_rng, Rng};
    use std::time::Instant;

    pub fn group(criterion: &mut Criterion) {
        bench_binary(criterion, "redc", mont_reduce_128);
        bench_binary(criterion, "redc/asm", mont_reduce_128_asm);
    }

    #[must_use]
    pub fn rand_n<const N: usize>() -> [uint64x2_t; N] {
        let mut rng = thread_rng();
        let mut a = [pair(0, 0); N];
        a.iter_mut().for_each(|n| {
            *n = pair(rng.gen::<u64>() % MODULUS, rng.gen::<u64>() % MODULUS);
        });
        unsafe { std::mem::transmute(a) }
    }

    /// Benchmark a binary (two-input) function.
    ///
    /// Does small batches of different sizes to test instruction level
    /// parallelism.
    pub fn bench_binary(
        criterion: &mut Criterion,
        name: &str,
        f: impl Fn(uint64x2_t, uint64x2_t) -> uint64x2_t,
    ) {
        let f = &f;
        let mut group = criterion.benchmark_group("field/aarch64");
        bench_binary_n::<1>(&mut group, name, f);
        bench_binary_n::<2>(&mut group, name, f);
        bench_binary_n::<4>(&mut group, name, f);
        bench_binary_n::<8>(&mut group, name, f);
        bench_binary_n::<16>(&mut group, name, f);
        bench_binary_n::<32>(&mut group, name, f);
    }

    fn bench_binary_n<const N: usize>(
        group: &mut BenchmarkGroup<WallTime>,
        name: &str,
        f: impl Fn(uint64x2_t, uint64x2_t) -> uint64x2_t,
    ) {
        group.throughput(Throughput::Elements(2 * N as u64));
        group.bench_function(BenchmarkId::new(name, N), move |bencher| {
            bencher.iter_custom(|iters| {
                // Accumulate the result to force N-parallel execution.
                let mut a = rand_n::<N>();
                let b = rand_n::<N>();
                let start = Instant::now();
                for _ in 0..iters {
                    for i in 0..N {
                        a[i] = f(a[i], b[i]);
                    }
                }
                let elapsed = start.elapsed();
                black_box(a);
                elapsed
            });
        });
    }
}
