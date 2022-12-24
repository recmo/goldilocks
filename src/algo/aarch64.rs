// See <https://courses.cs.washington.edu/courses/cse469/19wi/arm64.pdf>
// See <https://dougallj.github.io/applecpu/firestorm.html>
// See <https://7-cpu.com/cpu/Apple_M1.html>

const MODULUS: u64 = 0xffff_ffff_0000_0001;

#[inline(always)]
pub fn mul(a: u64, b: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    let res: u64;
    unsafe {
        core::arch::asm!(r#"
            ; Compute 128-bit product of a and b
            mul     x8, {a}, {b}
            umulh   x9, {a}, {b}

            ; Subtract n3 from [n0, n1] mod p
            subs    x8, x8, x9, lsr #32  ; [n0, n1] -= n3
            mov     x11, #-4294967295    ; Correct for underflow by adding the modulus
            add     x10, x8, x11
            csel    x8, x8, x10, cs      ; Select adjusted result if carry is set

            lsl     x10, x9, #32
            sub     x9, x10, w9, uxtw    ; x9 = x10 - (x9 & 0xffffffff)

            mov     w10, #-1
            adds    x8, x9, x8
            add     x9, x8, x10
            cset    w10, hs
            
            mov     x11, #-4294967296
            cmp     x8, x11
            cset    w11, hi

            orr     w10, w10, w11
            cmp     w10, #0
            csel    {res}, x9, x8, ne
        "#,
        res = lateout(reg) res,
        a = in(reg) a,
        b = in(reg) b,
        // clobbers
        out("x8") _,
        out("x9") _,
        out("x10") _,
        out("x11") _,
        options(pure, nomem, nostack),
        );
    }
    res
}

#[cfg(test)]
mod test {
    use super::{super::generic, *};
    use proptest::{prop_assume, proptest};

    #[test]
    fn test_mul_ref() {
        // TODO: Test unlikely overflow cases.
        proptest!(|(a: u64, b: u64)| {
            prop_assume!(a < MODULUS && b < MODULUS);
            assert_eq!(mul(a, b), generic::mul(a, b));
        });
    }
}

#[cfg(feature = "criterion")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use core::hint::black_box;
    use criterion::{BatchSize, Criterion};
    use rand::{thread_rng, Rng};

    pub fn group(criterion: &mut Criterion) {
        bench_mul(criterion);
    }

    fn bench_mul(criterion: &mut Criterion) {
        let mut rng = thread_rng();
        criterion.bench_function("field/mul/aarch64", move |bencher| {
            bencher.iter_batched(
                || (rng.gen::<u64>() % MODULUS, rng.gen::<u64>() % MODULUS),
                |(a, b)| black_box(mul(a, b)),
                BatchSize::SmallInput,
            );
        });
    }
}
