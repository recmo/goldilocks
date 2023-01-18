pub mod generic;

pub use self::generic::{add, inv, mul, pow, reduce_128, reduce_64, root, root_384, shift, sub};

/// p = φ² - φ + 1 = 2⁶⁴ - 2³² + 1
const MODULUS: u64 = 0xffff_ffff_0000_0001;

/// The preferred generator for the field.
const GENERATOR: u64 = 2717;

/// Order of the multiplicative group
const ORDER: u64 = MODULUS - 1;

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use core::hint::black_box;
    use criterion::{
        measurement::WallTime, BatchSize, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
    };
    use rand::{thread_rng, Rng};
    use std::time::Instant;

    pub fn group(criterion: &mut Criterion) {
        generic::bench::group(criterion);
    }

    pub fn rand() -> u64 {
        let mut rng = thread_rng();
        rng.gen::<u64>() % MODULUS
    }

    pub fn rand_n<const N: usize>() -> [u64; N] {
        let mut rng = thread_rng();
        let mut a = [0u64; N];
        a.iter_mut().for_each(|n| *n = rng.gen::<u64>() % MODULUS);
        a
    }

    pub fn bench_unary(criterion: &mut Criterion, name: &str, f: impl Fn(u64) -> u64) {
        let f = &f;
        let a = rand();
        let mut group = criterion.benchmark_group("field");
        group.throughput(Throughput::Elements(1));
        group.bench_function(BenchmarkId::new(name, 1), move |bencher| {
            bencher.iter(|| black_box(f(black_box(a))));
        });
        // Do multiple at once to saturate instruction level parallelism.
        let (a0, a1, a2, a3, a4) = (rand(), rand(), rand(), rand(), rand());
        group.throughput(Throughput::Elements(5));
        group.bench_function(BenchmarkId::new(name, 5), move |bencher| {
            bencher.iter(|| black_box((f(a0), f(a1), f(a2), f(a3), f(a4))));
        });
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
