/// ! Platform agnostic implementations.
/// !
/// ! Also serve as reference implementations for tests.

/// p = φ² - φ + 1 = 2⁶⁴ - 2³² + 1
const MODULUS: u64 = 0xffff_ffff_0000_0001;

/// The preferred generator for the field.
const GENERATOR: u64 = 7;

/// Order of the multiplicative group
const ORDER: u64 = MODULUS - 1;

/// φ = 2³²
const PHI: u64 = 0x1_0000_0000;

/// φ² mod p = 2⁶⁴ mod p = 2³² - 1
const PHI2: u64 = 0xffff_ffff;

/// Reduce a `u64`
pub(crate) const fn reduce_1(mut n: u64) -> u64 {
    if n > MODULUS {
        n -= MODULUS;
    }
    n
}

/// Reduce a 128 bit number
fn reduce_2(low: u64, high: u64) -> u64 {
    let (mid, high) = (high as u32, high >> 32);
    reduce_3(low, mid, high)
}

/// Reduce a 159 bit number
/// See <https://cp4space.hatsya.com/2021/09/01/an-efficient-prime-for-number-theoretic-transforms/>
fn reduce_3(low: u64, mid: u32, high: u64) -> u64 {
    debug_assert!(high <= u64::MAX >> 1);
    let (mut low2, carry) = low.overflowing_sub(high);
    if carry {
        low2 = low2.wrapping_add(MODULUS);
    }

    let mut product = ((mid as u64) << 32);
    product -= (product >> 32);

    let (mut result, carry) = product.overflowing_add(low2);
    if carry || result >= MODULUS {
        result = result.wrapping_sub(MODULUS);
    }
    debug_assert!(result < MODULUS);
    result
}

/// Adds two field elements.
///
/// Requires `a` and `b` to be reduced.
pub(crate) const fn add(a: u64, b: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    let (mut result, carry) = a.overflowing_add(b);
    if carry || result >= MODULUS {
        result = result.wrapping_sub(MODULUS);
    }
    debug_assert!(result < MODULUS);
    result
}

/// Multiplies two field elements.
pub(crate) fn mul(a: u64, b: u64) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(b < MODULUS);
    let r = (a as u128) * (b as u128);
    let (low, high) = (r as u64, (r >> 64) as u64);
    reduce_2(low, high)
}

// OPT: Dedicated `square` fn

/// Compute a^e
pub(crate) fn pow(mut a: u64, mut e: u64) -> u64 {
    debug_assert!(a < MODULUS);

    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mul(r, a);
        }
        a = mul(a, a);
        e = e >> 1;
    }
    r
}

/// Compute the preferred n-th root of unity.
pub(crate) fn omega(n: u64) -> u64 {
    assert_eq!(ORDER % n, 0);
    pow(GENERATOR, ORDER / n)
}

/// Compute a ⋅ 2^n
pub(crate) fn shift(mut a: u64, n: u64) -> u64 {
    debug_assert!(a < MODULUS);
    let (q, r) = (n / 96, n % 96);
    if q & 1 == 1 {
        a = MODULUS - a;
    }
    match r {
        0 => a,
        1..=32 => reduce_3(a << r, (a >> (64 - r)) as u32, 0),
        33..=63 => reduce_3(a << r, (a >> (64 - r)) as u32, a >> (96 - r)),
        64..=95 => reduce_3(0, (a << (r - 64)) as u32, a >> (96 - r)),
        96.. => unreachable!(),
    }
}

// ω_{384}^{2⋅i} = 2^{SHIFT_EVEN[i]}
const SHIFT_EVEN: [u8; 192] = [
    0, 77, 154, 39, 116, 1, 78, 155, 40, 117, 2, 79, 156, 41, 118, 3, 80, 157, 42, 119, 4, 81, 158,
    43, 120, 5, 82, 159, 44, 121, 6, 83, 160, 45, 122, 7, 84, 161, 46, 123, 8, 85, 162, 47, 124, 9,
    86, 163, 48, 125, 10, 87, 164, 49, 126, 11, 88, 165, 50, 127, 12, 89, 166, 51, 128, 13, 90,
    167, 52, 129, 14, 91, 168, 53, 130, 15, 92, 169, 54, 131, 16, 93, 170, 55, 132, 17, 94, 171,
    56, 133, 18, 95, 172, 57, 134, 19, 96, 173, 58, 135, 20, 97, 174, 59, 136, 21, 98, 175, 60,
    137, 22, 99, 176, 61, 138, 23, 100, 177, 62, 139, 24, 101, 178, 63, 140, 25, 102, 179, 64, 141,
    26, 103, 180, 65, 142, 27, 104, 181, 66, 143, 28, 105, 182, 67, 144, 29, 106, 183, 68, 145, 30,
    107, 184, 69, 146, 31, 108, 185, 70, 147, 32, 109, 186, 71, 148, 33, 110, 187, 72, 149, 34,
    111, 188, 73, 150, 35, 112, 189, 74, 151, 36, 113, 190, 75, 152, 37, 114, 191, 76, 153, 38,
    115,
];

// ω_{384}^{2⋅i + 1} = 2^{SHIFT_ODD[i].0} + 2^{SHIFT_ODD[i].1}
#[rustfmt::skip]
const SHIFT_ODD: [(u8, u8); 192] = [
    (110, 158), (43, 187), (72, 120), (5, 149), (34, 82), (111, 159), (44, 188), (73, 121),
    (6, 150), (35, 83), (112, 160), (45, 189), (74, 122), (7, 151), (36, 84), (113, 161),
    (46, 190), (75, 123), (8, 152), (37, 85), (114, 162), (47, 191), (76, 124), (9, 153), (38, 86),
    (115, 163), (0, 48), (77, 125), (10, 154), (39, 87), (116, 164), (1, 49), (78, 126), (11, 155),
    (40, 88), (117, 165), (2, 50), (79, 127), (12, 156), (41, 89), (118, 166), (3, 51), (80, 128),
    (13, 157), (42, 90), (119, 167), (4, 52), (81, 129), (14, 158), (43, 91), (120, 168), (5, 53),
    (82, 130), (15, 159), (44, 92), (121, 169), (6, 54), (83, 131), (16, 160), (45, 93),
    (122, 170), (7, 55), (84, 132), (17, 161), (46, 94), (123, 171), (8, 56), (85, 133), (18, 162),
    (47, 95), (124, 172), (9, 57), (86, 134), (19, 163), (48, 96), (125, 173), (10, 58), (87, 135),
    (20, 164), (49, 97), (126, 174), (11, 59), (88, 136), (21, 165), (50, 98), (127, 175),
    (12, 60), (89, 137), (22, 166), (51, 99), (128, 176), (13, 61), (90, 138), (23, 167),
    (52, 100), (129, 177), (14, 62), (91, 139), (24, 168), (53, 101), (130, 178), (15, 63),
    (92, 140), (25, 169), (54, 102), (131, 179), (16, 64), (93, 141), (26, 170), (55, 103),
    (132, 180), (17, 65), (94, 142), (27, 171), (56, 104), (133, 181), (18, 66), (95, 143),
    (28, 172), (57, 105), (134, 182), (19, 67), (96, 144), (29, 173), (58, 106), (135, 183),
    (20, 68), (97, 145), (30, 174), (59, 107), (136, 184), (21, 69), (98, 146), (31, 175),
    (60, 108), (137, 185), (22, 70), (99, 147), (32, 176), (61, 109), (138, 186), (23, 71),
    (100, 148), (33, 177), (62, 110), (139, 187), (24, 72), (101, 149), (34, 178), (63, 111),
    (140, 188), (25, 73), (102, 150), (35, 179), (64, 112), (141, 189), (26, 74), (103, 151),
    (36, 180), (65, 113), (142, 190), (27, 75), (104, 152), (37, 181), (66, 114), (143, 191),
    (28, 76), (105, 153), (38, 182), (67, 115), (0, 144), (29, 77), (106, 154), (39, 183),
    (68, 116), (1, 145), (30, 78), (107, 155), (40, 184), (69, 117), (2, 146), (31, 79),
    (108, 156), (41, 185), (70, 118), (3, 147), (32, 80), (109, 157), (42, 186), (71, 119),
    (4, 148), (33, 81)
];

/// Compute a ⋅ ω₃₈₄ⁱ
pub(crate) fn omega_384(a: u64, i: usize) -> u64 {
    debug_assert!(a < MODULUS);
    debug_assert!(i < 384);

    if i & 1 == 0 {
        shift(a, SHIFT_EVEN[i >> 1] as u64)
    } else {
        let (first, second) = SHIFT_ODD[i >> 1];
        add(shift(a, first as u64), shift(a, second as u64))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::{prop_assume, proptest};

    #[test]
    fn test_add_identity() {
        proptest!(|(a: u64)| {
            prop_assume!(a < MODULUS);
            let b = add(a, 0);
            assert_eq!(a, b);
        });
    }

    #[test]
    fn test_add_commutative() {
        proptest!(|(a: u64, b: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let ab = add(a, b);
            let ba = add(b, a);
            assert_eq!(ab, ba);
        });
    }

    #[test]
    fn test_add_associative() {
        proptest!(|(a: u64, b: u64, c: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let a_bc = add(a, add(b, c));
            let ab_c = add(add(a, b), c);
            assert_eq!(a_bc, ab_c);
        });
    }

    #[test]
    fn test_mul_identity() {
        proptest!(|(a: u64)| {
            prop_assume!(a < MODULUS);
            let b = mul(a, 1);
            assert_eq!(a, b);
        });
    }

    #[test]
    fn test_mul_commutative() {
        proptest!(|(a: u64, b: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let ab = mul(a, b);
            let ba = mul(b, a);
            assert_eq!(ab, ba);
        });
    }

    #[test]
    fn test_mul_associative() {
        proptest!(|(a: u64, b: u64, c: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let a_bc = mul(a, mul(b, c));
            let ab_c = mul(mul(a, b), c);
            assert_eq!(a_bc, ab_c);
        });
    }

    #[test]
    fn test_distributive() {
        proptest!(|(a: u64, b: u64, c: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let a_bc = mul(a, add(b, c));
            let ab_ac = add(mul(a, b), mul(a, c));
            assert_eq!(a_bc, ab_ac);
        });
    }

    #[test]
    fn test_pow() {
        proptest!(|(a: u64, b in 0_u64..100, c in 0_u64..100)| {
            prop_assume!(a < MODULUS);
            let ab = pow(a, b);
            let ac = pow(a, c);
            let a_bc = pow(a, b + c);
            let ab_ac = mul(ab, ac);
            assert_eq!(a_bc, ab_ac);
        });
    }

    #[test]
    fn test_omega() {
        assert_eq!(omega(384), 13835058050987180033);
    }

    #[test]
    fn test_shift() {
        for i in 0..384 {
            assert_eq!(shift(1, i), pow(2, i));
        }
    }

    #[test]
    fn test_omega_384() {
        let root = omega(384);
        for i in 0_u64..384 {
            assert_eq!(omega_384(1, i as usize), pow(root, i));
        }
    }
}
