//! Proof of work by partial inversion of the K12 hash.
//!
//! The prover is deterministic in that it will always produce the smallest
//! satisfying nonce for a given seed.
use crate::k12::k12;
use std::sync::atomic::{AtomicU64, Ordering};
use rayon::prelude::*;

pub struct Pow {
    challenge: [u8; 32],
    threshold: u64,
}

impl Pow {
    pub fn threshold_from_bits(bits: f64) -> u64 {
        u64::MAX - 2.0_f64.powf(64.0 - bits) as u64
    }

    pub fn threshold_from_probability(probability: f64) -> u64 {
        assert!(probability > 0.0);
        assert!(probability <= 1.0);
        u64::MAX - (u64::MAX as f64 * probability) as u64
    }

    pub fn new(challenge: [u8; 32], threshold: u64) -> Self {
        Self {
            challenge,
            threshold,
        }
    }

    pub fn verify(&self, nonce: u64) -> bool {
        let mut input = [0_u8; 40];
        input[0..32].copy_from_slice(&self.challenge);
        input[32..40].copy_from_slice(&nonce.to_le_bytes());
        u64::from_le_bytes(k12(&input)) >= self.threshold
    }

    pub fn solve(&self) -> u64 {
        const BATCH: u64 = 2;
        let mut lowest = AtomicU64::new(u64::MAX);
        let num_threads = rayon::current_num_threads() as u64;
        (0..num_threads).into_par_iter().for_each(|thread_id| {
            for nonce in (thread_id * BATCH..).step_by((BATCH * num_threads) as usize) {
                let (a, b) = crate::k12::k12_pow_2(&self.challenge, nonce, nonce + 1);
                if a >= self.threshold {
                    lowest.fetch_min(nonce, Ordering::AcqRel);
                    break;
                }
                if b >= self.threshold {
                    lowest.fetch_min(nonce + 1, Ordering::AcqRel);
                    break;
                }
                if nonce >= lowest.load(Ordering::Acquire) {
                    break;
                }
            }
        });
        lowest.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_0() {
        let mut challenge = [0x55_u8; 32];
        let pow = Pow::new(challenge, 0);
        assert!(pow.verify(0));
        assert!(pow.verify(1));
        assert!(pow.verify(2));
        assert!(pow.verify(3));
    }

    #[test]
    fn test_pow_1() {
        let challenge = [0x55_u8; 32];
        let pow = Pow::new(challenge, 1 << 63);
        assert!(!pow.verify(0));
        assert!(!pow.verify(1));
        assert!(!pow.verify(2));
        assert!(pow.verify(3));
    }

    #[test]
    fn test_pow_solve() {
        let challenge = [0x55_u8; 32];
        let pow = Pow::new(challenge, 1 << 63);
        assert_eq!(pow.solve(), 3);
    }

    #[test]
    fn test_pow_solve_16() {
        let challenge = [0x55_u8; 32];
        let pow = Pow::new(challenge, Pow::threshold_from_bits(16.0));
        assert_eq!(pow.solve(), 18455);
    }

    #[test]
    fn test_pow_solve_32() {
        let challenge = [0xaa_u8; 32];
        let pow = Pow::new(challenge, Pow::threshold_from_bits(32.0));
        assert_eq!(pow.solve(), 440258853);
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
        let mut group = criterion.benchmark_group("pow");
        group.throughput(Throughput::Elements(18455));
        group.sample_size(10);
        group.bench_function(BenchmarkId::new("name", 16), move |bencher| {
            bencher.iter_custom(|iters| {
                let pow = Pow::new([0x55_u8; 32], Pow::threshold_from_bits(32.0));
                let pow = black_box(pow);
                let start = Instant::now();
                for _ in 0..iters {
                    black_box(pow.solve());
                }
                let elapsed = start.elapsed();
                elapsed
            });
        });
    }
}
