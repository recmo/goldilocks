// See <https://dougallj.github.io/applecpu/firestorm.html>

pub(crate) fn inv4(mut a: u64) -> u64 {
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
    core::arch::asm!("
        0:
            sub     x12, x12, x9
            add     x8, x8, x11
            rbit    x13, x12
            clz     x13, x13
            lsr     x14, x12, x13
            lsl     x12, x11, x13
            add     w10, w10, w13
            add     w13, w10, #96
            cmp     x14, x9
            csel    x11, x8, x12, lo
            csel    x8, x12, x8, lo
            csel    x12, x9, x14, lo
            csel    x14, x14, x9, lo
            csel    w10, w13, w10, lo
            mov     x9, x14
            cmp     x12, x14
            b.ne    0
        ",
        inout("x12") u,
        inout("x9") v,
        inout("x12") t0,
        inout("x12") t1,
        inout("w10") twos,
    );

    // Dividing by 2^shift is equivalent to multiplying by 2^(191 * shift)
    // because 2 is a 192th primitive root.
    twos *= 191;
    t0 = shift(t0, twos as u64);
    debug_assert!(t0 < MODULUS);
    debug_assert_eq!(mul(t0, a), 1);
    t0
}
