pub mod aarch64;
pub mod generic;

pub use self::generic::{inv, mont_mul, mont_pow, mul, reduce_128, reduce_64, root_384, shift};
// pub use self::aarch64::{add, sub};
pub use self::generic::{add, sub};

/// p = φ² - φ + 1 = 2⁶⁴ - 2³² + 1
pub const MODULUS: u64 = 0xffff_ffff_0000_0001;

/// The preferred generator for the field.
pub const GENERATOR: u64 = 2717;
pub const GENERATOR_R: u64 = 0xa9c_ffff_f563;

/// Order of the multiplicative group
pub const ORDER: u64 = MODULUS - 1;

pub const MONT_R1: u64 = 0xffff_ffff;
pub const MONT_R2: u64 = 0xffff_fffe_0000_0001;
pub const MONT_R3: u64 = 1;

#[cfg(test)]
mod test {
    use super::*;
    use proptest::{prop_assume, proptest};

    #[test]
    fn test_add_naive() {
        proptest!(|(a: u64, b: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let value = add(a, b);
            let expected = (u128::from(a) + u128::from(b)).rem_euclid(u128::from(MODULUS)) as u64;
            assert_eq!(value, expected);
        });
    }

    #[test]
    fn test_sub_naive() {
        proptest!(|(a: u64, b: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let value = sub(a, b);
            let expected = (i128::from(a) - i128::from(b)).rem_euclid(i128::from(MODULUS)) as u64;
            assert_eq!(value, expected);
        });
    }

    #[test]
    fn test_mul_naive() {
        proptest!(|(a: u64, b: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let value = mul(a, b);
            let expected = (u128::from(a) * u128::from(b)).rem_euclid(u128::from(MODULUS)) as u64;
            assert_eq!(value, expected);
        });
    }

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
    fn test_sub() {
        proptest!(|(a: u64, b: u64)| {
            prop_assume!(a < MODULUS);
            prop_assume!(b < MODULUS);
            let ab = add(a, b);
            let ab_b = sub(ab, b);
            assert_eq!(a, ab_b);
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
    fn test_mont_mul_identity() {
        proptest!(|(a: u64)| {
            prop_assume!(a < MODULUS);
            let b = mont_mul(a, MONT_R1);
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
    fn test_mont_pow() {
        proptest!(|(a: u64, b in 0_u64..100, c in 0_u64..100)| {
            prop_assume!(a < MODULUS);
            let ab = mont_pow(a, b);
            let ac = mont_pow(a, c);
            let a_bc = mont_pow(a, b + c);
            let ab_ac = mont_mul(ab, ac);
            assert_eq!(a_bc, ab_ac);
        });
    }

    // #[test]
    // fn test_sqrt_two() {
    //     // Verify that we take the positive square root of two
    //     assert_eq!(root(384), add(root(8), pow(root(8), 7)));
    // }

    #[test]
    fn test_shift() {
        assert_eq!(shift(1, 192), 1);
        assert_eq!(shift(1, 96), MODULUS - 1);
        for s in 0..192 {
            proptest!(|(a: u64)| {
                prop_assume!(a < MODULUS);
                assert_eq!(shift(a, s), mul(a, mont_pow(2, s)));
            });
        }
        proptest!(|(a: u64, s: u64)| {
            prop_assume!(a < MODULUS);
            assert_eq!(shift(a, s), mont_mul(a, mont_pow(2, s)));
        });
    }

    // #[test]
    // fn test_root_192() {
    //     let root = root(192);
    //     proptest!(|(a: u64, i in 0_u64..192)| {
    //         assert_eq!(root_384(a, 2 * i), mul(a, pow(root, i)));
    //     });
    // }

    // #[test]
    // fn test_root_384() {
    //     let root = root(384);
    //     proptest!(|(a: u64, i in 0_u64..384)| {
    //         assert_eq!(root_384(a, i), mul(a, pow(root, i)));
    //     });
    // }

    #[test]
    fn test_inv() {
        proptest!(|(a: u64)| {
            prop_assume!(a < MODULUS);
            let b = inv(a);
            if a == 0 {
                assert_eq!(b, 0);
            } else {
                assert_eq!(mul(a, b), 1);
            }
        });
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use core::hint::black_box;
    use criterion::{measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion, Throughput};
    use rand::{thread_rng, Rng};
    use std::time::Instant;

    pub fn group(criterion: &mut Criterion) {
        bench_binary(criterion, "add", add);
        bench_binary(criterion, "sub", sub);
        bench_binary(criterion, "mul", mul);
        bench_binary(criterion, "mont_mul", mont_mul);
        bench_binary(criterion, "redc", |x0, x1| {
            generic::mont_reduce_128(((x1 as u128) << 64) | x0 as u128)
        });
        bench_unary(criterion, "inv", inv);
        bench_unary(criterion, "shift/48", |a| shift(a, 48));
        bench_unary(criterion, "shift/32", |a| shift(a, 32));
        bench_unary(criterion, "shift/64", |a| shift(a, 64));

        aarch64::bench::group(criterion);
    }

    #[must_use]
    pub fn rand() -> u64 {
        let mut rng = thread_rng();
        rng.gen::<u64>() % MODULUS
    }

    #[must_use]
    pub fn rand_n<const N: usize>() -> [u64; N] {
        let mut rng = thread_rng();
        let mut a = [0u64; N];
        a.iter_mut().for_each(|n| *n = rng.gen::<u64>() % MODULUS);
        a
    }

    /// Benchmark a unary (ine-input) function.
    ///
    /// Does small batches of different sizes to test instruction level
    /// parallelism.
    pub fn bench_unary(criterion: &mut Criterion, name: &str, f: impl Fn(u64) -> u64) {
        let f = &f;
        let mut group = criterion.benchmark_group("field");
        bench_unary_n::<1>(&mut group, name, f);
        bench_unary_n::<2>(&mut group, name, f);
        bench_unary_n::<4>(&mut group, name, f);
        bench_unary_n::<8>(&mut group, name, f);
        bench_unary_n::<16>(&mut group, name, f);
        bench_unary_n::<32>(&mut group, name, f);
    }

    /// Benchmark a binary (two-input) function.
    ///
    /// Does small batches of different sizes to test instruction level
    /// parallelism.
    pub fn bench_binary(criterion: &mut Criterion, name: &str, f: impl Fn(u64, u64) -> u64) {
        let f = &f;
        let mut group = criterion.benchmark_group("field");
        bench_binary_n::<1>(&mut group, name, f);
        bench_binary_n::<2>(&mut group, name, f);
        bench_binary_n::<4>(&mut group, name, f);
        bench_binary_n::<8>(&mut group, name, f);
        bench_binary_n::<16>(&mut group, name, f);
        bench_binary_n::<32>(&mut group, name, f);
    }

    fn bench_unary_n<const N: usize>(
        group: &mut BenchmarkGroup<WallTime>,
        name: &str,
        f: impl Fn(u64) -> u64,
    ) {
        group.throughput(Throughput::Elements(N as u64));
        group.bench_function(BenchmarkId::new(name, N), move |bencher| {
            bencher.iter_custom(|iters| {
                // Accumulate the result to force N-parallel execution.
                let mut a = rand_n::<N>();
                let start = Instant::now();
                for _ in 0..iters {
                    for i in 0..N {
                        a[i] = f(a[i]);
                    }
                }
                let elapsed = start.elapsed();
                black_box(a);
                elapsed
            });
        });
    }

    fn bench_binary_n<const N: usize>(
        group: &mut BenchmarkGroup<WallTime>,
        name: &str,
        f: impl Fn(u64, u64) -> u64,
    ) {
        group.throughput(Throughput::Elements(N as u64));
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
