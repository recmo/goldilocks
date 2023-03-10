#![cfg(feature = "xkcp")]
use super::{next_layer_size, BLOCK_SIZE, HASH_SIZE};
use xkcp_k12::hash;

pub fn process_layer(input: &[u8], output: &mut [u8]) {
    assert!(output.len() >= next_layer_size(output.len()));

    for (input, output) in input
        .chunks(BLOCK_SIZE)
        .zip(output.chunks_exact_mut(HASH_SIZE))
    {
        output.copy_from_slice(hash(input).as_bytes());
    }
}
