//! Kangaroo Twelve
//! 
//! <https://keccak.team/kangarootwelve.html>

const RC: [u64; 12] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808a,
    0x8000000080008000,
    0x000000000000808b,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008a,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000a,
];

pub fn k12_pow(challenge: &[u8; 32], target: u64) {
}

/// Double K12 on ARMv8-A with FEAT_SHA3.
///
/// Adapted from <>
///
pub fn k12_2(input_a: &[u64; 21], input_b: &[u64; 21], output_a: &mut [u64; 4], output_b: &mut [u64; 4]) {
    unsafe {
        core::arch::asm!("
            // Read input_a into v0-v20 lower 64-bit.
            ld4 {{ v0.1d- v3.1d}}[0], [{a}], #32
            ld4 {{ v4.1d- v7.1d}}[0], [{a}], #32
            ld4 {{ v8.1d-v11.1d}}[0], [{a}], #32
            ld4 {{v12.1d-v15.1d}}[0], [{a}], #32
            ld4 {{v16.1d-v19.1d}}[0], [{a}], #32
            ld1 {{v20.1d       }}[0], [{a}], #8

            // Read input_b into v0-v20 upper 64-bit.
            ld4 {{ v0.1d- v3.1d}}[1], [{b}], #32
            ld4 {{ v4.1d- v7.1d}}[1], [{b}], #32
            ld4 {{ v8.1d-v11.1d}}[1], [{b}], #32
            ld4 {{v12.1d-v15.1d}}[1], [{b}], #32
            ld4 {{v16.1d-v19.1d}}[1], [{b}], #32
            ld1 {{v20.1d       }}[1], [{b}], #8

            // Write padding bits to v21-v24.
            ld1 {{v20.1d-v23.1d}}, [{a}], #32
            ld1 {{v24.1d}}, [{a}]

            // Loop 12 rounds
            mov	{loop}, #12
            
        0:  sub	{loop}, {loop}, #1

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
            cbnz    {loop}, 0b
            
            // Write output (first 256 bits of state)
            st1 {{ v0.1d- v3.1d}}[0], [{out_a}]
            st1 {{ v0.1d- v3.1d}}[1], [{out_b}]

        ",
            a = inout("reg") input_a.as_ptr(),
            b = inout("reg") input_b.as_ptr(),
            out_a = inout("reg") out_a.as_mut_ptr(),
            out_b = inout("reg") out_b.as_mut_ptr(),
            rc = inout("reg") RC.as_ptr(),
            clobber_abi("C"),
            options(nostack)
        );
    }
}
