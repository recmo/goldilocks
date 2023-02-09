use super::{next_layer_size, BLOCK_SIZE, HASH_SIZE};
use core::mem;

const RC: [u64; 12] = [
    0x000000008000808b,
    0x800000000000008b,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800a,
    0x800000008000000a,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];
const PI: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
];
const RHO: [u32; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44,
];

pub fn keccak_p1600_12(state: &mut [u64; 25]) {
    let mut bc = [0; 5];
    for rc in RC {
        // Theta
        for x in 0..5 {
            bc[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
        }
        for x in 0..5 {
            let t = bc[(x + 4) % 5] ^ bc[(x + 1) % 5].rotate_left(1);
            state[x] ^= t;
            state[x + 5] ^= t;
            state[x + 10] ^= t;
            state[x + 15] ^= t;
            state[x + 20] ^= t;
        }

        // Rho and pi
        let mut last = state[1];
        for x in 0..24 {
            let t = state[PI[x]];
            state[PI[x]] = last.rotate_left(RHO[x]);
            last = t;
        }

        // Chi
        for slice in state.chunks_exact_mut(5) {
            bc.copy_from_slice(slice);
            for x in 0..5 {
                slice[x] = bc[x] ^ ((!bc[(x + 1) % 5]) & bc[(x + 2) % 5]);
            }
        }

        // Iota
        state[0] ^= rc;
    }
}

pub fn k12<const H: usize>(input: &[u8]) -> [u8; H] {
    // Make sure our choice of parameters is such that we only need a single
    // keccak_p1600_12 permutation.
    assert!(H <= 168);
    assert!(input.len() <= 166);

    let mut state = [0u8; 200];

    // Copy input into state.
    state[0..input.len()].copy_from_slice(input);

    // Zero length customization string.
    state[input.len()] = 0x00;

    // Padding.
    state[input.len() + 1] = 0x07;
    state[167] ^= 0x80;

    // Permute
    let lanes = unsafe {
        // SAFETY: The state is 200 bytes long, which is exactly 25 u64s.
        mem::transmute::<&mut [u8; 200], &mut [u64; 25]>(&mut state)
    };
    keccak_p1600_12(lanes);

    state[0..H].try_into().unwrap()
}

pub fn process_layer(input: &[u8], output: &mut [u8]) {
    assert!(output.len() >= next_layer_size(output.len()));
    for (input, output) in input
        .chunks(BLOCK_SIZE)
        .zip(output.chunks_exact_mut(HASH_SIZE))
    {
        output.copy_from_slice(&k12::<HASH_SIZE>(input));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;
    use std::fmt::Write;

    fn to_hex(bytes: &[u8]) -> String {
        let mut result = String::new();
        for (i, byte) in bytes.iter().enumerate() {
            if i > 0 {
                write!(result, " ").unwrap();
            }
            write!(result, "{byte:02x}").unwrap();
        }
        result
    }

    /// <https://github.com/XKCP/XKCP/blob/64404beeeb261b08a1076fe2f076e4e28dd9b040/tests/TestVectors/KangarooTwelve.txt#L1-L2>
    #[test]
    fn test_empty_32() {
        let mut input = [0u8; 0];
        let actual = k12::<32>(&mut input);
        assert_eq!(
            to_hex(&actual[..]),
            "1a c2 d4 50 fc 3b 42 05 d1 9d a7 bf ca 1b 37 51 3c 08 03 57 7a c7 16 7f 06 fe 2c e1 \
             f0 ef 39 e5"
        );
    }

    /// <https://github.com/XKCP/XKCP/blob/64404beeeb261b08a1076fe2f076e4e28dd9b040/tests/TestVectors/KangarooTwelve.txt#L4-L5>
    #[test]
    fn test_empty_64() {
        let mut input = [0u8; 0];
        let actual = k12::<64>(&mut input);
        assert_eq!(
            to_hex(&actual[..]),
            "1a c2 d4 50 fc 3b 42 05 d1 9d a7 bf ca 1b 37 51 3c 08 03 57 7a c7 16 7f 06 fe 2c e1 \
             f0 ef 39 e5 42 69 c0 56 b8 c8 2e 48 27 60 38 b6 d2 92 96 6c c0 7a 3d 46 45 27 2e 31 \
             ff 38 50 81 39 eb 0a 71"
        );
    }

    /// <https://github.com/XKCP/XKCP/blob/64404beeeb261b08a1076fe2f076e4e28dd9b040/tests/TestVectors/KangarooTwelve.txt#L10-L11>
    #[test]
    fn test_pattern_17_0() {
        let mut input = (0..0xfa).cycle().take(1).collect::<Vec<_>>();
        let actual = k12::<32>(&mut input);
        assert_eq!(
            to_hex(&actual[..]),
            "2b da 92 45 0e 8b 14 7f 8a 7c b6 29 e7 84 a0 58 ef ca 7c f7 d8 21 8e 02 d3 45 df aa \
             65 24 4a 1f"
        );
    }

    /// <https://github.com/XKCP/XKCP/blob/64404beeeb261b08a1076fe2f076e4e28dd9b040/tests/TestVectors/KangarooTwelve.txt#L13-L14>
    #[test]
    fn test_pattern_17_1() {
        let mut input = (0..0xfa).cycle().take(17).collect::<Vec<_>>();
        let actual = k12::<32>(&mut input);
        assert_eq!(
            to_hex(&actual[..]),
            "6b f7 5f a2 23 91 98 db 47 72 e3 64 78 f8 e1 9b 0f 37 12 05 f6 a9 a9 3a 27 3f 51 df \
             37 12 28 88"
        );
    }

    /// Compare against the XKCP bindings.
    #[test]
    fn test_xkcp() {
        use proptest::{collection::vec, num::u8};
        proptest!(|(input in vec(u8::ANY, 0..=166))| {
            let actual = k12::<32>(&input);
            let expected = xkcp_k12::hash(&input);
            assert_eq!(
                to_hex(&actual[..]),
                to_hex(expected.as_bytes()),
            );
        });
    }

    /// Compare against the RustCrypto implementation.
    #[test]
    fn test_rust_crypto() {
        use proptest::{collection::vec, num::u8};
        use rust_crypto_k12::digest::{ExtendableOutput, Update};
        proptest!(|(input in vec(u8::ANY, 0..=166))| {
            let actual = k12::<32>(&input);
            let mut k12 = rust_crypto_k12::KangarooTwelve::new();
            k12.update(&input);
            let mut expected = [0u8; 32];
            k12.finalize_xof_into(&mut expected);
            assert_eq!(
                to_hex(&actual[..]),
                to_hex(&expected[..]),
            );
        });
    }
}
