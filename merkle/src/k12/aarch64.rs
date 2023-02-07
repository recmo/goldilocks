#![cfg(all(target_arch = "aarch64", target_feature = "sha3"))]
use super::{generic, next_layer_size, BLOCK_SIZE, HASH_SIZE};

const RC: [u64; 13] = [
    0x8000000000000700, // Padding value
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
            ld1r.2d    {{v20}}, [{rc}], #8

            // Zero the capacity bits
            dup.2d v21, xzr
            dup.2d v22, xzr
            dup.2d v23, xzr
            dup.2d v24, xzr
            
        0:  sub	{loop:x}, {loop:x}, #1

            // Theta Calculations
            eor3.16b   v25, v20, v15, v10
            eor3.16b   v26, v21, v16, v11
            eor3.16b   v27, v22, v17, v12
            eor3.16b   v28, v23, v18, v13
            eor3.16b   v29, v24, v19, v14
            eor3.16b   v25, v25,  v5,  v0
            eor3.16b   v26, v26,  v6,  v1
            eor3.16b   v27, v27,  v7,  v2
            eor3.16b   v28, v28,  v8,  v3
            eor3.16b   v29, v29,  v9,  v4
            rax1.2d    v30, v25, v27
            rax1.2d    v31, v26, v28
            rax1.2d    v27, v27, v29
            rax1.2d    v28, v28, v25
            rax1.2d    v29, v29, v26
            
            // Rho and Phi
            eor.16b     v0,  v0, v29
            xar.2d     v25,  v1, v30, #64 -  1
            xar.2d      v1,  v6, v30, #64 - 44
            xar.2d      v6,  v9, v28, #64 - 20
            xar.2d      v9, v22, v31, #64 - 61
            xar.2d     v22, v14, v28, #64 - 39
            xar.2d     v14, v20, v29, #64 - 18
            xar.2d     v26,  v2, v31, #64 - 62
            xar.2d      v2, v12, v31, #64 - 43
            xar.2d     v12, v13, v27, #64 - 25
            xar.2d     v13, v19, v28, #64 -  8
            xar.2d     v19, v23, v27, #64 - 56
            xar.2d     v23, v15, v29, #64 - 41
            xar.2d     v15,  v4, v28, #64 - 27
            xar.2d     v28, v24, v28, #64 - 14
            xar.2d     v24, v21, v30, #64 -  2
            xar.2d      v8,  v8, v27, #64 - 55
            xar.2d      v4, v16, v30, #64 - 45
            xar.2d     v16,  v5, v29, #64 - 36
            xar.2d      v5,  v3, v27, #64 - 28
            xar.2d     v27, v18, v27, #64 - 21
            xar.2d      v3, v17, v31, #64 - 15
            xar.2d     v30, v11, v30, #64 - 10
            xar.2d     v31,  v7, v31, #64 -  6
            xar.2d     v29, v10, v29, #64 -  3

            // Chi and Iota
            bcax.16b   v20, v26, v22,  v8
            bcax.16b   v21,  v8, v23, v22
            bcax.16b   v22, v22, v24, v23
            bcax.16b   v23, v23, v26, v24
            bcax.16b   v24, v24,  v8, v26
            
            ld1r.2d    {{v26}}, [{rc}], #8
            bcax.16b   v17, v30, v19,  v3
            bcax.16b   v18,  v3, v15, v19
            bcax.16b   v19, v19, v16, v15
            bcax.16b   v15, v15, v30, v16
            bcax.16b   v16, v16,  v3, v30
            
            bcax.16b   v10, v25, v12, v31
            bcax.16b   v11, v31, v13, v12
            bcax.16b   v12, v12, v14, v13
            bcax.16b   v13, v13, v25, v14
            bcax.16b   v14, v14, v31, v25
            bcax.16b    v7, v29,  v9,  v4
            bcax.16b    v8,  v4,  v5,  v9
            bcax.16b    v9,  v9,  v6,  v5
            bcax.16b    v5,  v5, v29,  v6
            bcax.16b    v6,  v6,  v4, v29
            
            bcax.16b    v3, v27,  v0, v28
            bcax.16b    v4, v28,  v1,  v0
            bcax.16b    v0,  v0,  v2,  v1
            bcax.16b    v1,  v1, v27,  v2
            bcax.16b    v2,  v2, v28, v27
            eor.16b     v0,  v0, v26

            // Rounds loop
            cbnz    {loop:x}, 0b

            // Write output (first 256 bits of state)
            st4.d {{ v0- v3}}[0], [{output}], #32
            st4.d {{ v0- v3}}[1], [{output}], #32

        ",
            input = inout(reg) input.as_ptr() => _,
            output = inout(reg) output.as_mut_ptr() => _,
            loop = inout(reg) 12 => _,
            rc = inout(reg) RC.as_ptr() => _,
            out("v0") _,
            out("v1") _,
            out("v2") _,
            out("v3") _,
            out("v4") _,
            out("v5") _,
            out("v6") _,
            out("v7") _,
            out("v8") _,
            out("v9") _,
            out("v10") _,
            out("v11") _,
            out("v12") _,
            out("v13") _,
            out("v14") _,
            out("v15") _,
            out("v16") _,
            out("v17") _,
            out("v18") _,
            out("v19") _,
            out("v20") _,
            out("v21") _,
            out("v22") _,
            out("v23") _,
            out("v24") _,
            out("v25") _,
            out("v26") _,
            out("v27") _,
            out("v28") _,
            out("v29") _,
            out("v30") _,
            out("v31") _,
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
    fn test_layer() {
        let mut input = (0..1600).map(|n| n as u8).collect::<Vec<_>>();
        let mut output = vec![0; next_layer_size(input.len())];
        let mut expected = vec![0; next_layer_size(input.len())];
        process_layer(&input, &mut output);
        generic::process_layer(&input, &mut expected);
        assert_eq!(to_hex(&output[..]), to_hex(&expected[..]),);
    }
}
