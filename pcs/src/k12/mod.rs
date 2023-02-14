//! Kangaroo Twelve optimized for many short messages.
//!
//! <https://keccak.team/kangarootwelve.html>

mod aarch64;
mod generic;
mod xkcp;

use rayon::prelude::*;

#[cfg(all(target_arch = "aarch64", target_feature = "sha3"))]
pub use aarch64::{k12_pow_2, process_layer};

#[cfg(all(
    not(all(target_arch = "aarch64", target_feature = "sha3")),
    feature = "xkcp"
))]
pub use xkcp::process_layer;

#[cfg(all(
    not(all(target_arch = "aarch64", target_feature = "sha3")),
    not(feature = "xkcp")
))]
pub use generic::process_layer;

pub use generic::k12;

const BLOCK_SIZE: usize = 160; // Exactly 20 Field or 4 Hash.
const HASH_SIZE: usize = 32;

const MIN_WORK_SIZE: usize = 1 << 20;

pub const fn next_layer_size(size: usize) -> usize {
    div_round_up(size, BLOCK_SIZE) * HASH_SIZE
}

const fn div_round_up(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

pub fn merkle_tree(values: &[u8]) -> Vec<Vec<u8>> {
    let mut layers = Vec::new();
    let mut layer = values;
    // OPT: MaybeUninit.
    while layer.len() > HASH_SIZE {
        let mut next_layer = vec![0u8; next_layer_size(layer.len())];

        const WORK_SIZE: usize = div_round_up(MIN_WORK_SIZE, 2 * BLOCK_SIZE) * 2;
        layer
            .par_chunks(WORK_SIZE * BLOCK_SIZE)
            .zip(next_layer.par_chunks_mut(WORK_SIZE * HASH_SIZE))
            .for_each(|(input, output)| {
                process_layer(input, output);
            });

        layers.push(next_layer);
        layer = layers.last().unwrap();
    }
    layers
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    fn to_hex(bytes: &[u8]) -> String {
        let mut result = String::new();
        for (i, byte) in bytes.iter().enumerate() {
            write!(result, "{byte:02x}").unwrap();
        }
        result
    }

    #[test]
    fn test_merkle_tree() {
        let mut input = (0..0xfa).cycle().take(1 << 20).collect::<Vec<_>>();
        dbg!(input.len());
        let tree = merkle_tree(&input);
        for layer in tree {
            assert_eq!(layer.len() % HASH_SIZE, 0);
            dbg!(layer.len());
        }
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
        let mut group = criterion.benchmark_group("k12");

        bench_k12(&mut group, "generic", generic::process_layer);
        bench_k12(&mut group, "rust_crypto", rust_crypto);
        bench_k12(&mut group, "xkcp", xkcp::process_layer);

        #[cfg(target_arch = "aarch64")]
        bench_k12(&mut group, "aarch64", aarch64::process_layer);
    }

    fn bench_k12(group: &mut BenchmarkGroup<WallTime>, name: &str, f: impl Fn(&[u8], &mut [u8])) {
        bench_k12_n::<1>(group, name, &f);
        bench_k12_n::<1024>(group, name, &f);
    }

    fn bench_k12_n<const N: usize>(
        group: &mut BenchmarkGroup<WallTime>,
        name: &str,
        f: impl Fn(&[u8], &mut [u8]),
    ) {
        let input_size = N * BLOCK_SIZE;
        let output_size = next_layer_size(input_size);
        group.throughput(Throughput::Bytes(input_size as u64));
        group.bench_function(BenchmarkId::new(name, N), move |bencher| {
            bencher.iter_custom(|iters| {
                let mut input = vec![0_u8; input_size];
                let mut output = vec![0_u8; output_size];
                let mut rng = thread_rng();
                rng.fill(&mut input[..]);
                let start = Instant::now();
                for _ in 0..iters {
                    f(black_box(&input), black_box(&mut output));
                }
                let elapsed = start.elapsed();
                elapsed
            });
        });
    }

    fn rust_crypto(input: &[u8], output: &mut [u8]) {
        use rust_crypto_k12::digest::{ExtendableOutputReset, Update};
        assert!(output.len() >= next_layer_size(output.len()));

        let mut k12 = rust_crypto_k12::KangarooTwelve::new();

        for (input, output) in input
            .chunks(BLOCK_SIZE)
            .zip(output.chunks_exact_mut(HASH_SIZE))
        {
            k12.update(&input);
            k12.finalize_xof_reset_into(output);
        }
    }
}
