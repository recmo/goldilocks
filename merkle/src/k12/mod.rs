//! Kangaroo Twelve
//!
//! <https://keccak.team/kangarootwelve.html>
//!
//! TODO: Alignment so a single element doesn't straddle a block boundary.

mod aarch64;
mod generic;

use core::mem;
use rayon::prelude::*;

pub use aarch64::process_layer;

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
