#![cfg(all(target_arch = "aarch64", target_feature = "sha3"))]
use super::{generic, next_layer_size, BLOCK_SIZE, HASH_SIZE};

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

/// Double K12 on ARMv8-A with FEAT_SHA3.
///
/// See <https://github.com/XKCP/K12/blob/master/lib/ARMv8Asha3/KeccakP-1600-ARMv8Asha3.S>
fn k12_2(input: &[u8], output: &mut [u8]) {
    assert_eq!(input.len(), 320);
    assert_eq!(output.len(), 64);

    unsafe {
        core::arch::asm!("
            // Read first block into v0-v20 lower 64-bit.
            ld4.d {{ v0- v3}}[0], [{input}], #32
            ld4.d {{ v4- v7}}[0], [{input}], #32
            ld4.d {{ v8-v11}}[0], [{input}], #32
            ld4.d {{v12-v15}}[0], [{input}], #32
            ld4.d {{v16-v19}}[0], [{input}], #32

            // Read second block into v0-v20 upper 64-bit.
            ld4.d {{ v0- v3}}[1], [{input}], #32
            ld4.d {{ v4- v7}}[1], [{input}], #32
            ld4.d {{ v8-v11}}[1], [{input}], #32
            ld4.d {{v12-v15}}[1], [{input}], #32
            ld4.d {{v16-v19}}[1], [{input}], #32

            // Empty customization string and padding.
            movz {input}, #0x0700
            movk {input}, #0x8000, lsl #48
            dup.2d v20, {input}

            // Zero the capacity bits
            dup.2d v21, xzr
            dup.2d v22, xzr
            dup.2d v23, xzr
            dup.2d v24, xzr
        ",
        include_str!("aarch64.asm"),
        "
            // Write output (first 256 bits of state)
            st4.d {{ v0- v3}}[0], [{output}], #32
            st4.d {{ v0- v3}}[1], [{output}], #32

        ",
            input = inout(reg) input.as_ptr() => _,
            output = inout(reg) output.as_mut_ptr() => _,
            loop = inout(reg) 12 => _,
            rc = inout(reg) RC.as_ptr() => _,
            out("v0") _, out("v1") _, out("v2") _, out("v3") _, out("v4") _,
            out("v5") _, out("v6") _, out("v7") _, out("v8") _, out("v9") _,
            out("v10") _, out("v11") _, out("v12") _, out("v13") _, out("v14") _,
            out("v15") _, out("v16") _, out("v17") _, out("v18") _, out("v19") _,
            out("v20") _, out("v21") _, out("v22") _, out("v23") _, out("v24") _,
            out("v25") _, out("v26") _, out("v27") _, out("v28") _, out("v29") _,
            out("v30") _, out("v31") _,
            options(nostack)
        );
    }
    // Write full state
    // st4.d {{ v0- v3}}[0], [{state}], #32
    // st4.d {{ v4- v7}}[0], [{state}], #32
    // st4.d {{ v8-v11}}[0], [{state}], #32
    // st4.d {{v12-v15}}[0], [{state}], #32
    // st4.d {{v16-v19}}[0], [{state}], #32
    // st4.d {{v20-v23}}[0], [{state}], #32
    // st1.d {{v24}}[0], [{state}], #8
    // st4.d {{ v0- v3}}[1], [{state}], #32
    // st4.d {{ v4- v7}}[1], [{state}], #32
    // st4.d {{ v8-v11}}[1], [{state}], #32
    // st4.d {{v12-v15}}[1], [{state}], #32
    // st4.d {{v16-v19}}[1], [{state}], #32
    // st4.d {{v20-v23}}[1], [{state}], #32
    // st1.d {{v24}}[1], [{state}], #8
}

/// Compute two K12 proof-of-work hashes in parallel.
pub fn k12_pow_2(challenge: &[u8; 32], nonce_0: u64, nonce_1: u64) -> (u64, u64) {
    let out_0: u64;
    let out_1: u64;
    unsafe {
        core::arch::asm!("
            // Read challenge into v0-v3 replicated.
            ld4.2d {{ v0- v3}}, [{challenge}]
            dup.2d v0, v0[0]
            dup.2d v1, v1[0]
            dup.2d v2, v2[0]
            dup.2d v3, v3[0]

            // Read nonces into v4
            mov.d v4[0], {r0}
            mov.d v4[1], {r1}

            // Empty customization string and padding.
            // TODO: There must be a better way to do this, but LLVM asm rejects
            // everything I tried.
            mov {r0}, #0x0700
            dup.2d v5, {r0}

            // Zero padding
            // TODO: Unroll first round of loop
            dup.2d v6, xzr
            dup.2d v7, xzr
            dup.2d v8, xzr
            dup.2d v9, xzr
            dup.2d v10, xzr
            dup.2d v11, xzr
            dup.2d v12, xzr
            dup.2d v13, xzr
            dup.2d v14, xzr
            dup.2d v15, xzr
            dup.2d v16, xzr
            dup.2d v17, xzr
            dup.2d v18, xzr
            dup.2d v19, xzr

            // Set final padding bit
            mov {r0}, #0x8000000000000000
            dup.2d v20, {r0}
            
            // Zero the capacity bits
            dup.2d v21, xzr
            dup.2d v22, xzr
            dup.2d v23, xzr
            dup.2d v24, xzr
        ",
        include_str!("aarch64.asm"),
        "
            // Write outputs (first 64 bits of state)
            // TODO: Unroll last round of loop
            mov {r0}, v0.d[0]
            mov {r1}, v0.d[1]
        ",
            challenge = in(reg) challenge.as_ptr(),
            r0 = inout(reg) nonce_0 => out_0,
            r1 = inout(reg) nonce_1 => out_1,
            loop = inout(reg) 12 => _,
            rc = inout(reg) RC.as_ptr() => _,
            out("v0") _, out("v1") _, out("v2") _, out("v3") _, out("v4") _,
            out("v5") _, out("v6") _, out("v7") _, out("v8") _, out("v9") _,
            out("v10") _, out("v11") _, out("v12") _, out("v13") _, out("v14") _,
            out("v15") _, out("v16") _, out("v17") _, out("v18") _, out("v19") _,
            out("v20") _, out("v21") _, out("v22") _, out("v23") _, out("v24") _,
            out("v25") _, out("v26") _, out("v27") _, out("v28") _, out("v29") _,
            out("v30") _, out("v31") _,
            options(nostack)
        );
    }
    (out_0, out_1)
}

pub fn process_layer(input: &[u8], output: &mut [u8]) {
    assert!(output.len() >= next_layer_size(output.len()));

    // Full double-blocks
    let n = input.len() / (2 * BLOCK_SIZE);
    let (input, ir) = input.split_at(n * 2 * BLOCK_SIZE);
    let (output, or) = output.split_at_mut(n * 2 * HASH_SIZE);
    for (input, output) in input
        .chunks_exact(2 * BLOCK_SIZE)
        .zip(output.chunks_exact_mut(2 * HASH_SIZE))
    {
        k12_2(input, output);
    }

    // Remainder using generic algorithm
    generic::process_layer(ir, or);
}

#[cfg(test)]
mod tests {
    use super::{super::generic, *};
    use std::fmt::Write;

    fn to_hex(bytes: &[u8]) -> String {
        let mut result = String::new();
        for (i, byte) in bytes.iter().enumerate() {
            write!(result, "{byte:02x}").unwrap();
        }
        result
    }

    fn reference(input: &[u8], output: &mut [u8]) {
        assert_eq!(input.len(), 320);
        assert_eq!(output.len(), 64);
        output[..32].copy_from_slice(&generic::k12::<32>(&input[..160]));
        output[32..].copy_from_slice(&generic::k12::<32>(&input[160..]));
    }

    #[test]
    fn test_zeros() {
        let input = [0u8; 320];
        let mut output = [0u8; 64];
        let mut expected = [0u8; 64];
        k12_2(&input, &mut output);
        reference(&input, &mut expected);
        assert_eq!(to_hex(&output), to_hex(&expected));
    }

    #[test]
    fn test_nonzero() {
        let input = (0..320).map(|n| n as u8).collect::<Vec<_>>();
        let mut output = [0u8; 64];
        let mut expected = [0u8; 64];
        k12_2(&input, &mut output);
        reference(&input, &mut expected);
        assert_eq!(to_hex(&output), to_hex(&expected));
    }

    #[test]
    fn test_unaligned() {
        let input = (0..321).map(|n| n as u8).collect::<Vec<_>>();
        let mut output = [0u8; 64];
        let mut expected = [0u8; 64];
        k12_2(&input[1..], &mut output);
        reference(&input[1..], &mut expected);
        assert_eq!(to_hex(&output), to_hex(&expected));
    }

    #[test]
    fn test_pow_d() {
        let challenge = [0x55; 32];
        let (a, b) = k12_pow_2(&challenge, u64::MAX, 0);
        assert_eq!(a, 14608096357653754266);
        assert_eq!(b, 2237294155800119683);
    }

    #[test]
    fn test_layer() {
        let mut input = (0..1600).map(|n| n as u8).collect::<Vec<_>>();
        let mut output = vec![0; next_layer_size(input.len())];
        let mut expected = vec![0; next_layer_size(input.len())];
        process_layer(&input, &mut output);
        generic::process_layer(&input, &mut expected);
        assert_eq!(to_hex(&output[..]), to_hex(&expected[..]),);
    }
}
