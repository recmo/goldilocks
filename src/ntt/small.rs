//! Generated using `cargo run --bin codegen`
#![allow(
    unused_parens,
    clippy::similar_names,
    clippy::unreadable_literal,
    clippy::too_many_lines
)]
use crate::Field;

const R51: Field = Field::new(0x0e736627a0aeb983);
const R52: Field = Field::new(0xdb8edc802dc0b266);
const R53: Field = Field::new(0x02efb5c2a6f35241);
const R54: Field = Field::new(0x130e07948a9d41d6);

/// Apply a small NTT to `values`, or return `false` if the size is not
/// supported.
pub fn small_ntt(values: &mut [Field]) -> bool {
    match values.len() {
        ..=1 => return true,
        2 => ntt_2(values),
        3 => ntt_3(values),
        4 => ntt_4(values),
        5 => ntt_5(values),
        6 => ntt_6(values),
        8 => ntt_8(values),
        10 => ntt_10(values),
        12 => ntt_12(values),
        15 => ntt_15(values),
        16 => ntt_16(values),
        20 => ntt_20(values),
        24 => ntt_24(values),
        30 => ntt_30(values),
        32 => ntt_32(values),
        40 => ntt_40(values),
        48 => ntt_48(values),
        60 => ntt_60(values),
        64 => ntt_64(values),
        80 => ntt_80(values),
        96 => ntt_96(values),
        120 => ntt_120(values),
        128 => ntt_128(values),
        160 => ntt_160(values),
        192 => ntt_192(values),
        240 => ntt_240(values),
        256 => ntt_256(values),
        _ => return false,
    }
    true
}

/// Size 2 NTT.
fn ntt_2(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 2);
    let a0 = values[0];
    let a1 = values[1];
    let (a0, a1) = (a0 + a1, a0 - a1);
    values[0] = a0;
    values[1] = a1;
}

/// Size 3 NTT.
fn ntt_3(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 3);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let (a0, a1, a2) = (
        a0 + a1 + a2,
        a0 + (a1 << 64) - (a2 << 32),
        a0 - (a1 << 32) + (a2 << 64),
    );
    values[0] = a0;
    values[1] = a1;
    values[2] = a2;
}

/// Size 4 NTT.
fn ntt_4(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 4);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    values[0] = a0;
    values[1] = a2;
    values[2] = a1;
    values[3] = a3;
}

/// Size 5 NTT.
fn ntt_5(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 5);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let (a0, a1, a2, a3, a4) = (
        a0 + a1 + a2 + a3 + a4,
        a0 + a1 * R51 + a2 * R52 + a3 * R53 + a4 * R54,
        a0 + a1 * R52 + a2 * R54 + a3 * R51 + a4 * R53,
        a0 + a1 * R53 + a2 * R51 + a3 * R54 + a4 * R52,
        a0 + a1 * R54 + a2 * R53 + a3 * R52 + a4 * R51,
    );
    values[0] = a0;
    values[1] = a1;
    values[2] = a2;
    values[3] = a3;
    values[4] = a4;
}

/// Size 6 NTT.
fn ntt_6(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 6);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let (a0, a3) = (a0 + a3, a0 - a3);
    let (a1, a4) = (a1 + a4, a1 - a4);
    let (a2, a5) = (a2 + a5, a2 - a5);
    let a4 = (a4 << 32);
    let a5 = (a5 << 64);
    let (a0, a1, a2) = (
        a0 + a1 + a2,
        a0 + (a1 << 64) - (a2 << 32),
        a0 - (a1 << 32) + (a2 << 64),
    );
    let (a3, a4, a5) = (
        a3 + a4 + a5,
        a3 + (a4 << 64) - (a5 << 32),
        a3 - (a4 << 32) + (a5 << 64),
    );
    values[0] = a0;
    values[1] = a3;
    values[2] = a1;
    values[3] = a4;
    values[4] = a2;
    values[5] = a5;
}

/// Size 8 NTT.
fn ntt_8(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 8);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let a5 = (a5 << 24);
    let a6 = (a6 << 48);
    let a7 = (a7 << 72);
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    values[0] = a0;
    values[1] = a4;
    values[2] = a2;
    values[3] = a6;
    values[4] = a1;
    values[5] = a5;
    values[6] = a3;
    values[7] = a7;
}

/// Size 10 NTT.
fn ntt_10(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 10);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let (a0, a5) = (a0 + a5, a0 - a5);
    let (a1, a6) = (a1 + a6, a1 - a6);
    let (a2, a7) = (a2 + a7, a2 - a7);
    let (a3, a8) = (a3 + a8, a3 - a8);
    let (a4, a9) = (a4 + a9, a4 - a9);
    let a6 = a6 * Field::new(18235156514275634624);
    let a7 = a7 * Field::new(1041288259238279555);
    let a8 = a8 * Field::new(17073700798457888299);
    let a9 = a9 * Field::new(15820824984080659046);
    let (a0, a1, a2, a3, a4) = (
        a0 + a1 + a2 + a3 + a4,
        a0 + a1 * R51 + a2 * R52 + a3 * R53 + a4 * R54,
        a0 + a1 * R52 + a2 * R54 + a3 * R51 + a4 * R53,
        a0 + a1 * R53 + a2 * R51 + a3 * R54 + a4 * R52,
        a0 + a1 * R54 + a2 * R53 + a3 * R52 + a4 * R51,
    );
    let (a5, a6, a7, a8, a9) = (
        a5 + a6 + a7 + a8 + a9,
        a5 + a6 * R51 + a7 * R52 + a8 * R53 + a9 * R54,
        a5 + a6 * R52 + a7 * R54 + a8 * R51 + a9 * R53,
        a5 + a6 * R53 + a7 * R51 + a8 * R54 + a9 * R52,
        a5 + a6 * R54 + a7 * R53 + a8 * R52 + a9 * R51,
    );
    values[0] = a0;
    values[1] = a5;
    values[2] = a1;
    values[3] = a6;
    values[4] = a2;
    values[5] = a7;
    values[6] = a3;
    values[7] = a8;
    values[8] = a4;
    values[9] = a9;
}

/// Size 12 NTT.
fn ntt_12(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 12);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let (a0, a4, a8) = (
        a0 + a4 + a8,
        a0 + (a4 << 64) - (a8 << 32),
        a0 - (a4 << 32) + (a8 << 64),
    );
    let (a1, a5, a9) = (
        a1 + a5 + a9,
        a1 + (a5 << 64) - (a9 << 32),
        a1 - (a5 << 32) + (a9 << 64),
    );
    let (a2, a6, a10) = (
        a2 + a6 + a10,
        a2 + (a6 << 64) - (a10 << 32),
        a2 - (a6 << 32) + (a10 << 64),
    );
    let (a3, a7, a11) = (
        a3 + a7 + a11,
        a3 + (a7 << 64) - (a11 << 32),
        a3 - (a7 << 32) + (a11 << 64),
    );
    let a5 = (a5 << 16);
    let a9 = (a9 << 32);
    let a6 = (a6 << 32);
    let a10 = (a10 << 64);
    let a7 = (a7 << 48);
    let a11 = (-a11);
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    values[0] = a0;
    values[1] = a4;
    values[2] = a8;
    values[3] = a2;
    values[4] = a6;
    values[5] = a10;
    values[6] = a1;
    values[7] = a5;
    values[8] = a9;
    values[9] = a3;
    values[10] = a7;
    values[11] = a11;
}

/// Size 15 NTT.
fn ntt_15(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 15);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let (a0, a5, a10) = (
        a0 + a5 + a10,
        a0 + (a5 << 64) - (a10 << 32),
        a0 - (a5 << 32) + (a10 << 64),
    );
    let (a1, a6, a11) = (
        a1 + a6 + a11,
        a1 + (a6 << 64) - (a11 << 32),
        a1 - (a6 << 32) + (a11 << 64),
    );
    let (a2, a7, a12) = (
        a2 + a7 + a12,
        a2 + (a7 << 64) - (a12 << 32),
        a2 - (a7 << 32) + (a12 << 64),
    );
    let (a3, a8, a13) = (
        a3 + a8 + a13,
        a3 + (a8 << 64) - (a13 << 32),
        a3 - (a8 << 32) + (a13 << 64),
    );
    let (a4, a9, a14) = (
        a4 + a9 + a14,
        a4 + (a9 << 64) - (a14 << 32),
        a4 - (a9 << 32) + (a14 << 64),
    );
    let a6 = a6 * Field::new(17775832080808074370);
    let a11 = a11 * Field::new(9988211933311186582);
    let a7 = a7 * Field::new(9988211933311186582);
    let a12 = a12 * Field::new(6205107048362784195);
    let a8 = a8 * Field::new(1041288259238279555);
    let a13 = a13 * Field::new(15820824984080659046);
    let a9 = a9 * Field::new(6205107048362784195);
    let a14 = a14 * Field::new(11578395661369729110);
    let (a0, a1, a2, a3, a4) = (
        a0 + a1 + a2 + a3 + a4,
        a0 + a1 * R51 + a2 * R52 + a3 * R53 + a4 * R54,
        a0 + a1 * R52 + a2 * R54 + a3 * R51 + a4 * R53,
        a0 + a1 * R53 + a2 * R51 + a3 * R54 + a4 * R52,
        a0 + a1 * R54 + a2 * R53 + a3 * R52 + a4 * R51,
    );
    let (a5, a6, a7, a8, a9) = (
        a5 + a6 + a7 + a8 + a9,
        a5 + a6 * R51 + a7 * R52 + a8 * R53 + a9 * R54,
        a5 + a6 * R52 + a7 * R54 + a8 * R51 + a9 * R53,
        a5 + a6 * R53 + a7 * R51 + a8 * R54 + a9 * R52,
        a5 + a6 * R54 + a7 * R53 + a8 * R52 + a9 * R51,
    );
    let (a10, a11, a12, a13, a14) = (
        a10 + a11 + a12 + a13 + a14,
        a10 + a11 * R51 + a12 * R52 + a13 * R53 + a14 * R54,
        a10 + a11 * R52 + a12 * R54 + a13 * R51 + a14 * R53,
        a10 + a11 * R53 + a12 * R51 + a13 * R54 + a14 * R52,
        a10 + a11 * R54 + a12 * R53 + a13 * R52 + a14 * R51,
    );
    values[0] = a0;
    values[1] = a5;
    values[2] = a10;
    values[3] = a1;
    values[4] = a6;
    values[5] = a11;
    values[6] = a2;
    values[7] = a7;
    values[8] = a12;
    values[9] = a3;
    values[10] = a8;
    values[11] = a13;
    values[12] = a4;
    values[13] = a9;
    values[14] = a14;
}

/// Size 16 NTT.
fn ntt_16(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 16);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let (a0, a8) = (a0 + a8, a0 - a8);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let a12 = (a12 << 48);
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let (a5, a13) = (a5 + a13, a5 - a13);
    let a13 = (a13 << 48);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a2, a10) = (a2 + a10, a2 - a10);
    let (a6, a14) = (a6 + a14, a6 - a14);
    let a14 = (a14 << 48);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let (a7, a15) = (a7 + a15, a7 - a15);
    let a15 = (a15 << 48);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a9 = (a9 << 12);
    let a5 = (a5 << 24);
    let a13 = (a13 << 36);
    let a10 = (a10 << 24);
    let a6 = (a6 << 48);
    let a14 = (a14 << 72);
    let a11 = (a11 << 36);
    let a7 = (a7 << 72);
    let a15 = (-(a15 << 12));
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    values[0] = a0;
    values[1] = a8;
    values[2] = a4;
    values[3] = a12;
    values[4] = a2;
    values[5] = a10;
    values[6] = a6;
    values[7] = a14;
    values[8] = a1;
    values[9] = a9;
    values[10] = a5;
    values[11] = a13;
    values[12] = a3;
    values[13] = a11;
    values[14] = a7;
    values[15] = a15;
}

/// Size 20 NTT.
fn ntt_20(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 20);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let (a0, a10) = (a0 + a10, a0 - a10);
    let (a5, a15) = (a5 + a15, a5 - a15);
    let a15 = (a15 << 48);
    let (a0, a5) = (a0 + a5, a0 - a5);
    let (a10, a15) = (a10 + a15, a10 - a15);
    let (a1, a11) = (a1 + a11, a1 - a11);
    let (a6, a16) = (a6 + a16, a6 - a16);
    let a16 = (a16 << 48);
    let (a1, a6) = (a1 + a6, a1 - a6);
    let (a11, a16) = (a11 + a16, a11 - a16);
    let (a2, a12) = (a2 + a12, a2 - a12);
    let (a7, a17) = (a7 + a17, a7 - a17);
    let a17 = (a17 << 48);
    let (a2, a7) = (a2 + a7, a2 - a7);
    let (a12, a17) = (a12 + a17, a12 - a17);
    let (a3, a13) = (a3 + a13, a3 - a13);
    let (a8, a18) = (a8 + a18, a8 - a18);
    let a18 = (a18 << 48);
    let (a3, a8) = (a3 + a8, a3 - a8);
    let (a13, a18) = (a13 + a18, a13 - a18);
    let (a4, a14) = (a4 + a14, a4 - a14);
    let (a9, a19) = (a9 + a19, a9 - a19);
    let a19 = (a19 << 48);
    let (a4, a9) = (a4 + a9, a4 - a9);
    let (a14, a19) = (a14 + a19, a14 - a19);
    let a11 = a11 * Field::new(5290193119087387221);
    let a6 = a6 * Field::new(18235156514275634624);
    let a16 = a16 * Field::new(8149776168132872528);
    let a12 = a12 * Field::new(18235156514275634624);
    let a7 = a7 * Field::new(1041288259238279555);
    let a17 = a17 * Field::new(17073700798457888299);
    let a13 = a13 * Field::new(8149776168132872528);
    let a8 = a8 * Field::new(17073700798457888299);
    let a18 = a18 * Field::new(2281812832982421726);
    let a14 = a14 * Field::new(1041288259238279555);
    let a9 = a9 * Field::new(15820824984080659046);
    let a19 = a19 * Field::new(211587555138949697);
    let (a0, a1, a2, a3, a4) = (
        a0 + a1 + a2 + a3 + a4,
        a0 + a1 * R51 + a2 * R52 + a3 * R53 + a4 * R54,
        a0 + a1 * R52 + a2 * R54 + a3 * R51 + a4 * R53,
        a0 + a1 * R53 + a2 * R51 + a3 * R54 + a4 * R52,
        a0 + a1 * R54 + a2 * R53 + a3 * R52 + a4 * R51,
    );
    let (a10, a11, a12, a13, a14) = (
        a10 + a11 + a12 + a13 + a14,
        a10 + a11 * R51 + a12 * R52 + a13 * R53 + a14 * R54,
        a10 + a11 * R52 + a12 * R54 + a13 * R51 + a14 * R53,
        a10 + a11 * R53 + a12 * R51 + a13 * R54 + a14 * R52,
        a10 + a11 * R54 + a12 * R53 + a13 * R52 + a14 * R51,
    );
    let (a5, a6, a7, a8, a9) = (
        a5 + a6 + a7 + a8 + a9,
        a5 + a6 * R51 + a7 * R52 + a8 * R53 + a9 * R54,
        a5 + a6 * R52 + a7 * R54 + a8 * R51 + a9 * R53,
        a5 + a6 * R53 + a7 * R51 + a8 * R54 + a9 * R52,
        a5 + a6 * R54 + a7 * R53 + a8 * R52 + a9 * R51,
    );
    let (a15, a16, a17, a18, a19) = (
        a15 + a16 + a17 + a18 + a19,
        a15 + a16 * R51 + a17 * R52 + a18 * R53 + a19 * R54,
        a15 + a16 * R52 + a17 * R54 + a18 * R51 + a19 * R53,
        a15 + a16 * R53 + a17 * R51 + a18 * R54 + a19 * R52,
        a15 + a16 * R54 + a17 * R53 + a18 * R52 + a19 * R51,
    );
    values[0] = a0;
    values[1] = a10;
    values[2] = a5;
    values[3] = a15;
    values[4] = a1;
    values[5] = a11;
    values[6] = a6;
    values[7] = a16;
    values[8] = a2;
    values[9] = a12;
    values[10] = a7;
    values[11] = a17;
    values[12] = a3;
    values[13] = a13;
    values[14] = a8;
    values[15] = a18;
    values[16] = a4;
    values[17] = a14;
    values[18] = a9;
    values[19] = a19;
}

/// Size 24 NTT.
fn ntt_24(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 24);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let (a0, a12) = (a0 + a12, a0 - a12);
    let (a6, a18) = (a6 + a18, a6 - a18);
    let a18 = (a18 << 48);
    let (a0, a6) = (a0 + a6, a0 - a6);
    let (a12, a18) = (a12 + a18, a12 - a18);
    let (a1, a13) = (a1 + a13, a1 - a13);
    let (a7, a19) = (a7 + a19, a7 - a19);
    let a19 = (a19 << 48);
    let (a1, a7) = (a1 + a7, a1 - a7);
    let (a13, a19) = (a13 + a19, a13 - a19);
    let (a2, a14) = (a2 + a14, a2 - a14);
    let (a8, a20) = (a8 + a20, a8 - a20);
    let a20 = (a20 << 48);
    let (a2, a8) = (a2 + a8, a2 - a8);
    let (a14, a20) = (a14 + a20, a14 - a20);
    let (a3, a15) = (a3 + a15, a3 - a15);
    let (a9, a21) = (a9 + a21, a9 - a21);
    let a21 = (a21 << 48);
    let (a3, a9) = (a3 + a9, a3 - a9);
    let (a15, a21) = (a15 + a21, a15 - a21);
    let (a4, a16) = (a4 + a16, a4 - a16);
    let (a10, a22) = (a10 + a22, a10 - a22);
    let a22 = (a22 << 48);
    let (a4, a10) = (a4 + a10, a4 - a10);
    let (a16, a22) = (a16 + a22, a16 - a22);
    let (a5, a17) = (a5 + a17, a5 - a17);
    let (a11, a23) = (a11 + a23, a11 - a23);
    let a23 = (a23 << 48);
    let (a5, a11) = (a5 + a11, a5 - a11);
    let (a17, a23) = (a17 + a23, a17 - a23);
    let a13 = (a13 << 8);
    let a7 = (a7 << 16);
    let a19 = (a19 << 24);
    let a14 = (a14 << 16);
    let a8 = (a8 << 32);
    let a20 = (a20 << 48);
    let a15 = (a15 << 24);
    let a9 = (a9 << 48);
    let a21 = (a21 << 72);
    let a16 = (a16 << 32);
    let a10 = (a10 << 64);
    let a22 = (-a22);
    let a17 = (a17 << 40);
    let a11 = (a11 << 80);
    let a23 = (-(a23 << 24));
    let (a0, a3) = (a0 + a3, a0 - a3);
    let (a1, a4) = (a1 + a4, a1 - a4);
    let (a2, a5) = (a2 + a5, a2 - a5);
    let a4 = (a4 << 32);
    let a5 = (a5 << 64);
    let (a0, a1, a2) = (
        a0 + a1 + a2,
        a0 + (a1 << 64) - (a2 << 32),
        a0 - (a1 << 32) + (a2 << 64),
    );
    let (a3, a4, a5) = (
        a3 + a4 + a5,
        a3 + (a4 << 64) - (a5 << 32),
        a3 - (a4 << 32) + (a5 << 64),
    );
    let (a12, a15) = (a12 + a15, a12 - a15);
    let (a13, a16) = (a13 + a16, a13 - a16);
    let (a14, a17) = (a14 + a17, a14 - a17);
    let a16 = (a16 << 32);
    let a17 = (a17 << 64);
    let (a12, a13, a14) = (
        a12 + a13 + a14,
        a12 + (a13 << 64) - (a14 << 32),
        a12 - (a13 << 32) + (a14 << 64),
    );
    let (a15, a16, a17) = (
        a15 + a16 + a17,
        a15 + (a16 << 64) - (a17 << 32),
        a15 - (a16 << 32) + (a17 << 64),
    );
    let (a6, a9) = (a6 + a9, a6 - a9);
    let (a7, a10) = (a7 + a10, a7 - a10);
    let (a8, a11) = (a8 + a11, a8 - a11);
    let a10 = (a10 << 32);
    let a11 = (a11 << 64);
    let (a6, a7, a8) = (
        a6 + a7 + a8,
        a6 + (a7 << 64) - (a8 << 32),
        a6 - (a7 << 32) + (a8 << 64),
    );
    let (a9, a10, a11) = (
        a9 + a10 + a11,
        a9 + (a10 << 64) - (a11 << 32),
        a9 - (a10 << 32) + (a11 << 64),
    );
    let (a18, a21) = (a18 + a21, a18 - a21);
    let (a19, a22) = (a19 + a22, a19 - a22);
    let (a20, a23) = (a20 + a23, a20 - a23);
    let a22 = (a22 << 32);
    let a23 = (a23 << 64);
    let (a18, a19, a20) = (
        a18 + a19 + a20,
        a18 + (a19 << 64) - (a20 << 32),
        a18 - (a19 << 32) + (a20 << 64),
    );
    let (a21, a22, a23) = (
        a21 + a22 + a23,
        a21 + (a22 << 64) - (a23 << 32),
        a21 - (a22 << 32) + (a23 << 64),
    );
    values[0] = a0;
    values[1] = a12;
    values[2] = a6;
    values[3] = a18;
    values[4] = a3;
    values[5] = a15;
    values[6] = a9;
    values[7] = a21;
    values[8] = a1;
    values[9] = a13;
    values[10] = a7;
    values[11] = a19;
    values[12] = a4;
    values[13] = a16;
    values[14] = a10;
    values[15] = a22;
    values[16] = a2;
    values[17] = a14;
    values[18] = a8;
    values[19] = a20;
    values[20] = a5;
    values[21] = a17;
    values[22] = a11;
    values[23] = a23;
}

/// Size 30 NTT.
fn ntt_30(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 30);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let (a0, a6, a12, a18, a24) = (
        a0 + a6 + a12 + a18 + a24,
        a0 + a6 * R51 + a12 * R52 + a18 * R53 + a24 * R54,
        a0 + a6 * R52 + a12 * R54 + a18 * R51 + a24 * R53,
        a0 + a6 * R53 + a12 * R51 + a18 * R54 + a24 * R52,
        a0 + a6 * R54 + a12 * R53 + a18 * R52 + a24 * R51,
    );
    let (a1, a7, a13, a19, a25) = (
        a1 + a7 + a13 + a19 + a25,
        a1 + a7 * R51 + a13 * R52 + a19 * R53 + a25 * R54,
        a1 + a7 * R52 + a13 * R54 + a19 * R51 + a25 * R53,
        a1 + a7 * R53 + a13 * R51 + a19 * R54 + a25 * R52,
        a1 + a7 * R54 + a13 * R53 + a19 * R52 + a25 * R51,
    );
    let (a2, a8, a14, a20, a26) = (
        a2 + a8 + a14 + a20 + a26,
        a2 + a8 * R51 + a14 * R52 + a20 * R53 + a26 * R54,
        a2 + a8 * R52 + a14 * R54 + a20 * R51 + a26 * R53,
        a2 + a8 * R53 + a14 * R51 + a20 * R54 + a26 * R52,
        a2 + a8 * R54 + a14 * R53 + a20 * R52 + a26 * R51,
    );
    let (a3, a9, a15, a21, a27) = (
        a3 + a9 + a15 + a21 + a27,
        a3 + a9 * R51 + a15 * R52 + a21 * R53 + a27 * R54,
        a3 + a9 * R52 + a15 * R54 + a21 * R51 + a27 * R53,
        a3 + a9 * R53 + a15 * R51 + a21 * R54 + a27 * R52,
        a3 + a9 * R54 + a15 * R53 + a21 * R52 + a27 * R51,
    );
    let (a4, a10, a16, a22, a28) = (
        a4 + a10 + a16 + a22 + a28,
        a4 + a10 * R51 + a16 * R52 + a22 * R53 + a28 * R54,
        a4 + a10 * R52 + a16 * R54 + a22 * R51 + a28 * R53,
        a4 + a10 * R53 + a16 * R51 + a22 * R54 + a28 * R52,
        a4 + a10 * R54 + a16 * R53 + a22 * R52 + a28 * R51,
    );
    let (a5, a11, a17, a23, a29) = (
        a5 + a11 + a17 + a23 + a29,
        a5 + a11 * R51 + a17 * R52 + a23 * R53 + a29 * R54,
        a5 + a11 * R52 + a17 * R54 + a23 * R51 + a29 * R53,
        a5 + a11 * R53 + a17 * R51 + a23 * R54 + a29 * R52,
        a5 + a11 * R54 + a17 * R53 + a23 * R52 + a29 * R51,
    );
    let a7 = a7 * Field::new(6868348408044855211);
    let a13 = a13 * Field::new(17775832080808074370);
    let a19 = a19 * Field::new(18235156514275634624);
    let a25 = a25 * Field::new(9988211933311186582);
    let a8 = a8 * Field::new(17775832080808074370);
    let a14 = a14 * Field::new(9988211933311186582);
    let a20 = a20 * Field::new(1041288259238279555);
    let a26 = a26 * Field::new(6205107048362784195);
    let a9 = a9 * Field::new(18235156514275634624);
    let a15 = a15 * Field::new(1041288259238279555);
    let a21 = a21 * Field::new(17073700798457888299);
    let a27 = a27 * Field::new(15820824984080659046);
    let a10 = a10 * Field::new(9988211933311186582);
    let a16 = a16 * Field::new(6205107048362784195);
    let a22 = a22 * Field::new(15820824984080659046);
    let a28 = a28 * Field::new(11578395661369729110);
    let a11 = (a11 << 32);
    let a17 = (a17 << 64);
    let a23 = (-a23);
    let a29 = (-(a29 << 32));
    let (a0, a3) = (a0 + a3, a0 - a3);
    let (a1, a4) = (a1 + a4, a1 - a4);
    let (a2, a5) = (a2 + a5, a2 - a5);
    let a4 = (a4 << 32);
    let a5 = (a5 << 64);
    let (a0, a1, a2) = (
        a0 + a1 + a2,
        a0 + (a1 << 64) - (a2 << 32),
        a0 - (a1 << 32) + (a2 << 64),
    );
    let (a3, a4, a5) = (
        a3 + a4 + a5,
        a3 + (a4 << 64) - (a5 << 32),
        a3 - (a4 << 32) + (a5 << 64),
    );
    let (a6, a9) = (a6 + a9, a6 - a9);
    let (a7, a10) = (a7 + a10, a7 - a10);
    let (a8, a11) = (a8 + a11, a8 - a11);
    let a10 = (a10 << 32);
    let a11 = (a11 << 64);
    let (a6, a7, a8) = (
        a6 + a7 + a8,
        a6 + (a7 << 64) - (a8 << 32),
        a6 - (a7 << 32) + (a8 << 64),
    );
    let (a9, a10, a11) = (
        a9 + a10 + a11,
        a9 + (a10 << 64) - (a11 << 32),
        a9 - (a10 << 32) + (a11 << 64),
    );
    let (a12, a15) = (a12 + a15, a12 - a15);
    let (a13, a16) = (a13 + a16, a13 - a16);
    let (a14, a17) = (a14 + a17, a14 - a17);
    let a16 = (a16 << 32);
    let a17 = (a17 << 64);
    let (a12, a13, a14) = (
        a12 + a13 + a14,
        a12 + (a13 << 64) - (a14 << 32),
        a12 - (a13 << 32) + (a14 << 64),
    );
    let (a15, a16, a17) = (
        a15 + a16 + a17,
        a15 + (a16 << 64) - (a17 << 32),
        a15 - (a16 << 32) + (a17 << 64),
    );
    let (a18, a21) = (a18 + a21, a18 - a21);
    let (a19, a22) = (a19 + a22, a19 - a22);
    let (a20, a23) = (a20 + a23, a20 - a23);
    let a22 = (a22 << 32);
    let a23 = (a23 << 64);
    let (a18, a19, a20) = (
        a18 + a19 + a20,
        a18 + (a19 << 64) - (a20 << 32),
        a18 - (a19 << 32) + (a20 << 64),
    );
    let (a21, a22, a23) = (
        a21 + a22 + a23,
        a21 + (a22 << 64) - (a23 << 32),
        a21 - (a22 << 32) + (a23 << 64),
    );
    let (a24, a27) = (a24 + a27, a24 - a27);
    let (a25, a28) = (a25 + a28, a25 - a28);
    let (a26, a29) = (a26 + a29, a26 - a29);
    let a28 = (a28 << 32);
    let a29 = (a29 << 64);
    let (a24, a25, a26) = (
        a24 + a25 + a26,
        a24 + (a25 << 64) - (a26 << 32),
        a24 - (a25 << 32) + (a26 << 64),
    );
    let (a27, a28, a29) = (
        a27 + a28 + a29,
        a27 + (a28 << 64) - (a29 << 32),
        a27 - (a28 << 32) + (a29 << 64),
    );
    values[0] = a0;
    values[1] = a6;
    values[2] = a12;
    values[3] = a18;
    values[4] = a24;
    values[5] = a3;
    values[6] = a9;
    values[7] = a15;
    values[8] = a21;
    values[9] = a27;
    values[10] = a1;
    values[11] = a7;
    values[12] = a13;
    values[13] = a19;
    values[14] = a25;
    values[15] = a4;
    values[16] = a10;
    values[17] = a16;
    values[18] = a22;
    values[19] = a28;
    values[20] = a2;
    values[21] = a8;
    values[22] = a14;
    values[23] = a20;
    values[24] = a26;
    values[25] = a5;
    values[26] = a11;
    values[27] = a17;
    values[28] = a23;
    values[29] = a29;
}

/// Size 32 NTT.
fn ntt_32(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 32);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let (a0, a16) = (a0 + a16, a0 - a16);
    let (a8, a24) = (a8 + a24, a8 - a24);
    let a24 = (a24 << 48);
    let (a0, a8) = (a0 + a8, a0 - a8);
    let (a16, a24) = (a16 + a24, a16 - a24);
    let (a1, a17) = (a1 + a17, a1 - a17);
    let (a9, a25) = (a9 + a25, a9 - a25);
    let a25 = (a25 << 48);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let (a17, a25) = (a17 + a25, a17 - a25);
    let (a2, a18) = (a2 + a18, a2 - a18);
    let (a10, a26) = (a10 + a26, a10 - a26);
    let a26 = (a26 << 48);
    let (a2, a10) = (a2 + a10, a2 - a10);
    let (a18, a26) = (a18 + a26, a18 - a26);
    let (a3, a19) = (a3 + a19, a3 - a19);
    let (a11, a27) = (a11 + a27, a11 - a27);
    let a27 = (a27 << 48);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let (a19, a27) = (a19 + a27, a19 - a27);
    let (a4, a20) = (a4 + a20, a4 - a20);
    let (a12, a28) = (a12 + a28, a12 - a28);
    let a28 = (a28 << 48);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let (a20, a28) = (a20 + a28, a20 - a28);
    let (a5, a21) = (a5 + a21, a5 - a21);
    let (a13, a29) = (a13 + a29, a13 - a29);
    let a29 = (a29 << 48);
    let (a5, a13) = (a5 + a13, a5 - a13);
    let (a21, a29) = (a21 + a29, a21 - a29);
    let (a6, a22) = (a6 + a22, a6 - a22);
    let (a14, a30) = (a14 + a30, a14 - a30);
    let a30 = (a30 << 48);
    let (a6, a14) = (a6 + a14, a6 - a14);
    let (a22, a30) = (a22 + a30, a22 - a30);
    let (a7, a23) = (a7 + a23, a7 - a23);
    let (a15, a31) = (a15 + a31, a15 - a31);
    let a31 = (a31 << 48);
    let (a7, a15) = (a7 + a15, a7 - a15);
    let (a23, a31) = (a23 + a31, a23 - a31);
    let a17 = (a17 << 6);
    let a9 = (a9 << 12);
    let a25 = (a25 << 18);
    let a18 = (a18 << 12);
    let a10 = (a10 << 24);
    let a26 = (a26 << 36);
    let a19 = (a19 << 18);
    let a11 = (a11 << 36);
    let a27 = (a27 << 54);
    let a20 = (a20 << 24);
    let a12 = (a12 << 48);
    let a28 = (a28 << 72);
    let a21 = (a21 << 30);
    let a13 = (a13 << 60);
    let a29 = (a29 << 90);
    let a22 = (a22 << 36);
    let a14 = (a14 << 72);
    let a30 = (-(a30 << 12));
    let a23 = (a23 << 42);
    let a15 = (a15 << 84);
    let a31 = (-(a31 << 30));
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let a5 = (a5 << 24);
    let a6 = (a6 << 48);
    let a7 = (a7 << 72);
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a16, a20) = (a16 + a20, a16 - a20);
    let (a17, a21) = (a17 + a21, a17 - a21);
    let (a18, a22) = (a18 + a22, a18 - a22);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let a21 = (a21 << 24);
    let a22 = (a22 << 48);
    let a23 = (a23 << 72);
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a13 = (a13 << 24);
    let a14 = (a14 << 48);
    let a15 = (a15 << 72);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a25, a29) = (a25 + a29, a25 - a29);
    let (a26, a30) = (a26 + a30, a26 - a30);
    let (a27, a31) = (a27 + a31, a27 - a31);
    let a29 = (a29 << 24);
    let a30 = (a30 << 48);
    let a31 = (a31 << 72);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    values[0] = a0;
    values[1] = a16;
    values[2] = a8;
    values[3] = a24;
    values[4] = a4;
    values[5] = a20;
    values[6] = a12;
    values[7] = a28;
    values[8] = a2;
    values[9] = a18;
    values[10] = a10;
    values[11] = a26;
    values[12] = a6;
    values[13] = a22;
    values[14] = a14;
    values[15] = a30;
    values[16] = a1;
    values[17] = a17;
    values[18] = a9;
    values[19] = a25;
    values[20] = a5;
    values[21] = a21;
    values[22] = a13;
    values[23] = a29;
    values[24] = a3;
    values[25] = a19;
    values[26] = a11;
    values[27] = a27;
    values[28] = a7;
    values[29] = a23;
    values[30] = a15;
    values[31] = a31;
}

/// Size 40 NTT.
fn ntt_40(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 40);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let (a0, a8, a16, a24, a32) = (
        a0 + a8 + a16 + a24 + a32,
        a0 + a8 * R51 + a16 * R52 + a24 * R53 + a32 * R54,
        a0 + a8 * R52 + a16 * R54 + a24 * R51 + a32 * R53,
        a0 + a8 * R53 + a16 * R51 + a24 * R54 + a32 * R52,
        a0 + a8 * R54 + a16 * R53 + a24 * R52 + a32 * R51,
    );
    let (a1, a9, a17, a25, a33) = (
        a1 + a9 + a17 + a25 + a33,
        a1 + a9 * R51 + a17 * R52 + a25 * R53 + a33 * R54,
        a1 + a9 * R52 + a17 * R54 + a25 * R51 + a33 * R53,
        a1 + a9 * R53 + a17 * R51 + a25 * R54 + a33 * R52,
        a1 + a9 * R54 + a17 * R53 + a25 * R52 + a33 * R51,
    );
    let (a2, a10, a18, a26, a34) = (
        a2 + a10 + a18 + a26 + a34,
        a2 + a10 * R51 + a18 * R52 + a26 * R53 + a34 * R54,
        a2 + a10 * R52 + a18 * R54 + a26 * R51 + a34 * R53,
        a2 + a10 * R53 + a18 * R51 + a26 * R54 + a34 * R52,
        a2 + a10 * R54 + a18 * R53 + a26 * R52 + a34 * R51,
    );
    let (a3, a11, a19, a27, a35) = (
        a3 + a11 + a19 + a27 + a35,
        a3 + a11 * R51 + a19 * R52 + a27 * R53 + a35 * R54,
        a3 + a11 * R52 + a19 * R54 + a27 * R51 + a35 * R53,
        a3 + a11 * R53 + a19 * R51 + a27 * R54 + a35 * R52,
        a3 + a11 * R54 + a19 * R53 + a27 * R52 + a35 * R51,
    );
    let (a4, a12, a20, a28, a36) = (
        a4 + a12 + a20 + a28 + a36,
        a4 + a12 * R51 + a20 * R52 + a28 * R53 + a36 * R54,
        a4 + a12 * R52 + a20 * R54 + a28 * R51 + a36 * R53,
        a4 + a12 * R53 + a20 * R51 + a28 * R54 + a36 * R52,
        a4 + a12 * R54 + a20 * R53 + a28 * R52 + a36 * R51,
    );
    let (a5, a13, a21, a29, a37) = (
        a5 + a13 + a21 + a29 + a37,
        a5 + a13 * R51 + a21 * R52 + a29 * R53 + a37 * R54,
        a5 + a13 * R52 + a21 * R54 + a29 * R51 + a37 * R53,
        a5 + a13 * R53 + a21 * R51 + a29 * R54 + a37 * R52,
        a5 + a13 * R54 + a21 * R53 + a29 * R52 + a37 * R51,
    );
    let (a6, a14, a22, a30, a38) = (
        a6 + a14 + a22 + a30 + a38,
        a6 + a14 * R51 + a22 * R52 + a30 * R53 + a38 * R54,
        a6 + a14 * R52 + a22 * R54 + a30 * R51 + a38 * R53,
        a6 + a14 * R53 + a22 * R51 + a30 * R54 + a38 * R52,
        a6 + a14 * R54 + a22 * R53 + a30 * R52 + a38 * R51,
    );
    let (a7, a15, a23, a31, a39) = (
        a7 + a15 + a23 + a31 + a39,
        a7 + a15 * R51 + a23 * R52 + a31 * R53 + a39 * R54,
        a7 + a15 * R52 + a23 * R54 + a31 * R51 + a39 * R53,
        a7 + a15 * R53 + a23 * R51 + a31 * R54 + a39 * R52,
        a7 + a15 * R54 + a23 * R53 + a31 * R52 + a39 * R51,
    );
    let a9 = a9 * Field::new(9148693690730647261);
    let a17 = a17 * Field::new(5290193119087387221);
    let a25 = a25 * Field::new(5856505865097423521);
    let a33 = a33 * Field::new(18235156514275634624);
    let a10 = a10 * Field::new(5290193119087387221);
    let a18 = a18 * Field::new(18235156514275634624);
    let a26 = a26 * Field::new(8149776168132872528);
    let a34 = a34 * Field::new(1041288259238279555);
    let a11 = a11 * Field::new(5856505865097423521);
    let a19 = a19 * Field::new(8149776168132872528);
    let a27 = a27 * Field::new(4419751934697861046);
    let a35 = a35 * Field::new(17073700798457888299);
    let a12 = a12 * Field::new(18235156514275634624);
    let a20 = a20 * Field::new(1041288259238279555);
    let a28 = a28 * Field::new(17073700798457888299);
    let a36 = a36 * Field::new(15820824984080659046);
    let a13 = (a13 << 24);
    let a21 = (a21 << 48);
    let a29 = (a29 << 72);
    let a37 = (-a37);
    let a14 = a14 * Field::new(8149776168132872528);
    let a22 = a22 * Field::new(17073700798457888299);
    let a30 = a30 * Field::new(2281812832982421726);
    let a38 = a38 * Field::new(211587555138949697);
    let a15 = a15 * Field::new(11331573348451128694);
    let a23 = a23 * Field::new(17869255328328231396);
    let a31 = a31 * Field::new(9298050378683937060);
    let a39 = a39 * Field::new(17405455810176304766);
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let a5 = (a5 << 24);
    let a6 = (a6 << 48);
    let a7 = (a7 << 72);
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a13 = (a13 << 24);
    let a14 = (a14 << 48);
    let a15 = (a15 << 72);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a16, a20) = (a16 + a20, a16 - a20);
    let (a17, a21) = (a17 + a21, a17 - a21);
    let (a18, a22) = (a18 + a22, a18 - a22);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let a21 = (a21 << 24);
    let a22 = (a22 << 48);
    let a23 = (a23 << 72);
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a25, a29) = (a25 + a29, a25 - a29);
    let (a26, a30) = (a26 + a30, a26 - a30);
    let (a27, a31) = (a27 + a31, a27 - a31);
    let a29 = (a29 << 24);
    let a30 = (a30 << 48);
    let a31 = (a31 << 72);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a33, a37) = (a33 + a37, a33 - a37);
    let (a34, a38) = (a34 + a38, a34 - a38);
    let (a35, a39) = (a35 + a39, a35 - a39);
    let a37 = (a37 << 24);
    let a38 = (a38 << 48);
    let a39 = (a39 << 72);
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    values[0] = a0;
    values[1] = a8;
    values[2] = a16;
    values[3] = a24;
    values[4] = a32;
    values[5] = a4;
    values[6] = a12;
    values[7] = a20;
    values[8] = a28;
    values[9] = a36;
    values[10] = a2;
    values[11] = a10;
    values[12] = a18;
    values[13] = a26;
    values[14] = a34;
    values[15] = a6;
    values[16] = a14;
    values[17] = a22;
    values[18] = a30;
    values[19] = a38;
    values[20] = a1;
    values[21] = a9;
    values[22] = a17;
    values[23] = a25;
    values[24] = a33;
    values[25] = a5;
    values[26] = a13;
    values[27] = a21;
    values[28] = a29;
    values[29] = a37;
    values[30] = a3;
    values[31] = a11;
    values[32] = a19;
    values[33] = a27;
    values[34] = a35;
    values[35] = a7;
    values[36] = a15;
    values[37] = a23;
    values[38] = a31;
    values[39] = a39;
}

/// Size 48 NTT.
fn ntt_48(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 48);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let (a0, a24) = (a0 + a24, a0 - a24);
    let (a8, a32) = (a8 + a32, a8 - a32);
    let (a16, a40) = (a16 + a40, a16 - a40);
    let a32 = (a32 << 32);
    let a40 = (a40 << 64);
    let (a0, a8, a16) = (
        a0 + a8 + a16,
        a0 + (a8 << 64) - (a16 << 32),
        a0 - (a8 << 32) + (a16 << 64),
    );
    let (a24, a32, a40) = (
        a24 + a32 + a40,
        a24 + (a32 << 64) - (a40 << 32),
        a24 - (a32 << 32) + (a40 << 64),
    );
    let (a1, a25) = (a1 + a25, a1 - a25);
    let (a9, a33) = (a9 + a33, a9 - a33);
    let (a17, a41) = (a17 + a41, a17 - a41);
    let a33 = (a33 << 32);
    let a41 = (a41 << 64);
    let (a1, a9, a17) = (
        a1 + a9 + a17,
        a1 + (a9 << 64) - (a17 << 32),
        a1 - (a9 << 32) + (a17 << 64),
    );
    let (a25, a33, a41) = (
        a25 + a33 + a41,
        a25 + (a33 << 64) - (a41 << 32),
        a25 - (a33 << 32) + (a41 << 64),
    );
    let (a2, a26) = (a2 + a26, a2 - a26);
    let (a10, a34) = (a10 + a34, a10 - a34);
    let (a18, a42) = (a18 + a42, a18 - a42);
    let a34 = (a34 << 32);
    let a42 = (a42 << 64);
    let (a2, a10, a18) = (
        a2 + a10 + a18,
        a2 + (a10 << 64) - (a18 << 32),
        a2 - (a10 << 32) + (a18 << 64),
    );
    let (a26, a34, a42) = (
        a26 + a34 + a42,
        a26 + (a34 << 64) - (a42 << 32),
        a26 - (a34 << 32) + (a42 << 64),
    );
    let (a3, a27) = (a3 + a27, a3 - a27);
    let (a11, a35) = (a11 + a35, a11 - a35);
    let (a19, a43) = (a19 + a43, a19 - a43);
    let a35 = (a35 << 32);
    let a43 = (a43 << 64);
    let (a3, a11, a19) = (
        a3 + a11 + a19,
        a3 + (a11 << 64) - (a19 << 32),
        a3 - (a11 << 32) + (a19 << 64),
    );
    let (a27, a35, a43) = (
        a27 + a35 + a43,
        a27 + (a35 << 64) - (a43 << 32),
        a27 - (a35 << 32) + (a43 << 64),
    );
    let (a4, a28) = (a4 + a28, a4 - a28);
    let (a12, a36) = (a12 + a36, a12 - a36);
    let (a20, a44) = (a20 + a44, a20 - a44);
    let a36 = (a36 << 32);
    let a44 = (a44 << 64);
    let (a4, a12, a20) = (
        a4 + a12 + a20,
        a4 + (a12 << 64) - (a20 << 32),
        a4 - (a12 << 32) + (a20 << 64),
    );
    let (a28, a36, a44) = (
        a28 + a36 + a44,
        a28 + (a36 << 64) - (a44 << 32),
        a28 - (a36 << 32) + (a44 << 64),
    );
    let (a5, a29) = (a5 + a29, a5 - a29);
    let (a13, a37) = (a13 + a37, a13 - a37);
    let (a21, a45) = (a21 + a45, a21 - a45);
    let a37 = (a37 << 32);
    let a45 = (a45 << 64);
    let (a5, a13, a21) = (
        a5 + a13 + a21,
        a5 + (a13 << 64) - (a21 << 32),
        a5 - (a13 << 32) + (a21 << 64),
    );
    let (a29, a37, a45) = (
        a29 + a37 + a45,
        a29 + (a37 << 64) - (a45 << 32),
        a29 - (a37 << 32) + (a45 << 64),
    );
    let (a6, a30) = (a6 + a30, a6 - a30);
    let (a14, a38) = (a14 + a38, a14 - a38);
    let (a22, a46) = (a22 + a46, a22 - a46);
    let a38 = (a38 << 32);
    let a46 = (a46 << 64);
    let (a6, a14, a22) = (
        a6 + a14 + a22,
        a6 + (a14 << 64) - (a22 << 32),
        a6 - (a14 << 32) + (a22 << 64),
    );
    let (a30, a38, a46) = (
        a30 + a38 + a46,
        a30 + (a38 << 64) - (a46 << 32),
        a30 - (a38 << 32) + (a46 << 64),
    );
    let (a7, a31) = (a7 + a31, a7 - a31);
    let (a15, a39) = (a15 + a39, a15 - a39);
    let (a23, a47) = (a23 + a47, a23 - a47);
    let a39 = (a39 << 32);
    let a47 = (a47 << 64);
    let (a7, a15, a23) = (
        a7 + a15 + a23,
        a7 + (a15 << 64) - (a23 << 32),
        a7 - (a15 << 32) + (a23 << 64),
    );
    let (a31, a39, a47) = (
        a31 + a39 + a47,
        a31 + (a39 << 64) - (a47 << 32),
        a31 - (a39 << 32) + (a47 << 64),
    );
    let a25 = (a25 << 4);
    let a9 = (a9 << 8);
    let a33 = (a33 << 12);
    let a17 = (a17 << 16);
    let a41 = (a41 << 20);
    let a26 = (a26 << 8);
    let a10 = (a10 << 16);
    let a34 = (a34 << 24);
    let a18 = (a18 << 32);
    let a42 = (a42 << 40);
    let a27 = (a27 << 12);
    let a11 = (a11 << 24);
    let a35 = (a35 << 36);
    let a19 = (a19 << 48);
    let a43 = (a43 << 60);
    let a28 = (a28 << 16);
    let a12 = (a12 << 32);
    let a36 = (a36 << 48);
    let a20 = (a20 << 64);
    let a44 = (a44 << 80);
    let a29 = (a29 << 20);
    let a13 = (a13 << 40);
    let a37 = (a37 << 60);
    let a21 = (a21 << 80);
    let a45 = (-(a45 << 4));
    let a30 = (a30 << 24);
    let a14 = (a14 << 48);
    let a38 = (a38 << 72);
    let a22 = (-a22);
    let a46 = (-(a46 << 24));
    let a31 = (a31 << 28);
    let a15 = (a15 << 56);
    let a39 = (a39 << 84);
    let a23 = (-(a23 << 16));
    let a47 = (-(a47 << 44));
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let a5 = (a5 << 24);
    let a6 = (a6 << 48);
    let a7 = (a7 << 72);
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a25, a29) = (a25 + a29, a25 - a29);
    let (a26, a30) = (a26 + a30, a26 - a30);
    let (a27, a31) = (a27 + a31, a27 - a31);
    let a29 = (a29 << 24);
    let a30 = (a30 << 48);
    let a31 = (a31 << 72);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a13 = (a13 << 24);
    let a14 = (a14 << 48);
    let a15 = (a15 << 72);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a33, a37) = (a33 + a37, a33 - a37);
    let (a34, a38) = (a34 + a38, a34 - a38);
    let (a35, a39) = (a35 + a39, a35 - a39);
    let a37 = (a37 << 24);
    let a38 = (a38 << 48);
    let a39 = (a39 << 72);
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    let (a16, a20) = (a16 + a20, a16 - a20);
    let (a17, a21) = (a17 + a21, a17 - a21);
    let (a18, a22) = (a18 + a22, a18 - a22);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let a21 = (a21 << 24);
    let a22 = (a22 << 48);
    let a23 = (a23 << 72);
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a40, a44) = (a40 + a44, a40 - a44);
    let (a41, a45) = (a41 + a45, a41 - a45);
    let (a42, a46) = (a42 + a46, a42 - a46);
    let (a43, a47) = (a43 + a47, a43 - a47);
    let a45 = (a45 << 24);
    let a46 = (a46 << 48);
    let a47 = (a47 << 72);
    let (a40, a42) = (a40 + a42, a40 - a42);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let a43 = (a43 << 48);
    let (a40, a41) = (a40 + a41, a40 - a41);
    let (a42, a43) = (a42 + a43, a42 - a43);
    let (a44, a46) = (a44 + a46, a44 - a46);
    let (a45, a47) = (a45 + a47, a45 - a47);
    let a47 = (a47 << 48);
    let (a44, a45) = (a44 + a45, a44 - a45);
    let (a46, a47) = (a46 + a47, a46 - a47);
    values[0] = a0;
    values[1] = a24;
    values[2] = a8;
    values[3] = a32;
    values[4] = a16;
    values[5] = a40;
    values[6] = a4;
    values[7] = a28;
    values[8] = a12;
    values[9] = a36;
    values[10] = a20;
    values[11] = a44;
    values[12] = a2;
    values[13] = a26;
    values[14] = a10;
    values[15] = a34;
    values[16] = a18;
    values[17] = a42;
    values[18] = a6;
    values[19] = a30;
    values[20] = a14;
    values[21] = a38;
    values[22] = a22;
    values[23] = a46;
    values[24] = a1;
    values[25] = a25;
    values[26] = a9;
    values[27] = a33;
    values[28] = a17;
    values[29] = a41;
    values[30] = a5;
    values[31] = a29;
    values[32] = a13;
    values[33] = a37;
    values[34] = a21;
    values[35] = a45;
    values[36] = a3;
    values[37] = a27;
    values[38] = a11;
    values[39] = a35;
    values[40] = a19;
    values[41] = a43;
    values[42] = a7;
    values[43] = a31;
    values[44] = a15;
    values[45] = a39;
    values[46] = a23;
    values[47] = a47;
}

/// Size 60 NTT.
fn ntt_60(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 60);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let (a0, a30) = (a0 + a30, a0 - a30);
    let (a10, a40) = (a10 + a40, a10 - a40);
    let (a20, a50) = (a20 + a50, a20 - a50);
    let a40 = (a40 << 32);
    let a50 = (a50 << 64);
    let (a0, a10, a20) = (
        a0 + a10 + a20,
        a0 + (a10 << 64) - (a20 << 32),
        a0 - (a10 << 32) + (a20 << 64),
    );
    let (a30, a40, a50) = (
        a30 + a40 + a50,
        a30 + (a40 << 64) - (a50 << 32),
        a30 - (a40 << 32) + (a50 << 64),
    );
    let (a1, a31) = (a1 + a31, a1 - a31);
    let (a11, a41) = (a11 + a41, a11 - a41);
    let (a21, a51) = (a21 + a51, a21 - a51);
    let a41 = (a41 << 32);
    let a51 = (a51 << 64);
    let (a1, a11, a21) = (
        a1 + a11 + a21,
        a1 + (a11 << 64) - (a21 << 32),
        a1 - (a11 << 32) + (a21 << 64),
    );
    let (a31, a41, a51) = (
        a31 + a41 + a51,
        a31 + (a41 << 64) - (a51 << 32),
        a31 - (a41 << 32) + (a51 << 64),
    );
    let (a2, a32) = (a2 + a32, a2 - a32);
    let (a12, a42) = (a12 + a42, a12 - a42);
    let (a22, a52) = (a22 + a52, a22 - a52);
    let a42 = (a42 << 32);
    let a52 = (a52 << 64);
    let (a2, a12, a22) = (
        a2 + a12 + a22,
        a2 + (a12 << 64) - (a22 << 32),
        a2 - (a12 << 32) + (a22 << 64),
    );
    let (a32, a42, a52) = (
        a32 + a42 + a52,
        a32 + (a42 << 64) - (a52 << 32),
        a32 - (a42 << 32) + (a52 << 64),
    );
    let (a3, a33) = (a3 + a33, a3 - a33);
    let (a13, a43) = (a13 + a43, a13 - a43);
    let (a23, a53) = (a23 + a53, a23 - a53);
    let a43 = (a43 << 32);
    let a53 = (a53 << 64);
    let (a3, a13, a23) = (
        a3 + a13 + a23,
        a3 + (a13 << 64) - (a23 << 32),
        a3 - (a13 << 32) + (a23 << 64),
    );
    let (a33, a43, a53) = (
        a33 + a43 + a53,
        a33 + (a43 << 64) - (a53 << 32),
        a33 - (a43 << 32) + (a53 << 64),
    );
    let (a4, a34) = (a4 + a34, a4 - a34);
    let (a14, a44) = (a14 + a44, a14 - a44);
    let (a24, a54) = (a24 + a54, a24 - a54);
    let a44 = (a44 << 32);
    let a54 = (a54 << 64);
    let (a4, a14, a24) = (
        a4 + a14 + a24,
        a4 + (a14 << 64) - (a24 << 32),
        a4 - (a14 << 32) + (a24 << 64),
    );
    let (a34, a44, a54) = (
        a34 + a44 + a54,
        a34 + (a44 << 64) - (a54 << 32),
        a34 - (a44 << 32) + (a54 << 64),
    );
    let (a5, a35) = (a5 + a35, a5 - a35);
    let (a15, a45) = (a15 + a45, a15 - a45);
    let (a25, a55) = (a25 + a55, a25 - a55);
    let a45 = (a45 << 32);
    let a55 = (a55 << 64);
    let (a5, a15, a25) = (
        a5 + a15 + a25,
        a5 + (a15 << 64) - (a25 << 32),
        a5 - (a15 << 32) + (a25 << 64),
    );
    let (a35, a45, a55) = (
        a35 + a45 + a55,
        a35 + (a45 << 64) - (a55 << 32),
        a35 - (a45 << 32) + (a55 << 64),
    );
    let (a6, a36) = (a6 + a36, a6 - a36);
    let (a16, a46) = (a16 + a46, a16 - a46);
    let (a26, a56) = (a26 + a56, a26 - a56);
    let a46 = (a46 << 32);
    let a56 = (a56 << 64);
    let (a6, a16, a26) = (
        a6 + a16 + a26,
        a6 + (a16 << 64) - (a26 << 32),
        a6 - (a16 << 32) + (a26 << 64),
    );
    let (a36, a46, a56) = (
        a36 + a46 + a56,
        a36 + (a46 << 64) - (a56 << 32),
        a36 - (a46 << 32) + (a56 << 64),
    );
    let (a7, a37) = (a7 + a37, a7 - a37);
    let (a17, a47) = (a17 + a47, a17 - a47);
    let (a27, a57) = (a27 + a57, a27 - a57);
    let a47 = (a47 << 32);
    let a57 = (a57 << 64);
    let (a7, a17, a27) = (
        a7 + a17 + a27,
        a7 + (a17 << 64) - (a27 << 32),
        a7 - (a17 << 32) + (a27 << 64),
    );
    let (a37, a47, a57) = (
        a37 + a47 + a57,
        a37 + (a47 << 64) - (a57 << 32),
        a37 - (a47 << 32) + (a57 << 64),
    );
    let (a8, a38) = (a8 + a38, a8 - a38);
    let (a18, a48) = (a18 + a48, a18 - a48);
    let (a28, a58) = (a28 + a58, a28 - a58);
    let a48 = (a48 << 32);
    let a58 = (a58 << 64);
    let (a8, a18, a28) = (
        a8 + a18 + a28,
        a8 + (a18 << 64) - (a28 << 32),
        a8 - (a18 << 32) + (a28 << 64),
    );
    let (a38, a48, a58) = (
        a38 + a48 + a58,
        a38 + (a48 << 64) - (a58 << 32),
        a38 - (a48 << 32) + (a58 << 64),
    );
    let (a9, a39) = (a9 + a39, a9 - a39);
    let (a19, a49) = (a19 + a49, a19 - a49);
    let (a29, a59) = (a29 + a59, a29 - a59);
    let a49 = (a49 << 32);
    let a59 = (a59 << 64);
    let (a9, a19, a29) = (
        a9 + a19 + a29,
        a9 + (a19 << 64) - (a29 << 32),
        a9 - (a19 << 32) + (a29 << 64),
    );
    let (a39, a49, a59) = (
        a39 + a49 + a59,
        a39 + (a49 << 64) - (a59 << 32),
        a39 - (a49 << 32) + (a59 << 64),
    );
    let a31 = a31 * Field::new(5927015354646419725);
    let a11 = a11 * Field::new(6868348408044855211);
    let a41 = a41 * Field::new(5290193119087387221);
    let a21 = a21 * Field::new(17775832080808074370);
    let a51 = (a51 << 16);
    let a32 = a32 * Field::new(6868348408044855211);
    let a12 = a12 * Field::new(17775832080808074370);
    let a42 = a42 * Field::new(18235156514275634624);
    let a22 = a22 * Field::new(9988211933311186582);
    let a52 = (a52 << 32);
    let a33 = a33 * Field::new(5290193119087387221);
    let a13 = a13 * Field::new(18235156514275634624);
    let a43 = a43 * Field::new(8149776168132872528);
    let a23 = a23 * Field::new(1041288259238279555);
    let a53 = (a53 << 48);
    let a34 = a34 * Field::new(17775832080808074370);
    let a14 = a14 * Field::new(9988211933311186582);
    let a44 = a44 * Field::new(1041288259238279555);
    let a24 = a24 * Field::new(6205107048362784195);
    let a54 = (a54 << 64);
    let a35 = (a35 << 16);
    let a15 = (a15 << 32);
    let a45 = (a45 << 48);
    let a25 = (a25 << 64);
    let a55 = (a55 << 80);
    let a36 = a36 * Field::new(18235156514275634624);
    let a16 = a16 * Field::new(1041288259238279555);
    let a46 = a46 * Field::new(17073700798457888299);
    let a26 = a26 * Field::new(15820824984080659046);
    let a56 = (-a56);
    let a37 = a37 * Field::new(5079231842359091375);
    let a17 = a17 * Field::new(15149912995474149095);
    let a47 = a47 * Field::new(17869255328328231396);
    let a27 = a27 * Field::new(7085488865146701717);
    let a57 = (-(a57 << 16));
    let a38 = a38 * Field::new(9988211933311186582);
    let a18 = a18 * Field::new(6205107048362784195);
    let a48 = a48 * Field::new(15820824984080659046);
    let a28 = a28 * Field::new(11578395661369729110);
    let a58 = (-(a58 << 32));
    let a39 = a39 * Field::new(8149776168132872528);
    let a19 = a19 * Field::new(17073700798457888299);
    let a49 = a49 * Field::new(2281812832982421726);
    let a29 = a29 * Field::new(211587555138949697);
    let a59 = (-(a59 << 48));
    let (a0, a5) = (a0 + a5, a0 - a5);
    let (a1, a6) = (a1 + a6, a1 - a6);
    let (a2, a7) = (a2 + a7, a2 - a7);
    let (a3, a8) = (a3 + a8, a3 - a8);
    let (a4, a9) = (a4 + a9, a4 - a9);
    let a6 = a6 * Field::new(18235156514275634624);
    let a7 = a7 * Field::new(1041288259238279555);
    let a8 = a8 * Field::new(17073700798457888299);
    let a9 = a9 * Field::new(15820824984080659046);
    let (a0, a1, a2, a3, a4) = (
        a0 + a1 + a2 + a3 + a4,
        a0 + a1 * R51 + a2 * R52 + a3 * R53 + a4 * R54,
        a0 + a1 * R52 + a2 * R54 + a3 * R51 + a4 * R53,
        a0 + a1 * R53 + a2 * R51 + a3 * R54 + a4 * R52,
        a0 + a1 * R54 + a2 * R53 + a3 * R52 + a4 * R51,
    );
    let (a5, a6, a7, a8, a9) = (
        a5 + a6 + a7 + a8 + a9,
        a5 + a6 * R51 + a7 * R52 + a8 * R53 + a9 * R54,
        a5 + a6 * R52 + a7 * R54 + a8 * R51 + a9 * R53,
        a5 + a6 * R53 + a7 * R51 + a8 * R54 + a9 * R52,
        a5 + a6 * R54 + a7 * R53 + a8 * R52 + a9 * R51,
    );
    let (a30, a35) = (a30 + a35, a30 - a35);
    let (a31, a36) = (a31 + a36, a31 - a36);
    let (a32, a37) = (a32 + a37, a32 - a37);
    let (a33, a38) = (a33 + a38, a33 - a38);
    let (a34, a39) = (a34 + a39, a34 - a39);
    let a36 = a36 * Field::new(18235156514275634624);
    let a37 = a37 * Field::new(1041288259238279555);
    let a38 = a38 * Field::new(17073700798457888299);
    let a39 = a39 * Field::new(15820824984080659046);
    let (a30, a31, a32, a33, a34) = (
        a30 + a31 + a32 + a33 + a34,
        a30 + a31 * R51 + a32 * R52 + a33 * R53 + a34 * R54,
        a30 + a31 * R52 + a32 * R54 + a33 * R51 + a34 * R53,
        a30 + a31 * R53 + a32 * R51 + a33 * R54 + a34 * R52,
        a30 + a31 * R54 + a32 * R53 + a33 * R52 + a34 * R51,
    );
    let (a35, a36, a37, a38, a39) = (
        a35 + a36 + a37 + a38 + a39,
        a35 + a36 * R51 + a37 * R52 + a38 * R53 + a39 * R54,
        a35 + a36 * R52 + a37 * R54 + a38 * R51 + a39 * R53,
        a35 + a36 * R53 + a37 * R51 + a38 * R54 + a39 * R52,
        a35 + a36 * R54 + a37 * R53 + a38 * R52 + a39 * R51,
    );
    let (a10, a15) = (a10 + a15, a10 - a15);
    let (a11, a16) = (a11 + a16, a11 - a16);
    let (a12, a17) = (a12 + a17, a12 - a17);
    let (a13, a18) = (a13 + a18, a13 - a18);
    let (a14, a19) = (a14 + a19, a14 - a19);
    let a16 = a16 * Field::new(18235156514275634624);
    let a17 = a17 * Field::new(1041288259238279555);
    let a18 = a18 * Field::new(17073700798457888299);
    let a19 = a19 * Field::new(15820824984080659046);
    let (a10, a11, a12, a13, a14) = (
        a10 + a11 + a12 + a13 + a14,
        a10 + a11 * R51 + a12 * R52 + a13 * R53 + a14 * R54,
        a10 + a11 * R52 + a12 * R54 + a13 * R51 + a14 * R53,
        a10 + a11 * R53 + a12 * R51 + a13 * R54 + a14 * R52,
        a10 + a11 * R54 + a12 * R53 + a13 * R52 + a14 * R51,
    );
    let (a15, a16, a17, a18, a19) = (
        a15 + a16 + a17 + a18 + a19,
        a15 + a16 * R51 + a17 * R52 + a18 * R53 + a19 * R54,
        a15 + a16 * R52 + a17 * R54 + a18 * R51 + a19 * R53,
        a15 + a16 * R53 + a17 * R51 + a18 * R54 + a19 * R52,
        a15 + a16 * R54 + a17 * R53 + a18 * R52 + a19 * R51,
    );
    let (a40, a45) = (a40 + a45, a40 - a45);
    let (a41, a46) = (a41 + a46, a41 - a46);
    let (a42, a47) = (a42 + a47, a42 - a47);
    let (a43, a48) = (a43 + a48, a43 - a48);
    let (a44, a49) = (a44 + a49, a44 - a49);
    let a46 = a46 * Field::new(18235156514275634624);
    let a47 = a47 * Field::new(1041288259238279555);
    let a48 = a48 * Field::new(17073700798457888299);
    let a49 = a49 * Field::new(15820824984080659046);
    let (a40, a41, a42, a43, a44) = (
        a40 + a41 + a42 + a43 + a44,
        a40 + a41 * R51 + a42 * R52 + a43 * R53 + a44 * R54,
        a40 + a41 * R52 + a42 * R54 + a43 * R51 + a44 * R53,
        a40 + a41 * R53 + a42 * R51 + a43 * R54 + a44 * R52,
        a40 + a41 * R54 + a42 * R53 + a43 * R52 + a44 * R51,
    );
    let (a45, a46, a47, a48, a49) = (
        a45 + a46 + a47 + a48 + a49,
        a45 + a46 * R51 + a47 * R52 + a48 * R53 + a49 * R54,
        a45 + a46 * R52 + a47 * R54 + a48 * R51 + a49 * R53,
        a45 + a46 * R53 + a47 * R51 + a48 * R54 + a49 * R52,
        a45 + a46 * R54 + a47 * R53 + a48 * R52 + a49 * R51,
    );
    let (a20, a25) = (a20 + a25, a20 - a25);
    let (a21, a26) = (a21 + a26, a21 - a26);
    let (a22, a27) = (a22 + a27, a22 - a27);
    let (a23, a28) = (a23 + a28, a23 - a28);
    let (a24, a29) = (a24 + a29, a24 - a29);
    let a26 = a26 * Field::new(18235156514275634624);
    let a27 = a27 * Field::new(1041288259238279555);
    let a28 = a28 * Field::new(17073700798457888299);
    let a29 = a29 * Field::new(15820824984080659046);
    let (a20, a21, a22, a23, a24) = (
        a20 + a21 + a22 + a23 + a24,
        a20 + a21 * R51 + a22 * R52 + a23 * R53 + a24 * R54,
        a20 + a21 * R52 + a22 * R54 + a23 * R51 + a24 * R53,
        a20 + a21 * R53 + a22 * R51 + a23 * R54 + a24 * R52,
        a20 + a21 * R54 + a22 * R53 + a23 * R52 + a24 * R51,
    );
    let (a25, a26, a27, a28, a29) = (
        a25 + a26 + a27 + a28 + a29,
        a25 + a26 * R51 + a27 * R52 + a28 * R53 + a29 * R54,
        a25 + a26 * R52 + a27 * R54 + a28 * R51 + a29 * R53,
        a25 + a26 * R53 + a27 * R51 + a28 * R54 + a29 * R52,
        a25 + a26 * R54 + a27 * R53 + a28 * R52 + a29 * R51,
    );
    let (a50, a55) = (a50 + a55, a50 - a55);
    let (a51, a56) = (a51 + a56, a51 - a56);
    let (a52, a57) = (a52 + a57, a52 - a57);
    let (a53, a58) = (a53 + a58, a53 - a58);
    let (a54, a59) = (a54 + a59, a54 - a59);
    let a56 = a56 * Field::new(18235156514275634624);
    let a57 = a57 * Field::new(1041288259238279555);
    let a58 = a58 * Field::new(17073700798457888299);
    let a59 = a59 * Field::new(15820824984080659046);
    let (a50, a51, a52, a53, a54) = (
        a50 + a51 + a52 + a53 + a54,
        a50 + a51 * R51 + a52 * R52 + a53 * R53 + a54 * R54,
        a50 + a51 * R52 + a52 * R54 + a53 * R51 + a54 * R53,
        a50 + a51 * R53 + a52 * R51 + a53 * R54 + a54 * R52,
        a50 + a51 * R54 + a52 * R53 + a53 * R52 + a54 * R51,
    );
    let (a55, a56, a57, a58, a59) = (
        a55 + a56 + a57 + a58 + a59,
        a55 + a56 * R51 + a57 * R52 + a58 * R53 + a59 * R54,
        a55 + a56 * R52 + a57 * R54 + a58 * R51 + a59 * R53,
        a55 + a56 * R53 + a57 * R51 + a58 * R54 + a59 * R52,
        a55 + a56 * R54 + a57 * R53 + a58 * R52 + a59 * R51,
    );
    values[0] = a0;
    values[1] = a30;
    values[2] = a10;
    values[3] = a40;
    values[4] = a20;
    values[5] = a50;
    values[6] = a5;
    values[7] = a35;
    values[8] = a15;
    values[9] = a45;
    values[10] = a25;
    values[11] = a55;
    values[12] = a1;
    values[13] = a31;
    values[14] = a11;
    values[15] = a41;
    values[16] = a21;
    values[17] = a51;
    values[18] = a6;
    values[19] = a36;
    values[20] = a16;
    values[21] = a46;
    values[22] = a26;
    values[23] = a56;
    values[24] = a2;
    values[25] = a32;
    values[26] = a12;
    values[27] = a42;
    values[28] = a22;
    values[29] = a52;
    values[30] = a7;
    values[31] = a37;
    values[32] = a17;
    values[33] = a47;
    values[34] = a27;
    values[35] = a57;
    values[36] = a3;
    values[37] = a33;
    values[38] = a13;
    values[39] = a43;
    values[40] = a23;
    values[41] = a53;
    values[42] = a8;
    values[43] = a38;
    values[44] = a18;
    values[45] = a48;
    values[46] = a28;
    values[47] = a58;
    values[48] = a4;
    values[49] = a34;
    values[50] = a14;
    values[51] = a44;
    values[52] = a24;
    values[53] = a54;
    values[54] = a9;
    values[55] = a39;
    values[56] = a19;
    values[57] = a49;
    values[58] = a29;
    values[59] = a59;
}

/// Size 64 NTT.
fn ntt_64(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 64);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let a60 = values[60];
    let a61 = values[61];
    let a62 = values[62];
    let a63 = values[63];
    let (a0, a32) = (a0 + a32, a0 - a32);
    let (a8, a40) = (a8 + a40, a8 - a40);
    let (a16, a48) = (a16 + a48, a16 - a48);
    let (a24, a56) = (a24 + a56, a24 - a56);
    let a40 = (a40 << 24);
    let a48 = (a48 << 48);
    let a56 = (a56 << 72);
    let (a0, a16) = (a0 + a16, a0 - a16);
    let (a8, a24) = (a8 + a24, a8 - a24);
    let a24 = (a24 << 48);
    let (a0, a8) = (a0 + a8, a0 - a8);
    let (a16, a24) = (a16 + a24, a16 - a24);
    let (a32, a48) = (a32 + a48, a32 - a48);
    let (a40, a56) = (a40 + a56, a40 - a56);
    let a56 = (a56 << 48);
    let (a32, a40) = (a32 + a40, a32 - a40);
    let (a48, a56) = (a48 + a56, a48 - a56);
    let (a1, a33) = (a1 + a33, a1 - a33);
    let (a9, a41) = (a9 + a41, a9 - a41);
    let (a17, a49) = (a17 + a49, a17 - a49);
    let (a25, a57) = (a25 + a57, a25 - a57);
    let a41 = (a41 << 24);
    let a49 = (a49 << 48);
    let a57 = (a57 << 72);
    let (a1, a17) = (a1 + a17, a1 - a17);
    let (a9, a25) = (a9 + a25, a9 - a25);
    let a25 = (a25 << 48);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let (a17, a25) = (a17 + a25, a17 - a25);
    let (a33, a49) = (a33 + a49, a33 - a49);
    let (a41, a57) = (a41 + a57, a41 - a57);
    let a57 = (a57 << 48);
    let (a33, a41) = (a33 + a41, a33 - a41);
    let (a49, a57) = (a49 + a57, a49 - a57);
    let (a2, a34) = (a2 + a34, a2 - a34);
    let (a10, a42) = (a10 + a42, a10 - a42);
    let (a18, a50) = (a18 + a50, a18 - a50);
    let (a26, a58) = (a26 + a58, a26 - a58);
    let a42 = (a42 << 24);
    let a50 = (a50 << 48);
    let a58 = (a58 << 72);
    let (a2, a18) = (a2 + a18, a2 - a18);
    let (a10, a26) = (a10 + a26, a10 - a26);
    let a26 = (a26 << 48);
    let (a2, a10) = (a2 + a10, a2 - a10);
    let (a18, a26) = (a18 + a26, a18 - a26);
    let (a34, a50) = (a34 + a50, a34 - a50);
    let (a42, a58) = (a42 + a58, a42 - a58);
    let a58 = (a58 << 48);
    let (a34, a42) = (a34 + a42, a34 - a42);
    let (a50, a58) = (a50 + a58, a50 - a58);
    let (a3, a35) = (a3 + a35, a3 - a35);
    let (a11, a43) = (a11 + a43, a11 - a43);
    let (a19, a51) = (a19 + a51, a19 - a51);
    let (a27, a59) = (a27 + a59, a27 - a59);
    let a43 = (a43 << 24);
    let a51 = (a51 << 48);
    let a59 = (a59 << 72);
    let (a3, a19) = (a3 + a19, a3 - a19);
    let (a11, a27) = (a11 + a27, a11 - a27);
    let a27 = (a27 << 48);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let (a19, a27) = (a19 + a27, a19 - a27);
    let (a35, a51) = (a35 + a51, a35 - a51);
    let (a43, a59) = (a43 + a59, a43 - a59);
    let a59 = (a59 << 48);
    let (a35, a43) = (a35 + a43, a35 - a43);
    let (a51, a59) = (a51 + a59, a51 - a59);
    let (a4, a36) = (a4 + a36, a4 - a36);
    let (a12, a44) = (a12 + a44, a12 - a44);
    let (a20, a52) = (a20 + a52, a20 - a52);
    let (a28, a60) = (a28 + a60, a28 - a60);
    let a44 = (a44 << 24);
    let a52 = (a52 << 48);
    let a60 = (a60 << 72);
    let (a4, a20) = (a4 + a20, a4 - a20);
    let (a12, a28) = (a12 + a28, a12 - a28);
    let a28 = (a28 << 48);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let (a20, a28) = (a20 + a28, a20 - a28);
    let (a36, a52) = (a36 + a52, a36 - a52);
    let (a44, a60) = (a44 + a60, a44 - a60);
    let a60 = (a60 << 48);
    let (a36, a44) = (a36 + a44, a36 - a44);
    let (a52, a60) = (a52 + a60, a52 - a60);
    let (a5, a37) = (a5 + a37, a5 - a37);
    let (a13, a45) = (a13 + a45, a13 - a45);
    let (a21, a53) = (a21 + a53, a21 - a53);
    let (a29, a61) = (a29 + a61, a29 - a61);
    let a45 = (a45 << 24);
    let a53 = (a53 << 48);
    let a61 = (a61 << 72);
    let (a5, a21) = (a5 + a21, a5 - a21);
    let (a13, a29) = (a13 + a29, a13 - a29);
    let a29 = (a29 << 48);
    let (a5, a13) = (a5 + a13, a5 - a13);
    let (a21, a29) = (a21 + a29, a21 - a29);
    let (a37, a53) = (a37 + a53, a37 - a53);
    let (a45, a61) = (a45 + a61, a45 - a61);
    let a61 = (a61 << 48);
    let (a37, a45) = (a37 + a45, a37 - a45);
    let (a53, a61) = (a53 + a61, a53 - a61);
    let (a6, a38) = (a6 + a38, a6 - a38);
    let (a14, a46) = (a14 + a46, a14 - a46);
    let (a22, a54) = (a22 + a54, a22 - a54);
    let (a30, a62) = (a30 + a62, a30 - a62);
    let a46 = (a46 << 24);
    let a54 = (a54 << 48);
    let a62 = (a62 << 72);
    let (a6, a22) = (a6 + a22, a6 - a22);
    let (a14, a30) = (a14 + a30, a14 - a30);
    let a30 = (a30 << 48);
    let (a6, a14) = (a6 + a14, a6 - a14);
    let (a22, a30) = (a22 + a30, a22 - a30);
    let (a38, a54) = (a38 + a54, a38 - a54);
    let (a46, a62) = (a46 + a62, a46 - a62);
    let a62 = (a62 << 48);
    let (a38, a46) = (a38 + a46, a38 - a46);
    let (a54, a62) = (a54 + a62, a54 - a62);
    let (a7, a39) = (a7 + a39, a7 - a39);
    let (a15, a47) = (a15 + a47, a15 - a47);
    let (a23, a55) = (a23 + a55, a23 - a55);
    let (a31, a63) = (a31 + a63, a31 - a63);
    let a47 = (a47 << 24);
    let a55 = (a55 << 48);
    let a63 = (a63 << 72);
    let (a7, a23) = (a7 + a23, a7 - a23);
    let (a15, a31) = (a15 + a31, a15 - a31);
    let a31 = (a31 << 48);
    let (a7, a15) = (a7 + a15, a7 - a15);
    let (a23, a31) = (a23 + a31, a23 - a31);
    let (a39, a55) = (a39 + a55, a39 - a55);
    let (a47, a63) = (a47 + a63, a47 - a63);
    let a63 = (a63 << 48);
    let (a39, a47) = (a39 + a47, a39 - a47);
    let (a55, a63) = (a55 + a63, a55 - a63);
    let a33 = (a33 << 3);
    let a17 = (a17 << 6);
    let a49 = (a49 << 9);
    let a9 = (a9 << 12);
    let a41 = (a41 << 15);
    let a25 = (a25 << 18);
    let a57 = (a57 << 21);
    let a34 = (a34 << 6);
    let a18 = (a18 << 12);
    let a50 = (a50 << 18);
    let a10 = (a10 << 24);
    let a42 = (a42 << 30);
    let a26 = (a26 << 36);
    let a58 = (a58 << 42);
    let a35 = (a35 << 9);
    let a19 = (a19 << 18);
    let a51 = (a51 << 27);
    let a11 = (a11 << 36);
    let a43 = (a43 << 45);
    let a27 = (a27 << 54);
    let a59 = (a59 << 63);
    let a36 = (a36 << 12);
    let a20 = (a20 << 24);
    let a52 = (a52 << 36);
    let a12 = (a12 << 48);
    let a44 = (a44 << 60);
    let a28 = (a28 << 72);
    let a60 = (a60 << 84);
    let a37 = (a37 << 15);
    let a21 = (a21 << 30);
    let a53 = (a53 << 45);
    let a13 = (a13 << 60);
    let a45 = (a45 << 75);
    let a29 = (a29 << 90);
    let a61 = (-(a61 << 9));
    let a38 = (a38 << 18);
    let a22 = (a22 << 36);
    let a54 = (a54 << 54);
    let a14 = (a14 << 72);
    let a46 = (a46 << 90);
    let a30 = (-(a30 << 12));
    let a62 = (-(a62 << 30));
    let a39 = (a39 << 21);
    let a23 = (a23 << 42);
    let a55 = (a55 << 63);
    let a15 = (a15 << 84);
    let a47 = (-(a47 << 9));
    let a31 = (-(a31 << 30));
    let a63 = (-(a63 << 51));
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let a5 = (a5 << 24);
    let a6 = (a6 << 48);
    let a7 = (a7 << 72);
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a33, a37) = (a33 + a37, a33 - a37);
    let (a34, a38) = (a34 + a38, a34 - a38);
    let (a35, a39) = (a35 + a39, a35 - a39);
    let a37 = (a37 << 24);
    let a38 = (a38 << 48);
    let a39 = (a39 << 72);
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    let (a16, a20) = (a16 + a20, a16 - a20);
    let (a17, a21) = (a17 + a21, a17 - a21);
    let (a18, a22) = (a18 + a22, a18 - a22);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let a21 = (a21 << 24);
    let a22 = (a22 << 48);
    let a23 = (a23 << 72);
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a48, a52) = (a48 + a52, a48 - a52);
    let (a49, a53) = (a49 + a53, a49 - a53);
    let (a50, a54) = (a50 + a54, a50 - a54);
    let (a51, a55) = (a51 + a55, a51 - a55);
    let a53 = (a53 << 24);
    let a54 = (a54 << 48);
    let a55 = (a55 << 72);
    let (a48, a50) = (a48 + a50, a48 - a50);
    let (a49, a51) = (a49 + a51, a49 - a51);
    let a51 = (a51 << 48);
    let (a48, a49) = (a48 + a49, a48 - a49);
    let (a50, a51) = (a50 + a51, a50 - a51);
    let (a52, a54) = (a52 + a54, a52 - a54);
    let (a53, a55) = (a53 + a55, a53 - a55);
    let a55 = (a55 << 48);
    let (a52, a53) = (a52 + a53, a52 - a53);
    let (a54, a55) = (a54 + a55, a54 - a55);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a13 = (a13 << 24);
    let a14 = (a14 << 48);
    let a15 = (a15 << 72);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a40, a44) = (a40 + a44, a40 - a44);
    let (a41, a45) = (a41 + a45, a41 - a45);
    let (a42, a46) = (a42 + a46, a42 - a46);
    let (a43, a47) = (a43 + a47, a43 - a47);
    let a45 = (a45 << 24);
    let a46 = (a46 << 48);
    let a47 = (a47 << 72);
    let (a40, a42) = (a40 + a42, a40 - a42);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let a43 = (a43 << 48);
    let (a40, a41) = (a40 + a41, a40 - a41);
    let (a42, a43) = (a42 + a43, a42 - a43);
    let (a44, a46) = (a44 + a46, a44 - a46);
    let (a45, a47) = (a45 + a47, a45 - a47);
    let a47 = (a47 << 48);
    let (a44, a45) = (a44 + a45, a44 - a45);
    let (a46, a47) = (a46 + a47, a46 - a47);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a25, a29) = (a25 + a29, a25 - a29);
    let (a26, a30) = (a26 + a30, a26 - a30);
    let (a27, a31) = (a27 + a31, a27 - a31);
    let a29 = (a29 << 24);
    let a30 = (a30 << 48);
    let a31 = (a31 << 72);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a56, a60) = (a56 + a60, a56 - a60);
    let (a57, a61) = (a57 + a61, a57 - a61);
    let (a58, a62) = (a58 + a62, a58 - a62);
    let (a59, a63) = (a59 + a63, a59 - a63);
    let a61 = (a61 << 24);
    let a62 = (a62 << 48);
    let a63 = (a63 << 72);
    let (a56, a58) = (a56 + a58, a56 - a58);
    let (a57, a59) = (a57 + a59, a57 - a59);
    let a59 = (a59 << 48);
    let (a56, a57) = (a56 + a57, a56 - a57);
    let (a58, a59) = (a58 + a59, a58 - a59);
    let (a60, a62) = (a60 + a62, a60 - a62);
    let (a61, a63) = (a61 + a63, a61 - a63);
    let a63 = (a63 << 48);
    let (a60, a61) = (a60 + a61, a60 - a61);
    let (a62, a63) = (a62 + a63, a62 - a63);
    values[0] = a0;
    values[1] = a32;
    values[2] = a16;
    values[3] = a48;
    values[4] = a8;
    values[5] = a40;
    values[6] = a24;
    values[7] = a56;
    values[8] = a4;
    values[9] = a36;
    values[10] = a20;
    values[11] = a52;
    values[12] = a12;
    values[13] = a44;
    values[14] = a28;
    values[15] = a60;
    values[16] = a2;
    values[17] = a34;
    values[18] = a18;
    values[19] = a50;
    values[20] = a10;
    values[21] = a42;
    values[22] = a26;
    values[23] = a58;
    values[24] = a6;
    values[25] = a38;
    values[26] = a22;
    values[27] = a54;
    values[28] = a14;
    values[29] = a46;
    values[30] = a30;
    values[31] = a62;
    values[32] = a1;
    values[33] = a33;
    values[34] = a17;
    values[35] = a49;
    values[36] = a9;
    values[37] = a41;
    values[38] = a25;
    values[39] = a57;
    values[40] = a5;
    values[41] = a37;
    values[42] = a21;
    values[43] = a53;
    values[44] = a13;
    values[45] = a45;
    values[46] = a29;
    values[47] = a61;
    values[48] = a3;
    values[49] = a35;
    values[50] = a19;
    values[51] = a51;
    values[52] = a11;
    values[53] = a43;
    values[54] = a27;
    values[55] = a59;
    values[56] = a7;
    values[57] = a39;
    values[58] = a23;
    values[59] = a55;
    values[60] = a15;
    values[61] = a47;
    values[62] = a31;
    values[63] = a63;
}

/// Size 80 NTT.
fn ntt_80(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 80);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let a60 = values[60];
    let a61 = values[61];
    let a62 = values[62];
    let a63 = values[63];
    let a64 = values[64];
    let a65 = values[65];
    let a66 = values[66];
    let a67 = values[67];
    let a68 = values[68];
    let a69 = values[69];
    let a70 = values[70];
    let a71 = values[71];
    let a72 = values[72];
    let a73 = values[73];
    let a74 = values[74];
    let a75 = values[75];
    let a76 = values[76];
    let a77 = values[77];
    let a78 = values[78];
    let a79 = values[79];
    let (a0, a40) = (a0 + a40, a0 - a40);
    let (a10, a50) = (a10 + a50, a10 - a50);
    let (a20, a60) = (a20 + a60, a20 - a60);
    let (a30, a70) = (a30 + a70, a30 - a70);
    let a50 = (a50 << 24);
    let a60 = (a60 << 48);
    let a70 = (a70 << 72);
    let (a0, a20) = (a0 + a20, a0 - a20);
    let (a10, a30) = (a10 + a30, a10 - a30);
    let a30 = (a30 << 48);
    let (a0, a10) = (a0 + a10, a0 - a10);
    let (a20, a30) = (a20 + a30, a20 - a30);
    let (a40, a60) = (a40 + a60, a40 - a60);
    let (a50, a70) = (a50 + a70, a50 - a70);
    let a70 = (a70 << 48);
    let (a40, a50) = (a40 + a50, a40 - a50);
    let (a60, a70) = (a60 + a70, a60 - a70);
    let (a1, a41) = (a1 + a41, a1 - a41);
    let (a11, a51) = (a11 + a51, a11 - a51);
    let (a21, a61) = (a21 + a61, a21 - a61);
    let (a31, a71) = (a31 + a71, a31 - a71);
    let a51 = (a51 << 24);
    let a61 = (a61 << 48);
    let a71 = (a71 << 72);
    let (a1, a21) = (a1 + a21, a1 - a21);
    let (a11, a31) = (a11 + a31, a11 - a31);
    let a31 = (a31 << 48);
    let (a1, a11) = (a1 + a11, a1 - a11);
    let (a21, a31) = (a21 + a31, a21 - a31);
    let (a41, a61) = (a41 + a61, a41 - a61);
    let (a51, a71) = (a51 + a71, a51 - a71);
    let a71 = (a71 << 48);
    let (a41, a51) = (a41 + a51, a41 - a51);
    let (a61, a71) = (a61 + a71, a61 - a71);
    let (a2, a42) = (a2 + a42, a2 - a42);
    let (a12, a52) = (a12 + a52, a12 - a52);
    let (a22, a62) = (a22 + a62, a22 - a62);
    let (a32, a72) = (a32 + a72, a32 - a72);
    let a52 = (a52 << 24);
    let a62 = (a62 << 48);
    let a72 = (a72 << 72);
    let (a2, a22) = (a2 + a22, a2 - a22);
    let (a12, a32) = (a12 + a32, a12 - a32);
    let a32 = (a32 << 48);
    let (a2, a12) = (a2 + a12, a2 - a12);
    let (a22, a32) = (a22 + a32, a22 - a32);
    let (a42, a62) = (a42 + a62, a42 - a62);
    let (a52, a72) = (a52 + a72, a52 - a72);
    let a72 = (a72 << 48);
    let (a42, a52) = (a42 + a52, a42 - a52);
    let (a62, a72) = (a62 + a72, a62 - a72);
    let (a3, a43) = (a3 + a43, a3 - a43);
    let (a13, a53) = (a13 + a53, a13 - a53);
    let (a23, a63) = (a23 + a63, a23 - a63);
    let (a33, a73) = (a33 + a73, a33 - a73);
    let a53 = (a53 << 24);
    let a63 = (a63 << 48);
    let a73 = (a73 << 72);
    let (a3, a23) = (a3 + a23, a3 - a23);
    let (a13, a33) = (a13 + a33, a13 - a33);
    let a33 = (a33 << 48);
    let (a3, a13) = (a3 + a13, a3 - a13);
    let (a23, a33) = (a23 + a33, a23 - a33);
    let (a43, a63) = (a43 + a63, a43 - a63);
    let (a53, a73) = (a53 + a73, a53 - a73);
    let a73 = (a73 << 48);
    let (a43, a53) = (a43 + a53, a43 - a53);
    let (a63, a73) = (a63 + a73, a63 - a73);
    let (a4, a44) = (a4 + a44, a4 - a44);
    let (a14, a54) = (a14 + a54, a14 - a54);
    let (a24, a64) = (a24 + a64, a24 - a64);
    let (a34, a74) = (a34 + a74, a34 - a74);
    let a54 = (a54 << 24);
    let a64 = (a64 << 48);
    let a74 = (a74 << 72);
    let (a4, a24) = (a4 + a24, a4 - a24);
    let (a14, a34) = (a14 + a34, a14 - a34);
    let a34 = (a34 << 48);
    let (a4, a14) = (a4 + a14, a4 - a14);
    let (a24, a34) = (a24 + a34, a24 - a34);
    let (a44, a64) = (a44 + a64, a44 - a64);
    let (a54, a74) = (a54 + a74, a54 - a74);
    let a74 = (a74 << 48);
    let (a44, a54) = (a44 + a54, a44 - a54);
    let (a64, a74) = (a64 + a74, a64 - a74);
    let (a5, a45) = (a5 + a45, a5 - a45);
    let (a15, a55) = (a15 + a55, a15 - a55);
    let (a25, a65) = (a25 + a65, a25 - a65);
    let (a35, a75) = (a35 + a75, a35 - a75);
    let a55 = (a55 << 24);
    let a65 = (a65 << 48);
    let a75 = (a75 << 72);
    let (a5, a25) = (a5 + a25, a5 - a25);
    let (a15, a35) = (a15 + a35, a15 - a35);
    let a35 = (a35 << 48);
    let (a5, a15) = (a5 + a15, a5 - a15);
    let (a25, a35) = (a25 + a35, a25 - a35);
    let (a45, a65) = (a45 + a65, a45 - a65);
    let (a55, a75) = (a55 + a75, a55 - a75);
    let a75 = (a75 << 48);
    let (a45, a55) = (a45 + a55, a45 - a55);
    let (a65, a75) = (a65 + a75, a65 - a75);
    let (a6, a46) = (a6 + a46, a6 - a46);
    let (a16, a56) = (a16 + a56, a16 - a56);
    let (a26, a66) = (a26 + a66, a26 - a66);
    let (a36, a76) = (a36 + a76, a36 - a76);
    let a56 = (a56 << 24);
    let a66 = (a66 << 48);
    let a76 = (a76 << 72);
    let (a6, a26) = (a6 + a26, a6 - a26);
    let (a16, a36) = (a16 + a36, a16 - a36);
    let a36 = (a36 << 48);
    let (a6, a16) = (a6 + a16, a6 - a16);
    let (a26, a36) = (a26 + a36, a26 - a36);
    let (a46, a66) = (a46 + a66, a46 - a66);
    let (a56, a76) = (a56 + a76, a56 - a76);
    let a76 = (a76 << 48);
    let (a46, a56) = (a46 + a56, a46 - a56);
    let (a66, a76) = (a66 + a76, a66 - a76);
    let (a7, a47) = (a7 + a47, a7 - a47);
    let (a17, a57) = (a17 + a57, a17 - a57);
    let (a27, a67) = (a27 + a67, a27 - a67);
    let (a37, a77) = (a37 + a77, a37 - a77);
    let a57 = (a57 << 24);
    let a67 = (a67 << 48);
    let a77 = (a77 << 72);
    let (a7, a27) = (a7 + a27, a7 - a27);
    let (a17, a37) = (a17 + a37, a17 - a37);
    let a37 = (a37 << 48);
    let (a7, a17) = (a7 + a17, a7 - a17);
    let (a27, a37) = (a27 + a37, a27 - a37);
    let (a47, a67) = (a47 + a67, a47 - a67);
    let (a57, a77) = (a57 + a77, a57 - a77);
    let a77 = (a77 << 48);
    let (a47, a57) = (a47 + a57, a47 - a57);
    let (a67, a77) = (a67 + a77, a67 - a77);
    let (a8, a48) = (a8 + a48, a8 - a48);
    let (a18, a58) = (a18 + a58, a18 - a58);
    let (a28, a68) = (a28 + a68, a28 - a68);
    let (a38, a78) = (a38 + a78, a38 - a78);
    let a58 = (a58 << 24);
    let a68 = (a68 << 48);
    let a78 = (a78 << 72);
    let (a8, a28) = (a8 + a28, a8 - a28);
    let (a18, a38) = (a18 + a38, a18 - a38);
    let a38 = (a38 << 48);
    let (a8, a18) = (a8 + a18, a8 - a18);
    let (a28, a38) = (a28 + a38, a28 - a38);
    let (a48, a68) = (a48 + a68, a48 - a68);
    let (a58, a78) = (a58 + a78, a58 - a78);
    let a78 = (a78 << 48);
    let (a48, a58) = (a48 + a58, a48 - a58);
    let (a68, a78) = (a68 + a78, a68 - a78);
    let (a9, a49) = (a9 + a49, a9 - a49);
    let (a19, a59) = (a19 + a59, a19 - a59);
    let (a29, a69) = (a29 + a69, a29 - a69);
    let (a39, a79) = (a39 + a79, a39 - a79);
    let a59 = (a59 << 24);
    let a69 = (a69 << 48);
    let a79 = (a79 << 72);
    let (a9, a29) = (a9 + a29, a9 - a29);
    let (a19, a39) = (a19 + a39, a19 - a39);
    let a39 = (a39 << 48);
    let (a9, a19) = (a9 + a19, a9 - a19);
    let (a29, a39) = (a29 + a39, a29 - a39);
    let (a49, a69) = (a49 + a69, a49 - a69);
    let (a59, a79) = (a59 + a79, a59 - a79);
    let a79 = (a79 << 48);
    let (a49, a59) = (a49 + a59, a49 - a59);
    let (a69, a79) = (a69 + a79, a69 - a79);
    let a41 = a41 * Field::new(6193879297194861051);
    let a21 = a21 * Field::new(9148693690730647261);
    let a61 = a61 * Field::new(2598525327269793995);
    let a11 = a11 * Field::new(5290193119087387221);
    let a51 = (a51 << 12);
    let a31 = a31 * Field::new(5856505865097423521);
    let a71 = a71 * Field::new(7712152251710425105);
    let a42 = a42 * Field::new(9148693690730647261);
    let a22 = a22 * Field::new(5290193119087387221);
    let a62 = a62 * Field::new(5856505865097423521);
    let a12 = a12 * Field::new(18235156514275634624);
    let a52 = (a52 << 24);
    let a32 = a32 * Field::new(8149776168132872528);
    let a72 = a72 * Field::new(11331573348451128694);
    let a43 = a43 * Field::new(2598525327269793995);
    let a23 = a23 * Field::new(5856505865097423521);
    let a63 = a63 * Field::new(12153478289216064362);
    let a13 = a13 * Field::new(8149776168132872528);
    let a53 = (a53 << 36);
    let a33 = a33 * Field::new(4419751934697861046);
    let a73 = a73 * Field::new(3918829805224079129);
    let a44 = a44 * Field::new(5290193119087387221);
    let a24 = a24 * Field::new(18235156514275634624);
    let a64 = a64 * Field::new(8149776168132872528);
    let a14 = a14 * Field::new(1041288259238279555);
    let a54 = (a54 << 48);
    let a34 = a34 * Field::new(17073700798457888299);
    let a74 = a74 * Field::new(17869255328328231396);
    let a45 = (a45 << 12);
    let a25 = (a25 << 24);
    let a65 = (a65 << 36);
    let a15 = (a15 << 48);
    let a55 = (a55 << 60);
    let a35 = (a35 << 72);
    let a75 = (a75 << 84);
    let a46 = a46 * Field::new(5856505865097423521);
    let a26 = a26 * Field::new(8149776168132872528);
    let a66 = a66 * Field::new(4419751934697861046);
    let a16 = a16 * Field::new(17073700798457888299);
    let a56 = (a56 << 72);
    let a36 = a36 * Field::new(2281812832982421726);
    let a76 = a76 * Field::new(9298050378683937060);
    let a47 = a47 * Field::new(7712152251710425105);
    let a27 = a27 * Field::new(11331573348451128694);
    let a67 = a67 * Field::new(3918829805224079129);
    let a17 = a17 * Field::new(17869255328328231396);
    let a57 = (a57 << 84);
    let a37 = a37 * Field::new(9298050378683937060);
    let a77 = a77 * Field::new(6293265780198519959);
    let a48 = a48 * Field::new(18235156514275634624);
    let a28 = a28 * Field::new(1041288259238279555);
    let a68 = a68 * Field::new(17073700798457888299);
    let a18 = a18 * Field::new(15820824984080659046);
    let a58 = (-a58);
    let a38 = a38 * Field::new(211587555138949697);
    let a78 = a78 * Field::new(17405455810176304766);
    let a49 = a49 * Field::new(12153478289216064362);
    let a29 = a29 * Field::new(4419751934697861046);
    let a69 = a69 * Field::new(15685396404952554508);
    let a19 = a19 * Field::new(2281812832982421726);
    let a59 = (-(a59 << 12));
    let a39 = a39 * Field::new(7115170720963455627);
    let a79 = a79 * Field::new(11398751642682958806);
    let (a0, a5) = (a0 + a5, a0 - a5);
    let (a1, a6) = (a1 + a6, a1 - a6);
    let (a2, a7) = (a2 + a7, a2 - a7);
    let (a3, a8) = (a3 + a8, a3 - a8);
    let (a4, a9) = (a4 + a9, a4 - a9);
    let a6 = a6 * Field::new(18235156514275634624);
    let a7 = a7 * Field::new(1041288259238279555);
    let a8 = a8 * Field::new(17073700798457888299);
    let a9 = a9 * Field::new(15820824984080659046);
    let (a0, a1, a2, a3, a4) = (
        a0 + a1 + a2 + a3 + a4,
        a0 + a1 * R51 + a2 * R52 + a3 * R53 + a4 * R54,
        a0 + a1 * R52 + a2 * R54 + a3 * R51 + a4 * R53,
        a0 + a1 * R53 + a2 * R51 + a3 * R54 + a4 * R52,
        a0 + a1 * R54 + a2 * R53 + a3 * R52 + a4 * R51,
    );
    let (a5, a6, a7, a8, a9) = (
        a5 + a6 + a7 + a8 + a9,
        a5 + a6 * R51 + a7 * R52 + a8 * R53 + a9 * R54,
        a5 + a6 * R52 + a7 * R54 + a8 * R51 + a9 * R53,
        a5 + a6 * R53 + a7 * R51 + a8 * R54 + a9 * R52,
        a5 + a6 * R54 + a7 * R53 + a8 * R52 + a9 * R51,
    );
    let (a40, a45) = (a40 + a45, a40 - a45);
    let (a41, a46) = (a41 + a46, a41 - a46);
    let (a42, a47) = (a42 + a47, a42 - a47);
    let (a43, a48) = (a43 + a48, a43 - a48);
    let (a44, a49) = (a44 + a49, a44 - a49);
    let a46 = a46 * Field::new(18235156514275634624);
    let a47 = a47 * Field::new(1041288259238279555);
    let a48 = a48 * Field::new(17073700798457888299);
    let a49 = a49 * Field::new(15820824984080659046);
    let (a40, a41, a42, a43, a44) = (
        a40 + a41 + a42 + a43 + a44,
        a40 + a41 * R51 + a42 * R52 + a43 * R53 + a44 * R54,
        a40 + a41 * R52 + a42 * R54 + a43 * R51 + a44 * R53,
        a40 + a41 * R53 + a42 * R51 + a43 * R54 + a44 * R52,
        a40 + a41 * R54 + a42 * R53 + a43 * R52 + a44 * R51,
    );
    let (a45, a46, a47, a48, a49) = (
        a45 + a46 + a47 + a48 + a49,
        a45 + a46 * R51 + a47 * R52 + a48 * R53 + a49 * R54,
        a45 + a46 * R52 + a47 * R54 + a48 * R51 + a49 * R53,
        a45 + a46 * R53 + a47 * R51 + a48 * R54 + a49 * R52,
        a45 + a46 * R54 + a47 * R53 + a48 * R52 + a49 * R51,
    );
    let (a20, a25) = (a20 + a25, a20 - a25);
    let (a21, a26) = (a21 + a26, a21 - a26);
    let (a22, a27) = (a22 + a27, a22 - a27);
    let (a23, a28) = (a23 + a28, a23 - a28);
    let (a24, a29) = (a24 + a29, a24 - a29);
    let a26 = a26 * Field::new(18235156514275634624);
    let a27 = a27 * Field::new(1041288259238279555);
    let a28 = a28 * Field::new(17073700798457888299);
    let a29 = a29 * Field::new(15820824984080659046);
    let (a20, a21, a22, a23, a24) = (
        a20 + a21 + a22 + a23 + a24,
        a20 + a21 * R51 + a22 * R52 + a23 * R53 + a24 * R54,
        a20 + a21 * R52 + a22 * R54 + a23 * R51 + a24 * R53,
        a20 + a21 * R53 + a22 * R51 + a23 * R54 + a24 * R52,
        a20 + a21 * R54 + a22 * R53 + a23 * R52 + a24 * R51,
    );
    let (a25, a26, a27, a28, a29) = (
        a25 + a26 + a27 + a28 + a29,
        a25 + a26 * R51 + a27 * R52 + a28 * R53 + a29 * R54,
        a25 + a26 * R52 + a27 * R54 + a28 * R51 + a29 * R53,
        a25 + a26 * R53 + a27 * R51 + a28 * R54 + a29 * R52,
        a25 + a26 * R54 + a27 * R53 + a28 * R52 + a29 * R51,
    );
    let (a60, a65) = (a60 + a65, a60 - a65);
    let (a61, a66) = (a61 + a66, a61 - a66);
    let (a62, a67) = (a62 + a67, a62 - a67);
    let (a63, a68) = (a63 + a68, a63 - a68);
    let (a64, a69) = (a64 + a69, a64 - a69);
    let a66 = a66 * Field::new(18235156514275634624);
    let a67 = a67 * Field::new(1041288259238279555);
    let a68 = a68 * Field::new(17073700798457888299);
    let a69 = a69 * Field::new(15820824984080659046);
    let (a60, a61, a62, a63, a64) = (
        a60 + a61 + a62 + a63 + a64,
        a60 + a61 * R51 + a62 * R52 + a63 * R53 + a64 * R54,
        a60 + a61 * R52 + a62 * R54 + a63 * R51 + a64 * R53,
        a60 + a61 * R53 + a62 * R51 + a63 * R54 + a64 * R52,
        a60 + a61 * R54 + a62 * R53 + a63 * R52 + a64 * R51,
    );
    let (a65, a66, a67, a68, a69) = (
        a65 + a66 + a67 + a68 + a69,
        a65 + a66 * R51 + a67 * R52 + a68 * R53 + a69 * R54,
        a65 + a66 * R52 + a67 * R54 + a68 * R51 + a69 * R53,
        a65 + a66 * R53 + a67 * R51 + a68 * R54 + a69 * R52,
        a65 + a66 * R54 + a67 * R53 + a68 * R52 + a69 * R51,
    );
    let (a10, a15) = (a10 + a15, a10 - a15);
    let (a11, a16) = (a11 + a16, a11 - a16);
    let (a12, a17) = (a12 + a17, a12 - a17);
    let (a13, a18) = (a13 + a18, a13 - a18);
    let (a14, a19) = (a14 + a19, a14 - a19);
    let a16 = a16 * Field::new(18235156514275634624);
    let a17 = a17 * Field::new(1041288259238279555);
    let a18 = a18 * Field::new(17073700798457888299);
    let a19 = a19 * Field::new(15820824984080659046);
    let (a10, a11, a12, a13, a14) = (
        a10 + a11 + a12 + a13 + a14,
        a10 + a11 * R51 + a12 * R52 + a13 * R53 + a14 * R54,
        a10 + a11 * R52 + a12 * R54 + a13 * R51 + a14 * R53,
        a10 + a11 * R53 + a12 * R51 + a13 * R54 + a14 * R52,
        a10 + a11 * R54 + a12 * R53 + a13 * R52 + a14 * R51,
    );
    let (a15, a16, a17, a18, a19) = (
        a15 + a16 + a17 + a18 + a19,
        a15 + a16 * R51 + a17 * R52 + a18 * R53 + a19 * R54,
        a15 + a16 * R52 + a17 * R54 + a18 * R51 + a19 * R53,
        a15 + a16 * R53 + a17 * R51 + a18 * R54 + a19 * R52,
        a15 + a16 * R54 + a17 * R53 + a18 * R52 + a19 * R51,
    );
    let (a50, a55) = (a50 + a55, a50 - a55);
    let (a51, a56) = (a51 + a56, a51 - a56);
    let (a52, a57) = (a52 + a57, a52 - a57);
    let (a53, a58) = (a53 + a58, a53 - a58);
    let (a54, a59) = (a54 + a59, a54 - a59);
    let a56 = a56 * Field::new(18235156514275634624);
    let a57 = a57 * Field::new(1041288259238279555);
    let a58 = a58 * Field::new(17073700798457888299);
    let a59 = a59 * Field::new(15820824984080659046);
    let (a50, a51, a52, a53, a54) = (
        a50 + a51 + a52 + a53 + a54,
        a50 + a51 * R51 + a52 * R52 + a53 * R53 + a54 * R54,
        a50 + a51 * R52 + a52 * R54 + a53 * R51 + a54 * R53,
        a50 + a51 * R53 + a52 * R51 + a53 * R54 + a54 * R52,
        a50 + a51 * R54 + a52 * R53 + a53 * R52 + a54 * R51,
    );
    let (a55, a56, a57, a58, a59) = (
        a55 + a56 + a57 + a58 + a59,
        a55 + a56 * R51 + a57 * R52 + a58 * R53 + a59 * R54,
        a55 + a56 * R52 + a57 * R54 + a58 * R51 + a59 * R53,
        a55 + a56 * R53 + a57 * R51 + a58 * R54 + a59 * R52,
        a55 + a56 * R54 + a57 * R53 + a58 * R52 + a59 * R51,
    );
    let (a30, a35) = (a30 + a35, a30 - a35);
    let (a31, a36) = (a31 + a36, a31 - a36);
    let (a32, a37) = (a32 + a37, a32 - a37);
    let (a33, a38) = (a33 + a38, a33 - a38);
    let (a34, a39) = (a34 + a39, a34 - a39);
    let a36 = a36 * Field::new(18235156514275634624);
    let a37 = a37 * Field::new(1041288259238279555);
    let a38 = a38 * Field::new(17073700798457888299);
    let a39 = a39 * Field::new(15820824984080659046);
    let (a30, a31, a32, a33, a34) = (
        a30 + a31 + a32 + a33 + a34,
        a30 + a31 * R51 + a32 * R52 + a33 * R53 + a34 * R54,
        a30 + a31 * R52 + a32 * R54 + a33 * R51 + a34 * R53,
        a30 + a31 * R53 + a32 * R51 + a33 * R54 + a34 * R52,
        a30 + a31 * R54 + a32 * R53 + a33 * R52 + a34 * R51,
    );
    let (a35, a36, a37, a38, a39) = (
        a35 + a36 + a37 + a38 + a39,
        a35 + a36 * R51 + a37 * R52 + a38 * R53 + a39 * R54,
        a35 + a36 * R52 + a37 * R54 + a38 * R51 + a39 * R53,
        a35 + a36 * R53 + a37 * R51 + a38 * R54 + a39 * R52,
        a35 + a36 * R54 + a37 * R53 + a38 * R52 + a39 * R51,
    );
    let (a70, a75) = (a70 + a75, a70 - a75);
    let (a71, a76) = (a71 + a76, a71 - a76);
    let (a72, a77) = (a72 + a77, a72 - a77);
    let (a73, a78) = (a73 + a78, a73 - a78);
    let (a74, a79) = (a74 + a79, a74 - a79);
    let a76 = a76 * Field::new(18235156514275634624);
    let a77 = a77 * Field::new(1041288259238279555);
    let a78 = a78 * Field::new(17073700798457888299);
    let a79 = a79 * Field::new(15820824984080659046);
    let (a70, a71, a72, a73, a74) = (
        a70 + a71 + a72 + a73 + a74,
        a70 + a71 * R51 + a72 * R52 + a73 * R53 + a74 * R54,
        a70 + a71 * R52 + a72 * R54 + a73 * R51 + a74 * R53,
        a70 + a71 * R53 + a72 * R51 + a73 * R54 + a74 * R52,
        a70 + a71 * R54 + a72 * R53 + a73 * R52 + a74 * R51,
    );
    let (a75, a76, a77, a78, a79) = (
        a75 + a76 + a77 + a78 + a79,
        a75 + a76 * R51 + a77 * R52 + a78 * R53 + a79 * R54,
        a75 + a76 * R52 + a77 * R54 + a78 * R51 + a79 * R53,
        a75 + a76 * R53 + a77 * R51 + a78 * R54 + a79 * R52,
        a75 + a76 * R54 + a77 * R53 + a78 * R52 + a79 * R51,
    );
    values[0] = a0;
    values[1] = a40;
    values[2] = a20;
    values[3] = a60;
    values[4] = a10;
    values[5] = a50;
    values[6] = a30;
    values[7] = a70;
    values[8] = a5;
    values[9] = a45;
    values[10] = a25;
    values[11] = a65;
    values[12] = a15;
    values[13] = a55;
    values[14] = a35;
    values[15] = a75;
    values[16] = a1;
    values[17] = a41;
    values[18] = a21;
    values[19] = a61;
    values[20] = a11;
    values[21] = a51;
    values[22] = a31;
    values[23] = a71;
    values[24] = a6;
    values[25] = a46;
    values[26] = a26;
    values[27] = a66;
    values[28] = a16;
    values[29] = a56;
    values[30] = a36;
    values[31] = a76;
    values[32] = a2;
    values[33] = a42;
    values[34] = a22;
    values[35] = a62;
    values[36] = a12;
    values[37] = a52;
    values[38] = a32;
    values[39] = a72;
    values[40] = a7;
    values[41] = a47;
    values[42] = a27;
    values[43] = a67;
    values[44] = a17;
    values[45] = a57;
    values[46] = a37;
    values[47] = a77;
    values[48] = a3;
    values[49] = a43;
    values[50] = a23;
    values[51] = a63;
    values[52] = a13;
    values[53] = a53;
    values[54] = a33;
    values[55] = a73;
    values[56] = a8;
    values[57] = a48;
    values[58] = a28;
    values[59] = a68;
    values[60] = a18;
    values[61] = a58;
    values[62] = a38;
    values[63] = a78;
    values[64] = a4;
    values[65] = a44;
    values[66] = a24;
    values[67] = a64;
    values[68] = a14;
    values[69] = a54;
    values[70] = a34;
    values[71] = a74;
    values[72] = a9;
    values[73] = a49;
    values[74] = a29;
    values[75] = a69;
    values[76] = a19;
    values[77] = a59;
    values[78] = a39;
    values[79] = a79;
}

/// Size 96 NTT.
fn ntt_96(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 96);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let a60 = values[60];
    let a61 = values[61];
    let a62 = values[62];
    let a63 = values[63];
    let a64 = values[64];
    let a65 = values[65];
    let a66 = values[66];
    let a67 = values[67];
    let a68 = values[68];
    let a69 = values[69];
    let a70 = values[70];
    let a71 = values[71];
    let a72 = values[72];
    let a73 = values[73];
    let a74 = values[74];
    let a75 = values[75];
    let a76 = values[76];
    let a77 = values[77];
    let a78 = values[78];
    let a79 = values[79];
    let a80 = values[80];
    let a81 = values[81];
    let a82 = values[82];
    let a83 = values[83];
    let a84 = values[84];
    let a85 = values[85];
    let a86 = values[86];
    let a87 = values[87];
    let a88 = values[88];
    let a89 = values[89];
    let a90 = values[90];
    let a91 = values[91];
    let a92 = values[92];
    let a93 = values[93];
    let a94 = values[94];
    let a95 = values[95];
    let (a0, a48) = (a0 + a48, a0 - a48);
    let (a12, a60) = (a12 + a60, a12 - a60);
    let (a24, a72) = (a24 + a72, a24 - a72);
    let (a36, a84) = (a36 + a84, a36 - a84);
    let a60 = (a60 << 24);
    let a72 = (a72 << 48);
    let a84 = (a84 << 72);
    let (a0, a24) = (a0 + a24, a0 - a24);
    let (a12, a36) = (a12 + a36, a12 - a36);
    let a36 = (a36 << 48);
    let (a0, a12) = (a0 + a12, a0 - a12);
    let (a24, a36) = (a24 + a36, a24 - a36);
    let (a48, a72) = (a48 + a72, a48 - a72);
    let (a60, a84) = (a60 + a84, a60 - a84);
    let a84 = (a84 << 48);
    let (a48, a60) = (a48 + a60, a48 - a60);
    let (a72, a84) = (a72 + a84, a72 - a84);
    let (a1, a49) = (a1 + a49, a1 - a49);
    let (a13, a61) = (a13 + a61, a13 - a61);
    let (a25, a73) = (a25 + a73, a25 - a73);
    let (a37, a85) = (a37 + a85, a37 - a85);
    let a61 = (a61 << 24);
    let a73 = (a73 << 48);
    let a85 = (a85 << 72);
    let (a1, a25) = (a1 + a25, a1 - a25);
    let (a13, a37) = (a13 + a37, a13 - a37);
    let a37 = (a37 << 48);
    let (a1, a13) = (a1 + a13, a1 - a13);
    let (a25, a37) = (a25 + a37, a25 - a37);
    let (a49, a73) = (a49 + a73, a49 - a73);
    let (a61, a85) = (a61 + a85, a61 - a85);
    let a85 = (a85 << 48);
    let (a49, a61) = (a49 + a61, a49 - a61);
    let (a73, a85) = (a73 + a85, a73 - a85);
    let (a2, a50) = (a2 + a50, a2 - a50);
    let (a14, a62) = (a14 + a62, a14 - a62);
    let (a26, a74) = (a26 + a74, a26 - a74);
    let (a38, a86) = (a38 + a86, a38 - a86);
    let a62 = (a62 << 24);
    let a74 = (a74 << 48);
    let a86 = (a86 << 72);
    let (a2, a26) = (a2 + a26, a2 - a26);
    let (a14, a38) = (a14 + a38, a14 - a38);
    let a38 = (a38 << 48);
    let (a2, a14) = (a2 + a14, a2 - a14);
    let (a26, a38) = (a26 + a38, a26 - a38);
    let (a50, a74) = (a50 + a74, a50 - a74);
    let (a62, a86) = (a62 + a86, a62 - a86);
    let a86 = (a86 << 48);
    let (a50, a62) = (a50 + a62, a50 - a62);
    let (a74, a86) = (a74 + a86, a74 - a86);
    let (a3, a51) = (a3 + a51, a3 - a51);
    let (a15, a63) = (a15 + a63, a15 - a63);
    let (a27, a75) = (a27 + a75, a27 - a75);
    let (a39, a87) = (a39 + a87, a39 - a87);
    let a63 = (a63 << 24);
    let a75 = (a75 << 48);
    let a87 = (a87 << 72);
    let (a3, a27) = (a3 + a27, a3 - a27);
    let (a15, a39) = (a15 + a39, a15 - a39);
    let a39 = (a39 << 48);
    let (a3, a15) = (a3 + a15, a3 - a15);
    let (a27, a39) = (a27 + a39, a27 - a39);
    let (a51, a75) = (a51 + a75, a51 - a75);
    let (a63, a87) = (a63 + a87, a63 - a87);
    let a87 = (a87 << 48);
    let (a51, a63) = (a51 + a63, a51 - a63);
    let (a75, a87) = (a75 + a87, a75 - a87);
    let (a4, a52) = (a4 + a52, a4 - a52);
    let (a16, a64) = (a16 + a64, a16 - a64);
    let (a28, a76) = (a28 + a76, a28 - a76);
    let (a40, a88) = (a40 + a88, a40 - a88);
    let a64 = (a64 << 24);
    let a76 = (a76 << 48);
    let a88 = (a88 << 72);
    let (a4, a28) = (a4 + a28, a4 - a28);
    let (a16, a40) = (a16 + a40, a16 - a40);
    let a40 = (a40 << 48);
    let (a4, a16) = (a4 + a16, a4 - a16);
    let (a28, a40) = (a28 + a40, a28 - a40);
    let (a52, a76) = (a52 + a76, a52 - a76);
    let (a64, a88) = (a64 + a88, a64 - a88);
    let a88 = (a88 << 48);
    let (a52, a64) = (a52 + a64, a52 - a64);
    let (a76, a88) = (a76 + a88, a76 - a88);
    let (a5, a53) = (a5 + a53, a5 - a53);
    let (a17, a65) = (a17 + a65, a17 - a65);
    let (a29, a77) = (a29 + a77, a29 - a77);
    let (a41, a89) = (a41 + a89, a41 - a89);
    let a65 = (a65 << 24);
    let a77 = (a77 << 48);
    let a89 = (a89 << 72);
    let (a5, a29) = (a5 + a29, a5 - a29);
    let (a17, a41) = (a17 + a41, a17 - a41);
    let a41 = (a41 << 48);
    let (a5, a17) = (a5 + a17, a5 - a17);
    let (a29, a41) = (a29 + a41, a29 - a41);
    let (a53, a77) = (a53 + a77, a53 - a77);
    let (a65, a89) = (a65 + a89, a65 - a89);
    let a89 = (a89 << 48);
    let (a53, a65) = (a53 + a65, a53 - a65);
    let (a77, a89) = (a77 + a89, a77 - a89);
    let (a6, a54) = (a6 + a54, a6 - a54);
    let (a18, a66) = (a18 + a66, a18 - a66);
    let (a30, a78) = (a30 + a78, a30 - a78);
    let (a42, a90) = (a42 + a90, a42 - a90);
    let a66 = (a66 << 24);
    let a78 = (a78 << 48);
    let a90 = (a90 << 72);
    let (a6, a30) = (a6 + a30, a6 - a30);
    let (a18, a42) = (a18 + a42, a18 - a42);
    let a42 = (a42 << 48);
    let (a6, a18) = (a6 + a18, a6 - a18);
    let (a30, a42) = (a30 + a42, a30 - a42);
    let (a54, a78) = (a54 + a78, a54 - a78);
    let (a66, a90) = (a66 + a90, a66 - a90);
    let a90 = (a90 << 48);
    let (a54, a66) = (a54 + a66, a54 - a66);
    let (a78, a90) = (a78 + a90, a78 - a90);
    let (a7, a55) = (a7 + a55, a7 - a55);
    let (a19, a67) = (a19 + a67, a19 - a67);
    let (a31, a79) = (a31 + a79, a31 - a79);
    let (a43, a91) = (a43 + a91, a43 - a91);
    let a67 = (a67 << 24);
    let a79 = (a79 << 48);
    let a91 = (a91 << 72);
    let (a7, a31) = (a7 + a31, a7 - a31);
    let (a19, a43) = (a19 + a43, a19 - a43);
    let a43 = (a43 << 48);
    let (a7, a19) = (a7 + a19, a7 - a19);
    let (a31, a43) = (a31 + a43, a31 - a43);
    let (a55, a79) = (a55 + a79, a55 - a79);
    let (a67, a91) = (a67 + a91, a67 - a91);
    let a91 = (a91 << 48);
    let (a55, a67) = (a55 + a67, a55 - a67);
    let (a79, a91) = (a79 + a91, a79 - a91);
    let (a8, a56) = (a8 + a56, a8 - a56);
    let (a20, a68) = (a20 + a68, a20 - a68);
    let (a32, a80) = (a32 + a80, a32 - a80);
    let (a44, a92) = (a44 + a92, a44 - a92);
    let a68 = (a68 << 24);
    let a80 = (a80 << 48);
    let a92 = (a92 << 72);
    let (a8, a32) = (a8 + a32, a8 - a32);
    let (a20, a44) = (a20 + a44, a20 - a44);
    let a44 = (a44 << 48);
    let (a8, a20) = (a8 + a20, a8 - a20);
    let (a32, a44) = (a32 + a44, a32 - a44);
    let (a56, a80) = (a56 + a80, a56 - a80);
    let (a68, a92) = (a68 + a92, a68 - a92);
    let a92 = (a92 << 48);
    let (a56, a68) = (a56 + a68, a56 - a68);
    let (a80, a92) = (a80 + a92, a80 - a92);
    let (a9, a57) = (a9 + a57, a9 - a57);
    let (a21, a69) = (a21 + a69, a21 - a69);
    let (a33, a81) = (a33 + a81, a33 - a81);
    let (a45, a93) = (a45 + a93, a45 - a93);
    let a69 = (a69 << 24);
    let a81 = (a81 << 48);
    let a93 = (a93 << 72);
    let (a9, a33) = (a9 + a33, a9 - a33);
    let (a21, a45) = (a21 + a45, a21 - a45);
    let a45 = (a45 << 48);
    let (a9, a21) = (a9 + a21, a9 - a21);
    let (a33, a45) = (a33 + a45, a33 - a45);
    let (a57, a81) = (a57 + a81, a57 - a81);
    let (a69, a93) = (a69 + a93, a69 - a93);
    let a93 = (a93 << 48);
    let (a57, a69) = (a57 + a69, a57 - a69);
    let (a81, a93) = (a81 + a93, a81 - a93);
    let (a10, a58) = (a10 + a58, a10 - a58);
    let (a22, a70) = (a22 + a70, a22 - a70);
    let (a34, a82) = (a34 + a82, a34 - a82);
    let (a46, a94) = (a46 + a94, a46 - a94);
    let a70 = (a70 << 24);
    let a82 = (a82 << 48);
    let a94 = (a94 << 72);
    let (a10, a34) = (a10 + a34, a10 - a34);
    let (a22, a46) = (a22 + a46, a22 - a46);
    let a46 = (a46 << 48);
    let (a10, a22) = (a10 + a22, a10 - a22);
    let (a34, a46) = (a34 + a46, a34 - a46);
    let (a58, a82) = (a58 + a82, a58 - a82);
    let (a70, a94) = (a70 + a94, a70 - a94);
    let a94 = (a94 << 48);
    let (a58, a70) = (a58 + a70, a58 - a70);
    let (a82, a94) = (a82 + a94, a82 - a94);
    let (a11, a59) = (a11 + a59, a11 - a59);
    let (a23, a71) = (a23 + a71, a23 - a71);
    let (a35, a83) = (a35 + a83, a35 - a83);
    let (a47, a95) = (a47 + a95, a47 - a95);
    let a71 = (a71 << 24);
    let a83 = (a83 << 48);
    let a95 = (a95 << 72);
    let (a11, a35) = (a11 + a35, a11 - a35);
    let (a23, a47) = (a23 + a47, a23 - a47);
    let a47 = (a47 << 48);
    let (a11, a23) = (a11 + a23, a11 - a23);
    let (a35, a47) = (a35 + a47, a35 - a47);
    let (a59, a83) = (a59 + a83, a59 - a83);
    let (a71, a95) = (a71 + a95, a71 - a95);
    let a95 = (a95 << 48);
    let (a59, a71) = (a59 + a71, a59 - a71);
    let (a83, a95) = (a83 + a95, a83 - a95);
    let a49 = (a49 << 2);
    let a25 = (a25 << 4);
    let a73 = (a73 << 6);
    let a13 = (a13 << 8);
    let a61 = (a61 << 10);
    let a37 = (a37 << 12);
    let a85 = (a85 << 14);
    let a50 = (a50 << 4);
    let a26 = (a26 << 8);
    let a74 = (a74 << 12);
    let a14 = (a14 << 16);
    let a62 = (a62 << 20);
    let a38 = (a38 << 24);
    let a86 = (a86 << 28);
    let a51 = (a51 << 6);
    let a27 = (a27 << 12);
    let a75 = (a75 << 18);
    let a15 = (a15 << 24);
    let a63 = (a63 << 30);
    let a39 = (a39 << 36);
    let a87 = (a87 << 42);
    let a52 = (a52 << 8);
    let a28 = (a28 << 16);
    let a76 = (a76 << 24);
    let a16 = (a16 << 32);
    let a64 = (a64 << 40);
    let a40 = (a40 << 48);
    let a88 = (a88 << 56);
    let a53 = (a53 << 10);
    let a29 = (a29 << 20);
    let a77 = (a77 << 30);
    let a17 = (a17 << 40);
    let a65 = (a65 << 50);
    let a41 = (a41 << 60);
    let a89 = (a89 << 70);
    let a54 = (a54 << 12);
    let a30 = (a30 << 24);
    let a78 = (a78 << 36);
    let a18 = (a18 << 48);
    let a66 = (a66 << 60);
    let a42 = (a42 << 72);
    let a90 = (a90 << 84);
    let a55 = (a55 << 14);
    let a31 = (a31 << 28);
    let a79 = (a79 << 42);
    let a19 = (a19 << 56);
    let a67 = (a67 << 70);
    let a43 = (a43 << 84);
    let a91 = (-(a91 << 2));
    let a56 = (a56 << 16);
    let a32 = (a32 << 32);
    let a80 = (a80 << 48);
    let a20 = (a20 << 64);
    let a68 = (a68 << 80);
    let a44 = (-a44);
    let a92 = (-(a92 << 16));
    let a57 = (a57 << 18);
    let a33 = (a33 << 36);
    let a81 = (a81 << 54);
    let a21 = (a21 << 72);
    let a69 = (a69 << 90);
    let a45 = (-(a45 << 12));
    let a93 = (-(a93 << 30));
    let a58 = (a58 << 20);
    let a34 = (a34 << 40);
    let a82 = (a82 << 60);
    let a22 = (a22 << 80);
    let a70 = (-(a70 << 4));
    let a46 = (-(a46 << 24));
    let a94 = (-(a94 << 44));
    let a59 = (a59 << 22);
    let a35 = (a35 << 44);
    let a83 = (a83 << 66);
    let a23 = (a23 << 88);
    let a71 = (-(a71 << 14));
    let a47 = (-(a47 << 36));
    let a95 = (-(a95 << 58));
    let (a0, a4, a8) = (
        a0 + a4 + a8,
        a0 + (a4 << 64) - (a8 << 32),
        a0 - (a4 << 32) + (a8 << 64),
    );
    let (a1, a5, a9) = (
        a1 + a5 + a9,
        a1 + (a5 << 64) - (a9 << 32),
        a1 - (a5 << 32) + (a9 << 64),
    );
    let (a2, a6, a10) = (
        a2 + a6 + a10,
        a2 + (a6 << 64) - (a10 << 32),
        a2 - (a6 << 32) + (a10 << 64),
    );
    let (a3, a7, a11) = (
        a3 + a7 + a11,
        a3 + (a7 << 64) - (a11 << 32),
        a3 - (a7 << 32) + (a11 << 64),
    );
    let a5 = (a5 << 16);
    let a9 = (a9 << 32);
    let a6 = (a6 << 32);
    let a10 = (a10 << 64);
    let a7 = (a7 << 48);
    let a11 = (-a11);
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a48, a52, a56) = (
        a48 + a52 + a56,
        a48 + (a52 << 64) - (a56 << 32),
        a48 - (a52 << 32) + (a56 << 64),
    );
    let (a49, a53, a57) = (
        a49 + a53 + a57,
        a49 + (a53 << 64) - (a57 << 32),
        a49 - (a53 << 32) + (a57 << 64),
    );
    let (a50, a54, a58) = (
        a50 + a54 + a58,
        a50 + (a54 << 64) - (a58 << 32),
        a50 - (a54 << 32) + (a58 << 64),
    );
    let (a51, a55, a59) = (
        a51 + a55 + a59,
        a51 + (a55 << 64) - (a59 << 32),
        a51 - (a55 << 32) + (a59 << 64),
    );
    let a53 = (a53 << 16);
    let a57 = (a57 << 32);
    let a54 = (a54 << 32);
    let a58 = (a58 << 64);
    let a55 = (a55 << 48);
    let a59 = (-a59);
    let (a48, a50) = (a48 + a50, a48 - a50);
    let (a49, a51) = (a49 + a51, a49 - a51);
    let a51 = (a51 << 48);
    let (a48, a49) = (a48 + a49, a48 - a49);
    let (a50, a51) = (a50 + a51, a50 - a51);
    let (a52, a54) = (a52 + a54, a52 - a54);
    let (a53, a55) = (a53 + a55, a53 - a55);
    let a55 = (a55 << 48);
    let (a52, a53) = (a52 + a53, a52 - a53);
    let (a54, a55) = (a54 + a55, a54 - a55);
    let (a56, a58) = (a56 + a58, a56 - a58);
    let (a57, a59) = (a57 + a59, a57 - a59);
    let a59 = (a59 << 48);
    let (a56, a57) = (a56 + a57, a56 - a57);
    let (a58, a59) = (a58 + a59, a58 - a59);
    let (a24, a28, a32) = (
        a24 + a28 + a32,
        a24 + (a28 << 64) - (a32 << 32),
        a24 - (a28 << 32) + (a32 << 64),
    );
    let (a25, a29, a33) = (
        a25 + a29 + a33,
        a25 + (a29 << 64) - (a33 << 32),
        a25 - (a29 << 32) + (a33 << 64),
    );
    let (a26, a30, a34) = (
        a26 + a30 + a34,
        a26 + (a30 << 64) - (a34 << 32),
        a26 - (a30 << 32) + (a34 << 64),
    );
    let (a27, a31, a35) = (
        a27 + a31 + a35,
        a27 + (a31 << 64) - (a35 << 32),
        a27 - (a31 << 32) + (a35 << 64),
    );
    let a29 = (a29 << 16);
    let a33 = (a33 << 32);
    let a30 = (a30 << 32);
    let a34 = (a34 << 64);
    let a31 = (a31 << 48);
    let a35 = (-a35);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a72, a76, a80) = (
        a72 + a76 + a80,
        a72 + (a76 << 64) - (a80 << 32),
        a72 - (a76 << 32) + (a80 << 64),
    );
    let (a73, a77, a81) = (
        a73 + a77 + a81,
        a73 + (a77 << 64) - (a81 << 32),
        a73 - (a77 << 32) + (a81 << 64),
    );
    let (a74, a78, a82) = (
        a74 + a78 + a82,
        a74 + (a78 << 64) - (a82 << 32),
        a74 - (a78 << 32) + (a82 << 64),
    );
    let (a75, a79, a83) = (
        a75 + a79 + a83,
        a75 + (a79 << 64) - (a83 << 32),
        a75 - (a79 << 32) + (a83 << 64),
    );
    let a77 = (a77 << 16);
    let a81 = (a81 << 32);
    let a78 = (a78 << 32);
    let a82 = (a82 << 64);
    let a79 = (a79 << 48);
    let a83 = (-a83);
    let (a72, a74) = (a72 + a74, a72 - a74);
    let (a73, a75) = (a73 + a75, a73 - a75);
    let a75 = (a75 << 48);
    let (a72, a73) = (a72 + a73, a72 - a73);
    let (a74, a75) = (a74 + a75, a74 - a75);
    let (a76, a78) = (a76 + a78, a76 - a78);
    let (a77, a79) = (a77 + a79, a77 - a79);
    let a79 = (a79 << 48);
    let (a76, a77) = (a76 + a77, a76 - a77);
    let (a78, a79) = (a78 + a79, a78 - a79);
    let (a80, a82) = (a80 + a82, a80 - a82);
    let (a81, a83) = (a81 + a83, a81 - a83);
    let a83 = (a83 << 48);
    let (a80, a81) = (a80 + a81, a80 - a81);
    let (a82, a83) = (a82 + a83, a82 - a83);
    let (a12, a16, a20) = (
        a12 + a16 + a20,
        a12 + (a16 << 64) - (a20 << 32),
        a12 - (a16 << 32) + (a20 << 64),
    );
    let (a13, a17, a21) = (
        a13 + a17 + a21,
        a13 + (a17 << 64) - (a21 << 32),
        a13 - (a17 << 32) + (a21 << 64),
    );
    let (a14, a18, a22) = (
        a14 + a18 + a22,
        a14 + (a18 << 64) - (a22 << 32),
        a14 - (a18 << 32) + (a22 << 64),
    );
    let (a15, a19, a23) = (
        a15 + a19 + a23,
        a15 + (a19 << 64) - (a23 << 32),
        a15 - (a19 << 32) + (a23 << 64),
    );
    let a17 = (a17 << 16);
    let a21 = (a21 << 32);
    let a18 = (a18 << 32);
    let a22 = (a22 << 64);
    let a19 = (a19 << 48);
    let a23 = (-a23);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a60, a64, a68) = (
        a60 + a64 + a68,
        a60 + (a64 << 64) - (a68 << 32),
        a60 - (a64 << 32) + (a68 << 64),
    );
    let (a61, a65, a69) = (
        a61 + a65 + a69,
        a61 + (a65 << 64) - (a69 << 32),
        a61 - (a65 << 32) + (a69 << 64),
    );
    let (a62, a66, a70) = (
        a62 + a66 + a70,
        a62 + (a66 << 64) - (a70 << 32),
        a62 - (a66 << 32) + (a70 << 64),
    );
    let (a63, a67, a71) = (
        a63 + a67 + a71,
        a63 + (a67 << 64) - (a71 << 32),
        a63 - (a67 << 32) + (a71 << 64),
    );
    let a65 = (a65 << 16);
    let a69 = (a69 << 32);
    let a66 = (a66 << 32);
    let a70 = (a70 << 64);
    let a67 = (a67 << 48);
    let a71 = (-a71);
    let (a60, a62) = (a60 + a62, a60 - a62);
    let (a61, a63) = (a61 + a63, a61 - a63);
    let a63 = (a63 << 48);
    let (a60, a61) = (a60 + a61, a60 - a61);
    let (a62, a63) = (a62 + a63, a62 - a63);
    let (a64, a66) = (a64 + a66, a64 - a66);
    let (a65, a67) = (a65 + a67, a65 - a67);
    let a67 = (a67 << 48);
    let (a64, a65) = (a64 + a65, a64 - a65);
    let (a66, a67) = (a66 + a67, a66 - a67);
    let (a68, a70) = (a68 + a70, a68 - a70);
    let (a69, a71) = (a69 + a71, a69 - a71);
    let a71 = (a71 << 48);
    let (a68, a69) = (a68 + a69, a68 - a69);
    let (a70, a71) = (a70 + a71, a70 - a71);
    let (a36, a40, a44) = (
        a36 + a40 + a44,
        a36 + (a40 << 64) - (a44 << 32),
        a36 - (a40 << 32) + (a44 << 64),
    );
    let (a37, a41, a45) = (
        a37 + a41 + a45,
        a37 + (a41 << 64) - (a45 << 32),
        a37 - (a41 << 32) + (a45 << 64),
    );
    let (a38, a42, a46) = (
        a38 + a42 + a46,
        a38 + (a42 << 64) - (a46 << 32),
        a38 - (a42 << 32) + (a46 << 64),
    );
    let (a39, a43, a47) = (
        a39 + a43 + a47,
        a39 + (a43 << 64) - (a47 << 32),
        a39 - (a43 << 32) + (a47 << 64),
    );
    let a41 = (a41 << 16);
    let a45 = (a45 << 32);
    let a42 = (a42 << 32);
    let a46 = (a46 << 64);
    let a43 = (a43 << 48);
    let a47 = (-a47);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    let (a40, a42) = (a40 + a42, a40 - a42);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let a43 = (a43 << 48);
    let (a40, a41) = (a40 + a41, a40 - a41);
    let (a42, a43) = (a42 + a43, a42 - a43);
    let (a44, a46) = (a44 + a46, a44 - a46);
    let (a45, a47) = (a45 + a47, a45 - a47);
    let a47 = (a47 << 48);
    let (a44, a45) = (a44 + a45, a44 - a45);
    let (a46, a47) = (a46 + a47, a46 - a47);
    let (a84, a88, a92) = (
        a84 + a88 + a92,
        a84 + (a88 << 64) - (a92 << 32),
        a84 - (a88 << 32) + (a92 << 64),
    );
    let (a85, a89, a93) = (
        a85 + a89 + a93,
        a85 + (a89 << 64) - (a93 << 32),
        a85 - (a89 << 32) + (a93 << 64),
    );
    let (a86, a90, a94) = (
        a86 + a90 + a94,
        a86 + (a90 << 64) - (a94 << 32),
        a86 - (a90 << 32) + (a94 << 64),
    );
    let (a87, a91, a95) = (
        a87 + a91 + a95,
        a87 + (a91 << 64) - (a95 << 32),
        a87 - (a91 << 32) + (a95 << 64),
    );
    let a89 = (a89 << 16);
    let a93 = (a93 << 32);
    let a90 = (a90 << 32);
    let a94 = (a94 << 64);
    let a91 = (a91 << 48);
    let a95 = (-a95);
    let (a84, a86) = (a84 + a86, a84 - a86);
    let (a85, a87) = (a85 + a87, a85 - a87);
    let a87 = (a87 << 48);
    let (a84, a85) = (a84 + a85, a84 - a85);
    let (a86, a87) = (a86 + a87, a86 - a87);
    let (a88, a90) = (a88 + a90, a88 - a90);
    let (a89, a91) = (a89 + a91, a89 - a91);
    let a91 = (a91 << 48);
    let (a88, a89) = (a88 + a89, a88 - a89);
    let (a90, a91) = (a90 + a91, a90 - a91);
    let (a92, a94) = (a92 + a94, a92 - a94);
    let (a93, a95) = (a93 + a95, a93 - a95);
    let a95 = (a95 << 48);
    let (a92, a93) = (a92 + a93, a92 - a93);
    let (a94, a95) = (a94 + a95, a94 - a95);
    values[0] = a0;
    values[1] = a48;
    values[2] = a24;
    values[3] = a72;
    values[4] = a12;
    values[5] = a60;
    values[6] = a36;
    values[7] = a84;
    values[8] = a4;
    values[9] = a52;
    values[10] = a28;
    values[11] = a76;
    values[12] = a16;
    values[13] = a64;
    values[14] = a40;
    values[15] = a88;
    values[16] = a8;
    values[17] = a56;
    values[18] = a32;
    values[19] = a80;
    values[20] = a20;
    values[21] = a68;
    values[22] = a44;
    values[23] = a92;
    values[24] = a2;
    values[25] = a50;
    values[26] = a26;
    values[27] = a74;
    values[28] = a14;
    values[29] = a62;
    values[30] = a38;
    values[31] = a86;
    values[32] = a6;
    values[33] = a54;
    values[34] = a30;
    values[35] = a78;
    values[36] = a18;
    values[37] = a66;
    values[38] = a42;
    values[39] = a90;
    values[40] = a10;
    values[41] = a58;
    values[42] = a34;
    values[43] = a82;
    values[44] = a22;
    values[45] = a70;
    values[46] = a46;
    values[47] = a94;
    values[48] = a1;
    values[49] = a49;
    values[50] = a25;
    values[51] = a73;
    values[52] = a13;
    values[53] = a61;
    values[54] = a37;
    values[55] = a85;
    values[56] = a5;
    values[57] = a53;
    values[58] = a29;
    values[59] = a77;
    values[60] = a17;
    values[61] = a65;
    values[62] = a41;
    values[63] = a89;
    values[64] = a9;
    values[65] = a57;
    values[66] = a33;
    values[67] = a81;
    values[68] = a21;
    values[69] = a69;
    values[70] = a45;
    values[71] = a93;
    values[72] = a3;
    values[73] = a51;
    values[74] = a27;
    values[75] = a75;
    values[76] = a15;
    values[77] = a63;
    values[78] = a39;
    values[79] = a87;
    values[80] = a7;
    values[81] = a55;
    values[82] = a31;
    values[83] = a79;
    values[84] = a19;
    values[85] = a67;
    values[86] = a43;
    values[87] = a91;
    values[88] = a11;
    values[89] = a59;
    values[90] = a35;
    values[91] = a83;
    values[92] = a23;
    values[93] = a71;
    values[94] = a47;
    values[95] = a95;
}

/// Size 120 NTT.
fn ntt_120(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 120);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let a60 = values[60];
    let a61 = values[61];
    let a62 = values[62];
    let a63 = values[63];
    let a64 = values[64];
    let a65 = values[65];
    let a66 = values[66];
    let a67 = values[67];
    let a68 = values[68];
    let a69 = values[69];
    let a70 = values[70];
    let a71 = values[71];
    let a72 = values[72];
    let a73 = values[73];
    let a74 = values[74];
    let a75 = values[75];
    let a76 = values[76];
    let a77 = values[77];
    let a78 = values[78];
    let a79 = values[79];
    let a80 = values[80];
    let a81 = values[81];
    let a82 = values[82];
    let a83 = values[83];
    let a84 = values[84];
    let a85 = values[85];
    let a86 = values[86];
    let a87 = values[87];
    let a88 = values[88];
    let a89 = values[89];
    let a90 = values[90];
    let a91 = values[91];
    let a92 = values[92];
    let a93 = values[93];
    let a94 = values[94];
    let a95 = values[95];
    let a96 = values[96];
    let a97 = values[97];
    let a98 = values[98];
    let a99 = values[99];
    let a100 = values[100];
    let a101 = values[101];
    let a102 = values[102];
    let a103 = values[103];
    let a104 = values[104];
    let a105 = values[105];
    let a106 = values[106];
    let a107 = values[107];
    let a108 = values[108];
    let a109 = values[109];
    let a110 = values[110];
    let a111 = values[111];
    let a112 = values[112];
    let a113 = values[113];
    let a114 = values[114];
    let a115 = values[115];
    let a116 = values[116];
    let a117 = values[117];
    let a118 = values[118];
    let a119 = values[119];
    let (a0, a60) = (a0 + a60, a0 - a60);
    let (a12, a72) = (a12 + a72, a12 - a72);
    let (a24, a84) = (a24 + a84, a24 - a84);
    let (a36, a96) = (a36 + a96, a36 - a96);
    let (a48, a108) = (a48 + a108, a48 - a108);
    let a72 = a72 * Field::new(18235156514275634624);
    let a84 = a84 * Field::new(1041288259238279555);
    let a96 = a96 * Field::new(17073700798457888299);
    let a108 = a108 * Field::new(15820824984080659046);
    let (a0, a12, a24, a36, a48) = (
        a0 + a12 + a24 + a36 + a48,
        a0 + a12 * R51 + a24 * R52 + a36 * R53 + a48 * R54,
        a0 + a12 * R52 + a24 * R54 + a36 * R51 + a48 * R53,
        a0 + a12 * R53 + a24 * R51 + a36 * R54 + a48 * R52,
        a0 + a12 * R54 + a24 * R53 + a36 * R52 + a48 * R51,
    );
    let (a60, a72, a84, a96, a108) = (
        a60 + a72 + a84 + a96 + a108,
        a60 + a72 * R51 + a84 * R52 + a96 * R53 + a108 * R54,
        a60 + a72 * R52 + a84 * R54 + a96 * R51 + a108 * R53,
        a60 + a72 * R53 + a84 * R51 + a96 * R54 + a108 * R52,
        a60 + a72 * R54 + a84 * R53 + a96 * R52 + a108 * R51,
    );
    let (a1, a61) = (a1 + a61, a1 - a61);
    let (a13, a73) = (a13 + a73, a13 - a73);
    let (a25, a85) = (a25 + a85, a25 - a85);
    let (a37, a97) = (a37 + a97, a37 - a97);
    let (a49, a109) = (a49 + a109, a49 - a109);
    let a73 = a73 * Field::new(18235156514275634624);
    let a85 = a85 * Field::new(1041288259238279555);
    let a97 = a97 * Field::new(17073700798457888299);
    let a109 = a109 * Field::new(15820824984080659046);
    let (a1, a13, a25, a37, a49) = (
        a1 + a13 + a25 + a37 + a49,
        a1 + a13 * R51 + a25 * R52 + a37 * R53 + a49 * R54,
        a1 + a13 * R52 + a25 * R54 + a37 * R51 + a49 * R53,
        a1 + a13 * R53 + a25 * R51 + a37 * R54 + a49 * R52,
        a1 + a13 * R54 + a25 * R53 + a37 * R52 + a49 * R51,
    );
    let (a61, a73, a85, a97, a109) = (
        a61 + a73 + a85 + a97 + a109,
        a61 + a73 * R51 + a85 * R52 + a97 * R53 + a109 * R54,
        a61 + a73 * R52 + a85 * R54 + a97 * R51 + a109 * R53,
        a61 + a73 * R53 + a85 * R51 + a97 * R54 + a109 * R52,
        a61 + a73 * R54 + a85 * R53 + a97 * R52 + a109 * R51,
    );
    let (a2, a62) = (a2 + a62, a2 - a62);
    let (a14, a74) = (a14 + a74, a14 - a74);
    let (a26, a86) = (a26 + a86, a26 - a86);
    let (a38, a98) = (a38 + a98, a38 - a98);
    let (a50, a110) = (a50 + a110, a50 - a110);
    let a74 = a74 * Field::new(18235156514275634624);
    let a86 = a86 * Field::new(1041288259238279555);
    let a98 = a98 * Field::new(17073700798457888299);
    let a110 = a110 * Field::new(15820824984080659046);
    let (a2, a14, a26, a38, a50) = (
        a2 + a14 + a26 + a38 + a50,
        a2 + a14 * R51 + a26 * R52 + a38 * R53 + a50 * R54,
        a2 + a14 * R52 + a26 * R54 + a38 * R51 + a50 * R53,
        a2 + a14 * R53 + a26 * R51 + a38 * R54 + a50 * R52,
        a2 + a14 * R54 + a26 * R53 + a38 * R52 + a50 * R51,
    );
    let (a62, a74, a86, a98, a110) = (
        a62 + a74 + a86 + a98 + a110,
        a62 + a74 * R51 + a86 * R52 + a98 * R53 + a110 * R54,
        a62 + a74 * R52 + a86 * R54 + a98 * R51 + a110 * R53,
        a62 + a74 * R53 + a86 * R51 + a98 * R54 + a110 * R52,
        a62 + a74 * R54 + a86 * R53 + a98 * R52 + a110 * R51,
    );
    let (a3, a63) = (a3 + a63, a3 - a63);
    let (a15, a75) = (a15 + a75, a15 - a75);
    let (a27, a87) = (a27 + a87, a27 - a87);
    let (a39, a99) = (a39 + a99, a39 - a99);
    let (a51, a111) = (a51 + a111, a51 - a111);
    let a75 = a75 * Field::new(18235156514275634624);
    let a87 = a87 * Field::new(1041288259238279555);
    let a99 = a99 * Field::new(17073700798457888299);
    let a111 = a111 * Field::new(15820824984080659046);
    let (a3, a15, a27, a39, a51) = (
        a3 + a15 + a27 + a39 + a51,
        a3 + a15 * R51 + a27 * R52 + a39 * R53 + a51 * R54,
        a3 + a15 * R52 + a27 * R54 + a39 * R51 + a51 * R53,
        a3 + a15 * R53 + a27 * R51 + a39 * R54 + a51 * R52,
        a3 + a15 * R54 + a27 * R53 + a39 * R52 + a51 * R51,
    );
    let (a63, a75, a87, a99, a111) = (
        a63 + a75 + a87 + a99 + a111,
        a63 + a75 * R51 + a87 * R52 + a99 * R53 + a111 * R54,
        a63 + a75 * R52 + a87 * R54 + a99 * R51 + a111 * R53,
        a63 + a75 * R53 + a87 * R51 + a99 * R54 + a111 * R52,
        a63 + a75 * R54 + a87 * R53 + a99 * R52 + a111 * R51,
    );
    let (a4, a64) = (a4 + a64, a4 - a64);
    let (a16, a76) = (a16 + a76, a16 - a76);
    let (a28, a88) = (a28 + a88, a28 - a88);
    let (a40, a100) = (a40 + a100, a40 - a100);
    let (a52, a112) = (a52 + a112, a52 - a112);
    let a76 = a76 * Field::new(18235156514275634624);
    let a88 = a88 * Field::new(1041288259238279555);
    let a100 = a100 * Field::new(17073700798457888299);
    let a112 = a112 * Field::new(15820824984080659046);
    let (a4, a16, a28, a40, a52) = (
        a4 + a16 + a28 + a40 + a52,
        a4 + a16 * R51 + a28 * R52 + a40 * R53 + a52 * R54,
        a4 + a16 * R52 + a28 * R54 + a40 * R51 + a52 * R53,
        a4 + a16 * R53 + a28 * R51 + a40 * R54 + a52 * R52,
        a4 + a16 * R54 + a28 * R53 + a40 * R52 + a52 * R51,
    );
    let (a64, a76, a88, a100, a112) = (
        a64 + a76 + a88 + a100 + a112,
        a64 + a76 * R51 + a88 * R52 + a100 * R53 + a112 * R54,
        a64 + a76 * R52 + a88 * R54 + a100 * R51 + a112 * R53,
        a64 + a76 * R53 + a88 * R51 + a100 * R54 + a112 * R52,
        a64 + a76 * R54 + a88 * R53 + a100 * R52 + a112 * R51,
    );
    let (a5, a65) = (a5 + a65, a5 - a65);
    let (a17, a77) = (a17 + a77, a17 - a77);
    let (a29, a89) = (a29 + a89, a29 - a89);
    let (a41, a101) = (a41 + a101, a41 - a101);
    let (a53, a113) = (a53 + a113, a53 - a113);
    let a77 = a77 * Field::new(18235156514275634624);
    let a89 = a89 * Field::new(1041288259238279555);
    let a101 = a101 * Field::new(17073700798457888299);
    let a113 = a113 * Field::new(15820824984080659046);
    let (a5, a17, a29, a41, a53) = (
        a5 + a17 + a29 + a41 + a53,
        a5 + a17 * R51 + a29 * R52 + a41 * R53 + a53 * R54,
        a5 + a17 * R52 + a29 * R54 + a41 * R51 + a53 * R53,
        a5 + a17 * R53 + a29 * R51 + a41 * R54 + a53 * R52,
        a5 + a17 * R54 + a29 * R53 + a41 * R52 + a53 * R51,
    );
    let (a65, a77, a89, a101, a113) = (
        a65 + a77 + a89 + a101 + a113,
        a65 + a77 * R51 + a89 * R52 + a101 * R53 + a113 * R54,
        a65 + a77 * R52 + a89 * R54 + a101 * R51 + a113 * R53,
        a65 + a77 * R53 + a89 * R51 + a101 * R54 + a113 * R52,
        a65 + a77 * R54 + a89 * R53 + a101 * R52 + a113 * R51,
    );
    let (a6, a66) = (a6 + a66, a6 - a66);
    let (a18, a78) = (a18 + a78, a18 - a78);
    let (a30, a90) = (a30 + a90, a30 - a90);
    let (a42, a102) = (a42 + a102, a42 - a102);
    let (a54, a114) = (a54 + a114, a54 - a114);
    let a78 = a78 * Field::new(18235156514275634624);
    let a90 = a90 * Field::new(1041288259238279555);
    let a102 = a102 * Field::new(17073700798457888299);
    let a114 = a114 * Field::new(15820824984080659046);
    let (a6, a18, a30, a42, a54) = (
        a6 + a18 + a30 + a42 + a54,
        a6 + a18 * R51 + a30 * R52 + a42 * R53 + a54 * R54,
        a6 + a18 * R52 + a30 * R54 + a42 * R51 + a54 * R53,
        a6 + a18 * R53 + a30 * R51 + a42 * R54 + a54 * R52,
        a6 + a18 * R54 + a30 * R53 + a42 * R52 + a54 * R51,
    );
    let (a66, a78, a90, a102, a114) = (
        a66 + a78 + a90 + a102 + a114,
        a66 + a78 * R51 + a90 * R52 + a102 * R53 + a114 * R54,
        a66 + a78 * R52 + a90 * R54 + a102 * R51 + a114 * R53,
        a66 + a78 * R53 + a90 * R51 + a102 * R54 + a114 * R52,
        a66 + a78 * R54 + a90 * R53 + a102 * R52 + a114 * R51,
    );
    let (a7, a67) = (a7 + a67, a7 - a67);
    let (a19, a79) = (a19 + a79, a19 - a79);
    let (a31, a91) = (a31 + a91, a31 - a91);
    let (a43, a103) = (a43 + a103, a43 - a103);
    let (a55, a115) = (a55 + a115, a55 - a115);
    let a79 = a79 * Field::new(18235156514275634624);
    let a91 = a91 * Field::new(1041288259238279555);
    let a103 = a103 * Field::new(17073700798457888299);
    let a115 = a115 * Field::new(15820824984080659046);
    let (a7, a19, a31, a43, a55) = (
        a7 + a19 + a31 + a43 + a55,
        a7 + a19 * R51 + a31 * R52 + a43 * R53 + a55 * R54,
        a7 + a19 * R52 + a31 * R54 + a43 * R51 + a55 * R53,
        a7 + a19 * R53 + a31 * R51 + a43 * R54 + a55 * R52,
        a7 + a19 * R54 + a31 * R53 + a43 * R52 + a55 * R51,
    );
    let (a67, a79, a91, a103, a115) = (
        a67 + a79 + a91 + a103 + a115,
        a67 + a79 * R51 + a91 * R52 + a103 * R53 + a115 * R54,
        a67 + a79 * R52 + a91 * R54 + a103 * R51 + a115 * R53,
        a67 + a79 * R53 + a91 * R51 + a103 * R54 + a115 * R52,
        a67 + a79 * R54 + a91 * R53 + a103 * R52 + a115 * R51,
    );
    let (a8, a68) = (a8 + a68, a8 - a68);
    let (a20, a80) = (a20 + a80, a20 - a80);
    let (a32, a92) = (a32 + a92, a32 - a92);
    let (a44, a104) = (a44 + a104, a44 - a104);
    let (a56, a116) = (a56 + a116, a56 - a116);
    let a80 = a80 * Field::new(18235156514275634624);
    let a92 = a92 * Field::new(1041288259238279555);
    let a104 = a104 * Field::new(17073700798457888299);
    let a116 = a116 * Field::new(15820824984080659046);
    let (a8, a20, a32, a44, a56) = (
        a8 + a20 + a32 + a44 + a56,
        a8 + a20 * R51 + a32 * R52 + a44 * R53 + a56 * R54,
        a8 + a20 * R52 + a32 * R54 + a44 * R51 + a56 * R53,
        a8 + a20 * R53 + a32 * R51 + a44 * R54 + a56 * R52,
        a8 + a20 * R54 + a32 * R53 + a44 * R52 + a56 * R51,
    );
    let (a68, a80, a92, a104, a116) = (
        a68 + a80 + a92 + a104 + a116,
        a68 + a80 * R51 + a92 * R52 + a104 * R53 + a116 * R54,
        a68 + a80 * R52 + a92 * R54 + a104 * R51 + a116 * R53,
        a68 + a80 * R53 + a92 * R51 + a104 * R54 + a116 * R52,
        a68 + a80 * R54 + a92 * R53 + a104 * R52 + a116 * R51,
    );
    let (a9, a69) = (a9 + a69, a9 - a69);
    let (a21, a81) = (a21 + a81, a21 - a81);
    let (a33, a93) = (a33 + a93, a33 - a93);
    let (a45, a105) = (a45 + a105, a45 - a105);
    let (a57, a117) = (a57 + a117, a57 - a117);
    let a81 = a81 * Field::new(18235156514275634624);
    let a93 = a93 * Field::new(1041288259238279555);
    let a105 = a105 * Field::new(17073700798457888299);
    let a117 = a117 * Field::new(15820824984080659046);
    let (a9, a21, a33, a45, a57) = (
        a9 + a21 + a33 + a45 + a57,
        a9 + a21 * R51 + a33 * R52 + a45 * R53 + a57 * R54,
        a9 + a21 * R52 + a33 * R54 + a45 * R51 + a57 * R53,
        a9 + a21 * R53 + a33 * R51 + a45 * R54 + a57 * R52,
        a9 + a21 * R54 + a33 * R53 + a45 * R52 + a57 * R51,
    );
    let (a69, a81, a93, a105, a117) = (
        a69 + a81 + a93 + a105 + a117,
        a69 + a81 * R51 + a93 * R52 + a105 * R53 + a117 * R54,
        a69 + a81 * R52 + a93 * R54 + a105 * R51 + a117 * R53,
        a69 + a81 * R53 + a93 * R51 + a105 * R54 + a117 * R52,
        a69 + a81 * R54 + a93 * R53 + a105 * R52 + a117 * R51,
    );
    let (a10, a70) = (a10 + a70, a10 - a70);
    let (a22, a82) = (a22 + a82, a22 - a82);
    let (a34, a94) = (a34 + a94, a34 - a94);
    let (a46, a106) = (a46 + a106, a46 - a106);
    let (a58, a118) = (a58 + a118, a58 - a118);
    let a82 = a82 * Field::new(18235156514275634624);
    let a94 = a94 * Field::new(1041288259238279555);
    let a106 = a106 * Field::new(17073700798457888299);
    let a118 = a118 * Field::new(15820824984080659046);
    let (a10, a22, a34, a46, a58) = (
        a10 + a22 + a34 + a46 + a58,
        a10 + a22 * R51 + a34 * R52 + a46 * R53 + a58 * R54,
        a10 + a22 * R52 + a34 * R54 + a46 * R51 + a58 * R53,
        a10 + a22 * R53 + a34 * R51 + a46 * R54 + a58 * R52,
        a10 + a22 * R54 + a34 * R53 + a46 * R52 + a58 * R51,
    );
    let (a70, a82, a94, a106, a118) = (
        a70 + a82 + a94 + a106 + a118,
        a70 + a82 * R51 + a94 * R52 + a106 * R53 + a118 * R54,
        a70 + a82 * R52 + a94 * R54 + a106 * R51 + a118 * R53,
        a70 + a82 * R53 + a94 * R51 + a106 * R54 + a118 * R52,
        a70 + a82 * R54 + a94 * R53 + a106 * R52 + a118 * R51,
    );
    let (a11, a71) = (a11 + a71, a11 - a71);
    let (a23, a83) = (a23 + a83, a23 - a83);
    let (a35, a95) = (a35 + a95, a35 - a95);
    let (a47, a107) = (a47 + a107, a47 - a107);
    let (a59, a119) = (a59 + a119, a59 - a119);
    let a83 = a83 * Field::new(18235156514275634624);
    let a95 = a95 * Field::new(1041288259238279555);
    let a107 = a107 * Field::new(17073700798457888299);
    let a119 = a119 * Field::new(15820824984080659046);
    let (a11, a23, a35, a47, a59) = (
        a11 + a23 + a35 + a47 + a59,
        a11 + a23 * R51 + a35 * R52 + a47 * R53 + a59 * R54,
        a11 + a23 * R52 + a35 * R54 + a47 * R51 + a59 * R53,
        a11 + a23 * R53 + a35 * R51 + a47 * R54 + a59 * R52,
        a11 + a23 * R54 + a35 * R53 + a47 * R52 + a59 * R51,
    );
    let (a71, a83, a95, a107, a119) = (
        a71 + a83 + a95 + a107 + a119,
        a71 + a83 * R51 + a95 * R52 + a107 * R53 + a119 * R54,
        a71 + a83 * R52 + a95 * R54 + a107 * R51 + a119 * R53,
        a71 + a83 * R53 + a95 * R51 + a107 * R54 + a119 * R52,
        a71 + a83 * R54 + a95 * R53 + a107 * R52 + a119 * R51,
    );
    let a61 = a61 * Field::new(12342513394488208227);
    let a13 = a13 * Field::new(5927015354646419725);
    let a73 = a73 * Field::new(9148693690730647261);
    let a25 = a25 * Field::new(6868348408044855211);
    let a85 = (a85 << 8);
    let a37 = a37 * Field::new(5290193119087387221);
    let a97 = a97 * Field::new(4682917097487535278);
    let a49 = a49 * Field::new(17775832080808074370);
    let a109 = a109 * Field::new(5856505865097423521);
    let a62 = a62 * Field::new(5927015354646419725);
    let a14 = a14 * Field::new(6868348408044855211);
    let a74 = a74 * Field::new(5290193119087387221);
    let a26 = a26 * Field::new(17775832080808074370);
    let a86 = (a86 << 16);
    let a38 = a38 * Field::new(18235156514275634624);
    let a98 = a98 * Field::new(5079231842359091375);
    let a50 = a50 * Field::new(9988211933311186582);
    let a110 = a110 * Field::new(8149776168132872528);
    let a63 = a63 * Field::new(9148693690730647261);
    let a15 = a15 * Field::new(5290193119087387221);
    let a75 = a75 * Field::new(5856505865097423521);
    let a27 = a27 * Field::new(18235156514275634624);
    let a87 = (a87 << 24);
    let a39 = a39 * Field::new(8149776168132872528);
    let a99 = a99 * Field::new(11331573348451128694);
    let a51 = a51 * Field::new(1041288259238279555);
    let a111 = a111 * Field::new(4419751934697861046);
    let a64 = a64 * Field::new(6868348408044855211);
    let a16 = a16 * Field::new(17775832080808074370);
    let a76 = a76 * Field::new(18235156514275634624);
    let a28 = a28 * Field::new(9988211933311186582);
    let a88 = (a88 << 32);
    let a40 = a40 * Field::new(1041288259238279555);
    let a100 = a100 * Field::new(15149912995474149095);
    let a52 = a52 * Field::new(6205107048362784195);
    let a112 = a112 * Field::new(17073700798457888299);
    let a65 = (a65 << 8);
    let a17 = (a17 << 16);
    let a77 = (a77 << 24);
    let a29 = (a29 << 32);
    let a89 = (a89 << 40);
    let a41 = (a41 << 48);
    let a101 = (a101 << 56);
    let a53 = (a53 << 64);
    let a113 = (a113 << 72);
    let a66 = a66 * Field::new(5290193119087387221);
    let a18 = a18 * Field::new(18235156514275634624);
    let a78 = a78 * Field::new(8149776168132872528);
    let a30 = a30 * Field::new(1041288259238279555);
    let a90 = (a90 << 48);
    let a42 = a42 * Field::new(17073700798457888299);
    let a102 = a102 * Field::new(17869255328328231396);
    let a54 = a54 * Field::new(15820824984080659046);
    let a114 = a114 * Field::new(2281812832982421726);
    let a67 = a67 * Field::new(4682917097487535278);
    let a19 = a19 * Field::new(5079231842359091375);
    let a79 = a79 * Field::new(11331573348451128694);
    let a31 = a31 * Field::new(15149912995474149095);
    let a91 = (a91 << 56);
    let a43 = a43 * Field::new(17869255328328231396);
    let a103 = a103 * Field::new(2458871528097962065);
    let a55 = a55 * Field::new(7085488865146701717);
    let a115 = a115 * Field::new(9298050378683937060);
    let a68 = a68 * Field::new(17775832080808074370);
    let a20 = a20 * Field::new(9988211933311186582);
    let a80 = a80 * Field::new(1041288259238279555);
    let a32 = a32 * Field::new(6205107048362784195);
    let a92 = (a92 << 64);
    let a44 = a44 * Field::new(15820824984080659046);
    let a104 = a104 * Field::new(7085488865146701717);
    let a56 = a56 * Field::new(11578395661369729110);
    let a116 = a116 * Field::new(211587555138949697);
    let a69 = a69 * Field::new(5856505865097423521);
    let a21 = a21 * Field::new(8149776168132872528);
    let a81 = a81 * Field::new(4419751934697861046);
    let a33 = a33 * Field::new(17073700798457888299);
    let a93 = (a93 << 72);
    let a45 = a45 * Field::new(2281812832982421726);
    let a105 = a105 * Field::new(9298050378683937060);
    let a57 = a57 * Field::new(211587555138949697);
    let a117 = a117 * Field::new(7115170720963455627);
    let a70 = (a70 << 16);
    let a22 = (a22 << 32);
    let a82 = (a82 << 48);
    let a34 = (a34 << 64);
    let a94 = (a94 << 80);
    let a46 = (-a46);
    let a106 = (-(a106 << 16));
    let a58 = (-(a58 << 32));
    let a118 = (-(a118 << 48));
    let a71 = a71 * Field::new(7677121419106473143);
    let a23 = a23 * Field::new(5349526613560066800);
    let a83 = a83 * Field::new(4561472264319460910);
    let a35 = a35 * Field::new(12619683920608008665);
    let a95 = (a95 << 88);
    let a47 = a47 * Field::new(13156550950327197100);
    let a107 = a107 * Field::new(17272925976741953790);
    let a59 = a59 * Field::new(3296831073940435226);
    let a119 = a119 * Field::new(15587202262274831207);
    let (a0, a4, a8) = (
        a0 + a4 + a8,
        a0 + (a4 << 64) - (a8 << 32),
        a0 - (a4 << 32) + (a8 << 64),
    );
    let (a1, a5, a9) = (
        a1 + a5 + a9,
        a1 + (a5 << 64) - (a9 << 32),
        a1 - (a5 << 32) + (a9 << 64),
    );
    let (a2, a6, a10) = (
        a2 + a6 + a10,
        a2 + (a6 << 64) - (a10 << 32),
        a2 - (a6 << 32) + (a10 << 64),
    );
    let (a3, a7, a11) = (
        a3 + a7 + a11,
        a3 + (a7 << 64) - (a11 << 32),
        a3 - (a7 << 32) + (a11 << 64),
    );
    let a5 = (a5 << 16);
    let a9 = (a9 << 32);
    let a6 = (a6 << 32);
    let a10 = (a10 << 64);
    let a7 = (a7 << 48);
    let a11 = (-a11);
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a60, a64, a68) = (
        a60 + a64 + a68,
        a60 + (a64 << 64) - (a68 << 32),
        a60 - (a64 << 32) + (a68 << 64),
    );
    let (a61, a65, a69) = (
        a61 + a65 + a69,
        a61 + (a65 << 64) - (a69 << 32),
        a61 - (a65 << 32) + (a69 << 64),
    );
    let (a62, a66, a70) = (
        a62 + a66 + a70,
        a62 + (a66 << 64) - (a70 << 32),
        a62 - (a66 << 32) + (a70 << 64),
    );
    let (a63, a67, a71) = (
        a63 + a67 + a71,
        a63 + (a67 << 64) - (a71 << 32),
        a63 - (a67 << 32) + (a71 << 64),
    );
    let a65 = (a65 << 16);
    let a69 = (a69 << 32);
    let a66 = (a66 << 32);
    let a70 = (a70 << 64);
    let a67 = (a67 << 48);
    let a71 = (-a71);
    let (a60, a62) = (a60 + a62, a60 - a62);
    let (a61, a63) = (a61 + a63, a61 - a63);
    let a63 = (a63 << 48);
    let (a60, a61) = (a60 + a61, a60 - a61);
    let (a62, a63) = (a62 + a63, a62 - a63);
    let (a64, a66) = (a64 + a66, a64 - a66);
    let (a65, a67) = (a65 + a67, a65 - a67);
    let a67 = (a67 << 48);
    let (a64, a65) = (a64 + a65, a64 - a65);
    let (a66, a67) = (a66 + a67, a66 - a67);
    let (a68, a70) = (a68 + a70, a68 - a70);
    let (a69, a71) = (a69 + a71, a69 - a71);
    let a71 = (a71 << 48);
    let (a68, a69) = (a68 + a69, a68 - a69);
    let (a70, a71) = (a70 + a71, a70 - a71);
    let (a12, a16, a20) = (
        a12 + a16 + a20,
        a12 + (a16 << 64) - (a20 << 32),
        a12 - (a16 << 32) + (a20 << 64),
    );
    let (a13, a17, a21) = (
        a13 + a17 + a21,
        a13 + (a17 << 64) - (a21 << 32),
        a13 - (a17 << 32) + (a21 << 64),
    );
    let (a14, a18, a22) = (
        a14 + a18 + a22,
        a14 + (a18 << 64) - (a22 << 32),
        a14 - (a18 << 32) + (a22 << 64),
    );
    let (a15, a19, a23) = (
        a15 + a19 + a23,
        a15 + (a19 << 64) - (a23 << 32),
        a15 - (a19 << 32) + (a23 << 64),
    );
    let a17 = (a17 << 16);
    let a21 = (a21 << 32);
    let a18 = (a18 << 32);
    let a22 = (a22 << 64);
    let a19 = (a19 << 48);
    let a23 = (-a23);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a72, a76, a80) = (
        a72 + a76 + a80,
        a72 + (a76 << 64) - (a80 << 32),
        a72 - (a76 << 32) + (a80 << 64),
    );
    let (a73, a77, a81) = (
        a73 + a77 + a81,
        a73 + (a77 << 64) - (a81 << 32),
        a73 - (a77 << 32) + (a81 << 64),
    );
    let (a74, a78, a82) = (
        a74 + a78 + a82,
        a74 + (a78 << 64) - (a82 << 32),
        a74 - (a78 << 32) + (a82 << 64),
    );
    let (a75, a79, a83) = (
        a75 + a79 + a83,
        a75 + (a79 << 64) - (a83 << 32),
        a75 - (a79 << 32) + (a83 << 64),
    );
    let a77 = (a77 << 16);
    let a81 = (a81 << 32);
    let a78 = (a78 << 32);
    let a82 = (a82 << 64);
    let a79 = (a79 << 48);
    let a83 = (-a83);
    let (a72, a74) = (a72 + a74, a72 - a74);
    let (a73, a75) = (a73 + a75, a73 - a75);
    let a75 = (a75 << 48);
    let (a72, a73) = (a72 + a73, a72 - a73);
    let (a74, a75) = (a74 + a75, a74 - a75);
    let (a76, a78) = (a76 + a78, a76 - a78);
    let (a77, a79) = (a77 + a79, a77 - a79);
    let a79 = (a79 << 48);
    let (a76, a77) = (a76 + a77, a76 - a77);
    let (a78, a79) = (a78 + a79, a78 - a79);
    let (a80, a82) = (a80 + a82, a80 - a82);
    let (a81, a83) = (a81 + a83, a81 - a83);
    let a83 = (a83 << 48);
    let (a80, a81) = (a80 + a81, a80 - a81);
    let (a82, a83) = (a82 + a83, a82 - a83);
    let (a24, a28, a32) = (
        a24 + a28 + a32,
        a24 + (a28 << 64) - (a32 << 32),
        a24 - (a28 << 32) + (a32 << 64),
    );
    let (a25, a29, a33) = (
        a25 + a29 + a33,
        a25 + (a29 << 64) - (a33 << 32),
        a25 - (a29 << 32) + (a33 << 64),
    );
    let (a26, a30, a34) = (
        a26 + a30 + a34,
        a26 + (a30 << 64) - (a34 << 32),
        a26 - (a30 << 32) + (a34 << 64),
    );
    let (a27, a31, a35) = (
        a27 + a31 + a35,
        a27 + (a31 << 64) - (a35 << 32),
        a27 - (a31 << 32) + (a35 << 64),
    );
    let a29 = (a29 << 16);
    let a33 = (a33 << 32);
    let a30 = (a30 << 32);
    let a34 = (a34 << 64);
    let a31 = (a31 << 48);
    let a35 = (-a35);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a84, a88, a92) = (
        a84 + a88 + a92,
        a84 + (a88 << 64) - (a92 << 32),
        a84 - (a88 << 32) + (a92 << 64),
    );
    let (a85, a89, a93) = (
        a85 + a89 + a93,
        a85 + (a89 << 64) - (a93 << 32),
        a85 - (a89 << 32) + (a93 << 64),
    );
    let (a86, a90, a94) = (
        a86 + a90 + a94,
        a86 + (a90 << 64) - (a94 << 32),
        a86 - (a90 << 32) + (a94 << 64),
    );
    let (a87, a91, a95) = (
        a87 + a91 + a95,
        a87 + (a91 << 64) - (a95 << 32),
        a87 - (a91 << 32) + (a95 << 64),
    );
    let a89 = (a89 << 16);
    let a93 = (a93 << 32);
    let a90 = (a90 << 32);
    let a94 = (a94 << 64);
    let a91 = (a91 << 48);
    let a95 = (-a95);
    let (a84, a86) = (a84 + a86, a84 - a86);
    let (a85, a87) = (a85 + a87, a85 - a87);
    let a87 = (a87 << 48);
    let (a84, a85) = (a84 + a85, a84 - a85);
    let (a86, a87) = (a86 + a87, a86 - a87);
    let (a88, a90) = (a88 + a90, a88 - a90);
    let (a89, a91) = (a89 + a91, a89 - a91);
    let a91 = (a91 << 48);
    let (a88, a89) = (a88 + a89, a88 - a89);
    let (a90, a91) = (a90 + a91, a90 - a91);
    let (a92, a94) = (a92 + a94, a92 - a94);
    let (a93, a95) = (a93 + a95, a93 - a95);
    let a95 = (a95 << 48);
    let (a92, a93) = (a92 + a93, a92 - a93);
    let (a94, a95) = (a94 + a95, a94 - a95);
    let (a36, a40, a44) = (
        a36 + a40 + a44,
        a36 + (a40 << 64) - (a44 << 32),
        a36 - (a40 << 32) + (a44 << 64),
    );
    let (a37, a41, a45) = (
        a37 + a41 + a45,
        a37 + (a41 << 64) - (a45 << 32),
        a37 - (a41 << 32) + (a45 << 64),
    );
    let (a38, a42, a46) = (
        a38 + a42 + a46,
        a38 + (a42 << 64) - (a46 << 32),
        a38 - (a42 << 32) + (a46 << 64),
    );
    let (a39, a43, a47) = (
        a39 + a43 + a47,
        a39 + (a43 << 64) - (a47 << 32),
        a39 - (a43 << 32) + (a47 << 64),
    );
    let a41 = (a41 << 16);
    let a45 = (a45 << 32);
    let a42 = (a42 << 32);
    let a46 = (a46 << 64);
    let a43 = (a43 << 48);
    let a47 = (-a47);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    let (a40, a42) = (a40 + a42, a40 - a42);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let a43 = (a43 << 48);
    let (a40, a41) = (a40 + a41, a40 - a41);
    let (a42, a43) = (a42 + a43, a42 - a43);
    let (a44, a46) = (a44 + a46, a44 - a46);
    let (a45, a47) = (a45 + a47, a45 - a47);
    let a47 = (a47 << 48);
    let (a44, a45) = (a44 + a45, a44 - a45);
    let (a46, a47) = (a46 + a47, a46 - a47);
    let (a96, a100, a104) = (
        a96 + a100 + a104,
        a96 + (a100 << 64) - (a104 << 32),
        a96 - (a100 << 32) + (a104 << 64),
    );
    let (a97, a101, a105) = (
        a97 + a101 + a105,
        a97 + (a101 << 64) - (a105 << 32),
        a97 - (a101 << 32) + (a105 << 64),
    );
    let (a98, a102, a106) = (
        a98 + a102 + a106,
        a98 + (a102 << 64) - (a106 << 32),
        a98 - (a102 << 32) + (a106 << 64),
    );
    let (a99, a103, a107) = (
        a99 + a103 + a107,
        a99 + (a103 << 64) - (a107 << 32),
        a99 - (a103 << 32) + (a107 << 64),
    );
    let a101 = (a101 << 16);
    let a105 = (a105 << 32);
    let a102 = (a102 << 32);
    let a106 = (a106 << 64);
    let a103 = (a103 << 48);
    let a107 = (-a107);
    let (a96, a98) = (a96 + a98, a96 - a98);
    let (a97, a99) = (a97 + a99, a97 - a99);
    let a99 = (a99 << 48);
    let (a96, a97) = (a96 + a97, a96 - a97);
    let (a98, a99) = (a98 + a99, a98 - a99);
    let (a100, a102) = (a100 + a102, a100 - a102);
    let (a101, a103) = (a101 + a103, a101 - a103);
    let a103 = (a103 << 48);
    let (a100, a101) = (a100 + a101, a100 - a101);
    let (a102, a103) = (a102 + a103, a102 - a103);
    let (a104, a106) = (a104 + a106, a104 - a106);
    let (a105, a107) = (a105 + a107, a105 - a107);
    let a107 = (a107 << 48);
    let (a104, a105) = (a104 + a105, a104 - a105);
    let (a106, a107) = (a106 + a107, a106 - a107);
    let (a48, a52, a56) = (
        a48 + a52 + a56,
        a48 + (a52 << 64) - (a56 << 32),
        a48 - (a52 << 32) + (a56 << 64),
    );
    let (a49, a53, a57) = (
        a49 + a53 + a57,
        a49 + (a53 << 64) - (a57 << 32),
        a49 - (a53 << 32) + (a57 << 64),
    );
    let (a50, a54, a58) = (
        a50 + a54 + a58,
        a50 + (a54 << 64) - (a58 << 32),
        a50 - (a54 << 32) + (a58 << 64),
    );
    let (a51, a55, a59) = (
        a51 + a55 + a59,
        a51 + (a55 << 64) - (a59 << 32),
        a51 - (a55 << 32) + (a59 << 64),
    );
    let a53 = (a53 << 16);
    let a57 = (a57 << 32);
    let a54 = (a54 << 32);
    let a58 = (a58 << 64);
    let a55 = (a55 << 48);
    let a59 = (-a59);
    let (a48, a50) = (a48 + a50, a48 - a50);
    let (a49, a51) = (a49 + a51, a49 - a51);
    let a51 = (a51 << 48);
    let (a48, a49) = (a48 + a49, a48 - a49);
    let (a50, a51) = (a50 + a51, a50 - a51);
    let (a52, a54) = (a52 + a54, a52 - a54);
    let (a53, a55) = (a53 + a55, a53 - a55);
    let a55 = (a55 << 48);
    let (a52, a53) = (a52 + a53, a52 - a53);
    let (a54, a55) = (a54 + a55, a54 - a55);
    let (a56, a58) = (a56 + a58, a56 - a58);
    let (a57, a59) = (a57 + a59, a57 - a59);
    let a59 = (a59 << 48);
    let (a56, a57) = (a56 + a57, a56 - a57);
    let (a58, a59) = (a58 + a59, a58 - a59);
    let (a108, a112, a116) = (
        a108 + a112 + a116,
        a108 + (a112 << 64) - (a116 << 32),
        a108 - (a112 << 32) + (a116 << 64),
    );
    let (a109, a113, a117) = (
        a109 + a113 + a117,
        a109 + (a113 << 64) - (a117 << 32),
        a109 - (a113 << 32) + (a117 << 64),
    );
    let (a110, a114, a118) = (
        a110 + a114 + a118,
        a110 + (a114 << 64) - (a118 << 32),
        a110 - (a114 << 32) + (a118 << 64),
    );
    let (a111, a115, a119) = (
        a111 + a115 + a119,
        a111 + (a115 << 64) - (a119 << 32),
        a111 - (a115 << 32) + (a119 << 64),
    );
    let a113 = (a113 << 16);
    let a117 = (a117 << 32);
    let a114 = (a114 << 32);
    let a118 = (a118 << 64);
    let a115 = (a115 << 48);
    let a119 = (-a119);
    let (a108, a110) = (a108 + a110, a108 - a110);
    let (a109, a111) = (a109 + a111, a109 - a111);
    let a111 = (a111 << 48);
    let (a108, a109) = (a108 + a109, a108 - a109);
    let (a110, a111) = (a110 + a111, a110 - a111);
    let (a112, a114) = (a112 + a114, a112 - a114);
    let (a113, a115) = (a113 + a115, a113 - a115);
    let a115 = (a115 << 48);
    let (a112, a113) = (a112 + a113, a112 - a113);
    let (a114, a115) = (a114 + a115, a114 - a115);
    let (a116, a118) = (a116 + a118, a116 - a118);
    let (a117, a119) = (a117 + a119, a117 - a119);
    let a119 = (a119 << 48);
    let (a116, a117) = (a116 + a117, a116 - a117);
    let (a118, a119) = (a118 + a119, a118 - a119);
    values[0] = a0;
    values[1] = a60;
    values[2] = a12;
    values[3] = a72;
    values[4] = a24;
    values[5] = a84;
    values[6] = a36;
    values[7] = a96;
    values[8] = a48;
    values[9] = a108;
    values[10] = a4;
    values[11] = a64;
    values[12] = a16;
    values[13] = a76;
    values[14] = a28;
    values[15] = a88;
    values[16] = a40;
    values[17] = a100;
    values[18] = a52;
    values[19] = a112;
    values[20] = a8;
    values[21] = a68;
    values[22] = a20;
    values[23] = a80;
    values[24] = a32;
    values[25] = a92;
    values[26] = a44;
    values[27] = a104;
    values[28] = a56;
    values[29] = a116;
    values[30] = a2;
    values[31] = a62;
    values[32] = a14;
    values[33] = a74;
    values[34] = a26;
    values[35] = a86;
    values[36] = a38;
    values[37] = a98;
    values[38] = a50;
    values[39] = a110;
    values[40] = a6;
    values[41] = a66;
    values[42] = a18;
    values[43] = a78;
    values[44] = a30;
    values[45] = a90;
    values[46] = a42;
    values[47] = a102;
    values[48] = a54;
    values[49] = a114;
    values[50] = a10;
    values[51] = a70;
    values[52] = a22;
    values[53] = a82;
    values[54] = a34;
    values[55] = a94;
    values[56] = a46;
    values[57] = a106;
    values[58] = a58;
    values[59] = a118;
    values[60] = a1;
    values[61] = a61;
    values[62] = a13;
    values[63] = a73;
    values[64] = a25;
    values[65] = a85;
    values[66] = a37;
    values[67] = a97;
    values[68] = a49;
    values[69] = a109;
    values[70] = a5;
    values[71] = a65;
    values[72] = a17;
    values[73] = a77;
    values[74] = a29;
    values[75] = a89;
    values[76] = a41;
    values[77] = a101;
    values[78] = a53;
    values[79] = a113;
    values[80] = a9;
    values[81] = a69;
    values[82] = a21;
    values[83] = a81;
    values[84] = a33;
    values[85] = a93;
    values[86] = a45;
    values[87] = a105;
    values[88] = a57;
    values[89] = a117;
    values[90] = a3;
    values[91] = a63;
    values[92] = a15;
    values[93] = a75;
    values[94] = a27;
    values[95] = a87;
    values[96] = a39;
    values[97] = a99;
    values[98] = a51;
    values[99] = a111;
    values[100] = a7;
    values[101] = a67;
    values[102] = a19;
    values[103] = a79;
    values[104] = a31;
    values[105] = a91;
    values[106] = a43;
    values[107] = a103;
    values[108] = a55;
    values[109] = a115;
    values[110] = a11;
    values[111] = a71;
    values[112] = a23;
    values[113] = a83;
    values[114] = a35;
    values[115] = a95;
    values[116] = a47;
    values[117] = a107;
    values[118] = a59;
    values[119] = a119;
}

/// Size 128 NTT.
fn ntt_128(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 128);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let a60 = values[60];
    let a61 = values[61];
    let a62 = values[62];
    let a63 = values[63];
    let a64 = values[64];
    let a65 = values[65];
    let a66 = values[66];
    let a67 = values[67];
    let a68 = values[68];
    let a69 = values[69];
    let a70 = values[70];
    let a71 = values[71];
    let a72 = values[72];
    let a73 = values[73];
    let a74 = values[74];
    let a75 = values[75];
    let a76 = values[76];
    let a77 = values[77];
    let a78 = values[78];
    let a79 = values[79];
    let a80 = values[80];
    let a81 = values[81];
    let a82 = values[82];
    let a83 = values[83];
    let a84 = values[84];
    let a85 = values[85];
    let a86 = values[86];
    let a87 = values[87];
    let a88 = values[88];
    let a89 = values[89];
    let a90 = values[90];
    let a91 = values[91];
    let a92 = values[92];
    let a93 = values[93];
    let a94 = values[94];
    let a95 = values[95];
    let a96 = values[96];
    let a97 = values[97];
    let a98 = values[98];
    let a99 = values[99];
    let a100 = values[100];
    let a101 = values[101];
    let a102 = values[102];
    let a103 = values[103];
    let a104 = values[104];
    let a105 = values[105];
    let a106 = values[106];
    let a107 = values[107];
    let a108 = values[108];
    let a109 = values[109];
    let a110 = values[110];
    let a111 = values[111];
    let a112 = values[112];
    let a113 = values[113];
    let a114 = values[114];
    let a115 = values[115];
    let a116 = values[116];
    let a117 = values[117];
    let a118 = values[118];
    let a119 = values[119];
    let a120 = values[120];
    let a121 = values[121];
    let a122 = values[122];
    let a123 = values[123];
    let a124 = values[124];
    let a125 = values[125];
    let a126 = values[126];
    let a127 = values[127];
    let (a0, a64) = (a0 + a64, a0 - a64);
    let (a16, a80) = (a16 + a80, a16 - a80);
    let (a32, a96) = (a32 + a96, a32 - a96);
    let (a48, a112) = (a48 + a112, a48 - a112);
    let a80 = (a80 << 24);
    let a96 = (a96 << 48);
    let a112 = (a112 << 72);
    let (a0, a32) = (a0 + a32, a0 - a32);
    let (a16, a48) = (a16 + a48, a16 - a48);
    let a48 = (a48 << 48);
    let (a0, a16) = (a0 + a16, a0 - a16);
    let (a32, a48) = (a32 + a48, a32 - a48);
    let (a64, a96) = (a64 + a96, a64 - a96);
    let (a80, a112) = (a80 + a112, a80 - a112);
    let a112 = (a112 << 48);
    let (a64, a80) = (a64 + a80, a64 - a80);
    let (a96, a112) = (a96 + a112, a96 - a112);
    let (a1, a65) = (a1 + a65, a1 - a65);
    let (a17, a81) = (a17 + a81, a17 - a81);
    let (a33, a97) = (a33 + a97, a33 - a97);
    let (a49, a113) = (a49 + a113, a49 - a113);
    let a81 = (a81 << 24);
    let a97 = (a97 << 48);
    let a113 = (a113 << 72);
    let (a1, a33) = (a1 + a33, a1 - a33);
    let (a17, a49) = (a17 + a49, a17 - a49);
    let a49 = (a49 << 48);
    let (a1, a17) = (a1 + a17, a1 - a17);
    let (a33, a49) = (a33 + a49, a33 - a49);
    let (a65, a97) = (a65 + a97, a65 - a97);
    let (a81, a113) = (a81 + a113, a81 - a113);
    let a113 = (a113 << 48);
    let (a65, a81) = (a65 + a81, a65 - a81);
    let (a97, a113) = (a97 + a113, a97 - a113);
    let (a2, a66) = (a2 + a66, a2 - a66);
    let (a18, a82) = (a18 + a82, a18 - a82);
    let (a34, a98) = (a34 + a98, a34 - a98);
    let (a50, a114) = (a50 + a114, a50 - a114);
    let a82 = (a82 << 24);
    let a98 = (a98 << 48);
    let a114 = (a114 << 72);
    let (a2, a34) = (a2 + a34, a2 - a34);
    let (a18, a50) = (a18 + a50, a18 - a50);
    let a50 = (a50 << 48);
    let (a2, a18) = (a2 + a18, a2 - a18);
    let (a34, a50) = (a34 + a50, a34 - a50);
    let (a66, a98) = (a66 + a98, a66 - a98);
    let (a82, a114) = (a82 + a114, a82 - a114);
    let a114 = (a114 << 48);
    let (a66, a82) = (a66 + a82, a66 - a82);
    let (a98, a114) = (a98 + a114, a98 - a114);
    let (a3, a67) = (a3 + a67, a3 - a67);
    let (a19, a83) = (a19 + a83, a19 - a83);
    let (a35, a99) = (a35 + a99, a35 - a99);
    let (a51, a115) = (a51 + a115, a51 - a115);
    let a83 = (a83 << 24);
    let a99 = (a99 << 48);
    let a115 = (a115 << 72);
    let (a3, a35) = (a3 + a35, a3 - a35);
    let (a19, a51) = (a19 + a51, a19 - a51);
    let a51 = (a51 << 48);
    let (a3, a19) = (a3 + a19, a3 - a19);
    let (a35, a51) = (a35 + a51, a35 - a51);
    let (a67, a99) = (a67 + a99, a67 - a99);
    let (a83, a115) = (a83 + a115, a83 - a115);
    let a115 = (a115 << 48);
    let (a67, a83) = (a67 + a83, a67 - a83);
    let (a99, a115) = (a99 + a115, a99 - a115);
    let (a4, a68) = (a4 + a68, a4 - a68);
    let (a20, a84) = (a20 + a84, a20 - a84);
    let (a36, a100) = (a36 + a100, a36 - a100);
    let (a52, a116) = (a52 + a116, a52 - a116);
    let a84 = (a84 << 24);
    let a100 = (a100 << 48);
    let a116 = (a116 << 72);
    let (a4, a36) = (a4 + a36, a4 - a36);
    let (a20, a52) = (a20 + a52, a20 - a52);
    let a52 = (a52 << 48);
    let (a4, a20) = (a4 + a20, a4 - a20);
    let (a36, a52) = (a36 + a52, a36 - a52);
    let (a68, a100) = (a68 + a100, a68 - a100);
    let (a84, a116) = (a84 + a116, a84 - a116);
    let a116 = (a116 << 48);
    let (a68, a84) = (a68 + a84, a68 - a84);
    let (a100, a116) = (a100 + a116, a100 - a116);
    let (a5, a69) = (a5 + a69, a5 - a69);
    let (a21, a85) = (a21 + a85, a21 - a85);
    let (a37, a101) = (a37 + a101, a37 - a101);
    let (a53, a117) = (a53 + a117, a53 - a117);
    let a85 = (a85 << 24);
    let a101 = (a101 << 48);
    let a117 = (a117 << 72);
    let (a5, a37) = (a5 + a37, a5 - a37);
    let (a21, a53) = (a21 + a53, a21 - a53);
    let a53 = (a53 << 48);
    let (a5, a21) = (a5 + a21, a5 - a21);
    let (a37, a53) = (a37 + a53, a37 - a53);
    let (a69, a101) = (a69 + a101, a69 - a101);
    let (a85, a117) = (a85 + a117, a85 - a117);
    let a117 = (a117 << 48);
    let (a69, a85) = (a69 + a85, a69 - a85);
    let (a101, a117) = (a101 + a117, a101 - a117);
    let (a6, a70) = (a6 + a70, a6 - a70);
    let (a22, a86) = (a22 + a86, a22 - a86);
    let (a38, a102) = (a38 + a102, a38 - a102);
    let (a54, a118) = (a54 + a118, a54 - a118);
    let a86 = (a86 << 24);
    let a102 = (a102 << 48);
    let a118 = (a118 << 72);
    let (a6, a38) = (a6 + a38, a6 - a38);
    let (a22, a54) = (a22 + a54, a22 - a54);
    let a54 = (a54 << 48);
    let (a6, a22) = (a6 + a22, a6 - a22);
    let (a38, a54) = (a38 + a54, a38 - a54);
    let (a70, a102) = (a70 + a102, a70 - a102);
    let (a86, a118) = (a86 + a118, a86 - a118);
    let a118 = (a118 << 48);
    let (a70, a86) = (a70 + a86, a70 - a86);
    let (a102, a118) = (a102 + a118, a102 - a118);
    let (a7, a71) = (a7 + a71, a7 - a71);
    let (a23, a87) = (a23 + a87, a23 - a87);
    let (a39, a103) = (a39 + a103, a39 - a103);
    let (a55, a119) = (a55 + a119, a55 - a119);
    let a87 = (a87 << 24);
    let a103 = (a103 << 48);
    let a119 = (a119 << 72);
    let (a7, a39) = (a7 + a39, a7 - a39);
    let (a23, a55) = (a23 + a55, a23 - a55);
    let a55 = (a55 << 48);
    let (a7, a23) = (a7 + a23, a7 - a23);
    let (a39, a55) = (a39 + a55, a39 - a55);
    let (a71, a103) = (a71 + a103, a71 - a103);
    let (a87, a119) = (a87 + a119, a87 - a119);
    let a119 = (a119 << 48);
    let (a71, a87) = (a71 + a87, a71 - a87);
    let (a103, a119) = (a103 + a119, a103 - a119);
    let (a8, a72) = (a8 + a72, a8 - a72);
    let (a24, a88) = (a24 + a88, a24 - a88);
    let (a40, a104) = (a40 + a104, a40 - a104);
    let (a56, a120) = (a56 + a120, a56 - a120);
    let a88 = (a88 << 24);
    let a104 = (a104 << 48);
    let a120 = (a120 << 72);
    let (a8, a40) = (a8 + a40, a8 - a40);
    let (a24, a56) = (a24 + a56, a24 - a56);
    let a56 = (a56 << 48);
    let (a8, a24) = (a8 + a24, a8 - a24);
    let (a40, a56) = (a40 + a56, a40 - a56);
    let (a72, a104) = (a72 + a104, a72 - a104);
    let (a88, a120) = (a88 + a120, a88 - a120);
    let a120 = (a120 << 48);
    let (a72, a88) = (a72 + a88, a72 - a88);
    let (a104, a120) = (a104 + a120, a104 - a120);
    let (a9, a73) = (a9 + a73, a9 - a73);
    let (a25, a89) = (a25 + a89, a25 - a89);
    let (a41, a105) = (a41 + a105, a41 - a105);
    let (a57, a121) = (a57 + a121, a57 - a121);
    let a89 = (a89 << 24);
    let a105 = (a105 << 48);
    let a121 = (a121 << 72);
    let (a9, a41) = (a9 + a41, a9 - a41);
    let (a25, a57) = (a25 + a57, a25 - a57);
    let a57 = (a57 << 48);
    let (a9, a25) = (a9 + a25, a9 - a25);
    let (a41, a57) = (a41 + a57, a41 - a57);
    let (a73, a105) = (a73 + a105, a73 - a105);
    let (a89, a121) = (a89 + a121, a89 - a121);
    let a121 = (a121 << 48);
    let (a73, a89) = (a73 + a89, a73 - a89);
    let (a105, a121) = (a105 + a121, a105 - a121);
    let (a10, a74) = (a10 + a74, a10 - a74);
    let (a26, a90) = (a26 + a90, a26 - a90);
    let (a42, a106) = (a42 + a106, a42 - a106);
    let (a58, a122) = (a58 + a122, a58 - a122);
    let a90 = (a90 << 24);
    let a106 = (a106 << 48);
    let a122 = (a122 << 72);
    let (a10, a42) = (a10 + a42, a10 - a42);
    let (a26, a58) = (a26 + a58, a26 - a58);
    let a58 = (a58 << 48);
    let (a10, a26) = (a10 + a26, a10 - a26);
    let (a42, a58) = (a42 + a58, a42 - a58);
    let (a74, a106) = (a74 + a106, a74 - a106);
    let (a90, a122) = (a90 + a122, a90 - a122);
    let a122 = (a122 << 48);
    let (a74, a90) = (a74 + a90, a74 - a90);
    let (a106, a122) = (a106 + a122, a106 - a122);
    let (a11, a75) = (a11 + a75, a11 - a75);
    let (a27, a91) = (a27 + a91, a27 - a91);
    let (a43, a107) = (a43 + a107, a43 - a107);
    let (a59, a123) = (a59 + a123, a59 - a123);
    let a91 = (a91 << 24);
    let a107 = (a107 << 48);
    let a123 = (a123 << 72);
    let (a11, a43) = (a11 + a43, a11 - a43);
    let (a27, a59) = (a27 + a59, a27 - a59);
    let a59 = (a59 << 48);
    let (a11, a27) = (a11 + a27, a11 - a27);
    let (a43, a59) = (a43 + a59, a43 - a59);
    let (a75, a107) = (a75 + a107, a75 - a107);
    let (a91, a123) = (a91 + a123, a91 - a123);
    let a123 = (a123 << 48);
    let (a75, a91) = (a75 + a91, a75 - a91);
    let (a107, a123) = (a107 + a123, a107 - a123);
    let (a12, a76) = (a12 + a76, a12 - a76);
    let (a28, a92) = (a28 + a92, a28 - a92);
    let (a44, a108) = (a44 + a108, a44 - a108);
    let (a60, a124) = (a60 + a124, a60 - a124);
    let a92 = (a92 << 24);
    let a108 = (a108 << 48);
    let a124 = (a124 << 72);
    let (a12, a44) = (a12 + a44, a12 - a44);
    let (a28, a60) = (a28 + a60, a28 - a60);
    let a60 = (a60 << 48);
    let (a12, a28) = (a12 + a28, a12 - a28);
    let (a44, a60) = (a44 + a60, a44 - a60);
    let (a76, a108) = (a76 + a108, a76 - a108);
    let (a92, a124) = (a92 + a124, a92 - a124);
    let a124 = (a124 << 48);
    let (a76, a92) = (a76 + a92, a76 - a92);
    let (a108, a124) = (a108 + a124, a108 - a124);
    let (a13, a77) = (a13 + a77, a13 - a77);
    let (a29, a93) = (a29 + a93, a29 - a93);
    let (a45, a109) = (a45 + a109, a45 - a109);
    let (a61, a125) = (a61 + a125, a61 - a125);
    let a93 = (a93 << 24);
    let a109 = (a109 << 48);
    let a125 = (a125 << 72);
    let (a13, a45) = (a13 + a45, a13 - a45);
    let (a29, a61) = (a29 + a61, a29 - a61);
    let a61 = (a61 << 48);
    let (a13, a29) = (a13 + a29, a13 - a29);
    let (a45, a61) = (a45 + a61, a45 - a61);
    let (a77, a109) = (a77 + a109, a77 - a109);
    let (a93, a125) = (a93 + a125, a93 - a125);
    let a125 = (a125 << 48);
    let (a77, a93) = (a77 + a93, a77 - a93);
    let (a109, a125) = (a109 + a125, a109 - a125);
    let (a14, a78) = (a14 + a78, a14 - a78);
    let (a30, a94) = (a30 + a94, a30 - a94);
    let (a46, a110) = (a46 + a110, a46 - a110);
    let (a62, a126) = (a62 + a126, a62 - a126);
    let a94 = (a94 << 24);
    let a110 = (a110 << 48);
    let a126 = (a126 << 72);
    let (a14, a46) = (a14 + a46, a14 - a46);
    let (a30, a62) = (a30 + a62, a30 - a62);
    let a62 = (a62 << 48);
    let (a14, a30) = (a14 + a30, a14 - a30);
    let (a46, a62) = (a46 + a62, a46 - a62);
    let (a78, a110) = (a78 + a110, a78 - a110);
    let (a94, a126) = (a94 + a126, a94 - a126);
    let a126 = (a126 << 48);
    let (a78, a94) = (a78 + a94, a78 - a94);
    let (a110, a126) = (a110 + a126, a110 - a126);
    let (a15, a79) = (a15 + a79, a15 - a79);
    let (a31, a95) = (a31 + a95, a31 - a95);
    let (a47, a111) = (a47 + a111, a47 - a111);
    let (a63, a127) = (a63 + a127, a63 - a127);
    let a95 = (a95 << 24);
    let a111 = (a111 << 48);
    let a127 = (a127 << 72);
    let (a15, a47) = (a15 + a47, a15 - a47);
    let (a31, a63) = (a31 + a63, a31 - a63);
    let a63 = (a63 << 48);
    let (a15, a31) = (a15 + a31, a15 - a31);
    let (a47, a63) = (a47 + a63, a47 - a63);
    let (a79, a111) = (a79 + a111, a79 - a111);
    let (a95, a127) = (a95 + a127, a95 - a127);
    let a127 = (a127 << 48);
    let (a79, a95) = (a79 + a95, a79 - a95);
    let (a111, a127) = (a111 + a127, a111 - a127);
    let a65 = (a65 << 25) + (-(a65 << 73));
    let a33 = (a33 << 3);
    let a97 = (a97 << 28) + (-(a97 << 76));
    let a17 = (a17 << 6);
    let a81 = (a81 << 31) + (-(a81 << 79));
    let a49 = (a49 << 9);
    let a113 = (a113 << 34) + (-(a113 << 82));
    let a66 = (a66 << 3);
    let a34 = (a34 << 6);
    let a98 = (a98 << 9);
    let a18 = (a18 << 12);
    let a82 = (a82 << 15);
    let a50 = (a50 << 18);
    let a114 = (a114 << 21);
    let a67 = (a67 << 28) + (-(a67 << 76));
    let a35 = (a35 << 9);
    let a99 = (a99 << 37) + (-(a99 << 85));
    let a19 = (a19 << 18);
    let a83 = (a83 << 46) + (-(a83 << 94));
    let a51 = (a51 << 27);
    let a115 = (a115 << 55) + (a115 << 7);
    let a68 = (a68 << 6);
    let a36 = (a36 << 12);
    let a100 = (a100 << 18);
    let a20 = (a20 << 24);
    let a84 = (a84 << 30);
    let a52 = (a52 << 36);
    let a116 = (a116 << 42);
    let a69 = (a69 << 31) + (-(a69 << 79));
    let a37 = (a37 << 15);
    let a101 = (a101 << 46) + (-(a101 << 94));
    let a21 = (a21 << 30);
    let a85 = (a85 << 61) + (a85 << 13);
    let a53 = (a53 << 45);
    let a117 = (a117 << 76) + (a117 << 28);
    let a70 = (a70 << 9);
    let a38 = (a38 << 18);
    let a102 = (a102 << 27);
    let a22 = (a22 << 36);
    let a86 = (a86 << 45);
    let a54 = (a54 << 54);
    let a118 = (a118 << 63);
    let a71 = (a71 << 34) + (-(a71 << 82));
    let a39 = (a39 << 21);
    let a103 = (a103 << 55) + (a103 << 7);
    let a23 = (a23 << 42);
    let a87 = (a87 << 76) + (a87 << 28);
    let a55 = (a55 << 63);
    let a119 = (-(a119 << 1)) + (a119 << 49);
    let a72 = (a72 << 12);
    let a40 = (a40 << 24);
    let a104 = (a104 << 36);
    let a24 = (a24 << 48);
    let a88 = (a88 << 60);
    let a56 = (a56 << 72);
    let a120 = (a120 << 84);
    let a73 = (a73 << 37) + (-(a73 << 85));
    let a41 = (a41 << 27);
    let a105 = (a105 << 64) + (a105 << 16);
    let a25 = (a25 << 54);
    let a89 = (a89 << 91) + (a89 << 43);
    let a57 = (a57 << 81);
    let a121 = (-(a121 << 22)) + (a121 << 70);
    let a74 = (a74 << 15);
    let a42 = (a42 << 30);
    let a106 = (a106 << 45);
    let a26 = (a26 << 60);
    let a90 = (a90 << 75);
    let a58 = (a58 << 90);
    let a122 = (-(a122 << 9));
    let a75 = (a75 << 40) + (-(a75 << 88));
    let a43 = (a43 << 33);
    let a107 = (a107 << 73) + (a107 << 25);
    let a27 = (a27 << 66);
    let a91 = (-(a91 << 10)) + (a91 << 58);
    let a59 = (-(a59 << 3));
    let a123 = (-(a123 << 43)) + (a123 << 91);
    let a76 = (a76 << 18);
    let a44 = (a44 << 36);
    let a108 = (a108 << 54);
    let a28 = (a28 << 72);
    let a92 = (a92 << 90);
    let a60 = (-(a60 << 12));
    let a124 = (-(a124 << 30));
    let a77 = (a77 << 43) + (-(a77 << 91));
    let a45 = (a45 << 39);
    let a109 = (a109 << 82) + (a109 << 34);
    let a29 = (a29 << 78);
    let a93 = (-(a93 << 25)) + (a93 << 73);
    let a61 = (-(a61 << 21));
    let a125 = (-(a125 << 64)) + (-(a125 << 16));
    let a78 = (a78 << 21);
    let a46 = (a46 << 42);
    let a110 = (a110 << 63);
    let a30 = (a30 << 84);
    let a94 = (-(a94 << 9));
    let a62 = (-(a62 << 30));
    let a126 = (-(a126 << 51));
    let a79 = (a79 << 46) + (-(a79 << 94));
    let a47 = (a47 << 45);
    let a111 = (a111 << 91) + (a111 << 43);
    let a31 = (a31 << 90);
    let a95 = (-(a95 << 40)) + (a95 << 88);
    let a63 = (-(a63 << 39));
    let a127 = (-(a127 << 85)) + (-(a127 << 37));
    let (a0, a8) = (a0 + a8, a0 - a8);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let a12 = (a12 << 48);
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let (a5, a13) = (a5 + a13, a5 - a13);
    let a13 = (a13 << 48);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a2, a10) = (a2 + a10, a2 - a10);
    let (a6, a14) = (a6 + a14, a6 - a14);
    let a14 = (a14 << 48);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let (a7, a15) = (a7 + a15, a7 - a15);
    let a15 = (a15 << 48);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a9 = (a9 << 12);
    let a5 = (a5 << 24);
    let a13 = (a13 << 36);
    let a10 = (a10 << 24);
    let a6 = (a6 << 48);
    let a14 = (a14 << 72);
    let a11 = (a11 << 36);
    let a7 = (a7 << 72);
    let a15 = (-(a15 << 12));
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a64, a72) = (a64 + a72, a64 - a72);
    let (a68, a76) = (a68 + a76, a68 - a76);
    let a76 = (a76 << 48);
    let (a64, a68) = (a64 + a68, a64 - a68);
    let (a72, a76) = (a72 + a76, a72 - a76);
    let (a65, a73) = (a65 + a73, a65 - a73);
    let (a69, a77) = (a69 + a77, a69 - a77);
    let a77 = (a77 << 48);
    let (a65, a69) = (a65 + a69, a65 - a69);
    let (a73, a77) = (a73 + a77, a73 - a77);
    let (a66, a74) = (a66 + a74, a66 - a74);
    let (a70, a78) = (a70 + a78, a70 - a78);
    let a78 = (a78 << 48);
    let (a66, a70) = (a66 + a70, a66 - a70);
    let (a74, a78) = (a74 + a78, a74 - a78);
    let (a67, a75) = (a67 + a75, a67 - a75);
    let (a71, a79) = (a71 + a79, a71 - a79);
    let a79 = (a79 << 48);
    let (a67, a71) = (a67 + a71, a67 - a71);
    let (a75, a79) = (a75 + a79, a75 - a79);
    let a73 = (a73 << 12);
    let a69 = (a69 << 24);
    let a77 = (a77 << 36);
    let a74 = (a74 << 24);
    let a70 = (a70 << 48);
    let a78 = (a78 << 72);
    let a75 = (a75 << 36);
    let a71 = (a71 << 72);
    let a79 = (-(a79 << 12));
    let (a64, a66) = (a64 + a66, a64 - a66);
    let (a65, a67) = (a65 + a67, a65 - a67);
    let a67 = (a67 << 48);
    let (a64, a65) = (a64 + a65, a64 - a65);
    let (a66, a67) = (a66 + a67, a66 - a67);
    let (a72, a74) = (a72 + a74, a72 - a74);
    let (a73, a75) = (a73 + a75, a73 - a75);
    let a75 = (a75 << 48);
    let (a72, a73) = (a72 + a73, a72 - a73);
    let (a74, a75) = (a74 + a75, a74 - a75);
    let (a68, a70) = (a68 + a70, a68 - a70);
    let (a69, a71) = (a69 + a71, a69 - a71);
    let a71 = (a71 << 48);
    let (a68, a69) = (a68 + a69, a68 - a69);
    let (a70, a71) = (a70 + a71, a70 - a71);
    let (a76, a78) = (a76 + a78, a76 - a78);
    let (a77, a79) = (a77 + a79, a77 - a79);
    let a79 = (a79 << 48);
    let (a76, a77) = (a76 + a77, a76 - a77);
    let (a78, a79) = (a78 + a79, a78 - a79);
    let (a32, a40) = (a32 + a40, a32 - a40);
    let (a36, a44) = (a36 + a44, a36 - a44);
    let a44 = (a44 << 48);
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a40, a44) = (a40 + a44, a40 - a44);
    let (a33, a41) = (a33 + a41, a33 - a41);
    let (a37, a45) = (a37 + a45, a37 - a45);
    let a45 = (a45 << 48);
    let (a33, a37) = (a33 + a37, a33 - a37);
    let (a41, a45) = (a41 + a45, a41 - a45);
    let (a34, a42) = (a34 + a42, a34 - a42);
    let (a38, a46) = (a38 + a46, a38 - a46);
    let a46 = (a46 << 48);
    let (a34, a38) = (a34 + a38, a34 - a38);
    let (a42, a46) = (a42 + a46, a42 - a46);
    let (a35, a43) = (a35 + a43, a35 - a43);
    let (a39, a47) = (a39 + a47, a39 - a47);
    let a47 = (a47 << 48);
    let (a35, a39) = (a35 + a39, a35 - a39);
    let (a43, a47) = (a43 + a47, a43 - a47);
    let a41 = (a41 << 12);
    let a37 = (a37 << 24);
    let a45 = (a45 << 36);
    let a42 = (a42 << 24);
    let a38 = (a38 << 48);
    let a46 = (a46 << 72);
    let a43 = (a43 << 36);
    let a39 = (a39 << 72);
    let a47 = (-(a47 << 12));
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a40, a42) = (a40 + a42, a40 - a42);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let a43 = (a43 << 48);
    let (a40, a41) = (a40 + a41, a40 - a41);
    let (a42, a43) = (a42 + a43, a42 - a43);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    let (a44, a46) = (a44 + a46, a44 - a46);
    let (a45, a47) = (a45 + a47, a45 - a47);
    let a47 = (a47 << 48);
    let (a44, a45) = (a44 + a45, a44 - a45);
    let (a46, a47) = (a46 + a47, a46 - a47);
    let (a96, a104) = (a96 + a104, a96 - a104);
    let (a100, a108) = (a100 + a108, a100 - a108);
    let a108 = (a108 << 48);
    let (a96, a100) = (a96 + a100, a96 - a100);
    let (a104, a108) = (a104 + a108, a104 - a108);
    let (a97, a105) = (a97 + a105, a97 - a105);
    let (a101, a109) = (a101 + a109, a101 - a109);
    let a109 = (a109 << 48);
    let (a97, a101) = (a97 + a101, a97 - a101);
    let (a105, a109) = (a105 + a109, a105 - a109);
    let (a98, a106) = (a98 + a106, a98 - a106);
    let (a102, a110) = (a102 + a110, a102 - a110);
    let a110 = (a110 << 48);
    let (a98, a102) = (a98 + a102, a98 - a102);
    let (a106, a110) = (a106 + a110, a106 - a110);
    let (a99, a107) = (a99 + a107, a99 - a107);
    let (a103, a111) = (a103 + a111, a103 - a111);
    let a111 = (a111 << 48);
    let (a99, a103) = (a99 + a103, a99 - a103);
    let (a107, a111) = (a107 + a111, a107 - a111);
    let a105 = (a105 << 12);
    let a101 = (a101 << 24);
    let a109 = (a109 << 36);
    let a106 = (a106 << 24);
    let a102 = (a102 << 48);
    let a110 = (a110 << 72);
    let a107 = (a107 << 36);
    let a103 = (a103 << 72);
    let a111 = (-(a111 << 12));
    let (a96, a98) = (a96 + a98, a96 - a98);
    let (a97, a99) = (a97 + a99, a97 - a99);
    let a99 = (a99 << 48);
    let (a96, a97) = (a96 + a97, a96 - a97);
    let (a98, a99) = (a98 + a99, a98 - a99);
    let (a104, a106) = (a104 + a106, a104 - a106);
    let (a105, a107) = (a105 + a107, a105 - a107);
    let a107 = (a107 << 48);
    let (a104, a105) = (a104 + a105, a104 - a105);
    let (a106, a107) = (a106 + a107, a106 - a107);
    let (a100, a102) = (a100 + a102, a100 - a102);
    let (a101, a103) = (a101 + a103, a101 - a103);
    let a103 = (a103 << 48);
    let (a100, a101) = (a100 + a101, a100 - a101);
    let (a102, a103) = (a102 + a103, a102 - a103);
    let (a108, a110) = (a108 + a110, a108 - a110);
    let (a109, a111) = (a109 + a111, a109 - a111);
    let a111 = (a111 << 48);
    let (a108, a109) = (a108 + a109, a108 - a109);
    let (a110, a111) = (a110 + a111, a110 - a111);
    let (a16, a24) = (a16 + a24, a16 - a24);
    let (a20, a28) = (a20 + a28, a20 - a28);
    let a28 = (a28 << 48);
    let (a16, a20) = (a16 + a20, a16 - a20);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a17, a25) = (a17 + a25, a17 - a25);
    let (a21, a29) = (a21 + a29, a21 - a29);
    let a29 = (a29 << 48);
    let (a17, a21) = (a17 + a21, a17 - a21);
    let (a25, a29) = (a25 + a29, a25 - a29);
    let (a18, a26) = (a18 + a26, a18 - a26);
    let (a22, a30) = (a22 + a30, a22 - a30);
    let a30 = (a30 << 48);
    let (a18, a22) = (a18 + a22, a18 - a22);
    let (a26, a30) = (a26 + a30, a26 - a30);
    let (a19, a27) = (a19 + a27, a19 - a27);
    let (a23, a31) = (a23 + a31, a23 - a31);
    let a31 = (a31 << 48);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let (a27, a31) = (a27 + a31, a27 - a31);
    let a25 = (a25 << 12);
    let a21 = (a21 << 24);
    let a29 = (a29 << 36);
    let a26 = (a26 << 24);
    let a22 = (a22 << 48);
    let a30 = (a30 << 72);
    let a27 = (a27 << 36);
    let a23 = (a23 << 72);
    let a31 = (-(a31 << 12));
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a80, a88) = (a80 + a88, a80 - a88);
    let (a84, a92) = (a84 + a92, a84 - a92);
    let a92 = (a92 << 48);
    let (a80, a84) = (a80 + a84, a80 - a84);
    let (a88, a92) = (a88 + a92, a88 - a92);
    let (a81, a89) = (a81 + a89, a81 - a89);
    let (a85, a93) = (a85 + a93, a85 - a93);
    let a93 = (a93 << 48);
    let (a81, a85) = (a81 + a85, a81 - a85);
    let (a89, a93) = (a89 + a93, a89 - a93);
    let (a82, a90) = (a82 + a90, a82 - a90);
    let (a86, a94) = (a86 + a94, a86 - a94);
    let a94 = (a94 << 48);
    let (a82, a86) = (a82 + a86, a82 - a86);
    let (a90, a94) = (a90 + a94, a90 - a94);
    let (a83, a91) = (a83 + a91, a83 - a91);
    let (a87, a95) = (a87 + a95, a87 - a95);
    let a95 = (a95 << 48);
    let (a83, a87) = (a83 + a87, a83 - a87);
    let (a91, a95) = (a91 + a95, a91 - a95);
    let a89 = (a89 << 12);
    let a85 = (a85 << 24);
    let a93 = (a93 << 36);
    let a90 = (a90 << 24);
    let a86 = (a86 << 48);
    let a94 = (a94 << 72);
    let a91 = (a91 << 36);
    let a87 = (a87 << 72);
    let a95 = (-(a95 << 12));
    let (a80, a82) = (a80 + a82, a80 - a82);
    let (a81, a83) = (a81 + a83, a81 - a83);
    let a83 = (a83 << 48);
    let (a80, a81) = (a80 + a81, a80 - a81);
    let (a82, a83) = (a82 + a83, a82 - a83);
    let (a88, a90) = (a88 + a90, a88 - a90);
    let (a89, a91) = (a89 + a91, a89 - a91);
    let a91 = (a91 << 48);
    let (a88, a89) = (a88 + a89, a88 - a89);
    let (a90, a91) = (a90 + a91, a90 - a91);
    let (a84, a86) = (a84 + a86, a84 - a86);
    let (a85, a87) = (a85 + a87, a85 - a87);
    let a87 = (a87 << 48);
    let (a84, a85) = (a84 + a85, a84 - a85);
    let (a86, a87) = (a86 + a87, a86 - a87);
    let (a92, a94) = (a92 + a94, a92 - a94);
    let (a93, a95) = (a93 + a95, a93 - a95);
    let a95 = (a95 << 48);
    let (a92, a93) = (a92 + a93, a92 - a93);
    let (a94, a95) = (a94 + a95, a94 - a95);
    let (a48, a56) = (a48 + a56, a48 - a56);
    let (a52, a60) = (a52 + a60, a52 - a60);
    let a60 = (a60 << 48);
    let (a48, a52) = (a48 + a52, a48 - a52);
    let (a56, a60) = (a56 + a60, a56 - a60);
    let (a49, a57) = (a49 + a57, a49 - a57);
    let (a53, a61) = (a53 + a61, a53 - a61);
    let a61 = (a61 << 48);
    let (a49, a53) = (a49 + a53, a49 - a53);
    let (a57, a61) = (a57 + a61, a57 - a61);
    let (a50, a58) = (a50 + a58, a50 - a58);
    let (a54, a62) = (a54 + a62, a54 - a62);
    let a62 = (a62 << 48);
    let (a50, a54) = (a50 + a54, a50 - a54);
    let (a58, a62) = (a58 + a62, a58 - a62);
    let (a51, a59) = (a51 + a59, a51 - a59);
    let (a55, a63) = (a55 + a63, a55 - a63);
    let a63 = (a63 << 48);
    let (a51, a55) = (a51 + a55, a51 - a55);
    let (a59, a63) = (a59 + a63, a59 - a63);
    let a57 = (a57 << 12);
    let a53 = (a53 << 24);
    let a61 = (a61 << 36);
    let a58 = (a58 << 24);
    let a54 = (a54 << 48);
    let a62 = (a62 << 72);
    let a59 = (a59 << 36);
    let a55 = (a55 << 72);
    let a63 = (-(a63 << 12));
    let (a48, a50) = (a48 + a50, a48 - a50);
    let (a49, a51) = (a49 + a51, a49 - a51);
    let a51 = (a51 << 48);
    let (a48, a49) = (a48 + a49, a48 - a49);
    let (a50, a51) = (a50 + a51, a50 - a51);
    let (a56, a58) = (a56 + a58, a56 - a58);
    let (a57, a59) = (a57 + a59, a57 - a59);
    let a59 = (a59 << 48);
    let (a56, a57) = (a56 + a57, a56 - a57);
    let (a58, a59) = (a58 + a59, a58 - a59);
    let (a52, a54) = (a52 + a54, a52 - a54);
    let (a53, a55) = (a53 + a55, a53 - a55);
    let a55 = (a55 << 48);
    let (a52, a53) = (a52 + a53, a52 - a53);
    let (a54, a55) = (a54 + a55, a54 - a55);
    let (a60, a62) = (a60 + a62, a60 - a62);
    let (a61, a63) = (a61 + a63, a61 - a63);
    let a63 = (a63 << 48);
    let (a60, a61) = (a60 + a61, a60 - a61);
    let (a62, a63) = (a62 + a63, a62 - a63);
    let (a112, a120) = (a112 + a120, a112 - a120);
    let (a116, a124) = (a116 + a124, a116 - a124);
    let a124 = (a124 << 48);
    let (a112, a116) = (a112 + a116, a112 - a116);
    let (a120, a124) = (a120 + a124, a120 - a124);
    let (a113, a121) = (a113 + a121, a113 - a121);
    let (a117, a125) = (a117 + a125, a117 - a125);
    let a125 = (a125 << 48);
    let (a113, a117) = (a113 + a117, a113 - a117);
    let (a121, a125) = (a121 + a125, a121 - a125);
    let (a114, a122) = (a114 + a122, a114 - a122);
    let (a118, a126) = (a118 + a126, a118 - a126);
    let a126 = (a126 << 48);
    let (a114, a118) = (a114 + a118, a114 - a118);
    let (a122, a126) = (a122 + a126, a122 - a126);
    let (a115, a123) = (a115 + a123, a115 - a123);
    let (a119, a127) = (a119 + a127, a119 - a127);
    let a127 = (a127 << 48);
    let (a115, a119) = (a115 + a119, a115 - a119);
    let (a123, a127) = (a123 + a127, a123 - a127);
    let a121 = (a121 << 12);
    let a117 = (a117 << 24);
    let a125 = (a125 << 36);
    let a122 = (a122 << 24);
    let a118 = (a118 << 48);
    let a126 = (a126 << 72);
    let a123 = (a123 << 36);
    let a119 = (a119 << 72);
    let a127 = (-(a127 << 12));
    let (a112, a114) = (a112 + a114, a112 - a114);
    let (a113, a115) = (a113 + a115, a113 - a115);
    let a115 = (a115 << 48);
    let (a112, a113) = (a112 + a113, a112 - a113);
    let (a114, a115) = (a114 + a115, a114 - a115);
    let (a120, a122) = (a120 + a122, a120 - a122);
    let (a121, a123) = (a121 + a123, a121 - a123);
    let a123 = (a123 << 48);
    let (a120, a121) = (a120 + a121, a120 - a121);
    let (a122, a123) = (a122 + a123, a122 - a123);
    let (a116, a118) = (a116 + a118, a116 - a118);
    let (a117, a119) = (a117 + a119, a117 - a119);
    let a119 = (a119 << 48);
    let (a116, a117) = (a116 + a117, a116 - a117);
    let (a118, a119) = (a118 + a119, a118 - a119);
    let (a124, a126) = (a124 + a126, a124 - a126);
    let (a125, a127) = (a125 + a127, a125 - a127);
    let a127 = (a127 << 48);
    let (a124, a125) = (a124 + a125, a124 - a125);
    let (a126, a127) = (a126 + a127, a126 - a127);
    values[0] = a0;
    values[1] = a64;
    values[2] = a32;
    values[3] = a96;
    values[4] = a16;
    values[5] = a80;
    values[6] = a48;
    values[7] = a112;
    values[8] = a8;
    values[9] = a72;
    values[10] = a40;
    values[11] = a104;
    values[12] = a24;
    values[13] = a88;
    values[14] = a56;
    values[15] = a120;
    values[16] = a4;
    values[17] = a68;
    values[18] = a36;
    values[19] = a100;
    values[20] = a20;
    values[21] = a84;
    values[22] = a52;
    values[23] = a116;
    values[24] = a12;
    values[25] = a76;
    values[26] = a44;
    values[27] = a108;
    values[28] = a28;
    values[29] = a92;
    values[30] = a60;
    values[31] = a124;
    values[32] = a2;
    values[33] = a66;
    values[34] = a34;
    values[35] = a98;
    values[36] = a18;
    values[37] = a82;
    values[38] = a50;
    values[39] = a114;
    values[40] = a10;
    values[41] = a74;
    values[42] = a42;
    values[43] = a106;
    values[44] = a26;
    values[45] = a90;
    values[46] = a58;
    values[47] = a122;
    values[48] = a6;
    values[49] = a70;
    values[50] = a38;
    values[51] = a102;
    values[52] = a22;
    values[53] = a86;
    values[54] = a54;
    values[55] = a118;
    values[56] = a14;
    values[57] = a78;
    values[58] = a46;
    values[59] = a110;
    values[60] = a30;
    values[61] = a94;
    values[62] = a62;
    values[63] = a126;
    values[64] = a1;
    values[65] = a65;
    values[66] = a33;
    values[67] = a97;
    values[68] = a17;
    values[69] = a81;
    values[70] = a49;
    values[71] = a113;
    values[72] = a9;
    values[73] = a73;
    values[74] = a41;
    values[75] = a105;
    values[76] = a25;
    values[77] = a89;
    values[78] = a57;
    values[79] = a121;
    values[80] = a5;
    values[81] = a69;
    values[82] = a37;
    values[83] = a101;
    values[84] = a21;
    values[85] = a85;
    values[86] = a53;
    values[87] = a117;
    values[88] = a13;
    values[89] = a77;
    values[90] = a45;
    values[91] = a109;
    values[92] = a29;
    values[93] = a93;
    values[94] = a61;
    values[95] = a125;
    values[96] = a3;
    values[97] = a67;
    values[98] = a35;
    values[99] = a99;
    values[100] = a19;
    values[101] = a83;
    values[102] = a51;
    values[103] = a115;
    values[104] = a11;
    values[105] = a75;
    values[106] = a43;
    values[107] = a107;
    values[108] = a27;
    values[109] = a91;
    values[110] = a59;
    values[111] = a123;
    values[112] = a7;
    values[113] = a71;
    values[114] = a39;
    values[115] = a103;
    values[116] = a23;
    values[117] = a87;
    values[118] = a55;
    values[119] = a119;
    values[120] = a15;
    values[121] = a79;
    values[122] = a47;
    values[123] = a111;
    values[124] = a31;
    values[125] = a95;
    values[126] = a63;
    values[127] = a127;
}

/// Size 160 NTT.
fn ntt_160(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 160);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let a60 = values[60];
    let a61 = values[61];
    let a62 = values[62];
    let a63 = values[63];
    let a64 = values[64];
    let a65 = values[65];
    let a66 = values[66];
    let a67 = values[67];
    let a68 = values[68];
    let a69 = values[69];
    let a70 = values[70];
    let a71 = values[71];
    let a72 = values[72];
    let a73 = values[73];
    let a74 = values[74];
    let a75 = values[75];
    let a76 = values[76];
    let a77 = values[77];
    let a78 = values[78];
    let a79 = values[79];
    let a80 = values[80];
    let a81 = values[81];
    let a82 = values[82];
    let a83 = values[83];
    let a84 = values[84];
    let a85 = values[85];
    let a86 = values[86];
    let a87 = values[87];
    let a88 = values[88];
    let a89 = values[89];
    let a90 = values[90];
    let a91 = values[91];
    let a92 = values[92];
    let a93 = values[93];
    let a94 = values[94];
    let a95 = values[95];
    let a96 = values[96];
    let a97 = values[97];
    let a98 = values[98];
    let a99 = values[99];
    let a100 = values[100];
    let a101 = values[101];
    let a102 = values[102];
    let a103 = values[103];
    let a104 = values[104];
    let a105 = values[105];
    let a106 = values[106];
    let a107 = values[107];
    let a108 = values[108];
    let a109 = values[109];
    let a110 = values[110];
    let a111 = values[111];
    let a112 = values[112];
    let a113 = values[113];
    let a114 = values[114];
    let a115 = values[115];
    let a116 = values[116];
    let a117 = values[117];
    let a118 = values[118];
    let a119 = values[119];
    let a120 = values[120];
    let a121 = values[121];
    let a122 = values[122];
    let a123 = values[123];
    let a124 = values[124];
    let a125 = values[125];
    let a126 = values[126];
    let a127 = values[127];
    let a128 = values[128];
    let a129 = values[129];
    let a130 = values[130];
    let a131 = values[131];
    let a132 = values[132];
    let a133 = values[133];
    let a134 = values[134];
    let a135 = values[135];
    let a136 = values[136];
    let a137 = values[137];
    let a138 = values[138];
    let a139 = values[139];
    let a140 = values[140];
    let a141 = values[141];
    let a142 = values[142];
    let a143 = values[143];
    let a144 = values[144];
    let a145 = values[145];
    let a146 = values[146];
    let a147 = values[147];
    let a148 = values[148];
    let a149 = values[149];
    let a150 = values[150];
    let a151 = values[151];
    let a152 = values[152];
    let a153 = values[153];
    let a154 = values[154];
    let a155 = values[155];
    let a156 = values[156];
    let a157 = values[157];
    let a158 = values[158];
    let a159 = values[159];
    let (a0, a80) = (a0 + a80, a0 - a80);
    let (a16, a96) = (a16 + a96, a16 - a96);
    let (a32, a112) = (a32 + a112, a32 - a112);
    let (a48, a128) = (a48 + a128, a48 - a128);
    let (a64, a144) = (a64 + a144, a64 - a144);
    let a96 = a96 * Field::new(18235156514275634624);
    let a112 = a112 * Field::new(1041288259238279555);
    let a128 = a128 * Field::new(17073700798457888299);
    let a144 = a144 * Field::new(15820824984080659046);
    let (a0, a16, a32, a48, a64) = (
        a0 + a16 + a32 + a48 + a64,
        a0 + a16 * R51 + a32 * R52 + a48 * R53 + a64 * R54,
        a0 + a16 * R52 + a32 * R54 + a48 * R51 + a64 * R53,
        a0 + a16 * R53 + a32 * R51 + a48 * R54 + a64 * R52,
        a0 + a16 * R54 + a32 * R53 + a48 * R52 + a64 * R51,
    );
    let (a80, a96, a112, a128, a144) = (
        a80 + a96 + a112 + a128 + a144,
        a80 + a96 * R51 + a112 * R52 + a128 * R53 + a144 * R54,
        a80 + a96 * R52 + a112 * R54 + a128 * R51 + a144 * R53,
        a80 + a96 * R53 + a112 * R51 + a128 * R54 + a144 * R52,
        a80 + a96 * R54 + a112 * R53 + a128 * R52 + a144 * R51,
    );
    let (a1, a81) = (a1 + a81, a1 - a81);
    let (a17, a97) = (a17 + a97, a17 - a97);
    let (a33, a113) = (a33 + a113, a33 - a113);
    let (a49, a129) = (a49 + a129, a49 - a129);
    let (a65, a145) = (a65 + a145, a65 - a145);
    let a97 = a97 * Field::new(18235156514275634624);
    let a113 = a113 * Field::new(1041288259238279555);
    let a129 = a129 * Field::new(17073700798457888299);
    let a145 = a145 * Field::new(15820824984080659046);
    let (a1, a17, a33, a49, a65) = (
        a1 + a17 + a33 + a49 + a65,
        a1 + a17 * R51 + a33 * R52 + a49 * R53 + a65 * R54,
        a1 + a17 * R52 + a33 * R54 + a49 * R51 + a65 * R53,
        a1 + a17 * R53 + a33 * R51 + a49 * R54 + a65 * R52,
        a1 + a17 * R54 + a33 * R53 + a49 * R52 + a65 * R51,
    );
    let (a81, a97, a113, a129, a145) = (
        a81 + a97 + a113 + a129 + a145,
        a81 + a97 * R51 + a113 * R52 + a129 * R53 + a145 * R54,
        a81 + a97 * R52 + a113 * R54 + a129 * R51 + a145 * R53,
        a81 + a97 * R53 + a113 * R51 + a129 * R54 + a145 * R52,
        a81 + a97 * R54 + a113 * R53 + a129 * R52 + a145 * R51,
    );
    let (a2, a82) = (a2 + a82, a2 - a82);
    let (a18, a98) = (a18 + a98, a18 - a98);
    let (a34, a114) = (a34 + a114, a34 - a114);
    let (a50, a130) = (a50 + a130, a50 - a130);
    let (a66, a146) = (a66 + a146, a66 - a146);
    let a98 = a98 * Field::new(18235156514275634624);
    let a114 = a114 * Field::new(1041288259238279555);
    let a130 = a130 * Field::new(17073700798457888299);
    let a146 = a146 * Field::new(15820824984080659046);
    let (a2, a18, a34, a50, a66) = (
        a2 + a18 + a34 + a50 + a66,
        a2 + a18 * R51 + a34 * R52 + a50 * R53 + a66 * R54,
        a2 + a18 * R52 + a34 * R54 + a50 * R51 + a66 * R53,
        a2 + a18 * R53 + a34 * R51 + a50 * R54 + a66 * R52,
        a2 + a18 * R54 + a34 * R53 + a50 * R52 + a66 * R51,
    );
    let (a82, a98, a114, a130, a146) = (
        a82 + a98 + a114 + a130 + a146,
        a82 + a98 * R51 + a114 * R52 + a130 * R53 + a146 * R54,
        a82 + a98 * R52 + a114 * R54 + a130 * R51 + a146 * R53,
        a82 + a98 * R53 + a114 * R51 + a130 * R54 + a146 * R52,
        a82 + a98 * R54 + a114 * R53 + a130 * R52 + a146 * R51,
    );
    let (a3, a83) = (a3 + a83, a3 - a83);
    let (a19, a99) = (a19 + a99, a19 - a99);
    let (a35, a115) = (a35 + a115, a35 - a115);
    let (a51, a131) = (a51 + a131, a51 - a131);
    let (a67, a147) = (a67 + a147, a67 - a147);
    let a99 = a99 * Field::new(18235156514275634624);
    let a115 = a115 * Field::new(1041288259238279555);
    let a131 = a131 * Field::new(17073700798457888299);
    let a147 = a147 * Field::new(15820824984080659046);
    let (a3, a19, a35, a51, a67) = (
        a3 + a19 + a35 + a51 + a67,
        a3 + a19 * R51 + a35 * R52 + a51 * R53 + a67 * R54,
        a3 + a19 * R52 + a35 * R54 + a51 * R51 + a67 * R53,
        a3 + a19 * R53 + a35 * R51 + a51 * R54 + a67 * R52,
        a3 + a19 * R54 + a35 * R53 + a51 * R52 + a67 * R51,
    );
    let (a83, a99, a115, a131, a147) = (
        a83 + a99 + a115 + a131 + a147,
        a83 + a99 * R51 + a115 * R52 + a131 * R53 + a147 * R54,
        a83 + a99 * R52 + a115 * R54 + a131 * R51 + a147 * R53,
        a83 + a99 * R53 + a115 * R51 + a131 * R54 + a147 * R52,
        a83 + a99 * R54 + a115 * R53 + a131 * R52 + a147 * R51,
    );
    let (a4, a84) = (a4 + a84, a4 - a84);
    let (a20, a100) = (a20 + a100, a20 - a100);
    let (a36, a116) = (a36 + a116, a36 - a116);
    let (a52, a132) = (a52 + a132, a52 - a132);
    let (a68, a148) = (a68 + a148, a68 - a148);
    let a100 = a100 * Field::new(18235156514275634624);
    let a116 = a116 * Field::new(1041288259238279555);
    let a132 = a132 * Field::new(17073700798457888299);
    let a148 = a148 * Field::new(15820824984080659046);
    let (a4, a20, a36, a52, a68) = (
        a4 + a20 + a36 + a52 + a68,
        a4 + a20 * R51 + a36 * R52 + a52 * R53 + a68 * R54,
        a4 + a20 * R52 + a36 * R54 + a52 * R51 + a68 * R53,
        a4 + a20 * R53 + a36 * R51 + a52 * R54 + a68 * R52,
        a4 + a20 * R54 + a36 * R53 + a52 * R52 + a68 * R51,
    );
    let (a84, a100, a116, a132, a148) = (
        a84 + a100 + a116 + a132 + a148,
        a84 + a100 * R51 + a116 * R52 + a132 * R53 + a148 * R54,
        a84 + a100 * R52 + a116 * R54 + a132 * R51 + a148 * R53,
        a84 + a100 * R53 + a116 * R51 + a132 * R54 + a148 * R52,
        a84 + a100 * R54 + a116 * R53 + a132 * R52 + a148 * R51,
    );
    let (a5, a85) = (a5 + a85, a5 - a85);
    let (a21, a101) = (a21 + a101, a21 - a101);
    let (a37, a117) = (a37 + a117, a37 - a117);
    let (a53, a133) = (a53 + a133, a53 - a133);
    let (a69, a149) = (a69 + a149, a69 - a149);
    let a101 = a101 * Field::new(18235156514275634624);
    let a117 = a117 * Field::new(1041288259238279555);
    let a133 = a133 * Field::new(17073700798457888299);
    let a149 = a149 * Field::new(15820824984080659046);
    let (a5, a21, a37, a53, a69) = (
        a5 + a21 + a37 + a53 + a69,
        a5 + a21 * R51 + a37 * R52 + a53 * R53 + a69 * R54,
        a5 + a21 * R52 + a37 * R54 + a53 * R51 + a69 * R53,
        a5 + a21 * R53 + a37 * R51 + a53 * R54 + a69 * R52,
        a5 + a21 * R54 + a37 * R53 + a53 * R52 + a69 * R51,
    );
    let (a85, a101, a117, a133, a149) = (
        a85 + a101 + a117 + a133 + a149,
        a85 + a101 * R51 + a117 * R52 + a133 * R53 + a149 * R54,
        a85 + a101 * R52 + a117 * R54 + a133 * R51 + a149 * R53,
        a85 + a101 * R53 + a117 * R51 + a133 * R54 + a149 * R52,
        a85 + a101 * R54 + a117 * R53 + a133 * R52 + a149 * R51,
    );
    let (a6, a86) = (a6 + a86, a6 - a86);
    let (a22, a102) = (a22 + a102, a22 - a102);
    let (a38, a118) = (a38 + a118, a38 - a118);
    let (a54, a134) = (a54 + a134, a54 - a134);
    let (a70, a150) = (a70 + a150, a70 - a150);
    let a102 = a102 * Field::new(18235156514275634624);
    let a118 = a118 * Field::new(1041288259238279555);
    let a134 = a134 * Field::new(17073700798457888299);
    let a150 = a150 * Field::new(15820824984080659046);
    let (a6, a22, a38, a54, a70) = (
        a6 + a22 + a38 + a54 + a70,
        a6 + a22 * R51 + a38 * R52 + a54 * R53 + a70 * R54,
        a6 + a22 * R52 + a38 * R54 + a54 * R51 + a70 * R53,
        a6 + a22 * R53 + a38 * R51 + a54 * R54 + a70 * R52,
        a6 + a22 * R54 + a38 * R53 + a54 * R52 + a70 * R51,
    );
    let (a86, a102, a118, a134, a150) = (
        a86 + a102 + a118 + a134 + a150,
        a86 + a102 * R51 + a118 * R52 + a134 * R53 + a150 * R54,
        a86 + a102 * R52 + a118 * R54 + a134 * R51 + a150 * R53,
        a86 + a102 * R53 + a118 * R51 + a134 * R54 + a150 * R52,
        a86 + a102 * R54 + a118 * R53 + a134 * R52 + a150 * R51,
    );
    let (a7, a87) = (a7 + a87, a7 - a87);
    let (a23, a103) = (a23 + a103, a23 - a103);
    let (a39, a119) = (a39 + a119, a39 - a119);
    let (a55, a135) = (a55 + a135, a55 - a135);
    let (a71, a151) = (a71 + a151, a71 - a151);
    let a103 = a103 * Field::new(18235156514275634624);
    let a119 = a119 * Field::new(1041288259238279555);
    let a135 = a135 * Field::new(17073700798457888299);
    let a151 = a151 * Field::new(15820824984080659046);
    let (a7, a23, a39, a55, a71) = (
        a7 + a23 + a39 + a55 + a71,
        a7 + a23 * R51 + a39 * R52 + a55 * R53 + a71 * R54,
        a7 + a23 * R52 + a39 * R54 + a55 * R51 + a71 * R53,
        a7 + a23 * R53 + a39 * R51 + a55 * R54 + a71 * R52,
        a7 + a23 * R54 + a39 * R53 + a55 * R52 + a71 * R51,
    );
    let (a87, a103, a119, a135, a151) = (
        a87 + a103 + a119 + a135 + a151,
        a87 + a103 * R51 + a119 * R52 + a135 * R53 + a151 * R54,
        a87 + a103 * R52 + a119 * R54 + a135 * R51 + a151 * R53,
        a87 + a103 * R53 + a119 * R51 + a135 * R54 + a151 * R52,
        a87 + a103 * R54 + a119 * R53 + a135 * R52 + a151 * R51,
    );
    let (a8, a88) = (a8 + a88, a8 - a88);
    let (a24, a104) = (a24 + a104, a24 - a104);
    let (a40, a120) = (a40 + a120, a40 - a120);
    let (a56, a136) = (a56 + a136, a56 - a136);
    let (a72, a152) = (a72 + a152, a72 - a152);
    let a104 = a104 * Field::new(18235156514275634624);
    let a120 = a120 * Field::new(1041288259238279555);
    let a136 = a136 * Field::new(17073700798457888299);
    let a152 = a152 * Field::new(15820824984080659046);
    let (a8, a24, a40, a56, a72) = (
        a8 + a24 + a40 + a56 + a72,
        a8 + a24 * R51 + a40 * R52 + a56 * R53 + a72 * R54,
        a8 + a24 * R52 + a40 * R54 + a56 * R51 + a72 * R53,
        a8 + a24 * R53 + a40 * R51 + a56 * R54 + a72 * R52,
        a8 + a24 * R54 + a40 * R53 + a56 * R52 + a72 * R51,
    );
    let (a88, a104, a120, a136, a152) = (
        a88 + a104 + a120 + a136 + a152,
        a88 + a104 * R51 + a120 * R52 + a136 * R53 + a152 * R54,
        a88 + a104 * R52 + a120 * R54 + a136 * R51 + a152 * R53,
        a88 + a104 * R53 + a120 * R51 + a136 * R54 + a152 * R52,
        a88 + a104 * R54 + a120 * R53 + a136 * R52 + a152 * R51,
    );
    let (a9, a89) = (a9 + a89, a9 - a89);
    let (a25, a105) = (a25 + a105, a25 - a105);
    let (a41, a121) = (a41 + a121, a41 - a121);
    let (a57, a137) = (a57 + a137, a57 - a137);
    let (a73, a153) = (a73 + a153, a73 - a153);
    let a105 = a105 * Field::new(18235156514275634624);
    let a121 = a121 * Field::new(1041288259238279555);
    let a137 = a137 * Field::new(17073700798457888299);
    let a153 = a153 * Field::new(15820824984080659046);
    let (a9, a25, a41, a57, a73) = (
        a9 + a25 + a41 + a57 + a73,
        a9 + a25 * R51 + a41 * R52 + a57 * R53 + a73 * R54,
        a9 + a25 * R52 + a41 * R54 + a57 * R51 + a73 * R53,
        a9 + a25 * R53 + a41 * R51 + a57 * R54 + a73 * R52,
        a9 + a25 * R54 + a41 * R53 + a57 * R52 + a73 * R51,
    );
    let (a89, a105, a121, a137, a153) = (
        a89 + a105 + a121 + a137 + a153,
        a89 + a105 * R51 + a121 * R52 + a137 * R53 + a153 * R54,
        a89 + a105 * R52 + a121 * R54 + a137 * R51 + a153 * R53,
        a89 + a105 * R53 + a121 * R51 + a137 * R54 + a153 * R52,
        a89 + a105 * R54 + a121 * R53 + a137 * R52 + a153 * R51,
    );
    let (a10, a90) = (a10 + a90, a10 - a90);
    let (a26, a106) = (a26 + a106, a26 - a106);
    let (a42, a122) = (a42 + a122, a42 - a122);
    let (a58, a138) = (a58 + a138, a58 - a138);
    let (a74, a154) = (a74 + a154, a74 - a154);
    let a106 = a106 * Field::new(18235156514275634624);
    let a122 = a122 * Field::new(1041288259238279555);
    let a138 = a138 * Field::new(17073700798457888299);
    let a154 = a154 * Field::new(15820824984080659046);
    let (a10, a26, a42, a58, a74) = (
        a10 + a26 + a42 + a58 + a74,
        a10 + a26 * R51 + a42 * R52 + a58 * R53 + a74 * R54,
        a10 + a26 * R52 + a42 * R54 + a58 * R51 + a74 * R53,
        a10 + a26 * R53 + a42 * R51 + a58 * R54 + a74 * R52,
        a10 + a26 * R54 + a42 * R53 + a58 * R52 + a74 * R51,
    );
    let (a90, a106, a122, a138, a154) = (
        a90 + a106 + a122 + a138 + a154,
        a90 + a106 * R51 + a122 * R52 + a138 * R53 + a154 * R54,
        a90 + a106 * R52 + a122 * R54 + a138 * R51 + a154 * R53,
        a90 + a106 * R53 + a122 * R51 + a138 * R54 + a154 * R52,
        a90 + a106 * R54 + a122 * R53 + a138 * R52 + a154 * R51,
    );
    let (a11, a91) = (a11 + a91, a11 - a91);
    let (a27, a107) = (a27 + a107, a27 - a107);
    let (a43, a123) = (a43 + a123, a43 - a123);
    let (a59, a139) = (a59 + a139, a59 - a139);
    let (a75, a155) = (a75 + a155, a75 - a155);
    let a107 = a107 * Field::new(18235156514275634624);
    let a123 = a123 * Field::new(1041288259238279555);
    let a139 = a139 * Field::new(17073700798457888299);
    let a155 = a155 * Field::new(15820824984080659046);
    let (a11, a27, a43, a59, a75) = (
        a11 + a27 + a43 + a59 + a75,
        a11 + a27 * R51 + a43 * R52 + a59 * R53 + a75 * R54,
        a11 + a27 * R52 + a43 * R54 + a59 * R51 + a75 * R53,
        a11 + a27 * R53 + a43 * R51 + a59 * R54 + a75 * R52,
        a11 + a27 * R54 + a43 * R53 + a59 * R52 + a75 * R51,
    );
    let (a91, a107, a123, a139, a155) = (
        a91 + a107 + a123 + a139 + a155,
        a91 + a107 * R51 + a123 * R52 + a139 * R53 + a155 * R54,
        a91 + a107 * R52 + a123 * R54 + a139 * R51 + a155 * R53,
        a91 + a107 * R53 + a123 * R51 + a139 * R54 + a155 * R52,
        a91 + a107 * R54 + a123 * R53 + a139 * R52 + a155 * R51,
    );
    let (a12, a92) = (a12 + a92, a12 - a92);
    let (a28, a108) = (a28 + a108, a28 - a108);
    let (a44, a124) = (a44 + a124, a44 - a124);
    let (a60, a140) = (a60 + a140, a60 - a140);
    let (a76, a156) = (a76 + a156, a76 - a156);
    let a108 = a108 * Field::new(18235156514275634624);
    let a124 = a124 * Field::new(1041288259238279555);
    let a140 = a140 * Field::new(17073700798457888299);
    let a156 = a156 * Field::new(15820824984080659046);
    let (a12, a28, a44, a60, a76) = (
        a12 + a28 + a44 + a60 + a76,
        a12 + a28 * R51 + a44 * R52 + a60 * R53 + a76 * R54,
        a12 + a28 * R52 + a44 * R54 + a60 * R51 + a76 * R53,
        a12 + a28 * R53 + a44 * R51 + a60 * R54 + a76 * R52,
        a12 + a28 * R54 + a44 * R53 + a60 * R52 + a76 * R51,
    );
    let (a92, a108, a124, a140, a156) = (
        a92 + a108 + a124 + a140 + a156,
        a92 + a108 * R51 + a124 * R52 + a140 * R53 + a156 * R54,
        a92 + a108 * R52 + a124 * R54 + a140 * R51 + a156 * R53,
        a92 + a108 * R53 + a124 * R51 + a140 * R54 + a156 * R52,
        a92 + a108 * R54 + a124 * R53 + a140 * R52 + a156 * R51,
    );
    let (a13, a93) = (a13 + a93, a13 - a93);
    let (a29, a109) = (a29 + a109, a29 - a109);
    let (a45, a125) = (a45 + a125, a45 - a125);
    let (a61, a141) = (a61 + a141, a61 - a141);
    let (a77, a157) = (a77 + a157, a77 - a157);
    let a109 = a109 * Field::new(18235156514275634624);
    let a125 = a125 * Field::new(1041288259238279555);
    let a141 = a141 * Field::new(17073700798457888299);
    let a157 = a157 * Field::new(15820824984080659046);
    let (a13, a29, a45, a61, a77) = (
        a13 + a29 + a45 + a61 + a77,
        a13 + a29 * R51 + a45 * R52 + a61 * R53 + a77 * R54,
        a13 + a29 * R52 + a45 * R54 + a61 * R51 + a77 * R53,
        a13 + a29 * R53 + a45 * R51 + a61 * R54 + a77 * R52,
        a13 + a29 * R54 + a45 * R53 + a61 * R52 + a77 * R51,
    );
    let (a93, a109, a125, a141, a157) = (
        a93 + a109 + a125 + a141 + a157,
        a93 + a109 * R51 + a125 * R52 + a141 * R53 + a157 * R54,
        a93 + a109 * R52 + a125 * R54 + a141 * R51 + a157 * R53,
        a93 + a109 * R53 + a125 * R51 + a141 * R54 + a157 * R52,
        a93 + a109 * R54 + a125 * R53 + a141 * R52 + a157 * R51,
    );
    let (a14, a94) = (a14 + a94, a14 - a94);
    let (a30, a110) = (a30 + a110, a30 - a110);
    let (a46, a126) = (a46 + a126, a46 - a126);
    let (a62, a142) = (a62 + a142, a62 - a142);
    let (a78, a158) = (a78 + a158, a78 - a158);
    let a110 = a110 * Field::new(18235156514275634624);
    let a126 = a126 * Field::new(1041288259238279555);
    let a142 = a142 * Field::new(17073700798457888299);
    let a158 = a158 * Field::new(15820824984080659046);
    let (a14, a30, a46, a62, a78) = (
        a14 + a30 + a46 + a62 + a78,
        a14 + a30 * R51 + a46 * R52 + a62 * R53 + a78 * R54,
        a14 + a30 * R52 + a46 * R54 + a62 * R51 + a78 * R53,
        a14 + a30 * R53 + a46 * R51 + a62 * R54 + a78 * R52,
        a14 + a30 * R54 + a46 * R53 + a62 * R52 + a78 * R51,
    );
    let (a94, a110, a126, a142, a158) = (
        a94 + a110 + a126 + a142 + a158,
        a94 + a110 * R51 + a126 * R52 + a142 * R53 + a158 * R54,
        a94 + a110 * R52 + a126 * R54 + a142 * R51 + a158 * R53,
        a94 + a110 * R53 + a126 * R51 + a142 * R54 + a158 * R52,
        a94 + a110 * R54 + a126 * R53 + a142 * R52 + a158 * R51,
    );
    let (a15, a95) = (a15 + a95, a15 - a95);
    let (a31, a111) = (a31 + a111, a31 - a111);
    let (a47, a127) = (a47 + a127, a47 - a127);
    let (a63, a143) = (a63 + a143, a63 - a143);
    let (a79, a159) = (a79 + a159, a79 - a159);
    let a111 = a111 * Field::new(18235156514275634624);
    let a127 = a127 * Field::new(1041288259238279555);
    let a143 = a143 * Field::new(17073700798457888299);
    let a159 = a159 * Field::new(15820824984080659046);
    let (a15, a31, a47, a63, a79) = (
        a15 + a31 + a47 + a63 + a79,
        a15 + a31 * R51 + a47 * R52 + a63 * R53 + a79 * R54,
        a15 + a31 * R52 + a47 * R54 + a63 * R51 + a79 * R53,
        a15 + a31 * R53 + a47 * R51 + a63 * R54 + a79 * R52,
        a15 + a31 * R54 + a47 * R53 + a63 * R52 + a79 * R51,
    );
    let (a95, a111, a127, a143, a159) = (
        a95 + a111 + a127 + a143 + a159,
        a95 + a111 * R51 + a127 * R52 + a143 * R53 + a159 * R54,
        a95 + a111 * R52 + a127 * R54 + a143 * R51 + a159 * R53,
        a95 + a111 * R53 + a127 * R51 + a143 * R54 + a159 * R52,
        a95 + a111 * R54 + a127 * R53 + a143 * R52 + a159 * R51,
    );
    let a81 = a81 * Field::new(15316811890722543172);
    let a17 = a17 * Field::new(6193879297194861051);
    let a97 = a97 * Field::new(12476565439123664266);
    let a33 = a33 * Field::new(9148693690730647261);
    let a113 = (a113 << 6);
    let a49 = a49 * Field::new(2598525327269793995);
    let a129 = a129 * Field::new(9026649562764836523);
    let a65 = a65 * Field::new(5290193119087387221);
    let a145 = a145 * Field::new(13667330054909310753);
    let a82 = a82 * Field::new(6193879297194861051);
    let a18 = a18 * Field::new(9148693690730647261);
    let a98 = a98 * Field::new(2598525327269793995);
    let a34 = a34 * Field::new(5290193119087387221);
    let a114 = (a114 << 12);
    let a50 = a50 * Field::new(5856505865097423521);
    let a130 = a130 * Field::new(7712152251710425105);
    let a66 = a66 * Field::new(18235156514275634624);
    let a146 = a146 * Field::new(12153478289216064362);
    let a83 = a83 * Field::new(12476565439123664266);
    let a19 = a19 * Field::new(2598525327269793995);
    let a99 = a99 * Field::new(13667330054909310753);
    let a35 = a35 * Field::new(5856505865097423521);
    let a115 = (a115 << 18);
    let a51 = a51 * Field::new(12153478289216064362);
    let a131 = a131 * Field::new(4905140540521803713);
    let a67 = a67 * Field::new(8149776168132872528);
    let a147 = a147 * Field::new(17598323070211373799);
    let a84 = a84 * Field::new(9148693690730647261);
    let a20 = a20 * Field::new(5290193119087387221);
    let a100 = a100 * Field::new(5856505865097423521);
    let a36 = a36 * Field::new(18235156514275634624);
    let a116 = (a116 << 24);
    let a52 = a52 * Field::new(8149776168132872528);
    let a132 = a132 * Field::new(11331573348451128694);
    let a68 = a68 * Field::new(1041288259238279555);
    let a148 = a148 * Field::new(4419751934697861046);
    let a85 = (a85 << 6);
    let a21 = (a21 << 12);
    let a101 = (a101 << 18);
    let a37 = (a37 << 24);
    let a117 = (a117 << 30);
    let a53 = (a53 << 36);
    let a133 = (a133 << 42);
    let a69 = (a69 << 48);
    let a149 = (a149 << 54);
    let a86 = a86 * Field::new(2598525327269793995);
    let a22 = a22 * Field::new(5856505865097423521);
    let a102 = a102 * Field::new(12153478289216064362);
    let a38 = a38 * Field::new(8149776168132872528);
    let a118 = (a118 << 36);
    let a54 = a54 * Field::new(4419751934697861046);
    let a134 = a134 * Field::new(3918829805224079129);
    let a70 = a70 * Field::new(17073700798457888299);
    let a150 = a150 * Field::new(15685396404952554508);
    let a87 = a87 * Field::new(9026649562764836523);
    let a23 = a23 * Field::new(7712152251710425105);
    let a103 = a103 * Field::new(4905140540521803713);
    let a39 = a39 * Field::new(11331573348451128694);
    let a119 = (a119 << 42);
    let a55 = a55 * Field::new(3918829805224079129);
    let a135 = a135 * Field::new(15233063875226733425);
    let a71 = a71 * Field::new(17869255328328231396);
    let a151 = a151 * Field::new(16261804814661864505);
    let a88 = a88 * Field::new(5290193119087387221);
    let a24 = a24 * Field::new(18235156514275634624);
    let a104 = a104 * Field::new(8149776168132872528);
    let a40 = a40 * Field::new(1041288259238279555);
    let a120 = (a120 << 48);
    let a56 = a56 * Field::new(17073700798457888299);
    let a136 = a136 * Field::new(17869255328328231396);
    let a72 = a72 * Field::new(15820824984080659046);
    let a152 = a152 * Field::new(2281812832982421726);
    let a89 = a89 * Field::new(13667330054909310753);
    let a25 = a25 * Field::new(12153478289216064362);
    let a105 = a105 * Field::new(17598323070211373799);
    let a41 = a41 * Field::new(4419751934697861046);
    let a121 = (a121 << 54);
    let a57 = a57 * Field::new(15685396404952554508);
    let a137 = a137 * Field::new(16261804814661864505);
    let a73 = a73 * Field::new(2281812832982421726);
    let a153 = a153 * Field::new(3129932178692041149);
    let a90 = (a90 << 12);
    let a26 = (a26 << 24);
    let a106 = (a106 << 36);
    let a42 = (a42 << 48);
    let a122 = (a122 << 60);
    let a58 = (a58 << 72);
    let a138 = (a138 << 84);
    let a74 = (-a74);
    let a154 = (-(a154 << 12));
    let a91 = a91 * Field::new(284924320535556791);
    let a27 = a27 * Field::new(7480733200087124716);
    let a107 = a107 * Field::new(5797675593703447897);
    let a43 = a43 * Field::new(4561472264319460910);
    let a123 = (a123 << 66);
    let a59 = a59 * Field::new(14236101464779796609);
    let a139 = a139 * Field::new(16908812824972900217);
    let a75 = a75 * Field::new(13156550950327197100);
    let a155 = a155 * Field::new(4484345764726569947);
    let a92 = a92 * Field::new(5856505865097423521);
    let a28 = a28 * Field::new(8149776168132872528);
    let a108 = a108 * Field::new(4419751934697861046);
    let a44 = a44 * Field::new(17073700798457888299);
    let a124 = (a124 << 72);
    let a60 = a60 * Field::new(2281812832982421726);
    let a140 = a140 * Field::new(9298050378683937060);
    let a76 = a76 * Field::new(211587555138949697);
    let a156 = a156 * Field::new(7115170720963455627);
    let a93 = a93 * Field::new(6530966372130264366);
    let a29 = a29 * Field::new(334345413347504175);
    let a109 = a109 * Field::new(5259419773652843417);
    let a45 = a45 * Field::new(2859541807139753114);
    let a125 = (a125 << 78);
    let a61 = a61 * Field::new(2687357425859721546);
    let a141 = a141 * Field::new(18161819748879027530);
    let a77 = a77 * Field::new(10296967901281711793);
    let a157 = a157 * Field::new(7144527686408445764);
    let a94 = a94 * Field::new(7712152251710425105);
    let a30 = a30 * Field::new(11331573348451128694);
    let a110 = a110 * Field::new(3918829805224079129);
    let a46 = a46 * Field::new(17869255328328231396);
    let a126 = (a126 << 84);
    let a62 = a62 * Field::new(9298050378683937060);
    let a142 = a142 * Field::new(6293265780198519959);
    let a78 = a78 * Field::new(17405455810176304766);
    let a158 = a158 * Field::new(11398751642682958806);
    let a95 = (a95 << 18);
    let a31 = (a31 << 36);
    let a111 = (a111 << 54);
    let a47 = (a47 << 72);
    let a127 = (a127 << 90);
    let a63 = (-(a63 << 12));
    let a143 = (-(a143 << 30));
    let a79 = (-(a79 << 48));
    let a159 = (-(a159 << 66));
    let (a0, a8) = (a0 + a8, a0 - a8);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let a12 = (a12 << 48);
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let (a5, a13) = (a5 + a13, a5 - a13);
    let a13 = (a13 << 48);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a2, a10) = (a2 + a10, a2 - a10);
    let (a6, a14) = (a6 + a14, a6 - a14);
    let a14 = (a14 << 48);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let (a7, a15) = (a7 + a15, a7 - a15);
    let a15 = (a15 << 48);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a9 = (a9 << 12);
    let a5 = (a5 << 24);
    let a13 = (a13 << 36);
    let a10 = (a10 << 24);
    let a6 = (a6 << 48);
    let a14 = (a14 << 72);
    let a11 = (a11 << 36);
    let a7 = (a7 << 72);
    let a15 = (-(a15 << 12));
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a80, a88) = (a80 + a88, a80 - a88);
    let (a84, a92) = (a84 + a92, a84 - a92);
    let a92 = (a92 << 48);
    let (a80, a84) = (a80 + a84, a80 - a84);
    let (a88, a92) = (a88 + a92, a88 - a92);
    let (a81, a89) = (a81 + a89, a81 - a89);
    let (a85, a93) = (a85 + a93, a85 - a93);
    let a93 = (a93 << 48);
    let (a81, a85) = (a81 + a85, a81 - a85);
    let (a89, a93) = (a89 + a93, a89 - a93);
    let (a82, a90) = (a82 + a90, a82 - a90);
    let (a86, a94) = (a86 + a94, a86 - a94);
    let a94 = (a94 << 48);
    let (a82, a86) = (a82 + a86, a82 - a86);
    let (a90, a94) = (a90 + a94, a90 - a94);
    let (a83, a91) = (a83 + a91, a83 - a91);
    let (a87, a95) = (a87 + a95, a87 - a95);
    let a95 = (a95 << 48);
    let (a83, a87) = (a83 + a87, a83 - a87);
    let (a91, a95) = (a91 + a95, a91 - a95);
    let a89 = (a89 << 12);
    let a85 = (a85 << 24);
    let a93 = (a93 << 36);
    let a90 = (a90 << 24);
    let a86 = (a86 << 48);
    let a94 = (a94 << 72);
    let a91 = (a91 << 36);
    let a87 = (a87 << 72);
    let a95 = (-(a95 << 12));
    let (a80, a82) = (a80 + a82, a80 - a82);
    let (a81, a83) = (a81 + a83, a81 - a83);
    let a83 = (a83 << 48);
    let (a80, a81) = (a80 + a81, a80 - a81);
    let (a82, a83) = (a82 + a83, a82 - a83);
    let (a88, a90) = (a88 + a90, a88 - a90);
    let (a89, a91) = (a89 + a91, a89 - a91);
    let a91 = (a91 << 48);
    let (a88, a89) = (a88 + a89, a88 - a89);
    let (a90, a91) = (a90 + a91, a90 - a91);
    let (a84, a86) = (a84 + a86, a84 - a86);
    let (a85, a87) = (a85 + a87, a85 - a87);
    let a87 = (a87 << 48);
    let (a84, a85) = (a84 + a85, a84 - a85);
    let (a86, a87) = (a86 + a87, a86 - a87);
    let (a92, a94) = (a92 + a94, a92 - a94);
    let (a93, a95) = (a93 + a95, a93 - a95);
    let a95 = (a95 << 48);
    let (a92, a93) = (a92 + a93, a92 - a93);
    let (a94, a95) = (a94 + a95, a94 - a95);
    let (a16, a24) = (a16 + a24, a16 - a24);
    let (a20, a28) = (a20 + a28, a20 - a28);
    let a28 = (a28 << 48);
    let (a16, a20) = (a16 + a20, a16 - a20);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a17, a25) = (a17 + a25, a17 - a25);
    let (a21, a29) = (a21 + a29, a21 - a29);
    let a29 = (a29 << 48);
    let (a17, a21) = (a17 + a21, a17 - a21);
    let (a25, a29) = (a25 + a29, a25 - a29);
    let (a18, a26) = (a18 + a26, a18 - a26);
    let (a22, a30) = (a22 + a30, a22 - a30);
    let a30 = (a30 << 48);
    let (a18, a22) = (a18 + a22, a18 - a22);
    let (a26, a30) = (a26 + a30, a26 - a30);
    let (a19, a27) = (a19 + a27, a19 - a27);
    let (a23, a31) = (a23 + a31, a23 - a31);
    let a31 = (a31 << 48);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let (a27, a31) = (a27 + a31, a27 - a31);
    let a25 = (a25 << 12);
    let a21 = (a21 << 24);
    let a29 = (a29 << 36);
    let a26 = (a26 << 24);
    let a22 = (a22 << 48);
    let a30 = (a30 << 72);
    let a27 = (a27 << 36);
    let a23 = (a23 << 72);
    let a31 = (-(a31 << 12));
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a96, a104) = (a96 + a104, a96 - a104);
    let (a100, a108) = (a100 + a108, a100 - a108);
    let a108 = (a108 << 48);
    let (a96, a100) = (a96 + a100, a96 - a100);
    let (a104, a108) = (a104 + a108, a104 - a108);
    let (a97, a105) = (a97 + a105, a97 - a105);
    let (a101, a109) = (a101 + a109, a101 - a109);
    let a109 = (a109 << 48);
    let (a97, a101) = (a97 + a101, a97 - a101);
    let (a105, a109) = (a105 + a109, a105 - a109);
    let (a98, a106) = (a98 + a106, a98 - a106);
    let (a102, a110) = (a102 + a110, a102 - a110);
    let a110 = (a110 << 48);
    let (a98, a102) = (a98 + a102, a98 - a102);
    let (a106, a110) = (a106 + a110, a106 - a110);
    let (a99, a107) = (a99 + a107, a99 - a107);
    let (a103, a111) = (a103 + a111, a103 - a111);
    let a111 = (a111 << 48);
    let (a99, a103) = (a99 + a103, a99 - a103);
    let (a107, a111) = (a107 + a111, a107 - a111);
    let a105 = (a105 << 12);
    let a101 = (a101 << 24);
    let a109 = (a109 << 36);
    let a106 = (a106 << 24);
    let a102 = (a102 << 48);
    let a110 = (a110 << 72);
    let a107 = (a107 << 36);
    let a103 = (a103 << 72);
    let a111 = (-(a111 << 12));
    let (a96, a98) = (a96 + a98, a96 - a98);
    let (a97, a99) = (a97 + a99, a97 - a99);
    let a99 = (a99 << 48);
    let (a96, a97) = (a96 + a97, a96 - a97);
    let (a98, a99) = (a98 + a99, a98 - a99);
    let (a104, a106) = (a104 + a106, a104 - a106);
    let (a105, a107) = (a105 + a107, a105 - a107);
    let a107 = (a107 << 48);
    let (a104, a105) = (a104 + a105, a104 - a105);
    let (a106, a107) = (a106 + a107, a106 - a107);
    let (a100, a102) = (a100 + a102, a100 - a102);
    let (a101, a103) = (a101 + a103, a101 - a103);
    let a103 = (a103 << 48);
    let (a100, a101) = (a100 + a101, a100 - a101);
    let (a102, a103) = (a102 + a103, a102 - a103);
    let (a108, a110) = (a108 + a110, a108 - a110);
    let (a109, a111) = (a109 + a111, a109 - a111);
    let a111 = (a111 << 48);
    let (a108, a109) = (a108 + a109, a108 - a109);
    let (a110, a111) = (a110 + a111, a110 - a111);
    let (a32, a40) = (a32 + a40, a32 - a40);
    let (a36, a44) = (a36 + a44, a36 - a44);
    let a44 = (a44 << 48);
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a40, a44) = (a40 + a44, a40 - a44);
    let (a33, a41) = (a33 + a41, a33 - a41);
    let (a37, a45) = (a37 + a45, a37 - a45);
    let a45 = (a45 << 48);
    let (a33, a37) = (a33 + a37, a33 - a37);
    let (a41, a45) = (a41 + a45, a41 - a45);
    let (a34, a42) = (a34 + a42, a34 - a42);
    let (a38, a46) = (a38 + a46, a38 - a46);
    let a46 = (a46 << 48);
    let (a34, a38) = (a34 + a38, a34 - a38);
    let (a42, a46) = (a42 + a46, a42 - a46);
    let (a35, a43) = (a35 + a43, a35 - a43);
    let (a39, a47) = (a39 + a47, a39 - a47);
    let a47 = (a47 << 48);
    let (a35, a39) = (a35 + a39, a35 - a39);
    let (a43, a47) = (a43 + a47, a43 - a47);
    let a41 = (a41 << 12);
    let a37 = (a37 << 24);
    let a45 = (a45 << 36);
    let a42 = (a42 << 24);
    let a38 = (a38 << 48);
    let a46 = (a46 << 72);
    let a43 = (a43 << 36);
    let a39 = (a39 << 72);
    let a47 = (-(a47 << 12));
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a40, a42) = (a40 + a42, a40 - a42);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let a43 = (a43 << 48);
    let (a40, a41) = (a40 + a41, a40 - a41);
    let (a42, a43) = (a42 + a43, a42 - a43);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    let (a44, a46) = (a44 + a46, a44 - a46);
    let (a45, a47) = (a45 + a47, a45 - a47);
    let a47 = (a47 << 48);
    let (a44, a45) = (a44 + a45, a44 - a45);
    let (a46, a47) = (a46 + a47, a46 - a47);
    let (a112, a120) = (a112 + a120, a112 - a120);
    let (a116, a124) = (a116 + a124, a116 - a124);
    let a124 = (a124 << 48);
    let (a112, a116) = (a112 + a116, a112 - a116);
    let (a120, a124) = (a120 + a124, a120 - a124);
    let (a113, a121) = (a113 + a121, a113 - a121);
    let (a117, a125) = (a117 + a125, a117 - a125);
    let a125 = (a125 << 48);
    let (a113, a117) = (a113 + a117, a113 - a117);
    let (a121, a125) = (a121 + a125, a121 - a125);
    let (a114, a122) = (a114 + a122, a114 - a122);
    let (a118, a126) = (a118 + a126, a118 - a126);
    let a126 = (a126 << 48);
    let (a114, a118) = (a114 + a118, a114 - a118);
    let (a122, a126) = (a122 + a126, a122 - a126);
    let (a115, a123) = (a115 + a123, a115 - a123);
    let (a119, a127) = (a119 + a127, a119 - a127);
    let a127 = (a127 << 48);
    let (a115, a119) = (a115 + a119, a115 - a119);
    let (a123, a127) = (a123 + a127, a123 - a127);
    let a121 = (a121 << 12);
    let a117 = (a117 << 24);
    let a125 = (a125 << 36);
    let a122 = (a122 << 24);
    let a118 = (a118 << 48);
    let a126 = (a126 << 72);
    let a123 = (a123 << 36);
    let a119 = (a119 << 72);
    let a127 = (-(a127 << 12));
    let (a112, a114) = (a112 + a114, a112 - a114);
    let (a113, a115) = (a113 + a115, a113 - a115);
    let a115 = (a115 << 48);
    let (a112, a113) = (a112 + a113, a112 - a113);
    let (a114, a115) = (a114 + a115, a114 - a115);
    let (a120, a122) = (a120 + a122, a120 - a122);
    let (a121, a123) = (a121 + a123, a121 - a123);
    let a123 = (a123 << 48);
    let (a120, a121) = (a120 + a121, a120 - a121);
    let (a122, a123) = (a122 + a123, a122 - a123);
    let (a116, a118) = (a116 + a118, a116 - a118);
    let (a117, a119) = (a117 + a119, a117 - a119);
    let a119 = (a119 << 48);
    let (a116, a117) = (a116 + a117, a116 - a117);
    let (a118, a119) = (a118 + a119, a118 - a119);
    let (a124, a126) = (a124 + a126, a124 - a126);
    let (a125, a127) = (a125 + a127, a125 - a127);
    let a127 = (a127 << 48);
    let (a124, a125) = (a124 + a125, a124 - a125);
    let (a126, a127) = (a126 + a127, a126 - a127);
    let (a48, a56) = (a48 + a56, a48 - a56);
    let (a52, a60) = (a52 + a60, a52 - a60);
    let a60 = (a60 << 48);
    let (a48, a52) = (a48 + a52, a48 - a52);
    let (a56, a60) = (a56 + a60, a56 - a60);
    let (a49, a57) = (a49 + a57, a49 - a57);
    let (a53, a61) = (a53 + a61, a53 - a61);
    let a61 = (a61 << 48);
    let (a49, a53) = (a49 + a53, a49 - a53);
    let (a57, a61) = (a57 + a61, a57 - a61);
    let (a50, a58) = (a50 + a58, a50 - a58);
    let (a54, a62) = (a54 + a62, a54 - a62);
    let a62 = (a62 << 48);
    let (a50, a54) = (a50 + a54, a50 - a54);
    let (a58, a62) = (a58 + a62, a58 - a62);
    let (a51, a59) = (a51 + a59, a51 - a59);
    let (a55, a63) = (a55 + a63, a55 - a63);
    let a63 = (a63 << 48);
    let (a51, a55) = (a51 + a55, a51 - a55);
    let (a59, a63) = (a59 + a63, a59 - a63);
    let a57 = (a57 << 12);
    let a53 = (a53 << 24);
    let a61 = (a61 << 36);
    let a58 = (a58 << 24);
    let a54 = (a54 << 48);
    let a62 = (a62 << 72);
    let a59 = (a59 << 36);
    let a55 = (a55 << 72);
    let a63 = (-(a63 << 12));
    let (a48, a50) = (a48 + a50, a48 - a50);
    let (a49, a51) = (a49 + a51, a49 - a51);
    let a51 = (a51 << 48);
    let (a48, a49) = (a48 + a49, a48 - a49);
    let (a50, a51) = (a50 + a51, a50 - a51);
    let (a56, a58) = (a56 + a58, a56 - a58);
    let (a57, a59) = (a57 + a59, a57 - a59);
    let a59 = (a59 << 48);
    let (a56, a57) = (a56 + a57, a56 - a57);
    let (a58, a59) = (a58 + a59, a58 - a59);
    let (a52, a54) = (a52 + a54, a52 - a54);
    let (a53, a55) = (a53 + a55, a53 - a55);
    let a55 = (a55 << 48);
    let (a52, a53) = (a52 + a53, a52 - a53);
    let (a54, a55) = (a54 + a55, a54 - a55);
    let (a60, a62) = (a60 + a62, a60 - a62);
    let (a61, a63) = (a61 + a63, a61 - a63);
    let a63 = (a63 << 48);
    let (a60, a61) = (a60 + a61, a60 - a61);
    let (a62, a63) = (a62 + a63, a62 - a63);
    let (a128, a136) = (a128 + a136, a128 - a136);
    let (a132, a140) = (a132 + a140, a132 - a140);
    let a140 = (a140 << 48);
    let (a128, a132) = (a128 + a132, a128 - a132);
    let (a136, a140) = (a136 + a140, a136 - a140);
    let (a129, a137) = (a129 + a137, a129 - a137);
    let (a133, a141) = (a133 + a141, a133 - a141);
    let a141 = (a141 << 48);
    let (a129, a133) = (a129 + a133, a129 - a133);
    let (a137, a141) = (a137 + a141, a137 - a141);
    let (a130, a138) = (a130 + a138, a130 - a138);
    let (a134, a142) = (a134 + a142, a134 - a142);
    let a142 = (a142 << 48);
    let (a130, a134) = (a130 + a134, a130 - a134);
    let (a138, a142) = (a138 + a142, a138 - a142);
    let (a131, a139) = (a131 + a139, a131 - a139);
    let (a135, a143) = (a135 + a143, a135 - a143);
    let a143 = (a143 << 48);
    let (a131, a135) = (a131 + a135, a131 - a135);
    let (a139, a143) = (a139 + a143, a139 - a143);
    let a137 = (a137 << 12);
    let a133 = (a133 << 24);
    let a141 = (a141 << 36);
    let a138 = (a138 << 24);
    let a134 = (a134 << 48);
    let a142 = (a142 << 72);
    let a139 = (a139 << 36);
    let a135 = (a135 << 72);
    let a143 = (-(a143 << 12));
    let (a128, a130) = (a128 + a130, a128 - a130);
    let (a129, a131) = (a129 + a131, a129 - a131);
    let a131 = (a131 << 48);
    let (a128, a129) = (a128 + a129, a128 - a129);
    let (a130, a131) = (a130 + a131, a130 - a131);
    let (a136, a138) = (a136 + a138, a136 - a138);
    let (a137, a139) = (a137 + a139, a137 - a139);
    let a139 = (a139 << 48);
    let (a136, a137) = (a136 + a137, a136 - a137);
    let (a138, a139) = (a138 + a139, a138 - a139);
    let (a132, a134) = (a132 + a134, a132 - a134);
    let (a133, a135) = (a133 + a135, a133 - a135);
    let a135 = (a135 << 48);
    let (a132, a133) = (a132 + a133, a132 - a133);
    let (a134, a135) = (a134 + a135, a134 - a135);
    let (a140, a142) = (a140 + a142, a140 - a142);
    let (a141, a143) = (a141 + a143, a141 - a143);
    let a143 = (a143 << 48);
    let (a140, a141) = (a140 + a141, a140 - a141);
    let (a142, a143) = (a142 + a143, a142 - a143);
    let (a64, a72) = (a64 + a72, a64 - a72);
    let (a68, a76) = (a68 + a76, a68 - a76);
    let a76 = (a76 << 48);
    let (a64, a68) = (a64 + a68, a64 - a68);
    let (a72, a76) = (a72 + a76, a72 - a76);
    let (a65, a73) = (a65 + a73, a65 - a73);
    let (a69, a77) = (a69 + a77, a69 - a77);
    let a77 = (a77 << 48);
    let (a65, a69) = (a65 + a69, a65 - a69);
    let (a73, a77) = (a73 + a77, a73 - a77);
    let (a66, a74) = (a66 + a74, a66 - a74);
    let (a70, a78) = (a70 + a78, a70 - a78);
    let a78 = (a78 << 48);
    let (a66, a70) = (a66 + a70, a66 - a70);
    let (a74, a78) = (a74 + a78, a74 - a78);
    let (a67, a75) = (a67 + a75, a67 - a75);
    let (a71, a79) = (a71 + a79, a71 - a79);
    let a79 = (a79 << 48);
    let (a67, a71) = (a67 + a71, a67 - a71);
    let (a75, a79) = (a75 + a79, a75 - a79);
    let a73 = (a73 << 12);
    let a69 = (a69 << 24);
    let a77 = (a77 << 36);
    let a74 = (a74 << 24);
    let a70 = (a70 << 48);
    let a78 = (a78 << 72);
    let a75 = (a75 << 36);
    let a71 = (a71 << 72);
    let a79 = (-(a79 << 12));
    let (a64, a66) = (a64 + a66, a64 - a66);
    let (a65, a67) = (a65 + a67, a65 - a67);
    let a67 = (a67 << 48);
    let (a64, a65) = (a64 + a65, a64 - a65);
    let (a66, a67) = (a66 + a67, a66 - a67);
    let (a72, a74) = (a72 + a74, a72 - a74);
    let (a73, a75) = (a73 + a75, a73 - a75);
    let a75 = (a75 << 48);
    let (a72, a73) = (a72 + a73, a72 - a73);
    let (a74, a75) = (a74 + a75, a74 - a75);
    let (a68, a70) = (a68 + a70, a68 - a70);
    let (a69, a71) = (a69 + a71, a69 - a71);
    let a71 = (a71 << 48);
    let (a68, a69) = (a68 + a69, a68 - a69);
    let (a70, a71) = (a70 + a71, a70 - a71);
    let (a76, a78) = (a76 + a78, a76 - a78);
    let (a77, a79) = (a77 + a79, a77 - a79);
    let a79 = (a79 << 48);
    let (a76, a77) = (a76 + a77, a76 - a77);
    let (a78, a79) = (a78 + a79, a78 - a79);
    let (a144, a152) = (a144 + a152, a144 - a152);
    let (a148, a156) = (a148 + a156, a148 - a156);
    let a156 = (a156 << 48);
    let (a144, a148) = (a144 + a148, a144 - a148);
    let (a152, a156) = (a152 + a156, a152 - a156);
    let (a145, a153) = (a145 + a153, a145 - a153);
    let (a149, a157) = (a149 + a157, a149 - a157);
    let a157 = (a157 << 48);
    let (a145, a149) = (a145 + a149, a145 - a149);
    let (a153, a157) = (a153 + a157, a153 - a157);
    let (a146, a154) = (a146 + a154, a146 - a154);
    let (a150, a158) = (a150 + a158, a150 - a158);
    let a158 = (a158 << 48);
    let (a146, a150) = (a146 + a150, a146 - a150);
    let (a154, a158) = (a154 + a158, a154 - a158);
    let (a147, a155) = (a147 + a155, a147 - a155);
    let (a151, a159) = (a151 + a159, a151 - a159);
    let a159 = (a159 << 48);
    let (a147, a151) = (a147 + a151, a147 - a151);
    let (a155, a159) = (a155 + a159, a155 - a159);
    let a153 = (a153 << 12);
    let a149 = (a149 << 24);
    let a157 = (a157 << 36);
    let a154 = (a154 << 24);
    let a150 = (a150 << 48);
    let a158 = (a158 << 72);
    let a155 = (a155 << 36);
    let a151 = (a151 << 72);
    let a159 = (-(a159 << 12));
    let (a144, a146) = (a144 + a146, a144 - a146);
    let (a145, a147) = (a145 + a147, a145 - a147);
    let a147 = (a147 << 48);
    let (a144, a145) = (a144 + a145, a144 - a145);
    let (a146, a147) = (a146 + a147, a146 - a147);
    let (a152, a154) = (a152 + a154, a152 - a154);
    let (a153, a155) = (a153 + a155, a153 - a155);
    let a155 = (a155 << 48);
    let (a152, a153) = (a152 + a153, a152 - a153);
    let (a154, a155) = (a154 + a155, a154 - a155);
    let (a148, a150) = (a148 + a150, a148 - a150);
    let (a149, a151) = (a149 + a151, a149 - a151);
    let a151 = (a151 << 48);
    let (a148, a149) = (a148 + a149, a148 - a149);
    let (a150, a151) = (a150 + a151, a150 - a151);
    let (a156, a158) = (a156 + a158, a156 - a158);
    let (a157, a159) = (a157 + a159, a157 - a159);
    let a159 = (a159 << 48);
    let (a156, a157) = (a156 + a157, a156 - a157);
    let (a158, a159) = (a158 + a159, a158 - a159);
    values[0] = a0;
    values[1] = a80;
    values[2] = a16;
    values[3] = a96;
    values[4] = a32;
    values[5] = a112;
    values[6] = a48;
    values[7] = a128;
    values[8] = a64;
    values[9] = a144;
    values[10] = a8;
    values[11] = a88;
    values[12] = a24;
    values[13] = a104;
    values[14] = a40;
    values[15] = a120;
    values[16] = a56;
    values[17] = a136;
    values[18] = a72;
    values[19] = a152;
    values[20] = a4;
    values[21] = a84;
    values[22] = a20;
    values[23] = a100;
    values[24] = a36;
    values[25] = a116;
    values[26] = a52;
    values[27] = a132;
    values[28] = a68;
    values[29] = a148;
    values[30] = a12;
    values[31] = a92;
    values[32] = a28;
    values[33] = a108;
    values[34] = a44;
    values[35] = a124;
    values[36] = a60;
    values[37] = a140;
    values[38] = a76;
    values[39] = a156;
    values[40] = a2;
    values[41] = a82;
    values[42] = a18;
    values[43] = a98;
    values[44] = a34;
    values[45] = a114;
    values[46] = a50;
    values[47] = a130;
    values[48] = a66;
    values[49] = a146;
    values[50] = a10;
    values[51] = a90;
    values[52] = a26;
    values[53] = a106;
    values[54] = a42;
    values[55] = a122;
    values[56] = a58;
    values[57] = a138;
    values[58] = a74;
    values[59] = a154;
    values[60] = a6;
    values[61] = a86;
    values[62] = a22;
    values[63] = a102;
    values[64] = a38;
    values[65] = a118;
    values[66] = a54;
    values[67] = a134;
    values[68] = a70;
    values[69] = a150;
    values[70] = a14;
    values[71] = a94;
    values[72] = a30;
    values[73] = a110;
    values[74] = a46;
    values[75] = a126;
    values[76] = a62;
    values[77] = a142;
    values[78] = a78;
    values[79] = a158;
    values[80] = a1;
    values[81] = a81;
    values[82] = a17;
    values[83] = a97;
    values[84] = a33;
    values[85] = a113;
    values[86] = a49;
    values[87] = a129;
    values[88] = a65;
    values[89] = a145;
    values[90] = a9;
    values[91] = a89;
    values[92] = a25;
    values[93] = a105;
    values[94] = a41;
    values[95] = a121;
    values[96] = a57;
    values[97] = a137;
    values[98] = a73;
    values[99] = a153;
    values[100] = a5;
    values[101] = a85;
    values[102] = a21;
    values[103] = a101;
    values[104] = a37;
    values[105] = a117;
    values[106] = a53;
    values[107] = a133;
    values[108] = a69;
    values[109] = a149;
    values[110] = a13;
    values[111] = a93;
    values[112] = a29;
    values[113] = a109;
    values[114] = a45;
    values[115] = a125;
    values[116] = a61;
    values[117] = a141;
    values[118] = a77;
    values[119] = a157;
    values[120] = a3;
    values[121] = a83;
    values[122] = a19;
    values[123] = a99;
    values[124] = a35;
    values[125] = a115;
    values[126] = a51;
    values[127] = a131;
    values[128] = a67;
    values[129] = a147;
    values[130] = a11;
    values[131] = a91;
    values[132] = a27;
    values[133] = a107;
    values[134] = a43;
    values[135] = a123;
    values[136] = a59;
    values[137] = a139;
    values[138] = a75;
    values[139] = a155;
    values[140] = a7;
    values[141] = a87;
    values[142] = a23;
    values[143] = a103;
    values[144] = a39;
    values[145] = a119;
    values[146] = a55;
    values[147] = a135;
    values[148] = a71;
    values[149] = a151;
    values[150] = a15;
    values[151] = a95;
    values[152] = a31;
    values[153] = a111;
    values[154] = a47;
    values[155] = a127;
    values[156] = a63;
    values[157] = a143;
    values[158] = a79;
    values[159] = a159;
}

/// Size 192 NTT.
fn ntt_192(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 192);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let a60 = values[60];
    let a61 = values[61];
    let a62 = values[62];
    let a63 = values[63];
    let a64 = values[64];
    let a65 = values[65];
    let a66 = values[66];
    let a67 = values[67];
    let a68 = values[68];
    let a69 = values[69];
    let a70 = values[70];
    let a71 = values[71];
    let a72 = values[72];
    let a73 = values[73];
    let a74 = values[74];
    let a75 = values[75];
    let a76 = values[76];
    let a77 = values[77];
    let a78 = values[78];
    let a79 = values[79];
    let a80 = values[80];
    let a81 = values[81];
    let a82 = values[82];
    let a83 = values[83];
    let a84 = values[84];
    let a85 = values[85];
    let a86 = values[86];
    let a87 = values[87];
    let a88 = values[88];
    let a89 = values[89];
    let a90 = values[90];
    let a91 = values[91];
    let a92 = values[92];
    let a93 = values[93];
    let a94 = values[94];
    let a95 = values[95];
    let a96 = values[96];
    let a97 = values[97];
    let a98 = values[98];
    let a99 = values[99];
    let a100 = values[100];
    let a101 = values[101];
    let a102 = values[102];
    let a103 = values[103];
    let a104 = values[104];
    let a105 = values[105];
    let a106 = values[106];
    let a107 = values[107];
    let a108 = values[108];
    let a109 = values[109];
    let a110 = values[110];
    let a111 = values[111];
    let a112 = values[112];
    let a113 = values[113];
    let a114 = values[114];
    let a115 = values[115];
    let a116 = values[116];
    let a117 = values[117];
    let a118 = values[118];
    let a119 = values[119];
    let a120 = values[120];
    let a121 = values[121];
    let a122 = values[122];
    let a123 = values[123];
    let a124 = values[124];
    let a125 = values[125];
    let a126 = values[126];
    let a127 = values[127];
    let a128 = values[128];
    let a129 = values[129];
    let a130 = values[130];
    let a131 = values[131];
    let a132 = values[132];
    let a133 = values[133];
    let a134 = values[134];
    let a135 = values[135];
    let a136 = values[136];
    let a137 = values[137];
    let a138 = values[138];
    let a139 = values[139];
    let a140 = values[140];
    let a141 = values[141];
    let a142 = values[142];
    let a143 = values[143];
    let a144 = values[144];
    let a145 = values[145];
    let a146 = values[146];
    let a147 = values[147];
    let a148 = values[148];
    let a149 = values[149];
    let a150 = values[150];
    let a151 = values[151];
    let a152 = values[152];
    let a153 = values[153];
    let a154 = values[154];
    let a155 = values[155];
    let a156 = values[156];
    let a157 = values[157];
    let a158 = values[158];
    let a159 = values[159];
    let a160 = values[160];
    let a161 = values[161];
    let a162 = values[162];
    let a163 = values[163];
    let a164 = values[164];
    let a165 = values[165];
    let a166 = values[166];
    let a167 = values[167];
    let a168 = values[168];
    let a169 = values[169];
    let a170 = values[170];
    let a171 = values[171];
    let a172 = values[172];
    let a173 = values[173];
    let a174 = values[174];
    let a175 = values[175];
    let a176 = values[176];
    let a177 = values[177];
    let a178 = values[178];
    let a179 = values[179];
    let a180 = values[180];
    let a181 = values[181];
    let a182 = values[182];
    let a183 = values[183];
    let a184 = values[184];
    let a185 = values[185];
    let a186 = values[186];
    let a187 = values[187];
    let a188 = values[188];
    let a189 = values[189];
    let a190 = values[190];
    let a191 = values[191];
    let (a0, a64, a128) = (
        a0 + a64 + a128,
        a0 + (a64 << 64) - (a128 << 32),
        a0 - (a64 << 32) + (a128 << 64),
    );
    let (a16, a80, a144) = (
        a16 + a80 + a144,
        a16 + (a80 << 64) - (a144 << 32),
        a16 - (a80 << 32) + (a144 << 64),
    );
    let (a32, a96, a160) = (
        a32 + a96 + a160,
        a32 + (a96 << 64) - (a160 << 32),
        a32 - (a96 << 32) + (a160 << 64),
    );
    let (a48, a112, a176) = (
        a48 + a112 + a176,
        a48 + (a112 << 64) - (a176 << 32),
        a48 - (a112 << 32) + (a176 << 64),
    );
    let a80 = (a80 << 16);
    let a144 = (a144 << 32);
    let a96 = (a96 << 32);
    let a160 = (a160 << 64);
    let a112 = (a112 << 48);
    let a176 = (-a176);
    let (a0, a32) = (a0 + a32, a0 - a32);
    let (a16, a48) = (a16 + a48, a16 - a48);
    let a48 = (a48 << 48);
    let (a0, a16) = (a0 + a16, a0 - a16);
    let (a32, a48) = (a32 + a48, a32 - a48);
    let (a64, a96) = (a64 + a96, a64 - a96);
    let (a80, a112) = (a80 + a112, a80 - a112);
    let a112 = (a112 << 48);
    let (a64, a80) = (a64 + a80, a64 - a80);
    let (a96, a112) = (a96 + a112, a96 - a112);
    let (a128, a160) = (a128 + a160, a128 - a160);
    let (a144, a176) = (a144 + a176, a144 - a176);
    let a176 = (a176 << 48);
    let (a128, a144) = (a128 + a144, a128 - a144);
    let (a160, a176) = (a160 + a176, a160 - a176);
    let (a1, a65, a129) = (
        a1 + a65 + a129,
        a1 + (a65 << 64) - (a129 << 32),
        a1 - (a65 << 32) + (a129 << 64),
    );
    let (a17, a81, a145) = (
        a17 + a81 + a145,
        a17 + (a81 << 64) - (a145 << 32),
        a17 - (a81 << 32) + (a145 << 64),
    );
    let (a33, a97, a161) = (
        a33 + a97 + a161,
        a33 + (a97 << 64) - (a161 << 32),
        a33 - (a97 << 32) + (a161 << 64),
    );
    let (a49, a113, a177) = (
        a49 + a113 + a177,
        a49 + (a113 << 64) - (a177 << 32),
        a49 - (a113 << 32) + (a177 << 64),
    );
    let a81 = (a81 << 16);
    let a145 = (a145 << 32);
    let a97 = (a97 << 32);
    let a161 = (a161 << 64);
    let a113 = (a113 << 48);
    let a177 = (-a177);
    let (a1, a33) = (a1 + a33, a1 - a33);
    let (a17, a49) = (a17 + a49, a17 - a49);
    let a49 = (a49 << 48);
    let (a1, a17) = (a1 + a17, a1 - a17);
    let (a33, a49) = (a33 + a49, a33 - a49);
    let (a65, a97) = (a65 + a97, a65 - a97);
    let (a81, a113) = (a81 + a113, a81 - a113);
    let a113 = (a113 << 48);
    let (a65, a81) = (a65 + a81, a65 - a81);
    let (a97, a113) = (a97 + a113, a97 - a113);
    let (a129, a161) = (a129 + a161, a129 - a161);
    let (a145, a177) = (a145 + a177, a145 - a177);
    let a177 = (a177 << 48);
    let (a129, a145) = (a129 + a145, a129 - a145);
    let (a161, a177) = (a161 + a177, a161 - a177);
    let (a2, a66, a130) = (
        a2 + a66 + a130,
        a2 + (a66 << 64) - (a130 << 32),
        a2 - (a66 << 32) + (a130 << 64),
    );
    let (a18, a82, a146) = (
        a18 + a82 + a146,
        a18 + (a82 << 64) - (a146 << 32),
        a18 - (a82 << 32) + (a146 << 64),
    );
    let (a34, a98, a162) = (
        a34 + a98 + a162,
        a34 + (a98 << 64) - (a162 << 32),
        a34 - (a98 << 32) + (a162 << 64),
    );
    let (a50, a114, a178) = (
        a50 + a114 + a178,
        a50 + (a114 << 64) - (a178 << 32),
        a50 - (a114 << 32) + (a178 << 64),
    );
    let a82 = (a82 << 16);
    let a146 = (a146 << 32);
    let a98 = (a98 << 32);
    let a162 = (a162 << 64);
    let a114 = (a114 << 48);
    let a178 = (-a178);
    let (a2, a34) = (a2 + a34, a2 - a34);
    let (a18, a50) = (a18 + a50, a18 - a50);
    let a50 = (a50 << 48);
    let (a2, a18) = (a2 + a18, a2 - a18);
    let (a34, a50) = (a34 + a50, a34 - a50);
    let (a66, a98) = (a66 + a98, a66 - a98);
    let (a82, a114) = (a82 + a114, a82 - a114);
    let a114 = (a114 << 48);
    let (a66, a82) = (a66 + a82, a66 - a82);
    let (a98, a114) = (a98 + a114, a98 - a114);
    let (a130, a162) = (a130 + a162, a130 - a162);
    let (a146, a178) = (a146 + a178, a146 - a178);
    let a178 = (a178 << 48);
    let (a130, a146) = (a130 + a146, a130 - a146);
    let (a162, a178) = (a162 + a178, a162 - a178);
    let (a3, a67, a131) = (
        a3 + a67 + a131,
        a3 + (a67 << 64) - (a131 << 32),
        a3 - (a67 << 32) + (a131 << 64),
    );
    let (a19, a83, a147) = (
        a19 + a83 + a147,
        a19 + (a83 << 64) - (a147 << 32),
        a19 - (a83 << 32) + (a147 << 64),
    );
    let (a35, a99, a163) = (
        a35 + a99 + a163,
        a35 + (a99 << 64) - (a163 << 32),
        a35 - (a99 << 32) + (a163 << 64),
    );
    let (a51, a115, a179) = (
        a51 + a115 + a179,
        a51 + (a115 << 64) - (a179 << 32),
        a51 - (a115 << 32) + (a179 << 64),
    );
    let a83 = (a83 << 16);
    let a147 = (a147 << 32);
    let a99 = (a99 << 32);
    let a163 = (a163 << 64);
    let a115 = (a115 << 48);
    let a179 = (-a179);
    let (a3, a35) = (a3 + a35, a3 - a35);
    let (a19, a51) = (a19 + a51, a19 - a51);
    let a51 = (a51 << 48);
    let (a3, a19) = (a3 + a19, a3 - a19);
    let (a35, a51) = (a35 + a51, a35 - a51);
    let (a67, a99) = (a67 + a99, a67 - a99);
    let (a83, a115) = (a83 + a115, a83 - a115);
    let a115 = (a115 << 48);
    let (a67, a83) = (a67 + a83, a67 - a83);
    let (a99, a115) = (a99 + a115, a99 - a115);
    let (a131, a163) = (a131 + a163, a131 - a163);
    let (a147, a179) = (a147 + a179, a147 - a179);
    let a179 = (a179 << 48);
    let (a131, a147) = (a131 + a147, a131 - a147);
    let (a163, a179) = (a163 + a179, a163 - a179);
    let (a4, a68, a132) = (
        a4 + a68 + a132,
        a4 + (a68 << 64) - (a132 << 32),
        a4 - (a68 << 32) + (a132 << 64),
    );
    let (a20, a84, a148) = (
        a20 + a84 + a148,
        a20 + (a84 << 64) - (a148 << 32),
        a20 - (a84 << 32) + (a148 << 64),
    );
    let (a36, a100, a164) = (
        a36 + a100 + a164,
        a36 + (a100 << 64) - (a164 << 32),
        a36 - (a100 << 32) + (a164 << 64),
    );
    let (a52, a116, a180) = (
        a52 + a116 + a180,
        a52 + (a116 << 64) - (a180 << 32),
        a52 - (a116 << 32) + (a180 << 64),
    );
    let a84 = (a84 << 16);
    let a148 = (a148 << 32);
    let a100 = (a100 << 32);
    let a164 = (a164 << 64);
    let a116 = (a116 << 48);
    let a180 = (-a180);
    let (a4, a36) = (a4 + a36, a4 - a36);
    let (a20, a52) = (a20 + a52, a20 - a52);
    let a52 = (a52 << 48);
    let (a4, a20) = (a4 + a20, a4 - a20);
    let (a36, a52) = (a36 + a52, a36 - a52);
    let (a68, a100) = (a68 + a100, a68 - a100);
    let (a84, a116) = (a84 + a116, a84 - a116);
    let a116 = (a116 << 48);
    let (a68, a84) = (a68 + a84, a68 - a84);
    let (a100, a116) = (a100 + a116, a100 - a116);
    let (a132, a164) = (a132 + a164, a132 - a164);
    let (a148, a180) = (a148 + a180, a148 - a180);
    let a180 = (a180 << 48);
    let (a132, a148) = (a132 + a148, a132 - a148);
    let (a164, a180) = (a164 + a180, a164 - a180);
    let (a5, a69, a133) = (
        a5 + a69 + a133,
        a5 + (a69 << 64) - (a133 << 32),
        a5 - (a69 << 32) + (a133 << 64),
    );
    let (a21, a85, a149) = (
        a21 + a85 + a149,
        a21 + (a85 << 64) - (a149 << 32),
        a21 - (a85 << 32) + (a149 << 64),
    );
    let (a37, a101, a165) = (
        a37 + a101 + a165,
        a37 + (a101 << 64) - (a165 << 32),
        a37 - (a101 << 32) + (a165 << 64),
    );
    let (a53, a117, a181) = (
        a53 + a117 + a181,
        a53 + (a117 << 64) - (a181 << 32),
        a53 - (a117 << 32) + (a181 << 64),
    );
    let a85 = (a85 << 16);
    let a149 = (a149 << 32);
    let a101 = (a101 << 32);
    let a165 = (a165 << 64);
    let a117 = (a117 << 48);
    let a181 = (-a181);
    let (a5, a37) = (a5 + a37, a5 - a37);
    let (a21, a53) = (a21 + a53, a21 - a53);
    let a53 = (a53 << 48);
    let (a5, a21) = (a5 + a21, a5 - a21);
    let (a37, a53) = (a37 + a53, a37 - a53);
    let (a69, a101) = (a69 + a101, a69 - a101);
    let (a85, a117) = (a85 + a117, a85 - a117);
    let a117 = (a117 << 48);
    let (a69, a85) = (a69 + a85, a69 - a85);
    let (a101, a117) = (a101 + a117, a101 - a117);
    let (a133, a165) = (a133 + a165, a133 - a165);
    let (a149, a181) = (a149 + a181, a149 - a181);
    let a181 = (a181 << 48);
    let (a133, a149) = (a133 + a149, a133 - a149);
    let (a165, a181) = (a165 + a181, a165 - a181);
    let (a6, a70, a134) = (
        a6 + a70 + a134,
        a6 + (a70 << 64) - (a134 << 32),
        a6 - (a70 << 32) + (a134 << 64),
    );
    let (a22, a86, a150) = (
        a22 + a86 + a150,
        a22 + (a86 << 64) - (a150 << 32),
        a22 - (a86 << 32) + (a150 << 64),
    );
    let (a38, a102, a166) = (
        a38 + a102 + a166,
        a38 + (a102 << 64) - (a166 << 32),
        a38 - (a102 << 32) + (a166 << 64),
    );
    let (a54, a118, a182) = (
        a54 + a118 + a182,
        a54 + (a118 << 64) - (a182 << 32),
        a54 - (a118 << 32) + (a182 << 64),
    );
    let a86 = (a86 << 16);
    let a150 = (a150 << 32);
    let a102 = (a102 << 32);
    let a166 = (a166 << 64);
    let a118 = (a118 << 48);
    let a182 = (-a182);
    let (a6, a38) = (a6 + a38, a6 - a38);
    let (a22, a54) = (a22 + a54, a22 - a54);
    let a54 = (a54 << 48);
    let (a6, a22) = (a6 + a22, a6 - a22);
    let (a38, a54) = (a38 + a54, a38 - a54);
    let (a70, a102) = (a70 + a102, a70 - a102);
    let (a86, a118) = (a86 + a118, a86 - a118);
    let a118 = (a118 << 48);
    let (a70, a86) = (a70 + a86, a70 - a86);
    let (a102, a118) = (a102 + a118, a102 - a118);
    let (a134, a166) = (a134 + a166, a134 - a166);
    let (a150, a182) = (a150 + a182, a150 - a182);
    let a182 = (a182 << 48);
    let (a134, a150) = (a134 + a150, a134 - a150);
    let (a166, a182) = (a166 + a182, a166 - a182);
    let (a7, a71, a135) = (
        a7 + a71 + a135,
        a7 + (a71 << 64) - (a135 << 32),
        a7 - (a71 << 32) + (a135 << 64),
    );
    let (a23, a87, a151) = (
        a23 + a87 + a151,
        a23 + (a87 << 64) - (a151 << 32),
        a23 - (a87 << 32) + (a151 << 64),
    );
    let (a39, a103, a167) = (
        a39 + a103 + a167,
        a39 + (a103 << 64) - (a167 << 32),
        a39 - (a103 << 32) + (a167 << 64),
    );
    let (a55, a119, a183) = (
        a55 + a119 + a183,
        a55 + (a119 << 64) - (a183 << 32),
        a55 - (a119 << 32) + (a183 << 64),
    );
    let a87 = (a87 << 16);
    let a151 = (a151 << 32);
    let a103 = (a103 << 32);
    let a167 = (a167 << 64);
    let a119 = (a119 << 48);
    let a183 = (-a183);
    let (a7, a39) = (a7 + a39, a7 - a39);
    let (a23, a55) = (a23 + a55, a23 - a55);
    let a55 = (a55 << 48);
    let (a7, a23) = (a7 + a23, a7 - a23);
    let (a39, a55) = (a39 + a55, a39 - a55);
    let (a71, a103) = (a71 + a103, a71 - a103);
    let (a87, a119) = (a87 + a119, a87 - a119);
    let a119 = (a119 << 48);
    let (a71, a87) = (a71 + a87, a71 - a87);
    let (a103, a119) = (a103 + a119, a103 - a119);
    let (a135, a167) = (a135 + a167, a135 - a167);
    let (a151, a183) = (a151 + a183, a151 - a183);
    let a183 = (a183 << 48);
    let (a135, a151) = (a135 + a151, a135 - a151);
    let (a167, a183) = (a167 + a183, a167 - a183);
    let (a8, a72, a136) = (
        a8 + a72 + a136,
        a8 + (a72 << 64) - (a136 << 32),
        a8 - (a72 << 32) + (a136 << 64),
    );
    let (a24, a88, a152) = (
        a24 + a88 + a152,
        a24 + (a88 << 64) - (a152 << 32),
        a24 - (a88 << 32) + (a152 << 64),
    );
    let (a40, a104, a168) = (
        a40 + a104 + a168,
        a40 + (a104 << 64) - (a168 << 32),
        a40 - (a104 << 32) + (a168 << 64),
    );
    let (a56, a120, a184) = (
        a56 + a120 + a184,
        a56 + (a120 << 64) - (a184 << 32),
        a56 - (a120 << 32) + (a184 << 64),
    );
    let a88 = (a88 << 16);
    let a152 = (a152 << 32);
    let a104 = (a104 << 32);
    let a168 = (a168 << 64);
    let a120 = (a120 << 48);
    let a184 = (-a184);
    let (a8, a40) = (a8 + a40, a8 - a40);
    let (a24, a56) = (a24 + a56, a24 - a56);
    let a56 = (a56 << 48);
    let (a8, a24) = (a8 + a24, a8 - a24);
    let (a40, a56) = (a40 + a56, a40 - a56);
    let (a72, a104) = (a72 + a104, a72 - a104);
    let (a88, a120) = (a88 + a120, a88 - a120);
    let a120 = (a120 << 48);
    let (a72, a88) = (a72 + a88, a72 - a88);
    let (a104, a120) = (a104 + a120, a104 - a120);
    let (a136, a168) = (a136 + a168, a136 - a168);
    let (a152, a184) = (a152 + a184, a152 - a184);
    let a184 = (a184 << 48);
    let (a136, a152) = (a136 + a152, a136 - a152);
    let (a168, a184) = (a168 + a184, a168 - a184);
    let (a9, a73, a137) = (
        a9 + a73 + a137,
        a9 + (a73 << 64) - (a137 << 32),
        a9 - (a73 << 32) + (a137 << 64),
    );
    let (a25, a89, a153) = (
        a25 + a89 + a153,
        a25 + (a89 << 64) - (a153 << 32),
        a25 - (a89 << 32) + (a153 << 64),
    );
    let (a41, a105, a169) = (
        a41 + a105 + a169,
        a41 + (a105 << 64) - (a169 << 32),
        a41 - (a105 << 32) + (a169 << 64),
    );
    let (a57, a121, a185) = (
        a57 + a121 + a185,
        a57 + (a121 << 64) - (a185 << 32),
        a57 - (a121 << 32) + (a185 << 64),
    );
    let a89 = (a89 << 16);
    let a153 = (a153 << 32);
    let a105 = (a105 << 32);
    let a169 = (a169 << 64);
    let a121 = (a121 << 48);
    let a185 = (-a185);
    let (a9, a41) = (a9 + a41, a9 - a41);
    let (a25, a57) = (a25 + a57, a25 - a57);
    let a57 = (a57 << 48);
    let (a9, a25) = (a9 + a25, a9 - a25);
    let (a41, a57) = (a41 + a57, a41 - a57);
    let (a73, a105) = (a73 + a105, a73 - a105);
    let (a89, a121) = (a89 + a121, a89 - a121);
    let a121 = (a121 << 48);
    let (a73, a89) = (a73 + a89, a73 - a89);
    let (a105, a121) = (a105 + a121, a105 - a121);
    let (a137, a169) = (a137 + a169, a137 - a169);
    let (a153, a185) = (a153 + a185, a153 - a185);
    let a185 = (a185 << 48);
    let (a137, a153) = (a137 + a153, a137 - a153);
    let (a169, a185) = (a169 + a185, a169 - a185);
    let (a10, a74, a138) = (
        a10 + a74 + a138,
        a10 + (a74 << 64) - (a138 << 32),
        a10 - (a74 << 32) + (a138 << 64),
    );
    let (a26, a90, a154) = (
        a26 + a90 + a154,
        a26 + (a90 << 64) - (a154 << 32),
        a26 - (a90 << 32) + (a154 << 64),
    );
    let (a42, a106, a170) = (
        a42 + a106 + a170,
        a42 + (a106 << 64) - (a170 << 32),
        a42 - (a106 << 32) + (a170 << 64),
    );
    let (a58, a122, a186) = (
        a58 + a122 + a186,
        a58 + (a122 << 64) - (a186 << 32),
        a58 - (a122 << 32) + (a186 << 64),
    );
    let a90 = (a90 << 16);
    let a154 = (a154 << 32);
    let a106 = (a106 << 32);
    let a170 = (a170 << 64);
    let a122 = (a122 << 48);
    let a186 = (-a186);
    let (a10, a42) = (a10 + a42, a10 - a42);
    let (a26, a58) = (a26 + a58, a26 - a58);
    let a58 = (a58 << 48);
    let (a10, a26) = (a10 + a26, a10 - a26);
    let (a42, a58) = (a42 + a58, a42 - a58);
    let (a74, a106) = (a74 + a106, a74 - a106);
    let (a90, a122) = (a90 + a122, a90 - a122);
    let a122 = (a122 << 48);
    let (a74, a90) = (a74 + a90, a74 - a90);
    let (a106, a122) = (a106 + a122, a106 - a122);
    let (a138, a170) = (a138 + a170, a138 - a170);
    let (a154, a186) = (a154 + a186, a154 - a186);
    let a186 = (a186 << 48);
    let (a138, a154) = (a138 + a154, a138 - a154);
    let (a170, a186) = (a170 + a186, a170 - a186);
    let (a11, a75, a139) = (
        a11 + a75 + a139,
        a11 + (a75 << 64) - (a139 << 32),
        a11 - (a75 << 32) + (a139 << 64),
    );
    let (a27, a91, a155) = (
        a27 + a91 + a155,
        a27 + (a91 << 64) - (a155 << 32),
        a27 - (a91 << 32) + (a155 << 64),
    );
    let (a43, a107, a171) = (
        a43 + a107 + a171,
        a43 + (a107 << 64) - (a171 << 32),
        a43 - (a107 << 32) + (a171 << 64),
    );
    let (a59, a123, a187) = (
        a59 + a123 + a187,
        a59 + (a123 << 64) - (a187 << 32),
        a59 - (a123 << 32) + (a187 << 64),
    );
    let a91 = (a91 << 16);
    let a155 = (a155 << 32);
    let a107 = (a107 << 32);
    let a171 = (a171 << 64);
    let a123 = (a123 << 48);
    let a187 = (-a187);
    let (a11, a43) = (a11 + a43, a11 - a43);
    let (a27, a59) = (a27 + a59, a27 - a59);
    let a59 = (a59 << 48);
    let (a11, a27) = (a11 + a27, a11 - a27);
    let (a43, a59) = (a43 + a59, a43 - a59);
    let (a75, a107) = (a75 + a107, a75 - a107);
    let (a91, a123) = (a91 + a123, a91 - a123);
    let a123 = (a123 << 48);
    let (a75, a91) = (a75 + a91, a75 - a91);
    let (a107, a123) = (a107 + a123, a107 - a123);
    let (a139, a171) = (a139 + a171, a139 - a171);
    let (a155, a187) = (a155 + a187, a155 - a187);
    let a187 = (a187 << 48);
    let (a139, a155) = (a139 + a155, a139 - a155);
    let (a171, a187) = (a171 + a187, a171 - a187);
    let (a12, a76, a140) = (
        a12 + a76 + a140,
        a12 + (a76 << 64) - (a140 << 32),
        a12 - (a76 << 32) + (a140 << 64),
    );
    let (a28, a92, a156) = (
        a28 + a92 + a156,
        a28 + (a92 << 64) - (a156 << 32),
        a28 - (a92 << 32) + (a156 << 64),
    );
    let (a44, a108, a172) = (
        a44 + a108 + a172,
        a44 + (a108 << 64) - (a172 << 32),
        a44 - (a108 << 32) + (a172 << 64),
    );
    let (a60, a124, a188) = (
        a60 + a124 + a188,
        a60 + (a124 << 64) - (a188 << 32),
        a60 - (a124 << 32) + (a188 << 64),
    );
    let a92 = (a92 << 16);
    let a156 = (a156 << 32);
    let a108 = (a108 << 32);
    let a172 = (a172 << 64);
    let a124 = (a124 << 48);
    let a188 = (-a188);
    let (a12, a44) = (a12 + a44, a12 - a44);
    let (a28, a60) = (a28 + a60, a28 - a60);
    let a60 = (a60 << 48);
    let (a12, a28) = (a12 + a28, a12 - a28);
    let (a44, a60) = (a44 + a60, a44 - a60);
    let (a76, a108) = (a76 + a108, a76 - a108);
    let (a92, a124) = (a92 + a124, a92 - a124);
    let a124 = (a124 << 48);
    let (a76, a92) = (a76 + a92, a76 - a92);
    let (a108, a124) = (a108 + a124, a108 - a124);
    let (a140, a172) = (a140 + a172, a140 - a172);
    let (a156, a188) = (a156 + a188, a156 - a188);
    let a188 = (a188 << 48);
    let (a140, a156) = (a140 + a156, a140 - a156);
    let (a172, a188) = (a172 + a188, a172 - a188);
    let (a13, a77, a141) = (
        a13 + a77 + a141,
        a13 + (a77 << 64) - (a141 << 32),
        a13 - (a77 << 32) + (a141 << 64),
    );
    let (a29, a93, a157) = (
        a29 + a93 + a157,
        a29 + (a93 << 64) - (a157 << 32),
        a29 - (a93 << 32) + (a157 << 64),
    );
    let (a45, a109, a173) = (
        a45 + a109 + a173,
        a45 + (a109 << 64) - (a173 << 32),
        a45 - (a109 << 32) + (a173 << 64),
    );
    let (a61, a125, a189) = (
        a61 + a125 + a189,
        a61 + (a125 << 64) - (a189 << 32),
        a61 - (a125 << 32) + (a189 << 64),
    );
    let a93 = (a93 << 16);
    let a157 = (a157 << 32);
    let a109 = (a109 << 32);
    let a173 = (a173 << 64);
    let a125 = (a125 << 48);
    let a189 = (-a189);
    let (a13, a45) = (a13 + a45, a13 - a45);
    let (a29, a61) = (a29 + a61, a29 - a61);
    let a61 = (a61 << 48);
    let (a13, a29) = (a13 + a29, a13 - a29);
    let (a45, a61) = (a45 + a61, a45 - a61);
    let (a77, a109) = (a77 + a109, a77 - a109);
    let (a93, a125) = (a93 + a125, a93 - a125);
    let a125 = (a125 << 48);
    let (a77, a93) = (a77 + a93, a77 - a93);
    let (a109, a125) = (a109 + a125, a109 - a125);
    let (a141, a173) = (a141 + a173, a141 - a173);
    let (a157, a189) = (a157 + a189, a157 - a189);
    let a189 = (a189 << 48);
    let (a141, a157) = (a141 + a157, a141 - a157);
    let (a173, a189) = (a173 + a189, a173 - a189);
    let (a14, a78, a142) = (
        a14 + a78 + a142,
        a14 + (a78 << 64) - (a142 << 32),
        a14 - (a78 << 32) + (a142 << 64),
    );
    let (a30, a94, a158) = (
        a30 + a94 + a158,
        a30 + (a94 << 64) - (a158 << 32),
        a30 - (a94 << 32) + (a158 << 64),
    );
    let (a46, a110, a174) = (
        a46 + a110 + a174,
        a46 + (a110 << 64) - (a174 << 32),
        a46 - (a110 << 32) + (a174 << 64),
    );
    let (a62, a126, a190) = (
        a62 + a126 + a190,
        a62 + (a126 << 64) - (a190 << 32),
        a62 - (a126 << 32) + (a190 << 64),
    );
    let a94 = (a94 << 16);
    let a158 = (a158 << 32);
    let a110 = (a110 << 32);
    let a174 = (a174 << 64);
    let a126 = (a126 << 48);
    let a190 = (-a190);
    let (a14, a46) = (a14 + a46, a14 - a46);
    let (a30, a62) = (a30 + a62, a30 - a62);
    let a62 = (a62 << 48);
    let (a14, a30) = (a14 + a30, a14 - a30);
    let (a46, a62) = (a46 + a62, a46 - a62);
    let (a78, a110) = (a78 + a110, a78 - a110);
    let (a94, a126) = (a94 + a126, a94 - a126);
    let a126 = (a126 << 48);
    let (a78, a94) = (a78 + a94, a78 - a94);
    let (a110, a126) = (a110 + a126, a110 - a126);
    let (a142, a174) = (a142 + a174, a142 - a174);
    let (a158, a190) = (a158 + a190, a158 - a190);
    let a190 = (a190 << 48);
    let (a142, a158) = (a142 + a158, a142 - a158);
    let (a174, a190) = (a174 + a190, a174 - a190);
    let (a15, a79, a143) = (
        a15 + a79 + a143,
        a15 + (a79 << 64) - (a143 << 32),
        a15 - (a79 << 32) + (a143 << 64),
    );
    let (a31, a95, a159) = (
        a31 + a95 + a159,
        a31 + (a95 << 64) - (a159 << 32),
        a31 - (a95 << 32) + (a159 << 64),
    );
    let (a47, a111, a175) = (
        a47 + a111 + a175,
        a47 + (a111 << 64) - (a175 << 32),
        a47 - (a111 << 32) + (a175 << 64),
    );
    let (a63, a127, a191) = (
        a63 + a127 + a191,
        a63 + (a127 << 64) - (a191 << 32),
        a63 - (a127 << 32) + (a191 << 64),
    );
    let a95 = (a95 << 16);
    let a159 = (a159 << 32);
    let a111 = (a111 << 32);
    let a175 = (a175 << 64);
    let a127 = (a127 << 48);
    let a191 = (-a191);
    let (a15, a47) = (a15 + a47, a15 - a47);
    let (a31, a63) = (a31 + a63, a31 - a63);
    let a63 = (a63 << 48);
    let (a15, a31) = (a15 + a31, a15 - a31);
    let (a47, a63) = (a47 + a63, a47 - a63);
    let (a79, a111) = (a79 + a111, a79 - a111);
    let (a95, a127) = (a95 + a127, a95 - a127);
    let a127 = (a127 << 48);
    let (a79, a95) = (a79 + a95, a79 - a95);
    let (a111, a127) = (a111 + a127, a111 - a127);
    let (a143, a175) = (a143 + a175, a143 - a175);
    let (a159, a191) = (a159 + a191, a159 - a191);
    let a191 = (a191 << 48);
    let (a143, a159) = (a143 + a159, a143 - a159);
    let (a175, a191) = (a175 + a191, a175 - a191);
    let a65 = (a65 << 1);
    let a129 = (a129 << 2);
    let a33 = (a33 << 3);
    let a97 = (a97 << 4);
    let a161 = (a161 << 5);
    let a17 = (a17 << 6);
    let a81 = (a81 << 7);
    let a145 = (a145 << 8);
    let a49 = (a49 << 9);
    let a113 = (a113 << 10);
    let a177 = (a177 << 11);
    let a66 = (a66 << 2);
    let a130 = (a130 << 4);
    let a34 = (a34 << 6);
    let a98 = (a98 << 8);
    let a162 = (a162 << 10);
    let a18 = (a18 << 12);
    let a82 = (a82 << 14);
    let a146 = (a146 << 16);
    let a50 = (a50 << 18);
    let a114 = (a114 << 20);
    let a178 = (a178 << 22);
    let a67 = (a67 << 3);
    let a131 = (a131 << 6);
    let a35 = (a35 << 9);
    let a99 = (a99 << 12);
    let a163 = (a163 << 15);
    let a19 = (a19 << 18);
    let a83 = (a83 << 21);
    let a147 = (a147 << 24);
    let a51 = (a51 << 27);
    let a115 = (a115 << 30);
    let a179 = (a179 << 33);
    let a68 = (a68 << 4);
    let a132 = (a132 << 8);
    let a36 = (a36 << 12);
    let a100 = (a100 << 16);
    let a164 = (a164 << 20);
    let a20 = (a20 << 24);
    let a84 = (a84 << 28);
    let a148 = (a148 << 32);
    let a52 = (a52 << 36);
    let a116 = (a116 << 40);
    let a180 = (a180 << 44);
    let a69 = (a69 << 5);
    let a133 = (a133 << 10);
    let a37 = (a37 << 15);
    let a101 = (a101 << 20);
    let a165 = (a165 << 25);
    let a21 = (a21 << 30);
    let a85 = (a85 << 35);
    let a149 = (a149 << 40);
    let a53 = (a53 << 45);
    let a117 = (a117 << 50);
    let a181 = (a181 << 55);
    let a70 = (a70 << 6);
    let a134 = (a134 << 12);
    let a38 = (a38 << 18);
    let a102 = (a102 << 24);
    let a166 = (a166 << 30);
    let a22 = (a22 << 36);
    let a86 = (a86 << 42);
    let a150 = (a150 << 48);
    let a54 = (a54 << 54);
    let a118 = (a118 << 60);
    let a182 = (a182 << 66);
    let a71 = (a71 << 7);
    let a135 = (a135 << 14);
    let a39 = (a39 << 21);
    let a103 = (a103 << 28);
    let a167 = (a167 << 35);
    let a23 = (a23 << 42);
    let a87 = (a87 << 49);
    let a151 = (a151 << 56);
    let a55 = (a55 << 63);
    let a119 = (a119 << 70);
    let a183 = (a183 << 77);
    let a72 = (a72 << 8);
    let a136 = (a136 << 16);
    let a40 = (a40 << 24);
    let a104 = (a104 << 32);
    let a168 = (a168 << 40);
    let a24 = (a24 << 48);
    let a88 = (a88 << 56);
    let a152 = (a152 << 64);
    let a56 = (a56 << 72);
    let a120 = (a120 << 80);
    let a184 = (a184 << 88);
    let a73 = (a73 << 9);
    let a137 = (a137 << 18);
    let a41 = (a41 << 27);
    let a105 = (a105 << 36);
    let a169 = (a169 << 45);
    let a25 = (a25 << 54);
    let a89 = (a89 << 63);
    let a153 = (a153 << 72);
    let a57 = (a57 << 81);
    let a121 = (a121 << 90);
    let a185 = (-(a185 << 3));
    let a74 = (a74 << 10);
    let a138 = (a138 << 20);
    let a42 = (a42 << 30);
    let a106 = (a106 << 40);
    let a170 = (a170 << 50);
    let a26 = (a26 << 60);
    let a90 = (a90 << 70);
    let a154 = (a154 << 80);
    let a58 = (a58 << 90);
    let a122 = (-(a122 << 4));
    let a186 = (-(a186 << 14));
    let a75 = (a75 << 11);
    let a139 = (a139 << 22);
    let a43 = (a43 << 33);
    let a107 = (a107 << 44);
    let a171 = (a171 << 55);
    let a27 = (a27 << 66);
    let a91 = (a91 << 77);
    let a155 = (a155 << 88);
    let a59 = (-(a59 << 3));
    let a123 = (-(a123 << 14));
    let a187 = (-(a187 << 25));
    let a76 = (a76 << 12);
    let a140 = (a140 << 24);
    let a44 = (a44 << 36);
    let a108 = (a108 << 48);
    let a172 = (a172 << 60);
    let a28 = (a28 << 72);
    let a92 = (a92 << 84);
    let a156 = (-a156);
    let a60 = (-(a60 << 12));
    let a124 = (-(a124 << 24));
    let a188 = (-(a188 << 36));
    let a77 = (a77 << 13);
    let a141 = (a141 << 26);
    let a45 = (a45 << 39);
    let a109 = (a109 << 52);
    let a173 = (a173 << 65);
    let a29 = (a29 << 78);
    let a93 = (a93 << 91);
    let a157 = (-(a157 << 8));
    let a61 = (-(a61 << 21));
    let a125 = (-(a125 << 34));
    let a189 = (-(a189 << 47));
    let a78 = (a78 << 14);
    let a142 = (a142 << 28);
    let a46 = (a46 << 42);
    let a110 = (a110 << 56);
    let a174 = (a174 << 70);
    let a30 = (a30 << 84);
    let a94 = (-(a94 << 2));
    let a158 = (-(a158 << 16));
    let a62 = (-(a62 << 30));
    let a126 = (-(a126 << 44));
    let a190 = (-(a190 << 58));
    let a79 = (a79 << 15);
    let a143 = (a143 << 30);
    let a47 = (a47 << 45);
    let a111 = (a111 << 60);
    let a175 = (a175 << 75);
    let a31 = (a31 << 90);
    let a95 = (-(a95 << 9));
    let a159 = (-(a159 << 24));
    let a63 = (-(a63 << 39));
    let a127 = (-(a127 << 54));
    let a191 = (-(a191 << 69));
    let (a0, a8) = (a0 + a8, a0 - a8);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let a12 = (a12 << 48);
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let (a5, a13) = (a5 + a13, a5 - a13);
    let a13 = (a13 << 48);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a2, a10) = (a2 + a10, a2 - a10);
    let (a6, a14) = (a6 + a14, a6 - a14);
    let a14 = (a14 << 48);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let (a7, a15) = (a7 + a15, a7 - a15);
    let a15 = (a15 << 48);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a9 = (a9 << 12);
    let a5 = (a5 << 24);
    let a13 = (a13 << 36);
    let a10 = (a10 << 24);
    let a6 = (a6 << 48);
    let a14 = (a14 << 72);
    let a11 = (a11 << 36);
    let a7 = (a7 << 72);
    let a15 = (-(a15 << 12));
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a64, a72) = (a64 + a72, a64 - a72);
    let (a68, a76) = (a68 + a76, a68 - a76);
    let a76 = (a76 << 48);
    let (a64, a68) = (a64 + a68, a64 - a68);
    let (a72, a76) = (a72 + a76, a72 - a76);
    let (a65, a73) = (a65 + a73, a65 - a73);
    let (a69, a77) = (a69 + a77, a69 - a77);
    let a77 = (a77 << 48);
    let (a65, a69) = (a65 + a69, a65 - a69);
    let (a73, a77) = (a73 + a77, a73 - a77);
    let (a66, a74) = (a66 + a74, a66 - a74);
    let (a70, a78) = (a70 + a78, a70 - a78);
    let a78 = (a78 << 48);
    let (a66, a70) = (a66 + a70, a66 - a70);
    let (a74, a78) = (a74 + a78, a74 - a78);
    let (a67, a75) = (a67 + a75, a67 - a75);
    let (a71, a79) = (a71 + a79, a71 - a79);
    let a79 = (a79 << 48);
    let (a67, a71) = (a67 + a71, a67 - a71);
    let (a75, a79) = (a75 + a79, a75 - a79);
    let a73 = (a73 << 12);
    let a69 = (a69 << 24);
    let a77 = (a77 << 36);
    let a74 = (a74 << 24);
    let a70 = (a70 << 48);
    let a78 = (a78 << 72);
    let a75 = (a75 << 36);
    let a71 = (a71 << 72);
    let a79 = (-(a79 << 12));
    let (a64, a66) = (a64 + a66, a64 - a66);
    let (a65, a67) = (a65 + a67, a65 - a67);
    let a67 = (a67 << 48);
    let (a64, a65) = (a64 + a65, a64 - a65);
    let (a66, a67) = (a66 + a67, a66 - a67);
    let (a72, a74) = (a72 + a74, a72 - a74);
    let (a73, a75) = (a73 + a75, a73 - a75);
    let a75 = (a75 << 48);
    let (a72, a73) = (a72 + a73, a72 - a73);
    let (a74, a75) = (a74 + a75, a74 - a75);
    let (a68, a70) = (a68 + a70, a68 - a70);
    let (a69, a71) = (a69 + a71, a69 - a71);
    let a71 = (a71 << 48);
    let (a68, a69) = (a68 + a69, a68 - a69);
    let (a70, a71) = (a70 + a71, a70 - a71);
    let (a76, a78) = (a76 + a78, a76 - a78);
    let (a77, a79) = (a77 + a79, a77 - a79);
    let a79 = (a79 << 48);
    let (a76, a77) = (a76 + a77, a76 - a77);
    let (a78, a79) = (a78 + a79, a78 - a79);
    let (a128, a136) = (a128 + a136, a128 - a136);
    let (a132, a140) = (a132 + a140, a132 - a140);
    let a140 = (a140 << 48);
    let (a128, a132) = (a128 + a132, a128 - a132);
    let (a136, a140) = (a136 + a140, a136 - a140);
    let (a129, a137) = (a129 + a137, a129 - a137);
    let (a133, a141) = (a133 + a141, a133 - a141);
    let a141 = (a141 << 48);
    let (a129, a133) = (a129 + a133, a129 - a133);
    let (a137, a141) = (a137 + a141, a137 - a141);
    let (a130, a138) = (a130 + a138, a130 - a138);
    let (a134, a142) = (a134 + a142, a134 - a142);
    let a142 = (a142 << 48);
    let (a130, a134) = (a130 + a134, a130 - a134);
    let (a138, a142) = (a138 + a142, a138 - a142);
    let (a131, a139) = (a131 + a139, a131 - a139);
    let (a135, a143) = (a135 + a143, a135 - a143);
    let a143 = (a143 << 48);
    let (a131, a135) = (a131 + a135, a131 - a135);
    let (a139, a143) = (a139 + a143, a139 - a143);
    let a137 = (a137 << 12);
    let a133 = (a133 << 24);
    let a141 = (a141 << 36);
    let a138 = (a138 << 24);
    let a134 = (a134 << 48);
    let a142 = (a142 << 72);
    let a139 = (a139 << 36);
    let a135 = (a135 << 72);
    let a143 = (-(a143 << 12));
    let (a128, a130) = (a128 + a130, a128 - a130);
    let (a129, a131) = (a129 + a131, a129 - a131);
    let a131 = (a131 << 48);
    let (a128, a129) = (a128 + a129, a128 - a129);
    let (a130, a131) = (a130 + a131, a130 - a131);
    let (a136, a138) = (a136 + a138, a136 - a138);
    let (a137, a139) = (a137 + a139, a137 - a139);
    let a139 = (a139 << 48);
    let (a136, a137) = (a136 + a137, a136 - a137);
    let (a138, a139) = (a138 + a139, a138 - a139);
    let (a132, a134) = (a132 + a134, a132 - a134);
    let (a133, a135) = (a133 + a135, a133 - a135);
    let a135 = (a135 << 48);
    let (a132, a133) = (a132 + a133, a132 - a133);
    let (a134, a135) = (a134 + a135, a134 - a135);
    let (a140, a142) = (a140 + a142, a140 - a142);
    let (a141, a143) = (a141 + a143, a141 - a143);
    let a143 = (a143 << 48);
    let (a140, a141) = (a140 + a141, a140 - a141);
    let (a142, a143) = (a142 + a143, a142 - a143);
    let (a32, a40) = (a32 + a40, a32 - a40);
    let (a36, a44) = (a36 + a44, a36 - a44);
    let a44 = (a44 << 48);
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a40, a44) = (a40 + a44, a40 - a44);
    let (a33, a41) = (a33 + a41, a33 - a41);
    let (a37, a45) = (a37 + a45, a37 - a45);
    let a45 = (a45 << 48);
    let (a33, a37) = (a33 + a37, a33 - a37);
    let (a41, a45) = (a41 + a45, a41 - a45);
    let (a34, a42) = (a34 + a42, a34 - a42);
    let (a38, a46) = (a38 + a46, a38 - a46);
    let a46 = (a46 << 48);
    let (a34, a38) = (a34 + a38, a34 - a38);
    let (a42, a46) = (a42 + a46, a42 - a46);
    let (a35, a43) = (a35 + a43, a35 - a43);
    let (a39, a47) = (a39 + a47, a39 - a47);
    let a47 = (a47 << 48);
    let (a35, a39) = (a35 + a39, a35 - a39);
    let (a43, a47) = (a43 + a47, a43 - a47);
    let a41 = (a41 << 12);
    let a37 = (a37 << 24);
    let a45 = (a45 << 36);
    let a42 = (a42 << 24);
    let a38 = (a38 << 48);
    let a46 = (a46 << 72);
    let a43 = (a43 << 36);
    let a39 = (a39 << 72);
    let a47 = (-(a47 << 12));
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a40, a42) = (a40 + a42, a40 - a42);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let a43 = (a43 << 48);
    let (a40, a41) = (a40 + a41, a40 - a41);
    let (a42, a43) = (a42 + a43, a42 - a43);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    let (a44, a46) = (a44 + a46, a44 - a46);
    let (a45, a47) = (a45 + a47, a45 - a47);
    let a47 = (a47 << 48);
    let (a44, a45) = (a44 + a45, a44 - a45);
    let (a46, a47) = (a46 + a47, a46 - a47);
    let (a96, a104) = (a96 + a104, a96 - a104);
    let (a100, a108) = (a100 + a108, a100 - a108);
    let a108 = (a108 << 48);
    let (a96, a100) = (a96 + a100, a96 - a100);
    let (a104, a108) = (a104 + a108, a104 - a108);
    let (a97, a105) = (a97 + a105, a97 - a105);
    let (a101, a109) = (a101 + a109, a101 - a109);
    let a109 = (a109 << 48);
    let (a97, a101) = (a97 + a101, a97 - a101);
    let (a105, a109) = (a105 + a109, a105 - a109);
    let (a98, a106) = (a98 + a106, a98 - a106);
    let (a102, a110) = (a102 + a110, a102 - a110);
    let a110 = (a110 << 48);
    let (a98, a102) = (a98 + a102, a98 - a102);
    let (a106, a110) = (a106 + a110, a106 - a110);
    let (a99, a107) = (a99 + a107, a99 - a107);
    let (a103, a111) = (a103 + a111, a103 - a111);
    let a111 = (a111 << 48);
    let (a99, a103) = (a99 + a103, a99 - a103);
    let (a107, a111) = (a107 + a111, a107 - a111);
    let a105 = (a105 << 12);
    let a101 = (a101 << 24);
    let a109 = (a109 << 36);
    let a106 = (a106 << 24);
    let a102 = (a102 << 48);
    let a110 = (a110 << 72);
    let a107 = (a107 << 36);
    let a103 = (a103 << 72);
    let a111 = (-(a111 << 12));
    let (a96, a98) = (a96 + a98, a96 - a98);
    let (a97, a99) = (a97 + a99, a97 - a99);
    let a99 = (a99 << 48);
    let (a96, a97) = (a96 + a97, a96 - a97);
    let (a98, a99) = (a98 + a99, a98 - a99);
    let (a104, a106) = (a104 + a106, a104 - a106);
    let (a105, a107) = (a105 + a107, a105 - a107);
    let a107 = (a107 << 48);
    let (a104, a105) = (a104 + a105, a104 - a105);
    let (a106, a107) = (a106 + a107, a106 - a107);
    let (a100, a102) = (a100 + a102, a100 - a102);
    let (a101, a103) = (a101 + a103, a101 - a103);
    let a103 = (a103 << 48);
    let (a100, a101) = (a100 + a101, a100 - a101);
    let (a102, a103) = (a102 + a103, a102 - a103);
    let (a108, a110) = (a108 + a110, a108 - a110);
    let (a109, a111) = (a109 + a111, a109 - a111);
    let a111 = (a111 << 48);
    let (a108, a109) = (a108 + a109, a108 - a109);
    let (a110, a111) = (a110 + a111, a110 - a111);
    let (a160, a168) = (a160 + a168, a160 - a168);
    let (a164, a172) = (a164 + a172, a164 - a172);
    let a172 = (a172 << 48);
    let (a160, a164) = (a160 + a164, a160 - a164);
    let (a168, a172) = (a168 + a172, a168 - a172);
    let (a161, a169) = (a161 + a169, a161 - a169);
    let (a165, a173) = (a165 + a173, a165 - a173);
    let a173 = (a173 << 48);
    let (a161, a165) = (a161 + a165, a161 - a165);
    let (a169, a173) = (a169 + a173, a169 - a173);
    let (a162, a170) = (a162 + a170, a162 - a170);
    let (a166, a174) = (a166 + a174, a166 - a174);
    let a174 = (a174 << 48);
    let (a162, a166) = (a162 + a166, a162 - a166);
    let (a170, a174) = (a170 + a174, a170 - a174);
    let (a163, a171) = (a163 + a171, a163 - a171);
    let (a167, a175) = (a167 + a175, a167 - a175);
    let a175 = (a175 << 48);
    let (a163, a167) = (a163 + a167, a163 - a167);
    let (a171, a175) = (a171 + a175, a171 - a175);
    let a169 = (a169 << 12);
    let a165 = (a165 << 24);
    let a173 = (a173 << 36);
    let a170 = (a170 << 24);
    let a166 = (a166 << 48);
    let a174 = (a174 << 72);
    let a171 = (a171 << 36);
    let a167 = (a167 << 72);
    let a175 = (-(a175 << 12));
    let (a160, a162) = (a160 + a162, a160 - a162);
    let (a161, a163) = (a161 + a163, a161 - a163);
    let a163 = (a163 << 48);
    let (a160, a161) = (a160 + a161, a160 - a161);
    let (a162, a163) = (a162 + a163, a162 - a163);
    let (a168, a170) = (a168 + a170, a168 - a170);
    let (a169, a171) = (a169 + a171, a169 - a171);
    let a171 = (a171 << 48);
    let (a168, a169) = (a168 + a169, a168 - a169);
    let (a170, a171) = (a170 + a171, a170 - a171);
    let (a164, a166) = (a164 + a166, a164 - a166);
    let (a165, a167) = (a165 + a167, a165 - a167);
    let a167 = (a167 << 48);
    let (a164, a165) = (a164 + a165, a164 - a165);
    let (a166, a167) = (a166 + a167, a166 - a167);
    let (a172, a174) = (a172 + a174, a172 - a174);
    let (a173, a175) = (a173 + a175, a173 - a175);
    let a175 = (a175 << 48);
    let (a172, a173) = (a172 + a173, a172 - a173);
    let (a174, a175) = (a174 + a175, a174 - a175);
    let (a16, a24) = (a16 + a24, a16 - a24);
    let (a20, a28) = (a20 + a28, a20 - a28);
    let a28 = (a28 << 48);
    let (a16, a20) = (a16 + a20, a16 - a20);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a17, a25) = (a17 + a25, a17 - a25);
    let (a21, a29) = (a21 + a29, a21 - a29);
    let a29 = (a29 << 48);
    let (a17, a21) = (a17 + a21, a17 - a21);
    let (a25, a29) = (a25 + a29, a25 - a29);
    let (a18, a26) = (a18 + a26, a18 - a26);
    let (a22, a30) = (a22 + a30, a22 - a30);
    let a30 = (a30 << 48);
    let (a18, a22) = (a18 + a22, a18 - a22);
    let (a26, a30) = (a26 + a30, a26 - a30);
    let (a19, a27) = (a19 + a27, a19 - a27);
    let (a23, a31) = (a23 + a31, a23 - a31);
    let a31 = (a31 << 48);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let (a27, a31) = (a27 + a31, a27 - a31);
    let a25 = (a25 << 12);
    let a21 = (a21 << 24);
    let a29 = (a29 << 36);
    let a26 = (a26 << 24);
    let a22 = (a22 << 48);
    let a30 = (a30 << 72);
    let a27 = (a27 << 36);
    let a23 = (a23 << 72);
    let a31 = (-(a31 << 12));
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a80, a88) = (a80 + a88, a80 - a88);
    let (a84, a92) = (a84 + a92, a84 - a92);
    let a92 = (a92 << 48);
    let (a80, a84) = (a80 + a84, a80 - a84);
    let (a88, a92) = (a88 + a92, a88 - a92);
    let (a81, a89) = (a81 + a89, a81 - a89);
    let (a85, a93) = (a85 + a93, a85 - a93);
    let a93 = (a93 << 48);
    let (a81, a85) = (a81 + a85, a81 - a85);
    let (a89, a93) = (a89 + a93, a89 - a93);
    let (a82, a90) = (a82 + a90, a82 - a90);
    let (a86, a94) = (a86 + a94, a86 - a94);
    let a94 = (a94 << 48);
    let (a82, a86) = (a82 + a86, a82 - a86);
    let (a90, a94) = (a90 + a94, a90 - a94);
    let (a83, a91) = (a83 + a91, a83 - a91);
    let (a87, a95) = (a87 + a95, a87 - a95);
    let a95 = (a95 << 48);
    let (a83, a87) = (a83 + a87, a83 - a87);
    let (a91, a95) = (a91 + a95, a91 - a95);
    let a89 = (a89 << 12);
    let a85 = (a85 << 24);
    let a93 = (a93 << 36);
    let a90 = (a90 << 24);
    let a86 = (a86 << 48);
    let a94 = (a94 << 72);
    let a91 = (a91 << 36);
    let a87 = (a87 << 72);
    let a95 = (-(a95 << 12));
    let (a80, a82) = (a80 + a82, a80 - a82);
    let (a81, a83) = (a81 + a83, a81 - a83);
    let a83 = (a83 << 48);
    let (a80, a81) = (a80 + a81, a80 - a81);
    let (a82, a83) = (a82 + a83, a82 - a83);
    let (a88, a90) = (a88 + a90, a88 - a90);
    let (a89, a91) = (a89 + a91, a89 - a91);
    let a91 = (a91 << 48);
    let (a88, a89) = (a88 + a89, a88 - a89);
    let (a90, a91) = (a90 + a91, a90 - a91);
    let (a84, a86) = (a84 + a86, a84 - a86);
    let (a85, a87) = (a85 + a87, a85 - a87);
    let a87 = (a87 << 48);
    let (a84, a85) = (a84 + a85, a84 - a85);
    let (a86, a87) = (a86 + a87, a86 - a87);
    let (a92, a94) = (a92 + a94, a92 - a94);
    let (a93, a95) = (a93 + a95, a93 - a95);
    let a95 = (a95 << 48);
    let (a92, a93) = (a92 + a93, a92 - a93);
    let (a94, a95) = (a94 + a95, a94 - a95);
    let (a144, a152) = (a144 + a152, a144 - a152);
    let (a148, a156) = (a148 + a156, a148 - a156);
    let a156 = (a156 << 48);
    let (a144, a148) = (a144 + a148, a144 - a148);
    let (a152, a156) = (a152 + a156, a152 - a156);
    let (a145, a153) = (a145 + a153, a145 - a153);
    let (a149, a157) = (a149 + a157, a149 - a157);
    let a157 = (a157 << 48);
    let (a145, a149) = (a145 + a149, a145 - a149);
    let (a153, a157) = (a153 + a157, a153 - a157);
    let (a146, a154) = (a146 + a154, a146 - a154);
    let (a150, a158) = (a150 + a158, a150 - a158);
    let a158 = (a158 << 48);
    let (a146, a150) = (a146 + a150, a146 - a150);
    let (a154, a158) = (a154 + a158, a154 - a158);
    let (a147, a155) = (a147 + a155, a147 - a155);
    let (a151, a159) = (a151 + a159, a151 - a159);
    let a159 = (a159 << 48);
    let (a147, a151) = (a147 + a151, a147 - a151);
    let (a155, a159) = (a155 + a159, a155 - a159);
    let a153 = (a153 << 12);
    let a149 = (a149 << 24);
    let a157 = (a157 << 36);
    let a154 = (a154 << 24);
    let a150 = (a150 << 48);
    let a158 = (a158 << 72);
    let a155 = (a155 << 36);
    let a151 = (a151 << 72);
    let a159 = (-(a159 << 12));
    let (a144, a146) = (a144 + a146, a144 - a146);
    let (a145, a147) = (a145 + a147, a145 - a147);
    let a147 = (a147 << 48);
    let (a144, a145) = (a144 + a145, a144 - a145);
    let (a146, a147) = (a146 + a147, a146 - a147);
    let (a152, a154) = (a152 + a154, a152 - a154);
    let (a153, a155) = (a153 + a155, a153 - a155);
    let a155 = (a155 << 48);
    let (a152, a153) = (a152 + a153, a152 - a153);
    let (a154, a155) = (a154 + a155, a154 - a155);
    let (a148, a150) = (a148 + a150, a148 - a150);
    let (a149, a151) = (a149 + a151, a149 - a151);
    let a151 = (a151 << 48);
    let (a148, a149) = (a148 + a149, a148 - a149);
    let (a150, a151) = (a150 + a151, a150 - a151);
    let (a156, a158) = (a156 + a158, a156 - a158);
    let (a157, a159) = (a157 + a159, a157 - a159);
    let a159 = (a159 << 48);
    let (a156, a157) = (a156 + a157, a156 - a157);
    let (a158, a159) = (a158 + a159, a158 - a159);
    let (a48, a56) = (a48 + a56, a48 - a56);
    let (a52, a60) = (a52 + a60, a52 - a60);
    let a60 = (a60 << 48);
    let (a48, a52) = (a48 + a52, a48 - a52);
    let (a56, a60) = (a56 + a60, a56 - a60);
    let (a49, a57) = (a49 + a57, a49 - a57);
    let (a53, a61) = (a53 + a61, a53 - a61);
    let a61 = (a61 << 48);
    let (a49, a53) = (a49 + a53, a49 - a53);
    let (a57, a61) = (a57 + a61, a57 - a61);
    let (a50, a58) = (a50 + a58, a50 - a58);
    let (a54, a62) = (a54 + a62, a54 - a62);
    let a62 = (a62 << 48);
    let (a50, a54) = (a50 + a54, a50 - a54);
    let (a58, a62) = (a58 + a62, a58 - a62);
    let (a51, a59) = (a51 + a59, a51 - a59);
    let (a55, a63) = (a55 + a63, a55 - a63);
    let a63 = (a63 << 48);
    let (a51, a55) = (a51 + a55, a51 - a55);
    let (a59, a63) = (a59 + a63, a59 - a63);
    let a57 = (a57 << 12);
    let a53 = (a53 << 24);
    let a61 = (a61 << 36);
    let a58 = (a58 << 24);
    let a54 = (a54 << 48);
    let a62 = (a62 << 72);
    let a59 = (a59 << 36);
    let a55 = (a55 << 72);
    let a63 = (-(a63 << 12));
    let (a48, a50) = (a48 + a50, a48 - a50);
    let (a49, a51) = (a49 + a51, a49 - a51);
    let a51 = (a51 << 48);
    let (a48, a49) = (a48 + a49, a48 - a49);
    let (a50, a51) = (a50 + a51, a50 - a51);
    let (a56, a58) = (a56 + a58, a56 - a58);
    let (a57, a59) = (a57 + a59, a57 - a59);
    let a59 = (a59 << 48);
    let (a56, a57) = (a56 + a57, a56 - a57);
    let (a58, a59) = (a58 + a59, a58 - a59);
    let (a52, a54) = (a52 + a54, a52 - a54);
    let (a53, a55) = (a53 + a55, a53 - a55);
    let a55 = (a55 << 48);
    let (a52, a53) = (a52 + a53, a52 - a53);
    let (a54, a55) = (a54 + a55, a54 - a55);
    let (a60, a62) = (a60 + a62, a60 - a62);
    let (a61, a63) = (a61 + a63, a61 - a63);
    let a63 = (a63 << 48);
    let (a60, a61) = (a60 + a61, a60 - a61);
    let (a62, a63) = (a62 + a63, a62 - a63);
    let (a112, a120) = (a112 + a120, a112 - a120);
    let (a116, a124) = (a116 + a124, a116 - a124);
    let a124 = (a124 << 48);
    let (a112, a116) = (a112 + a116, a112 - a116);
    let (a120, a124) = (a120 + a124, a120 - a124);
    let (a113, a121) = (a113 + a121, a113 - a121);
    let (a117, a125) = (a117 + a125, a117 - a125);
    let a125 = (a125 << 48);
    let (a113, a117) = (a113 + a117, a113 - a117);
    let (a121, a125) = (a121 + a125, a121 - a125);
    let (a114, a122) = (a114 + a122, a114 - a122);
    let (a118, a126) = (a118 + a126, a118 - a126);
    let a126 = (a126 << 48);
    let (a114, a118) = (a114 + a118, a114 - a118);
    let (a122, a126) = (a122 + a126, a122 - a126);
    let (a115, a123) = (a115 + a123, a115 - a123);
    let (a119, a127) = (a119 + a127, a119 - a127);
    let a127 = (a127 << 48);
    let (a115, a119) = (a115 + a119, a115 - a119);
    let (a123, a127) = (a123 + a127, a123 - a127);
    let a121 = (a121 << 12);
    let a117 = (a117 << 24);
    let a125 = (a125 << 36);
    let a122 = (a122 << 24);
    let a118 = (a118 << 48);
    let a126 = (a126 << 72);
    let a123 = (a123 << 36);
    let a119 = (a119 << 72);
    let a127 = (-(a127 << 12));
    let (a112, a114) = (a112 + a114, a112 - a114);
    let (a113, a115) = (a113 + a115, a113 - a115);
    let a115 = (a115 << 48);
    let (a112, a113) = (a112 + a113, a112 - a113);
    let (a114, a115) = (a114 + a115, a114 - a115);
    let (a120, a122) = (a120 + a122, a120 - a122);
    let (a121, a123) = (a121 + a123, a121 - a123);
    let a123 = (a123 << 48);
    let (a120, a121) = (a120 + a121, a120 - a121);
    let (a122, a123) = (a122 + a123, a122 - a123);
    let (a116, a118) = (a116 + a118, a116 - a118);
    let (a117, a119) = (a117 + a119, a117 - a119);
    let a119 = (a119 << 48);
    let (a116, a117) = (a116 + a117, a116 - a117);
    let (a118, a119) = (a118 + a119, a118 - a119);
    let (a124, a126) = (a124 + a126, a124 - a126);
    let (a125, a127) = (a125 + a127, a125 - a127);
    let a127 = (a127 << 48);
    let (a124, a125) = (a124 + a125, a124 - a125);
    let (a126, a127) = (a126 + a127, a126 - a127);
    let (a176, a184) = (a176 + a184, a176 - a184);
    let (a180, a188) = (a180 + a188, a180 - a188);
    let a188 = (a188 << 48);
    let (a176, a180) = (a176 + a180, a176 - a180);
    let (a184, a188) = (a184 + a188, a184 - a188);
    let (a177, a185) = (a177 + a185, a177 - a185);
    let (a181, a189) = (a181 + a189, a181 - a189);
    let a189 = (a189 << 48);
    let (a177, a181) = (a177 + a181, a177 - a181);
    let (a185, a189) = (a185 + a189, a185 - a189);
    let (a178, a186) = (a178 + a186, a178 - a186);
    let (a182, a190) = (a182 + a190, a182 - a190);
    let a190 = (a190 << 48);
    let (a178, a182) = (a178 + a182, a178 - a182);
    let (a186, a190) = (a186 + a190, a186 - a190);
    let (a179, a187) = (a179 + a187, a179 - a187);
    let (a183, a191) = (a183 + a191, a183 - a191);
    let a191 = (a191 << 48);
    let (a179, a183) = (a179 + a183, a179 - a183);
    let (a187, a191) = (a187 + a191, a187 - a191);
    let a185 = (a185 << 12);
    let a181 = (a181 << 24);
    let a189 = (a189 << 36);
    let a186 = (a186 << 24);
    let a182 = (a182 << 48);
    let a190 = (a190 << 72);
    let a187 = (a187 << 36);
    let a183 = (a183 << 72);
    let a191 = (-(a191 << 12));
    let (a176, a178) = (a176 + a178, a176 - a178);
    let (a177, a179) = (a177 + a179, a177 - a179);
    let a179 = (a179 << 48);
    let (a176, a177) = (a176 + a177, a176 - a177);
    let (a178, a179) = (a178 + a179, a178 - a179);
    let (a184, a186) = (a184 + a186, a184 - a186);
    let (a185, a187) = (a185 + a187, a185 - a187);
    let a187 = (a187 << 48);
    let (a184, a185) = (a184 + a185, a184 - a185);
    let (a186, a187) = (a186 + a187, a186 - a187);
    let (a180, a182) = (a180 + a182, a180 - a182);
    let (a181, a183) = (a181 + a183, a181 - a183);
    let a183 = (a183 << 48);
    let (a180, a181) = (a180 + a181, a180 - a181);
    let (a182, a183) = (a182 + a183, a182 - a183);
    let (a188, a190) = (a188 + a190, a188 - a190);
    let (a189, a191) = (a189 + a191, a189 - a191);
    let a191 = (a191 << 48);
    let (a188, a189) = (a188 + a189, a188 - a189);
    let (a190, a191) = (a190 + a191, a190 - a191);
    values[0] = a0;
    values[1] = a64;
    values[2] = a128;
    values[3] = a32;
    values[4] = a96;
    values[5] = a160;
    values[6] = a16;
    values[7] = a80;
    values[8] = a144;
    values[9] = a48;
    values[10] = a112;
    values[11] = a176;
    values[12] = a8;
    values[13] = a72;
    values[14] = a136;
    values[15] = a40;
    values[16] = a104;
    values[17] = a168;
    values[18] = a24;
    values[19] = a88;
    values[20] = a152;
    values[21] = a56;
    values[22] = a120;
    values[23] = a184;
    values[24] = a4;
    values[25] = a68;
    values[26] = a132;
    values[27] = a36;
    values[28] = a100;
    values[29] = a164;
    values[30] = a20;
    values[31] = a84;
    values[32] = a148;
    values[33] = a52;
    values[34] = a116;
    values[35] = a180;
    values[36] = a12;
    values[37] = a76;
    values[38] = a140;
    values[39] = a44;
    values[40] = a108;
    values[41] = a172;
    values[42] = a28;
    values[43] = a92;
    values[44] = a156;
    values[45] = a60;
    values[46] = a124;
    values[47] = a188;
    values[48] = a2;
    values[49] = a66;
    values[50] = a130;
    values[51] = a34;
    values[52] = a98;
    values[53] = a162;
    values[54] = a18;
    values[55] = a82;
    values[56] = a146;
    values[57] = a50;
    values[58] = a114;
    values[59] = a178;
    values[60] = a10;
    values[61] = a74;
    values[62] = a138;
    values[63] = a42;
    values[64] = a106;
    values[65] = a170;
    values[66] = a26;
    values[67] = a90;
    values[68] = a154;
    values[69] = a58;
    values[70] = a122;
    values[71] = a186;
    values[72] = a6;
    values[73] = a70;
    values[74] = a134;
    values[75] = a38;
    values[76] = a102;
    values[77] = a166;
    values[78] = a22;
    values[79] = a86;
    values[80] = a150;
    values[81] = a54;
    values[82] = a118;
    values[83] = a182;
    values[84] = a14;
    values[85] = a78;
    values[86] = a142;
    values[87] = a46;
    values[88] = a110;
    values[89] = a174;
    values[90] = a30;
    values[91] = a94;
    values[92] = a158;
    values[93] = a62;
    values[94] = a126;
    values[95] = a190;
    values[96] = a1;
    values[97] = a65;
    values[98] = a129;
    values[99] = a33;
    values[100] = a97;
    values[101] = a161;
    values[102] = a17;
    values[103] = a81;
    values[104] = a145;
    values[105] = a49;
    values[106] = a113;
    values[107] = a177;
    values[108] = a9;
    values[109] = a73;
    values[110] = a137;
    values[111] = a41;
    values[112] = a105;
    values[113] = a169;
    values[114] = a25;
    values[115] = a89;
    values[116] = a153;
    values[117] = a57;
    values[118] = a121;
    values[119] = a185;
    values[120] = a5;
    values[121] = a69;
    values[122] = a133;
    values[123] = a37;
    values[124] = a101;
    values[125] = a165;
    values[126] = a21;
    values[127] = a85;
    values[128] = a149;
    values[129] = a53;
    values[130] = a117;
    values[131] = a181;
    values[132] = a13;
    values[133] = a77;
    values[134] = a141;
    values[135] = a45;
    values[136] = a109;
    values[137] = a173;
    values[138] = a29;
    values[139] = a93;
    values[140] = a157;
    values[141] = a61;
    values[142] = a125;
    values[143] = a189;
    values[144] = a3;
    values[145] = a67;
    values[146] = a131;
    values[147] = a35;
    values[148] = a99;
    values[149] = a163;
    values[150] = a19;
    values[151] = a83;
    values[152] = a147;
    values[153] = a51;
    values[154] = a115;
    values[155] = a179;
    values[156] = a11;
    values[157] = a75;
    values[158] = a139;
    values[159] = a43;
    values[160] = a107;
    values[161] = a171;
    values[162] = a27;
    values[163] = a91;
    values[164] = a155;
    values[165] = a59;
    values[166] = a123;
    values[167] = a187;
    values[168] = a7;
    values[169] = a71;
    values[170] = a135;
    values[171] = a39;
    values[172] = a103;
    values[173] = a167;
    values[174] = a23;
    values[175] = a87;
    values[176] = a151;
    values[177] = a55;
    values[178] = a119;
    values[179] = a183;
    values[180] = a15;
    values[181] = a79;
    values[182] = a143;
    values[183] = a47;
    values[184] = a111;
    values[185] = a175;
    values[186] = a31;
    values[187] = a95;
    values[188] = a159;
    values[189] = a63;
    values[190] = a127;
    values[191] = a191;
}

/// Size 240 NTT.
fn ntt_240(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 240);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let a60 = values[60];
    let a61 = values[61];
    let a62 = values[62];
    let a63 = values[63];
    let a64 = values[64];
    let a65 = values[65];
    let a66 = values[66];
    let a67 = values[67];
    let a68 = values[68];
    let a69 = values[69];
    let a70 = values[70];
    let a71 = values[71];
    let a72 = values[72];
    let a73 = values[73];
    let a74 = values[74];
    let a75 = values[75];
    let a76 = values[76];
    let a77 = values[77];
    let a78 = values[78];
    let a79 = values[79];
    let a80 = values[80];
    let a81 = values[81];
    let a82 = values[82];
    let a83 = values[83];
    let a84 = values[84];
    let a85 = values[85];
    let a86 = values[86];
    let a87 = values[87];
    let a88 = values[88];
    let a89 = values[89];
    let a90 = values[90];
    let a91 = values[91];
    let a92 = values[92];
    let a93 = values[93];
    let a94 = values[94];
    let a95 = values[95];
    let a96 = values[96];
    let a97 = values[97];
    let a98 = values[98];
    let a99 = values[99];
    let a100 = values[100];
    let a101 = values[101];
    let a102 = values[102];
    let a103 = values[103];
    let a104 = values[104];
    let a105 = values[105];
    let a106 = values[106];
    let a107 = values[107];
    let a108 = values[108];
    let a109 = values[109];
    let a110 = values[110];
    let a111 = values[111];
    let a112 = values[112];
    let a113 = values[113];
    let a114 = values[114];
    let a115 = values[115];
    let a116 = values[116];
    let a117 = values[117];
    let a118 = values[118];
    let a119 = values[119];
    let a120 = values[120];
    let a121 = values[121];
    let a122 = values[122];
    let a123 = values[123];
    let a124 = values[124];
    let a125 = values[125];
    let a126 = values[126];
    let a127 = values[127];
    let a128 = values[128];
    let a129 = values[129];
    let a130 = values[130];
    let a131 = values[131];
    let a132 = values[132];
    let a133 = values[133];
    let a134 = values[134];
    let a135 = values[135];
    let a136 = values[136];
    let a137 = values[137];
    let a138 = values[138];
    let a139 = values[139];
    let a140 = values[140];
    let a141 = values[141];
    let a142 = values[142];
    let a143 = values[143];
    let a144 = values[144];
    let a145 = values[145];
    let a146 = values[146];
    let a147 = values[147];
    let a148 = values[148];
    let a149 = values[149];
    let a150 = values[150];
    let a151 = values[151];
    let a152 = values[152];
    let a153 = values[153];
    let a154 = values[154];
    let a155 = values[155];
    let a156 = values[156];
    let a157 = values[157];
    let a158 = values[158];
    let a159 = values[159];
    let a160 = values[160];
    let a161 = values[161];
    let a162 = values[162];
    let a163 = values[163];
    let a164 = values[164];
    let a165 = values[165];
    let a166 = values[166];
    let a167 = values[167];
    let a168 = values[168];
    let a169 = values[169];
    let a170 = values[170];
    let a171 = values[171];
    let a172 = values[172];
    let a173 = values[173];
    let a174 = values[174];
    let a175 = values[175];
    let a176 = values[176];
    let a177 = values[177];
    let a178 = values[178];
    let a179 = values[179];
    let a180 = values[180];
    let a181 = values[181];
    let a182 = values[182];
    let a183 = values[183];
    let a184 = values[184];
    let a185 = values[185];
    let a186 = values[186];
    let a187 = values[187];
    let a188 = values[188];
    let a189 = values[189];
    let a190 = values[190];
    let a191 = values[191];
    let a192 = values[192];
    let a193 = values[193];
    let a194 = values[194];
    let a195 = values[195];
    let a196 = values[196];
    let a197 = values[197];
    let a198 = values[198];
    let a199 = values[199];
    let a200 = values[200];
    let a201 = values[201];
    let a202 = values[202];
    let a203 = values[203];
    let a204 = values[204];
    let a205 = values[205];
    let a206 = values[206];
    let a207 = values[207];
    let a208 = values[208];
    let a209 = values[209];
    let a210 = values[210];
    let a211 = values[211];
    let a212 = values[212];
    let a213 = values[213];
    let a214 = values[214];
    let a215 = values[215];
    let a216 = values[216];
    let a217 = values[217];
    let a218 = values[218];
    let a219 = values[219];
    let a220 = values[220];
    let a221 = values[221];
    let a222 = values[222];
    let a223 = values[223];
    let a224 = values[224];
    let a225 = values[225];
    let a226 = values[226];
    let a227 = values[227];
    let a228 = values[228];
    let a229 = values[229];
    let a230 = values[230];
    let a231 = values[231];
    let a232 = values[232];
    let a233 = values[233];
    let a234 = values[234];
    let a235 = values[235];
    let a236 = values[236];
    let a237 = values[237];
    let a238 = values[238];
    let a239 = values[239];
    let (a0, a80, a160) = (
        a0 + a80 + a160,
        a0 + (a80 << 64) - (a160 << 32),
        a0 - (a80 << 32) + (a160 << 64),
    );
    let (a16, a96, a176) = (
        a16 + a96 + a176,
        a16 + (a96 << 64) - (a176 << 32),
        a16 - (a96 << 32) + (a176 << 64),
    );
    let (a32, a112, a192) = (
        a32 + a112 + a192,
        a32 + (a112 << 64) - (a192 << 32),
        a32 - (a112 << 32) + (a192 << 64),
    );
    let (a48, a128, a208) = (
        a48 + a128 + a208,
        a48 + (a128 << 64) - (a208 << 32),
        a48 - (a128 << 32) + (a208 << 64),
    );
    let (a64, a144, a224) = (
        a64 + a144 + a224,
        a64 + (a144 << 64) - (a224 << 32),
        a64 - (a144 << 32) + (a224 << 64),
    );
    let a96 = a96 * Field::new(17775832080808074370);
    let a176 = a176 * Field::new(9988211933311186582);
    let a112 = a112 * Field::new(9988211933311186582);
    let a192 = a192 * Field::new(6205107048362784195);
    let a128 = a128 * Field::new(1041288259238279555);
    let a208 = a208 * Field::new(15820824984080659046);
    let a144 = a144 * Field::new(6205107048362784195);
    let a224 = a224 * Field::new(11578395661369729110);
    let (a0, a16, a32, a48, a64) = (
        a0 + a16 + a32 + a48 + a64,
        a0 + a16 * R51 + a32 * R52 + a48 * R53 + a64 * R54,
        a0 + a16 * R52 + a32 * R54 + a48 * R51 + a64 * R53,
        a0 + a16 * R53 + a32 * R51 + a48 * R54 + a64 * R52,
        a0 + a16 * R54 + a32 * R53 + a48 * R52 + a64 * R51,
    );
    let (a80, a96, a112, a128, a144) = (
        a80 + a96 + a112 + a128 + a144,
        a80 + a96 * R51 + a112 * R52 + a128 * R53 + a144 * R54,
        a80 + a96 * R52 + a112 * R54 + a128 * R51 + a144 * R53,
        a80 + a96 * R53 + a112 * R51 + a128 * R54 + a144 * R52,
        a80 + a96 * R54 + a112 * R53 + a128 * R52 + a144 * R51,
    );
    let (a160, a176, a192, a208, a224) = (
        a160 + a176 + a192 + a208 + a224,
        a160 + a176 * R51 + a192 * R52 + a208 * R53 + a224 * R54,
        a160 + a176 * R52 + a192 * R54 + a208 * R51 + a224 * R53,
        a160 + a176 * R53 + a192 * R51 + a208 * R54 + a224 * R52,
        a160 + a176 * R54 + a192 * R53 + a208 * R52 + a224 * R51,
    );
    let (a1, a81, a161) = (
        a1 + a81 + a161,
        a1 + (a81 << 64) - (a161 << 32),
        a1 - (a81 << 32) + (a161 << 64),
    );
    let (a17, a97, a177) = (
        a17 + a97 + a177,
        a17 + (a97 << 64) - (a177 << 32),
        a17 - (a97 << 32) + (a177 << 64),
    );
    let (a33, a113, a193) = (
        a33 + a113 + a193,
        a33 + (a113 << 64) - (a193 << 32),
        a33 - (a113 << 32) + (a193 << 64),
    );
    let (a49, a129, a209) = (
        a49 + a129 + a209,
        a49 + (a129 << 64) - (a209 << 32),
        a49 - (a129 << 32) + (a209 << 64),
    );
    let (a65, a145, a225) = (
        a65 + a145 + a225,
        a65 + (a145 << 64) - (a225 << 32),
        a65 - (a145 << 32) + (a225 << 64),
    );
    let a97 = a97 * Field::new(17775832080808074370);
    let a177 = a177 * Field::new(9988211933311186582);
    let a113 = a113 * Field::new(9988211933311186582);
    let a193 = a193 * Field::new(6205107048362784195);
    let a129 = a129 * Field::new(1041288259238279555);
    let a209 = a209 * Field::new(15820824984080659046);
    let a145 = a145 * Field::new(6205107048362784195);
    let a225 = a225 * Field::new(11578395661369729110);
    let (a1, a17, a33, a49, a65) = (
        a1 + a17 + a33 + a49 + a65,
        a1 + a17 * R51 + a33 * R52 + a49 * R53 + a65 * R54,
        a1 + a17 * R52 + a33 * R54 + a49 * R51 + a65 * R53,
        a1 + a17 * R53 + a33 * R51 + a49 * R54 + a65 * R52,
        a1 + a17 * R54 + a33 * R53 + a49 * R52 + a65 * R51,
    );
    let (a81, a97, a113, a129, a145) = (
        a81 + a97 + a113 + a129 + a145,
        a81 + a97 * R51 + a113 * R52 + a129 * R53 + a145 * R54,
        a81 + a97 * R52 + a113 * R54 + a129 * R51 + a145 * R53,
        a81 + a97 * R53 + a113 * R51 + a129 * R54 + a145 * R52,
        a81 + a97 * R54 + a113 * R53 + a129 * R52 + a145 * R51,
    );
    let (a161, a177, a193, a209, a225) = (
        a161 + a177 + a193 + a209 + a225,
        a161 + a177 * R51 + a193 * R52 + a209 * R53 + a225 * R54,
        a161 + a177 * R52 + a193 * R54 + a209 * R51 + a225 * R53,
        a161 + a177 * R53 + a193 * R51 + a209 * R54 + a225 * R52,
        a161 + a177 * R54 + a193 * R53 + a209 * R52 + a225 * R51,
    );
    let (a2, a82, a162) = (
        a2 + a82 + a162,
        a2 + (a82 << 64) - (a162 << 32),
        a2 - (a82 << 32) + (a162 << 64),
    );
    let (a18, a98, a178) = (
        a18 + a98 + a178,
        a18 + (a98 << 64) - (a178 << 32),
        a18 - (a98 << 32) + (a178 << 64),
    );
    let (a34, a114, a194) = (
        a34 + a114 + a194,
        a34 + (a114 << 64) - (a194 << 32),
        a34 - (a114 << 32) + (a194 << 64),
    );
    let (a50, a130, a210) = (
        a50 + a130 + a210,
        a50 + (a130 << 64) - (a210 << 32),
        a50 - (a130 << 32) + (a210 << 64),
    );
    let (a66, a146, a226) = (
        a66 + a146 + a226,
        a66 + (a146 << 64) - (a226 << 32),
        a66 - (a146 << 32) + (a226 << 64),
    );
    let a98 = a98 * Field::new(17775832080808074370);
    let a178 = a178 * Field::new(9988211933311186582);
    let a114 = a114 * Field::new(9988211933311186582);
    let a194 = a194 * Field::new(6205107048362784195);
    let a130 = a130 * Field::new(1041288259238279555);
    let a210 = a210 * Field::new(15820824984080659046);
    let a146 = a146 * Field::new(6205107048362784195);
    let a226 = a226 * Field::new(11578395661369729110);
    let (a2, a18, a34, a50, a66) = (
        a2 + a18 + a34 + a50 + a66,
        a2 + a18 * R51 + a34 * R52 + a50 * R53 + a66 * R54,
        a2 + a18 * R52 + a34 * R54 + a50 * R51 + a66 * R53,
        a2 + a18 * R53 + a34 * R51 + a50 * R54 + a66 * R52,
        a2 + a18 * R54 + a34 * R53 + a50 * R52 + a66 * R51,
    );
    let (a82, a98, a114, a130, a146) = (
        a82 + a98 + a114 + a130 + a146,
        a82 + a98 * R51 + a114 * R52 + a130 * R53 + a146 * R54,
        a82 + a98 * R52 + a114 * R54 + a130 * R51 + a146 * R53,
        a82 + a98 * R53 + a114 * R51 + a130 * R54 + a146 * R52,
        a82 + a98 * R54 + a114 * R53 + a130 * R52 + a146 * R51,
    );
    let (a162, a178, a194, a210, a226) = (
        a162 + a178 + a194 + a210 + a226,
        a162 + a178 * R51 + a194 * R52 + a210 * R53 + a226 * R54,
        a162 + a178 * R52 + a194 * R54 + a210 * R51 + a226 * R53,
        a162 + a178 * R53 + a194 * R51 + a210 * R54 + a226 * R52,
        a162 + a178 * R54 + a194 * R53 + a210 * R52 + a226 * R51,
    );
    let (a3, a83, a163) = (
        a3 + a83 + a163,
        a3 + (a83 << 64) - (a163 << 32),
        a3 - (a83 << 32) + (a163 << 64),
    );
    let (a19, a99, a179) = (
        a19 + a99 + a179,
        a19 + (a99 << 64) - (a179 << 32),
        a19 - (a99 << 32) + (a179 << 64),
    );
    let (a35, a115, a195) = (
        a35 + a115 + a195,
        a35 + (a115 << 64) - (a195 << 32),
        a35 - (a115 << 32) + (a195 << 64),
    );
    let (a51, a131, a211) = (
        a51 + a131 + a211,
        a51 + (a131 << 64) - (a211 << 32),
        a51 - (a131 << 32) + (a211 << 64),
    );
    let (a67, a147, a227) = (
        a67 + a147 + a227,
        a67 + (a147 << 64) - (a227 << 32),
        a67 - (a147 << 32) + (a227 << 64),
    );
    let a99 = a99 * Field::new(17775832080808074370);
    let a179 = a179 * Field::new(9988211933311186582);
    let a115 = a115 * Field::new(9988211933311186582);
    let a195 = a195 * Field::new(6205107048362784195);
    let a131 = a131 * Field::new(1041288259238279555);
    let a211 = a211 * Field::new(15820824984080659046);
    let a147 = a147 * Field::new(6205107048362784195);
    let a227 = a227 * Field::new(11578395661369729110);
    let (a3, a19, a35, a51, a67) = (
        a3 + a19 + a35 + a51 + a67,
        a3 + a19 * R51 + a35 * R52 + a51 * R53 + a67 * R54,
        a3 + a19 * R52 + a35 * R54 + a51 * R51 + a67 * R53,
        a3 + a19 * R53 + a35 * R51 + a51 * R54 + a67 * R52,
        a3 + a19 * R54 + a35 * R53 + a51 * R52 + a67 * R51,
    );
    let (a83, a99, a115, a131, a147) = (
        a83 + a99 + a115 + a131 + a147,
        a83 + a99 * R51 + a115 * R52 + a131 * R53 + a147 * R54,
        a83 + a99 * R52 + a115 * R54 + a131 * R51 + a147 * R53,
        a83 + a99 * R53 + a115 * R51 + a131 * R54 + a147 * R52,
        a83 + a99 * R54 + a115 * R53 + a131 * R52 + a147 * R51,
    );
    let (a163, a179, a195, a211, a227) = (
        a163 + a179 + a195 + a211 + a227,
        a163 + a179 * R51 + a195 * R52 + a211 * R53 + a227 * R54,
        a163 + a179 * R52 + a195 * R54 + a211 * R51 + a227 * R53,
        a163 + a179 * R53 + a195 * R51 + a211 * R54 + a227 * R52,
        a163 + a179 * R54 + a195 * R53 + a211 * R52 + a227 * R51,
    );
    let (a4, a84, a164) = (
        a4 + a84 + a164,
        a4 + (a84 << 64) - (a164 << 32),
        a4 - (a84 << 32) + (a164 << 64),
    );
    let (a20, a100, a180) = (
        a20 + a100 + a180,
        a20 + (a100 << 64) - (a180 << 32),
        a20 - (a100 << 32) + (a180 << 64),
    );
    let (a36, a116, a196) = (
        a36 + a116 + a196,
        a36 + (a116 << 64) - (a196 << 32),
        a36 - (a116 << 32) + (a196 << 64),
    );
    let (a52, a132, a212) = (
        a52 + a132 + a212,
        a52 + (a132 << 64) - (a212 << 32),
        a52 - (a132 << 32) + (a212 << 64),
    );
    let (a68, a148, a228) = (
        a68 + a148 + a228,
        a68 + (a148 << 64) - (a228 << 32),
        a68 - (a148 << 32) + (a228 << 64),
    );
    let a100 = a100 * Field::new(17775832080808074370);
    let a180 = a180 * Field::new(9988211933311186582);
    let a116 = a116 * Field::new(9988211933311186582);
    let a196 = a196 * Field::new(6205107048362784195);
    let a132 = a132 * Field::new(1041288259238279555);
    let a212 = a212 * Field::new(15820824984080659046);
    let a148 = a148 * Field::new(6205107048362784195);
    let a228 = a228 * Field::new(11578395661369729110);
    let (a4, a20, a36, a52, a68) = (
        a4 + a20 + a36 + a52 + a68,
        a4 + a20 * R51 + a36 * R52 + a52 * R53 + a68 * R54,
        a4 + a20 * R52 + a36 * R54 + a52 * R51 + a68 * R53,
        a4 + a20 * R53 + a36 * R51 + a52 * R54 + a68 * R52,
        a4 + a20 * R54 + a36 * R53 + a52 * R52 + a68 * R51,
    );
    let (a84, a100, a116, a132, a148) = (
        a84 + a100 + a116 + a132 + a148,
        a84 + a100 * R51 + a116 * R52 + a132 * R53 + a148 * R54,
        a84 + a100 * R52 + a116 * R54 + a132 * R51 + a148 * R53,
        a84 + a100 * R53 + a116 * R51 + a132 * R54 + a148 * R52,
        a84 + a100 * R54 + a116 * R53 + a132 * R52 + a148 * R51,
    );
    let (a164, a180, a196, a212, a228) = (
        a164 + a180 + a196 + a212 + a228,
        a164 + a180 * R51 + a196 * R52 + a212 * R53 + a228 * R54,
        a164 + a180 * R52 + a196 * R54 + a212 * R51 + a228 * R53,
        a164 + a180 * R53 + a196 * R51 + a212 * R54 + a228 * R52,
        a164 + a180 * R54 + a196 * R53 + a212 * R52 + a228 * R51,
    );
    let (a5, a85, a165) = (
        a5 + a85 + a165,
        a5 + (a85 << 64) - (a165 << 32),
        a5 - (a85 << 32) + (a165 << 64),
    );
    let (a21, a101, a181) = (
        a21 + a101 + a181,
        a21 + (a101 << 64) - (a181 << 32),
        a21 - (a101 << 32) + (a181 << 64),
    );
    let (a37, a117, a197) = (
        a37 + a117 + a197,
        a37 + (a117 << 64) - (a197 << 32),
        a37 - (a117 << 32) + (a197 << 64),
    );
    let (a53, a133, a213) = (
        a53 + a133 + a213,
        a53 + (a133 << 64) - (a213 << 32),
        a53 - (a133 << 32) + (a213 << 64),
    );
    let (a69, a149, a229) = (
        a69 + a149 + a229,
        a69 + (a149 << 64) - (a229 << 32),
        a69 - (a149 << 32) + (a229 << 64),
    );
    let a101 = a101 * Field::new(17775832080808074370);
    let a181 = a181 * Field::new(9988211933311186582);
    let a117 = a117 * Field::new(9988211933311186582);
    let a197 = a197 * Field::new(6205107048362784195);
    let a133 = a133 * Field::new(1041288259238279555);
    let a213 = a213 * Field::new(15820824984080659046);
    let a149 = a149 * Field::new(6205107048362784195);
    let a229 = a229 * Field::new(11578395661369729110);
    let (a5, a21, a37, a53, a69) = (
        a5 + a21 + a37 + a53 + a69,
        a5 + a21 * R51 + a37 * R52 + a53 * R53 + a69 * R54,
        a5 + a21 * R52 + a37 * R54 + a53 * R51 + a69 * R53,
        a5 + a21 * R53 + a37 * R51 + a53 * R54 + a69 * R52,
        a5 + a21 * R54 + a37 * R53 + a53 * R52 + a69 * R51,
    );
    let (a85, a101, a117, a133, a149) = (
        a85 + a101 + a117 + a133 + a149,
        a85 + a101 * R51 + a117 * R52 + a133 * R53 + a149 * R54,
        a85 + a101 * R52 + a117 * R54 + a133 * R51 + a149 * R53,
        a85 + a101 * R53 + a117 * R51 + a133 * R54 + a149 * R52,
        a85 + a101 * R54 + a117 * R53 + a133 * R52 + a149 * R51,
    );
    let (a165, a181, a197, a213, a229) = (
        a165 + a181 + a197 + a213 + a229,
        a165 + a181 * R51 + a197 * R52 + a213 * R53 + a229 * R54,
        a165 + a181 * R52 + a197 * R54 + a213 * R51 + a229 * R53,
        a165 + a181 * R53 + a197 * R51 + a213 * R54 + a229 * R52,
        a165 + a181 * R54 + a197 * R53 + a213 * R52 + a229 * R51,
    );
    let (a6, a86, a166) = (
        a6 + a86 + a166,
        a6 + (a86 << 64) - (a166 << 32),
        a6 - (a86 << 32) + (a166 << 64),
    );
    let (a22, a102, a182) = (
        a22 + a102 + a182,
        a22 + (a102 << 64) - (a182 << 32),
        a22 - (a102 << 32) + (a182 << 64),
    );
    let (a38, a118, a198) = (
        a38 + a118 + a198,
        a38 + (a118 << 64) - (a198 << 32),
        a38 - (a118 << 32) + (a198 << 64),
    );
    let (a54, a134, a214) = (
        a54 + a134 + a214,
        a54 + (a134 << 64) - (a214 << 32),
        a54 - (a134 << 32) + (a214 << 64),
    );
    let (a70, a150, a230) = (
        a70 + a150 + a230,
        a70 + (a150 << 64) - (a230 << 32),
        a70 - (a150 << 32) + (a230 << 64),
    );
    let a102 = a102 * Field::new(17775832080808074370);
    let a182 = a182 * Field::new(9988211933311186582);
    let a118 = a118 * Field::new(9988211933311186582);
    let a198 = a198 * Field::new(6205107048362784195);
    let a134 = a134 * Field::new(1041288259238279555);
    let a214 = a214 * Field::new(15820824984080659046);
    let a150 = a150 * Field::new(6205107048362784195);
    let a230 = a230 * Field::new(11578395661369729110);
    let (a6, a22, a38, a54, a70) = (
        a6 + a22 + a38 + a54 + a70,
        a6 + a22 * R51 + a38 * R52 + a54 * R53 + a70 * R54,
        a6 + a22 * R52 + a38 * R54 + a54 * R51 + a70 * R53,
        a6 + a22 * R53 + a38 * R51 + a54 * R54 + a70 * R52,
        a6 + a22 * R54 + a38 * R53 + a54 * R52 + a70 * R51,
    );
    let (a86, a102, a118, a134, a150) = (
        a86 + a102 + a118 + a134 + a150,
        a86 + a102 * R51 + a118 * R52 + a134 * R53 + a150 * R54,
        a86 + a102 * R52 + a118 * R54 + a134 * R51 + a150 * R53,
        a86 + a102 * R53 + a118 * R51 + a134 * R54 + a150 * R52,
        a86 + a102 * R54 + a118 * R53 + a134 * R52 + a150 * R51,
    );
    let (a166, a182, a198, a214, a230) = (
        a166 + a182 + a198 + a214 + a230,
        a166 + a182 * R51 + a198 * R52 + a214 * R53 + a230 * R54,
        a166 + a182 * R52 + a198 * R54 + a214 * R51 + a230 * R53,
        a166 + a182 * R53 + a198 * R51 + a214 * R54 + a230 * R52,
        a166 + a182 * R54 + a198 * R53 + a214 * R52 + a230 * R51,
    );
    let (a7, a87, a167) = (
        a7 + a87 + a167,
        a7 + (a87 << 64) - (a167 << 32),
        a7 - (a87 << 32) + (a167 << 64),
    );
    let (a23, a103, a183) = (
        a23 + a103 + a183,
        a23 + (a103 << 64) - (a183 << 32),
        a23 - (a103 << 32) + (a183 << 64),
    );
    let (a39, a119, a199) = (
        a39 + a119 + a199,
        a39 + (a119 << 64) - (a199 << 32),
        a39 - (a119 << 32) + (a199 << 64),
    );
    let (a55, a135, a215) = (
        a55 + a135 + a215,
        a55 + (a135 << 64) - (a215 << 32),
        a55 - (a135 << 32) + (a215 << 64),
    );
    let (a71, a151, a231) = (
        a71 + a151 + a231,
        a71 + (a151 << 64) - (a231 << 32),
        a71 - (a151 << 32) + (a231 << 64),
    );
    let a103 = a103 * Field::new(17775832080808074370);
    let a183 = a183 * Field::new(9988211933311186582);
    let a119 = a119 * Field::new(9988211933311186582);
    let a199 = a199 * Field::new(6205107048362784195);
    let a135 = a135 * Field::new(1041288259238279555);
    let a215 = a215 * Field::new(15820824984080659046);
    let a151 = a151 * Field::new(6205107048362784195);
    let a231 = a231 * Field::new(11578395661369729110);
    let (a7, a23, a39, a55, a71) = (
        a7 + a23 + a39 + a55 + a71,
        a7 + a23 * R51 + a39 * R52 + a55 * R53 + a71 * R54,
        a7 + a23 * R52 + a39 * R54 + a55 * R51 + a71 * R53,
        a7 + a23 * R53 + a39 * R51 + a55 * R54 + a71 * R52,
        a7 + a23 * R54 + a39 * R53 + a55 * R52 + a71 * R51,
    );
    let (a87, a103, a119, a135, a151) = (
        a87 + a103 + a119 + a135 + a151,
        a87 + a103 * R51 + a119 * R52 + a135 * R53 + a151 * R54,
        a87 + a103 * R52 + a119 * R54 + a135 * R51 + a151 * R53,
        a87 + a103 * R53 + a119 * R51 + a135 * R54 + a151 * R52,
        a87 + a103 * R54 + a119 * R53 + a135 * R52 + a151 * R51,
    );
    let (a167, a183, a199, a215, a231) = (
        a167 + a183 + a199 + a215 + a231,
        a167 + a183 * R51 + a199 * R52 + a215 * R53 + a231 * R54,
        a167 + a183 * R52 + a199 * R54 + a215 * R51 + a231 * R53,
        a167 + a183 * R53 + a199 * R51 + a215 * R54 + a231 * R52,
        a167 + a183 * R54 + a199 * R53 + a215 * R52 + a231 * R51,
    );
    let (a8, a88, a168) = (
        a8 + a88 + a168,
        a8 + (a88 << 64) - (a168 << 32),
        a8 - (a88 << 32) + (a168 << 64),
    );
    let (a24, a104, a184) = (
        a24 + a104 + a184,
        a24 + (a104 << 64) - (a184 << 32),
        a24 - (a104 << 32) + (a184 << 64),
    );
    let (a40, a120, a200) = (
        a40 + a120 + a200,
        a40 + (a120 << 64) - (a200 << 32),
        a40 - (a120 << 32) + (a200 << 64),
    );
    let (a56, a136, a216) = (
        a56 + a136 + a216,
        a56 + (a136 << 64) - (a216 << 32),
        a56 - (a136 << 32) + (a216 << 64),
    );
    let (a72, a152, a232) = (
        a72 + a152 + a232,
        a72 + (a152 << 64) - (a232 << 32),
        a72 - (a152 << 32) + (a232 << 64),
    );
    let a104 = a104 * Field::new(17775832080808074370);
    let a184 = a184 * Field::new(9988211933311186582);
    let a120 = a120 * Field::new(9988211933311186582);
    let a200 = a200 * Field::new(6205107048362784195);
    let a136 = a136 * Field::new(1041288259238279555);
    let a216 = a216 * Field::new(15820824984080659046);
    let a152 = a152 * Field::new(6205107048362784195);
    let a232 = a232 * Field::new(11578395661369729110);
    let (a8, a24, a40, a56, a72) = (
        a8 + a24 + a40 + a56 + a72,
        a8 + a24 * R51 + a40 * R52 + a56 * R53 + a72 * R54,
        a8 + a24 * R52 + a40 * R54 + a56 * R51 + a72 * R53,
        a8 + a24 * R53 + a40 * R51 + a56 * R54 + a72 * R52,
        a8 + a24 * R54 + a40 * R53 + a56 * R52 + a72 * R51,
    );
    let (a88, a104, a120, a136, a152) = (
        a88 + a104 + a120 + a136 + a152,
        a88 + a104 * R51 + a120 * R52 + a136 * R53 + a152 * R54,
        a88 + a104 * R52 + a120 * R54 + a136 * R51 + a152 * R53,
        a88 + a104 * R53 + a120 * R51 + a136 * R54 + a152 * R52,
        a88 + a104 * R54 + a120 * R53 + a136 * R52 + a152 * R51,
    );
    let (a168, a184, a200, a216, a232) = (
        a168 + a184 + a200 + a216 + a232,
        a168 + a184 * R51 + a200 * R52 + a216 * R53 + a232 * R54,
        a168 + a184 * R52 + a200 * R54 + a216 * R51 + a232 * R53,
        a168 + a184 * R53 + a200 * R51 + a216 * R54 + a232 * R52,
        a168 + a184 * R54 + a200 * R53 + a216 * R52 + a232 * R51,
    );
    let (a9, a89, a169) = (
        a9 + a89 + a169,
        a9 + (a89 << 64) - (a169 << 32),
        a9 - (a89 << 32) + (a169 << 64),
    );
    let (a25, a105, a185) = (
        a25 + a105 + a185,
        a25 + (a105 << 64) - (a185 << 32),
        a25 - (a105 << 32) + (a185 << 64),
    );
    let (a41, a121, a201) = (
        a41 + a121 + a201,
        a41 + (a121 << 64) - (a201 << 32),
        a41 - (a121 << 32) + (a201 << 64),
    );
    let (a57, a137, a217) = (
        a57 + a137 + a217,
        a57 + (a137 << 64) - (a217 << 32),
        a57 - (a137 << 32) + (a217 << 64),
    );
    let (a73, a153, a233) = (
        a73 + a153 + a233,
        a73 + (a153 << 64) - (a233 << 32),
        a73 - (a153 << 32) + (a233 << 64),
    );
    let a105 = a105 * Field::new(17775832080808074370);
    let a185 = a185 * Field::new(9988211933311186582);
    let a121 = a121 * Field::new(9988211933311186582);
    let a201 = a201 * Field::new(6205107048362784195);
    let a137 = a137 * Field::new(1041288259238279555);
    let a217 = a217 * Field::new(15820824984080659046);
    let a153 = a153 * Field::new(6205107048362784195);
    let a233 = a233 * Field::new(11578395661369729110);
    let (a9, a25, a41, a57, a73) = (
        a9 + a25 + a41 + a57 + a73,
        a9 + a25 * R51 + a41 * R52 + a57 * R53 + a73 * R54,
        a9 + a25 * R52 + a41 * R54 + a57 * R51 + a73 * R53,
        a9 + a25 * R53 + a41 * R51 + a57 * R54 + a73 * R52,
        a9 + a25 * R54 + a41 * R53 + a57 * R52 + a73 * R51,
    );
    let (a89, a105, a121, a137, a153) = (
        a89 + a105 + a121 + a137 + a153,
        a89 + a105 * R51 + a121 * R52 + a137 * R53 + a153 * R54,
        a89 + a105 * R52 + a121 * R54 + a137 * R51 + a153 * R53,
        a89 + a105 * R53 + a121 * R51 + a137 * R54 + a153 * R52,
        a89 + a105 * R54 + a121 * R53 + a137 * R52 + a153 * R51,
    );
    let (a169, a185, a201, a217, a233) = (
        a169 + a185 + a201 + a217 + a233,
        a169 + a185 * R51 + a201 * R52 + a217 * R53 + a233 * R54,
        a169 + a185 * R52 + a201 * R54 + a217 * R51 + a233 * R53,
        a169 + a185 * R53 + a201 * R51 + a217 * R54 + a233 * R52,
        a169 + a185 * R54 + a201 * R53 + a217 * R52 + a233 * R51,
    );
    let (a10, a90, a170) = (
        a10 + a90 + a170,
        a10 + (a90 << 64) - (a170 << 32),
        a10 - (a90 << 32) + (a170 << 64),
    );
    let (a26, a106, a186) = (
        a26 + a106 + a186,
        a26 + (a106 << 64) - (a186 << 32),
        a26 - (a106 << 32) + (a186 << 64),
    );
    let (a42, a122, a202) = (
        a42 + a122 + a202,
        a42 + (a122 << 64) - (a202 << 32),
        a42 - (a122 << 32) + (a202 << 64),
    );
    let (a58, a138, a218) = (
        a58 + a138 + a218,
        a58 + (a138 << 64) - (a218 << 32),
        a58 - (a138 << 32) + (a218 << 64),
    );
    let (a74, a154, a234) = (
        a74 + a154 + a234,
        a74 + (a154 << 64) - (a234 << 32),
        a74 - (a154 << 32) + (a234 << 64),
    );
    let a106 = a106 * Field::new(17775832080808074370);
    let a186 = a186 * Field::new(9988211933311186582);
    let a122 = a122 * Field::new(9988211933311186582);
    let a202 = a202 * Field::new(6205107048362784195);
    let a138 = a138 * Field::new(1041288259238279555);
    let a218 = a218 * Field::new(15820824984080659046);
    let a154 = a154 * Field::new(6205107048362784195);
    let a234 = a234 * Field::new(11578395661369729110);
    let (a10, a26, a42, a58, a74) = (
        a10 + a26 + a42 + a58 + a74,
        a10 + a26 * R51 + a42 * R52 + a58 * R53 + a74 * R54,
        a10 + a26 * R52 + a42 * R54 + a58 * R51 + a74 * R53,
        a10 + a26 * R53 + a42 * R51 + a58 * R54 + a74 * R52,
        a10 + a26 * R54 + a42 * R53 + a58 * R52 + a74 * R51,
    );
    let (a90, a106, a122, a138, a154) = (
        a90 + a106 + a122 + a138 + a154,
        a90 + a106 * R51 + a122 * R52 + a138 * R53 + a154 * R54,
        a90 + a106 * R52 + a122 * R54 + a138 * R51 + a154 * R53,
        a90 + a106 * R53 + a122 * R51 + a138 * R54 + a154 * R52,
        a90 + a106 * R54 + a122 * R53 + a138 * R52 + a154 * R51,
    );
    let (a170, a186, a202, a218, a234) = (
        a170 + a186 + a202 + a218 + a234,
        a170 + a186 * R51 + a202 * R52 + a218 * R53 + a234 * R54,
        a170 + a186 * R52 + a202 * R54 + a218 * R51 + a234 * R53,
        a170 + a186 * R53 + a202 * R51 + a218 * R54 + a234 * R52,
        a170 + a186 * R54 + a202 * R53 + a218 * R52 + a234 * R51,
    );
    let (a11, a91, a171) = (
        a11 + a91 + a171,
        a11 + (a91 << 64) - (a171 << 32),
        a11 - (a91 << 32) + (a171 << 64),
    );
    let (a27, a107, a187) = (
        a27 + a107 + a187,
        a27 + (a107 << 64) - (a187 << 32),
        a27 - (a107 << 32) + (a187 << 64),
    );
    let (a43, a123, a203) = (
        a43 + a123 + a203,
        a43 + (a123 << 64) - (a203 << 32),
        a43 - (a123 << 32) + (a203 << 64),
    );
    let (a59, a139, a219) = (
        a59 + a139 + a219,
        a59 + (a139 << 64) - (a219 << 32),
        a59 - (a139 << 32) + (a219 << 64),
    );
    let (a75, a155, a235) = (
        a75 + a155 + a235,
        a75 + (a155 << 64) - (a235 << 32),
        a75 - (a155 << 32) + (a235 << 64),
    );
    let a107 = a107 * Field::new(17775832080808074370);
    let a187 = a187 * Field::new(9988211933311186582);
    let a123 = a123 * Field::new(9988211933311186582);
    let a203 = a203 * Field::new(6205107048362784195);
    let a139 = a139 * Field::new(1041288259238279555);
    let a219 = a219 * Field::new(15820824984080659046);
    let a155 = a155 * Field::new(6205107048362784195);
    let a235 = a235 * Field::new(11578395661369729110);
    let (a11, a27, a43, a59, a75) = (
        a11 + a27 + a43 + a59 + a75,
        a11 + a27 * R51 + a43 * R52 + a59 * R53 + a75 * R54,
        a11 + a27 * R52 + a43 * R54 + a59 * R51 + a75 * R53,
        a11 + a27 * R53 + a43 * R51 + a59 * R54 + a75 * R52,
        a11 + a27 * R54 + a43 * R53 + a59 * R52 + a75 * R51,
    );
    let (a91, a107, a123, a139, a155) = (
        a91 + a107 + a123 + a139 + a155,
        a91 + a107 * R51 + a123 * R52 + a139 * R53 + a155 * R54,
        a91 + a107 * R52 + a123 * R54 + a139 * R51 + a155 * R53,
        a91 + a107 * R53 + a123 * R51 + a139 * R54 + a155 * R52,
        a91 + a107 * R54 + a123 * R53 + a139 * R52 + a155 * R51,
    );
    let (a171, a187, a203, a219, a235) = (
        a171 + a187 + a203 + a219 + a235,
        a171 + a187 * R51 + a203 * R52 + a219 * R53 + a235 * R54,
        a171 + a187 * R52 + a203 * R54 + a219 * R51 + a235 * R53,
        a171 + a187 * R53 + a203 * R51 + a219 * R54 + a235 * R52,
        a171 + a187 * R54 + a203 * R53 + a219 * R52 + a235 * R51,
    );
    let (a12, a92, a172) = (
        a12 + a92 + a172,
        a12 + (a92 << 64) - (a172 << 32),
        a12 - (a92 << 32) + (a172 << 64),
    );
    let (a28, a108, a188) = (
        a28 + a108 + a188,
        a28 + (a108 << 64) - (a188 << 32),
        a28 - (a108 << 32) + (a188 << 64),
    );
    let (a44, a124, a204) = (
        a44 + a124 + a204,
        a44 + (a124 << 64) - (a204 << 32),
        a44 - (a124 << 32) + (a204 << 64),
    );
    let (a60, a140, a220) = (
        a60 + a140 + a220,
        a60 + (a140 << 64) - (a220 << 32),
        a60 - (a140 << 32) + (a220 << 64),
    );
    let (a76, a156, a236) = (
        a76 + a156 + a236,
        a76 + (a156 << 64) - (a236 << 32),
        a76 - (a156 << 32) + (a236 << 64),
    );
    let a108 = a108 * Field::new(17775832080808074370);
    let a188 = a188 * Field::new(9988211933311186582);
    let a124 = a124 * Field::new(9988211933311186582);
    let a204 = a204 * Field::new(6205107048362784195);
    let a140 = a140 * Field::new(1041288259238279555);
    let a220 = a220 * Field::new(15820824984080659046);
    let a156 = a156 * Field::new(6205107048362784195);
    let a236 = a236 * Field::new(11578395661369729110);
    let (a12, a28, a44, a60, a76) = (
        a12 + a28 + a44 + a60 + a76,
        a12 + a28 * R51 + a44 * R52 + a60 * R53 + a76 * R54,
        a12 + a28 * R52 + a44 * R54 + a60 * R51 + a76 * R53,
        a12 + a28 * R53 + a44 * R51 + a60 * R54 + a76 * R52,
        a12 + a28 * R54 + a44 * R53 + a60 * R52 + a76 * R51,
    );
    let (a92, a108, a124, a140, a156) = (
        a92 + a108 + a124 + a140 + a156,
        a92 + a108 * R51 + a124 * R52 + a140 * R53 + a156 * R54,
        a92 + a108 * R52 + a124 * R54 + a140 * R51 + a156 * R53,
        a92 + a108 * R53 + a124 * R51 + a140 * R54 + a156 * R52,
        a92 + a108 * R54 + a124 * R53 + a140 * R52 + a156 * R51,
    );
    let (a172, a188, a204, a220, a236) = (
        a172 + a188 + a204 + a220 + a236,
        a172 + a188 * R51 + a204 * R52 + a220 * R53 + a236 * R54,
        a172 + a188 * R52 + a204 * R54 + a220 * R51 + a236 * R53,
        a172 + a188 * R53 + a204 * R51 + a220 * R54 + a236 * R52,
        a172 + a188 * R54 + a204 * R53 + a220 * R52 + a236 * R51,
    );
    let (a13, a93, a173) = (
        a13 + a93 + a173,
        a13 + (a93 << 64) - (a173 << 32),
        a13 - (a93 << 32) + (a173 << 64),
    );
    let (a29, a109, a189) = (
        a29 + a109 + a189,
        a29 + (a109 << 64) - (a189 << 32),
        a29 - (a109 << 32) + (a189 << 64),
    );
    let (a45, a125, a205) = (
        a45 + a125 + a205,
        a45 + (a125 << 64) - (a205 << 32),
        a45 - (a125 << 32) + (a205 << 64),
    );
    let (a61, a141, a221) = (
        a61 + a141 + a221,
        a61 + (a141 << 64) - (a221 << 32),
        a61 - (a141 << 32) + (a221 << 64),
    );
    let (a77, a157, a237) = (
        a77 + a157 + a237,
        a77 + (a157 << 64) - (a237 << 32),
        a77 - (a157 << 32) + (a237 << 64),
    );
    let a109 = a109 * Field::new(17775832080808074370);
    let a189 = a189 * Field::new(9988211933311186582);
    let a125 = a125 * Field::new(9988211933311186582);
    let a205 = a205 * Field::new(6205107048362784195);
    let a141 = a141 * Field::new(1041288259238279555);
    let a221 = a221 * Field::new(15820824984080659046);
    let a157 = a157 * Field::new(6205107048362784195);
    let a237 = a237 * Field::new(11578395661369729110);
    let (a13, a29, a45, a61, a77) = (
        a13 + a29 + a45 + a61 + a77,
        a13 + a29 * R51 + a45 * R52 + a61 * R53 + a77 * R54,
        a13 + a29 * R52 + a45 * R54 + a61 * R51 + a77 * R53,
        a13 + a29 * R53 + a45 * R51 + a61 * R54 + a77 * R52,
        a13 + a29 * R54 + a45 * R53 + a61 * R52 + a77 * R51,
    );
    let (a93, a109, a125, a141, a157) = (
        a93 + a109 + a125 + a141 + a157,
        a93 + a109 * R51 + a125 * R52 + a141 * R53 + a157 * R54,
        a93 + a109 * R52 + a125 * R54 + a141 * R51 + a157 * R53,
        a93 + a109 * R53 + a125 * R51 + a141 * R54 + a157 * R52,
        a93 + a109 * R54 + a125 * R53 + a141 * R52 + a157 * R51,
    );
    let (a173, a189, a205, a221, a237) = (
        a173 + a189 + a205 + a221 + a237,
        a173 + a189 * R51 + a205 * R52 + a221 * R53 + a237 * R54,
        a173 + a189 * R52 + a205 * R54 + a221 * R51 + a237 * R53,
        a173 + a189 * R53 + a205 * R51 + a221 * R54 + a237 * R52,
        a173 + a189 * R54 + a205 * R53 + a221 * R52 + a237 * R51,
    );
    let (a14, a94, a174) = (
        a14 + a94 + a174,
        a14 + (a94 << 64) - (a174 << 32),
        a14 - (a94 << 32) + (a174 << 64),
    );
    let (a30, a110, a190) = (
        a30 + a110 + a190,
        a30 + (a110 << 64) - (a190 << 32),
        a30 - (a110 << 32) + (a190 << 64),
    );
    let (a46, a126, a206) = (
        a46 + a126 + a206,
        a46 + (a126 << 64) - (a206 << 32),
        a46 - (a126 << 32) + (a206 << 64),
    );
    let (a62, a142, a222) = (
        a62 + a142 + a222,
        a62 + (a142 << 64) - (a222 << 32),
        a62 - (a142 << 32) + (a222 << 64),
    );
    let (a78, a158, a238) = (
        a78 + a158 + a238,
        a78 + (a158 << 64) - (a238 << 32),
        a78 - (a158 << 32) + (a238 << 64),
    );
    let a110 = a110 * Field::new(17775832080808074370);
    let a190 = a190 * Field::new(9988211933311186582);
    let a126 = a126 * Field::new(9988211933311186582);
    let a206 = a206 * Field::new(6205107048362784195);
    let a142 = a142 * Field::new(1041288259238279555);
    let a222 = a222 * Field::new(15820824984080659046);
    let a158 = a158 * Field::new(6205107048362784195);
    let a238 = a238 * Field::new(11578395661369729110);
    let (a14, a30, a46, a62, a78) = (
        a14 + a30 + a46 + a62 + a78,
        a14 + a30 * R51 + a46 * R52 + a62 * R53 + a78 * R54,
        a14 + a30 * R52 + a46 * R54 + a62 * R51 + a78 * R53,
        a14 + a30 * R53 + a46 * R51 + a62 * R54 + a78 * R52,
        a14 + a30 * R54 + a46 * R53 + a62 * R52 + a78 * R51,
    );
    let (a94, a110, a126, a142, a158) = (
        a94 + a110 + a126 + a142 + a158,
        a94 + a110 * R51 + a126 * R52 + a142 * R53 + a158 * R54,
        a94 + a110 * R52 + a126 * R54 + a142 * R51 + a158 * R53,
        a94 + a110 * R53 + a126 * R51 + a142 * R54 + a158 * R52,
        a94 + a110 * R54 + a126 * R53 + a142 * R52 + a158 * R51,
    );
    let (a174, a190, a206, a222, a238) = (
        a174 + a190 + a206 + a222 + a238,
        a174 + a190 * R51 + a206 * R52 + a222 * R53 + a238 * R54,
        a174 + a190 * R52 + a206 * R54 + a222 * R51 + a238 * R53,
        a174 + a190 * R53 + a206 * R51 + a222 * R54 + a238 * R52,
        a174 + a190 * R54 + a206 * R53 + a222 * R52 + a238 * R51,
    );
    let (a15, a95, a175) = (
        a15 + a95 + a175,
        a15 + (a95 << 64) - (a175 << 32),
        a15 - (a95 << 32) + (a175 << 64),
    );
    let (a31, a111, a191) = (
        a31 + a111 + a191,
        a31 + (a111 << 64) - (a191 << 32),
        a31 - (a111 << 32) + (a191 << 64),
    );
    let (a47, a127, a207) = (
        a47 + a127 + a207,
        a47 + (a127 << 64) - (a207 << 32),
        a47 - (a127 << 32) + (a207 << 64),
    );
    let (a63, a143, a223) = (
        a63 + a143 + a223,
        a63 + (a143 << 64) - (a223 << 32),
        a63 - (a143 << 32) + (a223 << 64),
    );
    let (a79, a159, a239) = (
        a79 + a159 + a239,
        a79 + (a159 << 64) - (a239 << 32),
        a79 - (a159 << 32) + (a239 << 64),
    );
    let a111 = a111 * Field::new(17775832080808074370);
    let a191 = a191 * Field::new(9988211933311186582);
    let a127 = a127 * Field::new(9988211933311186582);
    let a207 = a207 * Field::new(6205107048362784195);
    let a143 = a143 * Field::new(1041288259238279555);
    let a223 = a223 * Field::new(15820824984080659046);
    let a159 = a159 * Field::new(6205107048362784195);
    let a239 = a239 * Field::new(11578395661369729110);
    let (a15, a31, a47, a63, a79) = (
        a15 + a31 + a47 + a63 + a79,
        a15 + a31 * R51 + a47 * R52 + a63 * R53 + a79 * R54,
        a15 + a31 * R52 + a47 * R54 + a63 * R51 + a79 * R53,
        a15 + a31 * R53 + a47 * R51 + a63 * R54 + a79 * R52,
        a15 + a31 * R54 + a47 * R53 + a63 * R52 + a79 * R51,
    );
    let (a95, a111, a127, a143, a159) = (
        a95 + a111 + a127 + a143 + a159,
        a95 + a111 * R51 + a127 * R52 + a143 * R53 + a159 * R54,
        a95 + a111 * R52 + a127 * R54 + a143 * R51 + a159 * R53,
        a95 + a111 * R53 + a127 * R51 + a143 * R54 + a159 * R52,
        a95 + a111 * R54 + a127 * R53 + a143 * R52 + a159 * R51,
    );
    let (a175, a191, a207, a223, a239) = (
        a175 + a191 + a207 + a223 + a239,
        a175 + a191 * R51 + a207 * R52 + a223 * R53 + a239 * R54,
        a175 + a191 * R52 + a207 * R54 + a223 * R51 + a239 * R53,
        a175 + a191 * R53 + a207 * R51 + a223 * R54 + a239 * R52,
        a175 + a191 * R54 + a207 * R53 + a223 * R52 + a239 * R51,
    );
    let a81 = a81 * Field::new(4030557868685900014);
    let a161 = a161 * Field::new(12342513394488208227);
    let a17 = a17 * Field::new(6193879297194861051);
    let a97 = a97 * Field::new(5927015354646419725);
    let a177 = (a177 << 4);
    let a33 = a33 * Field::new(9148693690730647261);
    let a113 = a113 * Field::new(13012773617665488422);
    let a193 = a193 * Field::new(6868348408044855211);
    let a49 = a49 * Field::new(2598525327269793995);
    let a129 = (a129 << 8);
    let a209 = a209 * Field::new(17251890565788265929);
    let a65 = a65 * Field::new(5290193119087387221);
    let a145 = a145 * Field::new(17659854181644761771);
    let a225 = a225 * Field::new(4682917097487535278);
    let a82 = a82 * Field::new(12342513394488208227);
    let a162 = a162 * Field::new(5927015354646419725);
    let a18 = a18 * Field::new(9148693690730647261);
    let a98 = a98 * Field::new(6868348408044855211);
    let a178 = (a178 << 8);
    let a34 = a34 * Field::new(5290193119087387221);
    let a114 = a114 * Field::new(4682917097487535278);
    let a194 = a194 * Field::new(17775832080808074370);
    let a50 = a50 * Field::new(5856505865097423521);
    let a130 = (a130 << 16);
    let a210 = a210 * Field::new(7677121419106473143);
    let a66 = a66 * Field::new(18235156514275634624);
    let a146 = a146 * Field::new(12713971610879295754);
    let a226 = a226 * Field::new(5079231842359091375);
    let a83 = a83 * Field::new(6193879297194861051);
    let a163 = a163 * Field::new(9148693690730647261);
    let a19 = a19 * Field::new(2598525327269793995);
    let a99 = a99 * Field::new(5290193119087387221);
    let a179 = (a179 << 12);
    let a35 = a35 * Field::new(5856505865097423521);
    let a115 = a115 * Field::new(7712152251710425105);
    let a195 = a195 * Field::new(18235156514275634624);
    let a51 = a51 * Field::new(12153478289216064362);
    let a131 = (a131 << 24);
    let a211 = a211 * Field::new(7480733200087124716);
    let a67 = a67 * Field::new(8149776168132872528);
    let a147 = a147 * Field::new(334345413347504175);
    let a227 = a227 * Field::new(11331573348451128694);
    let a84 = a84 * Field::new(5927015354646419725);
    let a164 = a164 * Field::new(6868348408044855211);
    let a20 = a20 * Field::new(5290193119087387221);
    let a100 = a100 * Field::new(17775832080808074370);
    let a180 = (a180 << 16);
    let a36 = a36 * Field::new(18235156514275634624);
    let a116 = a116 * Field::new(5079231842359091375);
    let a196 = a196 * Field::new(9988211933311186582);
    let a52 = a52 * Field::new(8149776168132872528);
    let a132 = (a132 << 32);
    let a212 = a212 * Field::new(5349526613560066800);
    let a68 = a68 * Field::new(1041288259238279555);
    let a148 = a148 * Field::new(4743958305399207267);
    let a228 = a228 * Field::new(15149912995474149095);
    let a85 = (a85 << 4);
    let a165 = (a165 << 8);
    let a21 = (a21 << 12);
    let a101 = (a101 << 16);
    let a181 = (a181 << 20);
    let a37 = (a37 << 24);
    let a117 = (a117 << 28);
    let a197 = (a197 << 32);
    let a53 = (a53 << 36);
    let a133 = (a133 << 40);
    let a213 = (a213 << 44);
    let a69 = (a69 << 48);
    let a149 = (a149 << 52);
    let a229 = (a229 << 56);
    let a86 = a86 * Field::new(9148693690730647261);
    let a166 = a166 * Field::new(5290193119087387221);
    let a22 = a22 * Field::new(5856505865097423521);
    let a102 = a102 * Field::new(18235156514275634624);
    let a182 = (a182 << 24);
    let a38 = a38 * Field::new(8149776168132872528);
    let a118 = a118 * Field::new(11331573348451128694);
    let a198 = a198 * Field::new(1041288259238279555);
    let a54 = a54 * Field::new(4419751934697861046);
    let a134 = (a134 << 48);
    let a214 = a214 * Field::new(4561472264319460910);
    let a70 = a70 * Field::new(17073700798457888299);
    let a150 = a150 * Field::new(2859541807139753114);
    let a230 = a230 * Field::new(17869255328328231396);
    let a87 = a87 * Field::new(13012773617665488422);
    let a167 = a167 * Field::new(4682917097487535278);
    let a23 = a23 * Field::new(7712152251710425105);
    let a103 = a103 * Field::new(5079231842359091375);
    let a183 = (a183 << 28);
    let a39 = a39 * Field::new(11331573348451128694);
    let a119 = a119 * Field::new(11805449539302731516);
    let a199 = a199 * Field::new(15149912995474149095);
    let a55 = a55 * Field::new(3918829805224079129);
    let a135 = (a135 << 56);
    let a215 = a215 * Field::new(14924795803522032290);
    let a71 = a71 * Field::new(17869255328328231396);
    let a151 = a151 * Field::new(12518016604889156391);
    let a231 = a231 * Field::new(2458871528097962065);
    let a88 = a88 * Field::new(6868348408044855211);
    let a168 = a168 * Field::new(17775832080808074370);
    let a24 = a24 * Field::new(18235156514275634624);
    let a104 = a104 * Field::new(9988211933311186582);
    let a184 = (a184 << 32);
    let a40 = a40 * Field::new(1041288259238279555);
    let a120 = a120 * Field::new(15149912995474149095);
    let a200 = a200 * Field::new(6205107048362784195);
    let a56 = a56 * Field::new(17073700798457888299);
    let a136 = (a136 << 64);
    let a216 = a216 * Field::new(12619683920608008665);
    let a72 = a72 * Field::new(15820824984080659046);
    let a152 = a152 * Field::new(6416694603501733892);
    let a232 = a232 * Field::new(7085488865146701717);
    let a89 = a89 * Field::new(2598525327269793995);
    let a169 = a169 * Field::new(5856505865097423521);
    let a25 = a25 * Field::new(12153478289216064362);
    let a105 = a105 * Field::new(8149776168132872528);
    let a185 = (a185 << 36);
    let a41 = a41 * Field::new(4419751934697861046);
    let a121 = a121 * Field::new(3918829805224079129);
    let a201 = a201 * Field::new(17073700798457888299);
    let a57 = a57 * Field::new(15685396404952554508);
    let a137 = (a137 << 72);
    let a217 = a217 * Field::new(14236101464779796609);
    let a73 = a73 * Field::new(2281812832982421726);
    let a153 = a153 * Field::new(2687357425859721546);
    let a233 = a233 * Field::new(9298050378683937060);
    let a90 = (a90 << 8);
    let a170 = (a170 << 16);
    let a26 = (a26 << 24);
    let a106 = (a106 << 32);
    let a186 = (a186 << 40);
    let a42 = (a42 << 48);
    let a122 = (a122 << 56);
    let a202 = (a202 << 64);
    let a58 = (a58 << 72);
    let a138 = (a138 << 80);
    let a218 = (a218 << 88);
    let a74 = (-a74);
    let a154 = (-(a154 << 8));
    let a234 = (-(a234 << 16));
    let a91 = a91 * Field::new(17251890565788265929);
    let a171 = a171 * Field::new(7677121419106473143);
    let a27 = a27 * Field::new(7480733200087124716);
    let a107 = a107 * Field::new(5349526613560066800);
    let a187 = (a187 << 44);
    let a43 = a43 * Field::new(4561472264319460910);
    let a123 = a123 * Field::new(14924795803522032290);
    let a203 = a203 * Field::new(12619683920608008665);
    let a59 = a59 * Field::new(14236101464779796609);
    let a139 = (a139 << 88);
    let a219 = a219 * Field::new(14416186200728684307);
    let a75 = a75 * Field::new(13156550950327197100);
    let a155 = a155 * Field::new(16976370574928729590);
    let a235 = a235 * Field::new(17272925976741953790);
    let a92 = a92 * Field::new(5290193119087387221);
    let a172 = a172 * Field::new(18235156514275634624);
    let a28 = a28 * Field::new(8149776168132872528);
    let a108 = a108 * Field::new(1041288259238279555);
    let a188 = (a188 << 48);
    let a44 = a44 * Field::new(17073700798457888299);
    let a124 = a124 * Field::new(17869255328328231396);
    let a204 = a204 * Field::new(15820824984080659046);
    let a60 = a60 * Field::new(2281812832982421726);
    let a140 = (-a140);
    let a220 = a220 * Field::new(13156550950327197100);
    let a76 = a76 * Field::new(211587555138949697);
    let a156 = a156 * Field::new(10296967901281711793);
    let a236 = a236 * Field::new(17405455810176304766);
    let a93 = a93 * Field::new(17659854181644761771);
    let a173 = a173 * Field::new(12713971610879295754);
    let a29 = a29 * Field::new(334345413347504175);
    let a109 = a109 * Field::new(4743958305399207267);
    let a189 = (a189 << 52);
    let a45 = a45 * Field::new(2859541807139753114);
    let a125 = a125 * Field::new(12518016604889156391);
    let a205 = a205 * Field::new(6416694603501733892);
    let a61 = a61 * Field::new(2687357425859721546);
    let a141 = (-(a141 << 8));
    let a221 = a221 * Field::new(16976370574928729590);
    let a77 = a77 * Field::new(10296967901281711793);
    let a157 = a157 * Field::new(6641294530111852805);
    let a237 = a237 * Field::new(3031782399165504834);
    let a94 = a94 * Field::new(4682917097487535278);
    let a174 = a174 * Field::new(5079231842359091375);
    let a30 = a30 * Field::new(11331573348451128694);
    let a110 = a110 * Field::new(15149912995474149095);
    let a190 = (a190 << 56);
    let a46 = a46 * Field::new(17869255328328231396);
    let a126 = a126 * Field::new(2458871528097962065);
    let a206 = a206 * Field::new(7085488865146701717);
    let a62 = a62 * Field::new(9298050378683937060);
    let a142 = (-(a142 << 16));
    let a222 = a222 * Field::new(17272925976741953790);
    let a78 = a78 * Field::new(17405455810176304766);
    let a158 = a158 * Field::new(3031782399165504834);
    let a238 = a238 * Field::new(12854720776751403584);
    let a95 = (a95 << 12);
    let a175 = (a175 << 24);
    let a31 = (a31 << 36);
    let a111 = (a111 << 48);
    let a191 = (a191 << 60);
    let a47 = (a47 << 72);
    let a127 = (a127 << 84);
    let a207 = (-a207);
    let a63 = (-(a63 << 12));
    let a143 = (-(a143 << 24));
    let a223 = (-(a223 << 36));
    let a79 = (-(a79 << 48));
    let a159 = (-(a159 << 60));
    let a239 = (-(a239 << 72));
    let (a0, a8) = (a0 + a8, a0 - a8);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let a12 = (a12 << 48);
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let (a5, a13) = (a5 + a13, a5 - a13);
    let a13 = (a13 << 48);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a2, a10) = (a2 + a10, a2 - a10);
    let (a6, a14) = (a6 + a14, a6 - a14);
    let a14 = (a14 << 48);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let (a7, a15) = (a7 + a15, a7 - a15);
    let a15 = (a15 << 48);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a9 = (a9 << 12);
    let a5 = (a5 << 24);
    let a13 = (a13 << 36);
    let a10 = (a10 << 24);
    let a6 = (a6 << 48);
    let a14 = (a14 << 72);
    let a11 = (a11 << 36);
    let a7 = (a7 << 72);
    let a15 = (-(a15 << 12));
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a80, a88) = (a80 + a88, a80 - a88);
    let (a84, a92) = (a84 + a92, a84 - a92);
    let a92 = (a92 << 48);
    let (a80, a84) = (a80 + a84, a80 - a84);
    let (a88, a92) = (a88 + a92, a88 - a92);
    let (a81, a89) = (a81 + a89, a81 - a89);
    let (a85, a93) = (a85 + a93, a85 - a93);
    let a93 = (a93 << 48);
    let (a81, a85) = (a81 + a85, a81 - a85);
    let (a89, a93) = (a89 + a93, a89 - a93);
    let (a82, a90) = (a82 + a90, a82 - a90);
    let (a86, a94) = (a86 + a94, a86 - a94);
    let a94 = (a94 << 48);
    let (a82, a86) = (a82 + a86, a82 - a86);
    let (a90, a94) = (a90 + a94, a90 - a94);
    let (a83, a91) = (a83 + a91, a83 - a91);
    let (a87, a95) = (a87 + a95, a87 - a95);
    let a95 = (a95 << 48);
    let (a83, a87) = (a83 + a87, a83 - a87);
    let (a91, a95) = (a91 + a95, a91 - a95);
    let a89 = (a89 << 12);
    let a85 = (a85 << 24);
    let a93 = (a93 << 36);
    let a90 = (a90 << 24);
    let a86 = (a86 << 48);
    let a94 = (a94 << 72);
    let a91 = (a91 << 36);
    let a87 = (a87 << 72);
    let a95 = (-(a95 << 12));
    let (a80, a82) = (a80 + a82, a80 - a82);
    let (a81, a83) = (a81 + a83, a81 - a83);
    let a83 = (a83 << 48);
    let (a80, a81) = (a80 + a81, a80 - a81);
    let (a82, a83) = (a82 + a83, a82 - a83);
    let (a88, a90) = (a88 + a90, a88 - a90);
    let (a89, a91) = (a89 + a91, a89 - a91);
    let a91 = (a91 << 48);
    let (a88, a89) = (a88 + a89, a88 - a89);
    let (a90, a91) = (a90 + a91, a90 - a91);
    let (a84, a86) = (a84 + a86, a84 - a86);
    let (a85, a87) = (a85 + a87, a85 - a87);
    let a87 = (a87 << 48);
    let (a84, a85) = (a84 + a85, a84 - a85);
    let (a86, a87) = (a86 + a87, a86 - a87);
    let (a92, a94) = (a92 + a94, a92 - a94);
    let (a93, a95) = (a93 + a95, a93 - a95);
    let a95 = (a95 << 48);
    let (a92, a93) = (a92 + a93, a92 - a93);
    let (a94, a95) = (a94 + a95, a94 - a95);
    let (a160, a168) = (a160 + a168, a160 - a168);
    let (a164, a172) = (a164 + a172, a164 - a172);
    let a172 = (a172 << 48);
    let (a160, a164) = (a160 + a164, a160 - a164);
    let (a168, a172) = (a168 + a172, a168 - a172);
    let (a161, a169) = (a161 + a169, a161 - a169);
    let (a165, a173) = (a165 + a173, a165 - a173);
    let a173 = (a173 << 48);
    let (a161, a165) = (a161 + a165, a161 - a165);
    let (a169, a173) = (a169 + a173, a169 - a173);
    let (a162, a170) = (a162 + a170, a162 - a170);
    let (a166, a174) = (a166 + a174, a166 - a174);
    let a174 = (a174 << 48);
    let (a162, a166) = (a162 + a166, a162 - a166);
    let (a170, a174) = (a170 + a174, a170 - a174);
    let (a163, a171) = (a163 + a171, a163 - a171);
    let (a167, a175) = (a167 + a175, a167 - a175);
    let a175 = (a175 << 48);
    let (a163, a167) = (a163 + a167, a163 - a167);
    let (a171, a175) = (a171 + a175, a171 - a175);
    let a169 = (a169 << 12);
    let a165 = (a165 << 24);
    let a173 = (a173 << 36);
    let a170 = (a170 << 24);
    let a166 = (a166 << 48);
    let a174 = (a174 << 72);
    let a171 = (a171 << 36);
    let a167 = (a167 << 72);
    let a175 = (-(a175 << 12));
    let (a160, a162) = (a160 + a162, a160 - a162);
    let (a161, a163) = (a161 + a163, a161 - a163);
    let a163 = (a163 << 48);
    let (a160, a161) = (a160 + a161, a160 - a161);
    let (a162, a163) = (a162 + a163, a162 - a163);
    let (a168, a170) = (a168 + a170, a168 - a170);
    let (a169, a171) = (a169 + a171, a169 - a171);
    let a171 = (a171 << 48);
    let (a168, a169) = (a168 + a169, a168 - a169);
    let (a170, a171) = (a170 + a171, a170 - a171);
    let (a164, a166) = (a164 + a166, a164 - a166);
    let (a165, a167) = (a165 + a167, a165 - a167);
    let a167 = (a167 << 48);
    let (a164, a165) = (a164 + a165, a164 - a165);
    let (a166, a167) = (a166 + a167, a166 - a167);
    let (a172, a174) = (a172 + a174, a172 - a174);
    let (a173, a175) = (a173 + a175, a173 - a175);
    let a175 = (a175 << 48);
    let (a172, a173) = (a172 + a173, a172 - a173);
    let (a174, a175) = (a174 + a175, a174 - a175);
    let (a16, a24) = (a16 + a24, a16 - a24);
    let (a20, a28) = (a20 + a28, a20 - a28);
    let a28 = (a28 << 48);
    let (a16, a20) = (a16 + a20, a16 - a20);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a17, a25) = (a17 + a25, a17 - a25);
    let (a21, a29) = (a21 + a29, a21 - a29);
    let a29 = (a29 << 48);
    let (a17, a21) = (a17 + a21, a17 - a21);
    let (a25, a29) = (a25 + a29, a25 - a29);
    let (a18, a26) = (a18 + a26, a18 - a26);
    let (a22, a30) = (a22 + a30, a22 - a30);
    let a30 = (a30 << 48);
    let (a18, a22) = (a18 + a22, a18 - a22);
    let (a26, a30) = (a26 + a30, a26 - a30);
    let (a19, a27) = (a19 + a27, a19 - a27);
    let (a23, a31) = (a23 + a31, a23 - a31);
    let a31 = (a31 << 48);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let (a27, a31) = (a27 + a31, a27 - a31);
    let a25 = (a25 << 12);
    let a21 = (a21 << 24);
    let a29 = (a29 << 36);
    let a26 = (a26 << 24);
    let a22 = (a22 << 48);
    let a30 = (a30 << 72);
    let a27 = (a27 << 36);
    let a23 = (a23 << 72);
    let a31 = (-(a31 << 12));
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a96, a104) = (a96 + a104, a96 - a104);
    let (a100, a108) = (a100 + a108, a100 - a108);
    let a108 = (a108 << 48);
    let (a96, a100) = (a96 + a100, a96 - a100);
    let (a104, a108) = (a104 + a108, a104 - a108);
    let (a97, a105) = (a97 + a105, a97 - a105);
    let (a101, a109) = (a101 + a109, a101 - a109);
    let a109 = (a109 << 48);
    let (a97, a101) = (a97 + a101, a97 - a101);
    let (a105, a109) = (a105 + a109, a105 - a109);
    let (a98, a106) = (a98 + a106, a98 - a106);
    let (a102, a110) = (a102 + a110, a102 - a110);
    let a110 = (a110 << 48);
    let (a98, a102) = (a98 + a102, a98 - a102);
    let (a106, a110) = (a106 + a110, a106 - a110);
    let (a99, a107) = (a99 + a107, a99 - a107);
    let (a103, a111) = (a103 + a111, a103 - a111);
    let a111 = (a111 << 48);
    let (a99, a103) = (a99 + a103, a99 - a103);
    let (a107, a111) = (a107 + a111, a107 - a111);
    let a105 = (a105 << 12);
    let a101 = (a101 << 24);
    let a109 = (a109 << 36);
    let a106 = (a106 << 24);
    let a102 = (a102 << 48);
    let a110 = (a110 << 72);
    let a107 = (a107 << 36);
    let a103 = (a103 << 72);
    let a111 = (-(a111 << 12));
    let (a96, a98) = (a96 + a98, a96 - a98);
    let (a97, a99) = (a97 + a99, a97 - a99);
    let a99 = (a99 << 48);
    let (a96, a97) = (a96 + a97, a96 - a97);
    let (a98, a99) = (a98 + a99, a98 - a99);
    let (a104, a106) = (a104 + a106, a104 - a106);
    let (a105, a107) = (a105 + a107, a105 - a107);
    let a107 = (a107 << 48);
    let (a104, a105) = (a104 + a105, a104 - a105);
    let (a106, a107) = (a106 + a107, a106 - a107);
    let (a100, a102) = (a100 + a102, a100 - a102);
    let (a101, a103) = (a101 + a103, a101 - a103);
    let a103 = (a103 << 48);
    let (a100, a101) = (a100 + a101, a100 - a101);
    let (a102, a103) = (a102 + a103, a102 - a103);
    let (a108, a110) = (a108 + a110, a108 - a110);
    let (a109, a111) = (a109 + a111, a109 - a111);
    let a111 = (a111 << 48);
    let (a108, a109) = (a108 + a109, a108 - a109);
    let (a110, a111) = (a110 + a111, a110 - a111);
    let (a176, a184) = (a176 + a184, a176 - a184);
    let (a180, a188) = (a180 + a188, a180 - a188);
    let a188 = (a188 << 48);
    let (a176, a180) = (a176 + a180, a176 - a180);
    let (a184, a188) = (a184 + a188, a184 - a188);
    let (a177, a185) = (a177 + a185, a177 - a185);
    let (a181, a189) = (a181 + a189, a181 - a189);
    let a189 = (a189 << 48);
    let (a177, a181) = (a177 + a181, a177 - a181);
    let (a185, a189) = (a185 + a189, a185 - a189);
    let (a178, a186) = (a178 + a186, a178 - a186);
    let (a182, a190) = (a182 + a190, a182 - a190);
    let a190 = (a190 << 48);
    let (a178, a182) = (a178 + a182, a178 - a182);
    let (a186, a190) = (a186 + a190, a186 - a190);
    let (a179, a187) = (a179 + a187, a179 - a187);
    let (a183, a191) = (a183 + a191, a183 - a191);
    let a191 = (a191 << 48);
    let (a179, a183) = (a179 + a183, a179 - a183);
    let (a187, a191) = (a187 + a191, a187 - a191);
    let a185 = (a185 << 12);
    let a181 = (a181 << 24);
    let a189 = (a189 << 36);
    let a186 = (a186 << 24);
    let a182 = (a182 << 48);
    let a190 = (a190 << 72);
    let a187 = (a187 << 36);
    let a183 = (a183 << 72);
    let a191 = (-(a191 << 12));
    let (a176, a178) = (a176 + a178, a176 - a178);
    let (a177, a179) = (a177 + a179, a177 - a179);
    let a179 = (a179 << 48);
    let (a176, a177) = (a176 + a177, a176 - a177);
    let (a178, a179) = (a178 + a179, a178 - a179);
    let (a184, a186) = (a184 + a186, a184 - a186);
    let (a185, a187) = (a185 + a187, a185 - a187);
    let a187 = (a187 << 48);
    let (a184, a185) = (a184 + a185, a184 - a185);
    let (a186, a187) = (a186 + a187, a186 - a187);
    let (a180, a182) = (a180 + a182, a180 - a182);
    let (a181, a183) = (a181 + a183, a181 - a183);
    let a183 = (a183 << 48);
    let (a180, a181) = (a180 + a181, a180 - a181);
    let (a182, a183) = (a182 + a183, a182 - a183);
    let (a188, a190) = (a188 + a190, a188 - a190);
    let (a189, a191) = (a189 + a191, a189 - a191);
    let a191 = (a191 << 48);
    let (a188, a189) = (a188 + a189, a188 - a189);
    let (a190, a191) = (a190 + a191, a190 - a191);
    let (a32, a40) = (a32 + a40, a32 - a40);
    let (a36, a44) = (a36 + a44, a36 - a44);
    let a44 = (a44 << 48);
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a40, a44) = (a40 + a44, a40 - a44);
    let (a33, a41) = (a33 + a41, a33 - a41);
    let (a37, a45) = (a37 + a45, a37 - a45);
    let a45 = (a45 << 48);
    let (a33, a37) = (a33 + a37, a33 - a37);
    let (a41, a45) = (a41 + a45, a41 - a45);
    let (a34, a42) = (a34 + a42, a34 - a42);
    let (a38, a46) = (a38 + a46, a38 - a46);
    let a46 = (a46 << 48);
    let (a34, a38) = (a34 + a38, a34 - a38);
    let (a42, a46) = (a42 + a46, a42 - a46);
    let (a35, a43) = (a35 + a43, a35 - a43);
    let (a39, a47) = (a39 + a47, a39 - a47);
    let a47 = (a47 << 48);
    let (a35, a39) = (a35 + a39, a35 - a39);
    let (a43, a47) = (a43 + a47, a43 - a47);
    let a41 = (a41 << 12);
    let a37 = (a37 << 24);
    let a45 = (a45 << 36);
    let a42 = (a42 << 24);
    let a38 = (a38 << 48);
    let a46 = (a46 << 72);
    let a43 = (a43 << 36);
    let a39 = (a39 << 72);
    let a47 = (-(a47 << 12));
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a40, a42) = (a40 + a42, a40 - a42);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let a43 = (a43 << 48);
    let (a40, a41) = (a40 + a41, a40 - a41);
    let (a42, a43) = (a42 + a43, a42 - a43);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    let (a44, a46) = (a44 + a46, a44 - a46);
    let (a45, a47) = (a45 + a47, a45 - a47);
    let a47 = (a47 << 48);
    let (a44, a45) = (a44 + a45, a44 - a45);
    let (a46, a47) = (a46 + a47, a46 - a47);
    let (a112, a120) = (a112 + a120, a112 - a120);
    let (a116, a124) = (a116 + a124, a116 - a124);
    let a124 = (a124 << 48);
    let (a112, a116) = (a112 + a116, a112 - a116);
    let (a120, a124) = (a120 + a124, a120 - a124);
    let (a113, a121) = (a113 + a121, a113 - a121);
    let (a117, a125) = (a117 + a125, a117 - a125);
    let a125 = (a125 << 48);
    let (a113, a117) = (a113 + a117, a113 - a117);
    let (a121, a125) = (a121 + a125, a121 - a125);
    let (a114, a122) = (a114 + a122, a114 - a122);
    let (a118, a126) = (a118 + a126, a118 - a126);
    let a126 = (a126 << 48);
    let (a114, a118) = (a114 + a118, a114 - a118);
    let (a122, a126) = (a122 + a126, a122 - a126);
    let (a115, a123) = (a115 + a123, a115 - a123);
    let (a119, a127) = (a119 + a127, a119 - a127);
    let a127 = (a127 << 48);
    let (a115, a119) = (a115 + a119, a115 - a119);
    let (a123, a127) = (a123 + a127, a123 - a127);
    let a121 = (a121 << 12);
    let a117 = (a117 << 24);
    let a125 = (a125 << 36);
    let a122 = (a122 << 24);
    let a118 = (a118 << 48);
    let a126 = (a126 << 72);
    let a123 = (a123 << 36);
    let a119 = (a119 << 72);
    let a127 = (-(a127 << 12));
    let (a112, a114) = (a112 + a114, a112 - a114);
    let (a113, a115) = (a113 + a115, a113 - a115);
    let a115 = (a115 << 48);
    let (a112, a113) = (a112 + a113, a112 - a113);
    let (a114, a115) = (a114 + a115, a114 - a115);
    let (a120, a122) = (a120 + a122, a120 - a122);
    let (a121, a123) = (a121 + a123, a121 - a123);
    let a123 = (a123 << 48);
    let (a120, a121) = (a120 + a121, a120 - a121);
    let (a122, a123) = (a122 + a123, a122 - a123);
    let (a116, a118) = (a116 + a118, a116 - a118);
    let (a117, a119) = (a117 + a119, a117 - a119);
    let a119 = (a119 << 48);
    let (a116, a117) = (a116 + a117, a116 - a117);
    let (a118, a119) = (a118 + a119, a118 - a119);
    let (a124, a126) = (a124 + a126, a124 - a126);
    let (a125, a127) = (a125 + a127, a125 - a127);
    let a127 = (a127 << 48);
    let (a124, a125) = (a124 + a125, a124 - a125);
    let (a126, a127) = (a126 + a127, a126 - a127);
    let (a192, a200) = (a192 + a200, a192 - a200);
    let (a196, a204) = (a196 + a204, a196 - a204);
    let a204 = (a204 << 48);
    let (a192, a196) = (a192 + a196, a192 - a196);
    let (a200, a204) = (a200 + a204, a200 - a204);
    let (a193, a201) = (a193 + a201, a193 - a201);
    let (a197, a205) = (a197 + a205, a197 - a205);
    let a205 = (a205 << 48);
    let (a193, a197) = (a193 + a197, a193 - a197);
    let (a201, a205) = (a201 + a205, a201 - a205);
    let (a194, a202) = (a194 + a202, a194 - a202);
    let (a198, a206) = (a198 + a206, a198 - a206);
    let a206 = (a206 << 48);
    let (a194, a198) = (a194 + a198, a194 - a198);
    let (a202, a206) = (a202 + a206, a202 - a206);
    let (a195, a203) = (a195 + a203, a195 - a203);
    let (a199, a207) = (a199 + a207, a199 - a207);
    let a207 = (a207 << 48);
    let (a195, a199) = (a195 + a199, a195 - a199);
    let (a203, a207) = (a203 + a207, a203 - a207);
    let a201 = (a201 << 12);
    let a197 = (a197 << 24);
    let a205 = (a205 << 36);
    let a202 = (a202 << 24);
    let a198 = (a198 << 48);
    let a206 = (a206 << 72);
    let a203 = (a203 << 36);
    let a199 = (a199 << 72);
    let a207 = (-(a207 << 12));
    let (a192, a194) = (a192 + a194, a192 - a194);
    let (a193, a195) = (a193 + a195, a193 - a195);
    let a195 = (a195 << 48);
    let (a192, a193) = (a192 + a193, a192 - a193);
    let (a194, a195) = (a194 + a195, a194 - a195);
    let (a200, a202) = (a200 + a202, a200 - a202);
    let (a201, a203) = (a201 + a203, a201 - a203);
    let a203 = (a203 << 48);
    let (a200, a201) = (a200 + a201, a200 - a201);
    let (a202, a203) = (a202 + a203, a202 - a203);
    let (a196, a198) = (a196 + a198, a196 - a198);
    let (a197, a199) = (a197 + a199, a197 - a199);
    let a199 = (a199 << 48);
    let (a196, a197) = (a196 + a197, a196 - a197);
    let (a198, a199) = (a198 + a199, a198 - a199);
    let (a204, a206) = (a204 + a206, a204 - a206);
    let (a205, a207) = (a205 + a207, a205 - a207);
    let a207 = (a207 << 48);
    let (a204, a205) = (a204 + a205, a204 - a205);
    let (a206, a207) = (a206 + a207, a206 - a207);
    let (a48, a56) = (a48 + a56, a48 - a56);
    let (a52, a60) = (a52 + a60, a52 - a60);
    let a60 = (a60 << 48);
    let (a48, a52) = (a48 + a52, a48 - a52);
    let (a56, a60) = (a56 + a60, a56 - a60);
    let (a49, a57) = (a49 + a57, a49 - a57);
    let (a53, a61) = (a53 + a61, a53 - a61);
    let a61 = (a61 << 48);
    let (a49, a53) = (a49 + a53, a49 - a53);
    let (a57, a61) = (a57 + a61, a57 - a61);
    let (a50, a58) = (a50 + a58, a50 - a58);
    let (a54, a62) = (a54 + a62, a54 - a62);
    let a62 = (a62 << 48);
    let (a50, a54) = (a50 + a54, a50 - a54);
    let (a58, a62) = (a58 + a62, a58 - a62);
    let (a51, a59) = (a51 + a59, a51 - a59);
    let (a55, a63) = (a55 + a63, a55 - a63);
    let a63 = (a63 << 48);
    let (a51, a55) = (a51 + a55, a51 - a55);
    let (a59, a63) = (a59 + a63, a59 - a63);
    let a57 = (a57 << 12);
    let a53 = (a53 << 24);
    let a61 = (a61 << 36);
    let a58 = (a58 << 24);
    let a54 = (a54 << 48);
    let a62 = (a62 << 72);
    let a59 = (a59 << 36);
    let a55 = (a55 << 72);
    let a63 = (-(a63 << 12));
    let (a48, a50) = (a48 + a50, a48 - a50);
    let (a49, a51) = (a49 + a51, a49 - a51);
    let a51 = (a51 << 48);
    let (a48, a49) = (a48 + a49, a48 - a49);
    let (a50, a51) = (a50 + a51, a50 - a51);
    let (a56, a58) = (a56 + a58, a56 - a58);
    let (a57, a59) = (a57 + a59, a57 - a59);
    let a59 = (a59 << 48);
    let (a56, a57) = (a56 + a57, a56 - a57);
    let (a58, a59) = (a58 + a59, a58 - a59);
    let (a52, a54) = (a52 + a54, a52 - a54);
    let (a53, a55) = (a53 + a55, a53 - a55);
    let a55 = (a55 << 48);
    let (a52, a53) = (a52 + a53, a52 - a53);
    let (a54, a55) = (a54 + a55, a54 - a55);
    let (a60, a62) = (a60 + a62, a60 - a62);
    let (a61, a63) = (a61 + a63, a61 - a63);
    let a63 = (a63 << 48);
    let (a60, a61) = (a60 + a61, a60 - a61);
    let (a62, a63) = (a62 + a63, a62 - a63);
    let (a128, a136) = (a128 + a136, a128 - a136);
    let (a132, a140) = (a132 + a140, a132 - a140);
    let a140 = (a140 << 48);
    let (a128, a132) = (a128 + a132, a128 - a132);
    let (a136, a140) = (a136 + a140, a136 - a140);
    let (a129, a137) = (a129 + a137, a129 - a137);
    let (a133, a141) = (a133 + a141, a133 - a141);
    let a141 = (a141 << 48);
    let (a129, a133) = (a129 + a133, a129 - a133);
    let (a137, a141) = (a137 + a141, a137 - a141);
    let (a130, a138) = (a130 + a138, a130 - a138);
    let (a134, a142) = (a134 + a142, a134 - a142);
    let a142 = (a142 << 48);
    let (a130, a134) = (a130 + a134, a130 - a134);
    let (a138, a142) = (a138 + a142, a138 - a142);
    let (a131, a139) = (a131 + a139, a131 - a139);
    let (a135, a143) = (a135 + a143, a135 - a143);
    let a143 = (a143 << 48);
    let (a131, a135) = (a131 + a135, a131 - a135);
    let (a139, a143) = (a139 + a143, a139 - a143);
    let a137 = (a137 << 12);
    let a133 = (a133 << 24);
    let a141 = (a141 << 36);
    let a138 = (a138 << 24);
    let a134 = (a134 << 48);
    let a142 = (a142 << 72);
    let a139 = (a139 << 36);
    let a135 = (a135 << 72);
    let a143 = (-(a143 << 12));
    let (a128, a130) = (a128 + a130, a128 - a130);
    let (a129, a131) = (a129 + a131, a129 - a131);
    let a131 = (a131 << 48);
    let (a128, a129) = (a128 + a129, a128 - a129);
    let (a130, a131) = (a130 + a131, a130 - a131);
    let (a136, a138) = (a136 + a138, a136 - a138);
    let (a137, a139) = (a137 + a139, a137 - a139);
    let a139 = (a139 << 48);
    let (a136, a137) = (a136 + a137, a136 - a137);
    let (a138, a139) = (a138 + a139, a138 - a139);
    let (a132, a134) = (a132 + a134, a132 - a134);
    let (a133, a135) = (a133 + a135, a133 - a135);
    let a135 = (a135 << 48);
    let (a132, a133) = (a132 + a133, a132 - a133);
    let (a134, a135) = (a134 + a135, a134 - a135);
    let (a140, a142) = (a140 + a142, a140 - a142);
    let (a141, a143) = (a141 + a143, a141 - a143);
    let a143 = (a143 << 48);
    let (a140, a141) = (a140 + a141, a140 - a141);
    let (a142, a143) = (a142 + a143, a142 - a143);
    let (a208, a216) = (a208 + a216, a208 - a216);
    let (a212, a220) = (a212 + a220, a212 - a220);
    let a220 = (a220 << 48);
    let (a208, a212) = (a208 + a212, a208 - a212);
    let (a216, a220) = (a216 + a220, a216 - a220);
    let (a209, a217) = (a209 + a217, a209 - a217);
    let (a213, a221) = (a213 + a221, a213 - a221);
    let a221 = (a221 << 48);
    let (a209, a213) = (a209 + a213, a209 - a213);
    let (a217, a221) = (a217 + a221, a217 - a221);
    let (a210, a218) = (a210 + a218, a210 - a218);
    let (a214, a222) = (a214 + a222, a214 - a222);
    let a222 = (a222 << 48);
    let (a210, a214) = (a210 + a214, a210 - a214);
    let (a218, a222) = (a218 + a222, a218 - a222);
    let (a211, a219) = (a211 + a219, a211 - a219);
    let (a215, a223) = (a215 + a223, a215 - a223);
    let a223 = (a223 << 48);
    let (a211, a215) = (a211 + a215, a211 - a215);
    let (a219, a223) = (a219 + a223, a219 - a223);
    let a217 = (a217 << 12);
    let a213 = (a213 << 24);
    let a221 = (a221 << 36);
    let a218 = (a218 << 24);
    let a214 = (a214 << 48);
    let a222 = (a222 << 72);
    let a219 = (a219 << 36);
    let a215 = (a215 << 72);
    let a223 = (-(a223 << 12));
    let (a208, a210) = (a208 + a210, a208 - a210);
    let (a209, a211) = (a209 + a211, a209 - a211);
    let a211 = (a211 << 48);
    let (a208, a209) = (a208 + a209, a208 - a209);
    let (a210, a211) = (a210 + a211, a210 - a211);
    let (a216, a218) = (a216 + a218, a216 - a218);
    let (a217, a219) = (a217 + a219, a217 - a219);
    let a219 = (a219 << 48);
    let (a216, a217) = (a216 + a217, a216 - a217);
    let (a218, a219) = (a218 + a219, a218 - a219);
    let (a212, a214) = (a212 + a214, a212 - a214);
    let (a213, a215) = (a213 + a215, a213 - a215);
    let a215 = (a215 << 48);
    let (a212, a213) = (a212 + a213, a212 - a213);
    let (a214, a215) = (a214 + a215, a214 - a215);
    let (a220, a222) = (a220 + a222, a220 - a222);
    let (a221, a223) = (a221 + a223, a221 - a223);
    let a223 = (a223 << 48);
    let (a220, a221) = (a220 + a221, a220 - a221);
    let (a222, a223) = (a222 + a223, a222 - a223);
    let (a64, a72) = (a64 + a72, a64 - a72);
    let (a68, a76) = (a68 + a76, a68 - a76);
    let a76 = (a76 << 48);
    let (a64, a68) = (a64 + a68, a64 - a68);
    let (a72, a76) = (a72 + a76, a72 - a76);
    let (a65, a73) = (a65 + a73, a65 - a73);
    let (a69, a77) = (a69 + a77, a69 - a77);
    let a77 = (a77 << 48);
    let (a65, a69) = (a65 + a69, a65 - a69);
    let (a73, a77) = (a73 + a77, a73 - a77);
    let (a66, a74) = (a66 + a74, a66 - a74);
    let (a70, a78) = (a70 + a78, a70 - a78);
    let a78 = (a78 << 48);
    let (a66, a70) = (a66 + a70, a66 - a70);
    let (a74, a78) = (a74 + a78, a74 - a78);
    let (a67, a75) = (a67 + a75, a67 - a75);
    let (a71, a79) = (a71 + a79, a71 - a79);
    let a79 = (a79 << 48);
    let (a67, a71) = (a67 + a71, a67 - a71);
    let (a75, a79) = (a75 + a79, a75 - a79);
    let a73 = (a73 << 12);
    let a69 = (a69 << 24);
    let a77 = (a77 << 36);
    let a74 = (a74 << 24);
    let a70 = (a70 << 48);
    let a78 = (a78 << 72);
    let a75 = (a75 << 36);
    let a71 = (a71 << 72);
    let a79 = (-(a79 << 12));
    let (a64, a66) = (a64 + a66, a64 - a66);
    let (a65, a67) = (a65 + a67, a65 - a67);
    let a67 = (a67 << 48);
    let (a64, a65) = (a64 + a65, a64 - a65);
    let (a66, a67) = (a66 + a67, a66 - a67);
    let (a72, a74) = (a72 + a74, a72 - a74);
    let (a73, a75) = (a73 + a75, a73 - a75);
    let a75 = (a75 << 48);
    let (a72, a73) = (a72 + a73, a72 - a73);
    let (a74, a75) = (a74 + a75, a74 - a75);
    let (a68, a70) = (a68 + a70, a68 - a70);
    let (a69, a71) = (a69 + a71, a69 - a71);
    let a71 = (a71 << 48);
    let (a68, a69) = (a68 + a69, a68 - a69);
    let (a70, a71) = (a70 + a71, a70 - a71);
    let (a76, a78) = (a76 + a78, a76 - a78);
    let (a77, a79) = (a77 + a79, a77 - a79);
    let a79 = (a79 << 48);
    let (a76, a77) = (a76 + a77, a76 - a77);
    let (a78, a79) = (a78 + a79, a78 - a79);
    let (a144, a152) = (a144 + a152, a144 - a152);
    let (a148, a156) = (a148 + a156, a148 - a156);
    let a156 = (a156 << 48);
    let (a144, a148) = (a144 + a148, a144 - a148);
    let (a152, a156) = (a152 + a156, a152 - a156);
    let (a145, a153) = (a145 + a153, a145 - a153);
    let (a149, a157) = (a149 + a157, a149 - a157);
    let a157 = (a157 << 48);
    let (a145, a149) = (a145 + a149, a145 - a149);
    let (a153, a157) = (a153 + a157, a153 - a157);
    let (a146, a154) = (a146 + a154, a146 - a154);
    let (a150, a158) = (a150 + a158, a150 - a158);
    let a158 = (a158 << 48);
    let (a146, a150) = (a146 + a150, a146 - a150);
    let (a154, a158) = (a154 + a158, a154 - a158);
    let (a147, a155) = (a147 + a155, a147 - a155);
    let (a151, a159) = (a151 + a159, a151 - a159);
    let a159 = (a159 << 48);
    let (a147, a151) = (a147 + a151, a147 - a151);
    let (a155, a159) = (a155 + a159, a155 - a159);
    let a153 = (a153 << 12);
    let a149 = (a149 << 24);
    let a157 = (a157 << 36);
    let a154 = (a154 << 24);
    let a150 = (a150 << 48);
    let a158 = (a158 << 72);
    let a155 = (a155 << 36);
    let a151 = (a151 << 72);
    let a159 = (-(a159 << 12));
    let (a144, a146) = (a144 + a146, a144 - a146);
    let (a145, a147) = (a145 + a147, a145 - a147);
    let a147 = (a147 << 48);
    let (a144, a145) = (a144 + a145, a144 - a145);
    let (a146, a147) = (a146 + a147, a146 - a147);
    let (a152, a154) = (a152 + a154, a152 - a154);
    let (a153, a155) = (a153 + a155, a153 - a155);
    let a155 = (a155 << 48);
    let (a152, a153) = (a152 + a153, a152 - a153);
    let (a154, a155) = (a154 + a155, a154 - a155);
    let (a148, a150) = (a148 + a150, a148 - a150);
    let (a149, a151) = (a149 + a151, a149 - a151);
    let a151 = (a151 << 48);
    let (a148, a149) = (a148 + a149, a148 - a149);
    let (a150, a151) = (a150 + a151, a150 - a151);
    let (a156, a158) = (a156 + a158, a156 - a158);
    let (a157, a159) = (a157 + a159, a157 - a159);
    let a159 = (a159 << 48);
    let (a156, a157) = (a156 + a157, a156 - a157);
    let (a158, a159) = (a158 + a159, a158 - a159);
    let (a224, a232) = (a224 + a232, a224 - a232);
    let (a228, a236) = (a228 + a236, a228 - a236);
    let a236 = (a236 << 48);
    let (a224, a228) = (a224 + a228, a224 - a228);
    let (a232, a236) = (a232 + a236, a232 - a236);
    let (a225, a233) = (a225 + a233, a225 - a233);
    let (a229, a237) = (a229 + a237, a229 - a237);
    let a237 = (a237 << 48);
    let (a225, a229) = (a225 + a229, a225 - a229);
    let (a233, a237) = (a233 + a237, a233 - a237);
    let (a226, a234) = (a226 + a234, a226 - a234);
    let (a230, a238) = (a230 + a238, a230 - a238);
    let a238 = (a238 << 48);
    let (a226, a230) = (a226 + a230, a226 - a230);
    let (a234, a238) = (a234 + a238, a234 - a238);
    let (a227, a235) = (a227 + a235, a227 - a235);
    let (a231, a239) = (a231 + a239, a231 - a239);
    let a239 = (a239 << 48);
    let (a227, a231) = (a227 + a231, a227 - a231);
    let (a235, a239) = (a235 + a239, a235 - a239);
    let a233 = (a233 << 12);
    let a229 = (a229 << 24);
    let a237 = (a237 << 36);
    let a234 = (a234 << 24);
    let a230 = (a230 << 48);
    let a238 = (a238 << 72);
    let a235 = (a235 << 36);
    let a231 = (a231 << 72);
    let a239 = (-(a239 << 12));
    let (a224, a226) = (a224 + a226, a224 - a226);
    let (a225, a227) = (a225 + a227, a225 - a227);
    let a227 = (a227 << 48);
    let (a224, a225) = (a224 + a225, a224 - a225);
    let (a226, a227) = (a226 + a227, a226 - a227);
    let (a232, a234) = (a232 + a234, a232 - a234);
    let (a233, a235) = (a233 + a235, a233 - a235);
    let a235 = (a235 << 48);
    let (a232, a233) = (a232 + a233, a232 - a233);
    let (a234, a235) = (a234 + a235, a234 - a235);
    let (a228, a230) = (a228 + a230, a228 - a230);
    let (a229, a231) = (a229 + a231, a229 - a231);
    let a231 = (a231 << 48);
    let (a228, a229) = (a228 + a229, a228 - a229);
    let (a230, a231) = (a230 + a231, a230 - a231);
    let (a236, a238) = (a236 + a238, a236 - a238);
    let (a237, a239) = (a237 + a239, a237 - a239);
    let a239 = (a239 << 48);
    let (a236, a237) = (a236 + a237, a236 - a237);
    let (a238, a239) = (a238 + a239, a238 - a239);
    values[0] = a0;
    values[1] = a80;
    values[2] = a160;
    values[3] = a16;
    values[4] = a96;
    values[5] = a176;
    values[6] = a32;
    values[7] = a112;
    values[8] = a192;
    values[9] = a48;
    values[10] = a128;
    values[11] = a208;
    values[12] = a64;
    values[13] = a144;
    values[14] = a224;
    values[15] = a8;
    values[16] = a88;
    values[17] = a168;
    values[18] = a24;
    values[19] = a104;
    values[20] = a184;
    values[21] = a40;
    values[22] = a120;
    values[23] = a200;
    values[24] = a56;
    values[25] = a136;
    values[26] = a216;
    values[27] = a72;
    values[28] = a152;
    values[29] = a232;
    values[30] = a4;
    values[31] = a84;
    values[32] = a164;
    values[33] = a20;
    values[34] = a100;
    values[35] = a180;
    values[36] = a36;
    values[37] = a116;
    values[38] = a196;
    values[39] = a52;
    values[40] = a132;
    values[41] = a212;
    values[42] = a68;
    values[43] = a148;
    values[44] = a228;
    values[45] = a12;
    values[46] = a92;
    values[47] = a172;
    values[48] = a28;
    values[49] = a108;
    values[50] = a188;
    values[51] = a44;
    values[52] = a124;
    values[53] = a204;
    values[54] = a60;
    values[55] = a140;
    values[56] = a220;
    values[57] = a76;
    values[58] = a156;
    values[59] = a236;
    values[60] = a2;
    values[61] = a82;
    values[62] = a162;
    values[63] = a18;
    values[64] = a98;
    values[65] = a178;
    values[66] = a34;
    values[67] = a114;
    values[68] = a194;
    values[69] = a50;
    values[70] = a130;
    values[71] = a210;
    values[72] = a66;
    values[73] = a146;
    values[74] = a226;
    values[75] = a10;
    values[76] = a90;
    values[77] = a170;
    values[78] = a26;
    values[79] = a106;
    values[80] = a186;
    values[81] = a42;
    values[82] = a122;
    values[83] = a202;
    values[84] = a58;
    values[85] = a138;
    values[86] = a218;
    values[87] = a74;
    values[88] = a154;
    values[89] = a234;
    values[90] = a6;
    values[91] = a86;
    values[92] = a166;
    values[93] = a22;
    values[94] = a102;
    values[95] = a182;
    values[96] = a38;
    values[97] = a118;
    values[98] = a198;
    values[99] = a54;
    values[100] = a134;
    values[101] = a214;
    values[102] = a70;
    values[103] = a150;
    values[104] = a230;
    values[105] = a14;
    values[106] = a94;
    values[107] = a174;
    values[108] = a30;
    values[109] = a110;
    values[110] = a190;
    values[111] = a46;
    values[112] = a126;
    values[113] = a206;
    values[114] = a62;
    values[115] = a142;
    values[116] = a222;
    values[117] = a78;
    values[118] = a158;
    values[119] = a238;
    values[120] = a1;
    values[121] = a81;
    values[122] = a161;
    values[123] = a17;
    values[124] = a97;
    values[125] = a177;
    values[126] = a33;
    values[127] = a113;
    values[128] = a193;
    values[129] = a49;
    values[130] = a129;
    values[131] = a209;
    values[132] = a65;
    values[133] = a145;
    values[134] = a225;
    values[135] = a9;
    values[136] = a89;
    values[137] = a169;
    values[138] = a25;
    values[139] = a105;
    values[140] = a185;
    values[141] = a41;
    values[142] = a121;
    values[143] = a201;
    values[144] = a57;
    values[145] = a137;
    values[146] = a217;
    values[147] = a73;
    values[148] = a153;
    values[149] = a233;
    values[150] = a5;
    values[151] = a85;
    values[152] = a165;
    values[153] = a21;
    values[154] = a101;
    values[155] = a181;
    values[156] = a37;
    values[157] = a117;
    values[158] = a197;
    values[159] = a53;
    values[160] = a133;
    values[161] = a213;
    values[162] = a69;
    values[163] = a149;
    values[164] = a229;
    values[165] = a13;
    values[166] = a93;
    values[167] = a173;
    values[168] = a29;
    values[169] = a109;
    values[170] = a189;
    values[171] = a45;
    values[172] = a125;
    values[173] = a205;
    values[174] = a61;
    values[175] = a141;
    values[176] = a221;
    values[177] = a77;
    values[178] = a157;
    values[179] = a237;
    values[180] = a3;
    values[181] = a83;
    values[182] = a163;
    values[183] = a19;
    values[184] = a99;
    values[185] = a179;
    values[186] = a35;
    values[187] = a115;
    values[188] = a195;
    values[189] = a51;
    values[190] = a131;
    values[191] = a211;
    values[192] = a67;
    values[193] = a147;
    values[194] = a227;
    values[195] = a11;
    values[196] = a91;
    values[197] = a171;
    values[198] = a27;
    values[199] = a107;
    values[200] = a187;
    values[201] = a43;
    values[202] = a123;
    values[203] = a203;
    values[204] = a59;
    values[205] = a139;
    values[206] = a219;
    values[207] = a75;
    values[208] = a155;
    values[209] = a235;
    values[210] = a7;
    values[211] = a87;
    values[212] = a167;
    values[213] = a23;
    values[214] = a103;
    values[215] = a183;
    values[216] = a39;
    values[217] = a119;
    values[218] = a199;
    values[219] = a55;
    values[220] = a135;
    values[221] = a215;
    values[222] = a71;
    values[223] = a151;
    values[224] = a231;
    values[225] = a15;
    values[226] = a95;
    values[227] = a175;
    values[228] = a31;
    values[229] = a111;
    values[230] = a191;
    values[231] = a47;
    values[232] = a127;
    values[233] = a207;
    values[234] = a63;
    values[235] = a143;
    values[236] = a223;
    values[237] = a79;
    values[238] = a159;
    values[239] = a239;
}

/// Size 256 NTT.
fn ntt_256(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 256);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let a6 = values[6];
    let a7 = values[7];
    let a8 = values[8];
    let a9 = values[9];
    let a10 = values[10];
    let a11 = values[11];
    let a12 = values[12];
    let a13 = values[13];
    let a14 = values[14];
    let a15 = values[15];
    let a16 = values[16];
    let a17 = values[17];
    let a18 = values[18];
    let a19 = values[19];
    let a20 = values[20];
    let a21 = values[21];
    let a22 = values[22];
    let a23 = values[23];
    let a24 = values[24];
    let a25 = values[25];
    let a26 = values[26];
    let a27 = values[27];
    let a28 = values[28];
    let a29 = values[29];
    let a30 = values[30];
    let a31 = values[31];
    let a32 = values[32];
    let a33 = values[33];
    let a34 = values[34];
    let a35 = values[35];
    let a36 = values[36];
    let a37 = values[37];
    let a38 = values[38];
    let a39 = values[39];
    let a40 = values[40];
    let a41 = values[41];
    let a42 = values[42];
    let a43 = values[43];
    let a44 = values[44];
    let a45 = values[45];
    let a46 = values[46];
    let a47 = values[47];
    let a48 = values[48];
    let a49 = values[49];
    let a50 = values[50];
    let a51 = values[51];
    let a52 = values[52];
    let a53 = values[53];
    let a54 = values[54];
    let a55 = values[55];
    let a56 = values[56];
    let a57 = values[57];
    let a58 = values[58];
    let a59 = values[59];
    let a60 = values[60];
    let a61 = values[61];
    let a62 = values[62];
    let a63 = values[63];
    let a64 = values[64];
    let a65 = values[65];
    let a66 = values[66];
    let a67 = values[67];
    let a68 = values[68];
    let a69 = values[69];
    let a70 = values[70];
    let a71 = values[71];
    let a72 = values[72];
    let a73 = values[73];
    let a74 = values[74];
    let a75 = values[75];
    let a76 = values[76];
    let a77 = values[77];
    let a78 = values[78];
    let a79 = values[79];
    let a80 = values[80];
    let a81 = values[81];
    let a82 = values[82];
    let a83 = values[83];
    let a84 = values[84];
    let a85 = values[85];
    let a86 = values[86];
    let a87 = values[87];
    let a88 = values[88];
    let a89 = values[89];
    let a90 = values[90];
    let a91 = values[91];
    let a92 = values[92];
    let a93 = values[93];
    let a94 = values[94];
    let a95 = values[95];
    let a96 = values[96];
    let a97 = values[97];
    let a98 = values[98];
    let a99 = values[99];
    let a100 = values[100];
    let a101 = values[101];
    let a102 = values[102];
    let a103 = values[103];
    let a104 = values[104];
    let a105 = values[105];
    let a106 = values[106];
    let a107 = values[107];
    let a108 = values[108];
    let a109 = values[109];
    let a110 = values[110];
    let a111 = values[111];
    let a112 = values[112];
    let a113 = values[113];
    let a114 = values[114];
    let a115 = values[115];
    let a116 = values[116];
    let a117 = values[117];
    let a118 = values[118];
    let a119 = values[119];
    let a120 = values[120];
    let a121 = values[121];
    let a122 = values[122];
    let a123 = values[123];
    let a124 = values[124];
    let a125 = values[125];
    let a126 = values[126];
    let a127 = values[127];
    let a128 = values[128];
    let a129 = values[129];
    let a130 = values[130];
    let a131 = values[131];
    let a132 = values[132];
    let a133 = values[133];
    let a134 = values[134];
    let a135 = values[135];
    let a136 = values[136];
    let a137 = values[137];
    let a138 = values[138];
    let a139 = values[139];
    let a140 = values[140];
    let a141 = values[141];
    let a142 = values[142];
    let a143 = values[143];
    let a144 = values[144];
    let a145 = values[145];
    let a146 = values[146];
    let a147 = values[147];
    let a148 = values[148];
    let a149 = values[149];
    let a150 = values[150];
    let a151 = values[151];
    let a152 = values[152];
    let a153 = values[153];
    let a154 = values[154];
    let a155 = values[155];
    let a156 = values[156];
    let a157 = values[157];
    let a158 = values[158];
    let a159 = values[159];
    let a160 = values[160];
    let a161 = values[161];
    let a162 = values[162];
    let a163 = values[163];
    let a164 = values[164];
    let a165 = values[165];
    let a166 = values[166];
    let a167 = values[167];
    let a168 = values[168];
    let a169 = values[169];
    let a170 = values[170];
    let a171 = values[171];
    let a172 = values[172];
    let a173 = values[173];
    let a174 = values[174];
    let a175 = values[175];
    let a176 = values[176];
    let a177 = values[177];
    let a178 = values[178];
    let a179 = values[179];
    let a180 = values[180];
    let a181 = values[181];
    let a182 = values[182];
    let a183 = values[183];
    let a184 = values[184];
    let a185 = values[185];
    let a186 = values[186];
    let a187 = values[187];
    let a188 = values[188];
    let a189 = values[189];
    let a190 = values[190];
    let a191 = values[191];
    let a192 = values[192];
    let a193 = values[193];
    let a194 = values[194];
    let a195 = values[195];
    let a196 = values[196];
    let a197 = values[197];
    let a198 = values[198];
    let a199 = values[199];
    let a200 = values[200];
    let a201 = values[201];
    let a202 = values[202];
    let a203 = values[203];
    let a204 = values[204];
    let a205 = values[205];
    let a206 = values[206];
    let a207 = values[207];
    let a208 = values[208];
    let a209 = values[209];
    let a210 = values[210];
    let a211 = values[211];
    let a212 = values[212];
    let a213 = values[213];
    let a214 = values[214];
    let a215 = values[215];
    let a216 = values[216];
    let a217 = values[217];
    let a218 = values[218];
    let a219 = values[219];
    let a220 = values[220];
    let a221 = values[221];
    let a222 = values[222];
    let a223 = values[223];
    let a224 = values[224];
    let a225 = values[225];
    let a226 = values[226];
    let a227 = values[227];
    let a228 = values[228];
    let a229 = values[229];
    let a230 = values[230];
    let a231 = values[231];
    let a232 = values[232];
    let a233 = values[233];
    let a234 = values[234];
    let a235 = values[235];
    let a236 = values[236];
    let a237 = values[237];
    let a238 = values[238];
    let a239 = values[239];
    let a240 = values[240];
    let a241 = values[241];
    let a242 = values[242];
    let a243 = values[243];
    let a244 = values[244];
    let a245 = values[245];
    let a246 = values[246];
    let a247 = values[247];
    let a248 = values[248];
    let a249 = values[249];
    let a250 = values[250];
    let a251 = values[251];
    let a252 = values[252];
    let a253 = values[253];
    let a254 = values[254];
    let a255 = values[255];
    let (a0, a128) = (a0 + a128, a0 - a128);
    let (a64, a192) = (a64 + a192, a64 - a192);
    let a192 = (a192 << 48);
    let (a0, a64) = (a0 + a64, a0 - a64);
    let (a128, a192) = (a128 + a192, a128 - a192);
    let (a16, a144) = (a16 + a144, a16 - a144);
    let (a80, a208) = (a80 + a208, a80 - a208);
    let a208 = (a208 << 48);
    let (a16, a80) = (a16 + a80, a16 - a80);
    let (a144, a208) = (a144 + a208, a144 - a208);
    let (a32, a160) = (a32 + a160, a32 - a160);
    let (a96, a224) = (a96 + a224, a96 - a224);
    let a224 = (a224 << 48);
    let (a32, a96) = (a32 + a96, a32 - a96);
    let (a160, a224) = (a160 + a224, a160 - a224);
    let (a48, a176) = (a48 + a176, a48 - a176);
    let (a112, a240) = (a112 + a240, a112 - a240);
    let a240 = (a240 << 48);
    let (a48, a112) = (a48 + a112, a48 - a112);
    let (a176, a240) = (a176 + a240, a176 - a240);
    let a144 = (a144 << 12);
    let a80 = (a80 << 24);
    let a208 = (a208 << 36);
    let a160 = (a160 << 24);
    let a96 = (a96 << 48);
    let a224 = (a224 << 72);
    let a176 = (a176 << 36);
    let a112 = (a112 << 72);
    let a240 = (-(a240 << 12));
    let (a0, a32) = (a0 + a32, a0 - a32);
    let (a16, a48) = (a16 + a48, a16 - a48);
    let a48 = (a48 << 48);
    let (a0, a16) = (a0 + a16, a0 - a16);
    let (a32, a48) = (a32 + a48, a32 - a48);
    let (a128, a160) = (a128 + a160, a128 - a160);
    let (a144, a176) = (a144 + a176, a144 - a176);
    let a176 = (a176 << 48);
    let (a128, a144) = (a128 + a144, a128 - a144);
    let (a160, a176) = (a160 + a176, a160 - a176);
    let (a64, a96) = (a64 + a96, a64 - a96);
    let (a80, a112) = (a80 + a112, a80 - a112);
    let a112 = (a112 << 48);
    let (a64, a80) = (a64 + a80, a64 - a80);
    let (a96, a112) = (a96 + a112, a96 - a112);
    let (a192, a224) = (a192 + a224, a192 - a224);
    let (a208, a240) = (a208 + a240, a208 - a240);
    let a240 = (a240 << 48);
    let (a192, a208) = (a192 + a208, a192 - a208);
    let (a224, a240) = (a224 + a240, a224 - a240);
    let (a1, a129) = (a1 + a129, a1 - a129);
    let (a65, a193) = (a65 + a193, a65 - a193);
    let a193 = (a193 << 48);
    let (a1, a65) = (a1 + a65, a1 - a65);
    let (a129, a193) = (a129 + a193, a129 - a193);
    let (a17, a145) = (a17 + a145, a17 - a145);
    let (a81, a209) = (a81 + a209, a81 - a209);
    let a209 = (a209 << 48);
    let (a17, a81) = (a17 + a81, a17 - a81);
    let (a145, a209) = (a145 + a209, a145 - a209);
    let (a33, a161) = (a33 + a161, a33 - a161);
    let (a97, a225) = (a97 + a225, a97 - a225);
    let a225 = (a225 << 48);
    let (a33, a97) = (a33 + a97, a33 - a97);
    let (a161, a225) = (a161 + a225, a161 - a225);
    let (a49, a177) = (a49 + a177, a49 - a177);
    let (a113, a241) = (a113 + a241, a113 - a241);
    let a241 = (a241 << 48);
    let (a49, a113) = (a49 + a113, a49 - a113);
    let (a177, a241) = (a177 + a241, a177 - a241);
    let a145 = (a145 << 12);
    let a81 = (a81 << 24);
    let a209 = (a209 << 36);
    let a161 = (a161 << 24);
    let a97 = (a97 << 48);
    let a225 = (a225 << 72);
    let a177 = (a177 << 36);
    let a113 = (a113 << 72);
    let a241 = (-(a241 << 12));
    let (a1, a33) = (a1 + a33, a1 - a33);
    let (a17, a49) = (a17 + a49, a17 - a49);
    let a49 = (a49 << 48);
    let (a1, a17) = (a1 + a17, a1 - a17);
    let (a33, a49) = (a33 + a49, a33 - a49);
    let (a129, a161) = (a129 + a161, a129 - a161);
    let (a145, a177) = (a145 + a177, a145 - a177);
    let a177 = (a177 << 48);
    let (a129, a145) = (a129 + a145, a129 - a145);
    let (a161, a177) = (a161 + a177, a161 - a177);
    let (a65, a97) = (a65 + a97, a65 - a97);
    let (a81, a113) = (a81 + a113, a81 - a113);
    let a113 = (a113 << 48);
    let (a65, a81) = (a65 + a81, a65 - a81);
    let (a97, a113) = (a97 + a113, a97 - a113);
    let (a193, a225) = (a193 + a225, a193 - a225);
    let (a209, a241) = (a209 + a241, a209 - a241);
    let a241 = (a241 << 48);
    let (a193, a209) = (a193 + a209, a193 - a209);
    let (a225, a241) = (a225 + a241, a225 - a241);
    let (a2, a130) = (a2 + a130, a2 - a130);
    let (a66, a194) = (a66 + a194, a66 - a194);
    let a194 = (a194 << 48);
    let (a2, a66) = (a2 + a66, a2 - a66);
    let (a130, a194) = (a130 + a194, a130 - a194);
    let (a18, a146) = (a18 + a146, a18 - a146);
    let (a82, a210) = (a82 + a210, a82 - a210);
    let a210 = (a210 << 48);
    let (a18, a82) = (a18 + a82, a18 - a82);
    let (a146, a210) = (a146 + a210, a146 - a210);
    let (a34, a162) = (a34 + a162, a34 - a162);
    let (a98, a226) = (a98 + a226, a98 - a226);
    let a226 = (a226 << 48);
    let (a34, a98) = (a34 + a98, a34 - a98);
    let (a162, a226) = (a162 + a226, a162 - a226);
    let (a50, a178) = (a50 + a178, a50 - a178);
    let (a114, a242) = (a114 + a242, a114 - a242);
    let a242 = (a242 << 48);
    let (a50, a114) = (a50 + a114, a50 - a114);
    let (a178, a242) = (a178 + a242, a178 - a242);
    let a146 = (a146 << 12);
    let a82 = (a82 << 24);
    let a210 = (a210 << 36);
    let a162 = (a162 << 24);
    let a98 = (a98 << 48);
    let a226 = (a226 << 72);
    let a178 = (a178 << 36);
    let a114 = (a114 << 72);
    let a242 = (-(a242 << 12));
    let (a2, a34) = (a2 + a34, a2 - a34);
    let (a18, a50) = (a18 + a50, a18 - a50);
    let a50 = (a50 << 48);
    let (a2, a18) = (a2 + a18, a2 - a18);
    let (a34, a50) = (a34 + a50, a34 - a50);
    let (a130, a162) = (a130 + a162, a130 - a162);
    let (a146, a178) = (a146 + a178, a146 - a178);
    let a178 = (a178 << 48);
    let (a130, a146) = (a130 + a146, a130 - a146);
    let (a162, a178) = (a162 + a178, a162 - a178);
    let (a66, a98) = (a66 + a98, a66 - a98);
    let (a82, a114) = (a82 + a114, a82 - a114);
    let a114 = (a114 << 48);
    let (a66, a82) = (a66 + a82, a66 - a82);
    let (a98, a114) = (a98 + a114, a98 - a114);
    let (a194, a226) = (a194 + a226, a194 - a226);
    let (a210, a242) = (a210 + a242, a210 - a242);
    let a242 = (a242 << 48);
    let (a194, a210) = (a194 + a210, a194 - a210);
    let (a226, a242) = (a226 + a242, a226 - a242);
    let (a3, a131) = (a3 + a131, a3 - a131);
    let (a67, a195) = (a67 + a195, a67 - a195);
    let a195 = (a195 << 48);
    let (a3, a67) = (a3 + a67, a3 - a67);
    let (a131, a195) = (a131 + a195, a131 - a195);
    let (a19, a147) = (a19 + a147, a19 - a147);
    let (a83, a211) = (a83 + a211, a83 - a211);
    let a211 = (a211 << 48);
    let (a19, a83) = (a19 + a83, a19 - a83);
    let (a147, a211) = (a147 + a211, a147 - a211);
    let (a35, a163) = (a35 + a163, a35 - a163);
    let (a99, a227) = (a99 + a227, a99 - a227);
    let a227 = (a227 << 48);
    let (a35, a99) = (a35 + a99, a35 - a99);
    let (a163, a227) = (a163 + a227, a163 - a227);
    let (a51, a179) = (a51 + a179, a51 - a179);
    let (a115, a243) = (a115 + a243, a115 - a243);
    let a243 = (a243 << 48);
    let (a51, a115) = (a51 + a115, a51 - a115);
    let (a179, a243) = (a179 + a243, a179 - a243);
    let a147 = (a147 << 12);
    let a83 = (a83 << 24);
    let a211 = (a211 << 36);
    let a163 = (a163 << 24);
    let a99 = (a99 << 48);
    let a227 = (a227 << 72);
    let a179 = (a179 << 36);
    let a115 = (a115 << 72);
    let a243 = (-(a243 << 12));
    let (a3, a35) = (a3 + a35, a3 - a35);
    let (a19, a51) = (a19 + a51, a19 - a51);
    let a51 = (a51 << 48);
    let (a3, a19) = (a3 + a19, a3 - a19);
    let (a35, a51) = (a35 + a51, a35 - a51);
    let (a131, a163) = (a131 + a163, a131 - a163);
    let (a147, a179) = (a147 + a179, a147 - a179);
    let a179 = (a179 << 48);
    let (a131, a147) = (a131 + a147, a131 - a147);
    let (a163, a179) = (a163 + a179, a163 - a179);
    let (a67, a99) = (a67 + a99, a67 - a99);
    let (a83, a115) = (a83 + a115, a83 - a115);
    let a115 = (a115 << 48);
    let (a67, a83) = (a67 + a83, a67 - a83);
    let (a99, a115) = (a99 + a115, a99 - a115);
    let (a195, a227) = (a195 + a227, a195 - a227);
    let (a211, a243) = (a211 + a243, a211 - a243);
    let a243 = (a243 << 48);
    let (a195, a211) = (a195 + a211, a195 - a211);
    let (a227, a243) = (a227 + a243, a227 - a243);
    let (a4, a132) = (a4 + a132, a4 - a132);
    let (a68, a196) = (a68 + a196, a68 - a196);
    let a196 = (a196 << 48);
    let (a4, a68) = (a4 + a68, a4 - a68);
    let (a132, a196) = (a132 + a196, a132 - a196);
    let (a20, a148) = (a20 + a148, a20 - a148);
    let (a84, a212) = (a84 + a212, a84 - a212);
    let a212 = (a212 << 48);
    let (a20, a84) = (a20 + a84, a20 - a84);
    let (a148, a212) = (a148 + a212, a148 - a212);
    let (a36, a164) = (a36 + a164, a36 - a164);
    let (a100, a228) = (a100 + a228, a100 - a228);
    let a228 = (a228 << 48);
    let (a36, a100) = (a36 + a100, a36 - a100);
    let (a164, a228) = (a164 + a228, a164 - a228);
    let (a52, a180) = (a52 + a180, a52 - a180);
    let (a116, a244) = (a116 + a244, a116 - a244);
    let a244 = (a244 << 48);
    let (a52, a116) = (a52 + a116, a52 - a116);
    let (a180, a244) = (a180 + a244, a180 - a244);
    let a148 = (a148 << 12);
    let a84 = (a84 << 24);
    let a212 = (a212 << 36);
    let a164 = (a164 << 24);
    let a100 = (a100 << 48);
    let a228 = (a228 << 72);
    let a180 = (a180 << 36);
    let a116 = (a116 << 72);
    let a244 = (-(a244 << 12));
    let (a4, a36) = (a4 + a36, a4 - a36);
    let (a20, a52) = (a20 + a52, a20 - a52);
    let a52 = (a52 << 48);
    let (a4, a20) = (a4 + a20, a4 - a20);
    let (a36, a52) = (a36 + a52, a36 - a52);
    let (a132, a164) = (a132 + a164, a132 - a164);
    let (a148, a180) = (a148 + a180, a148 - a180);
    let a180 = (a180 << 48);
    let (a132, a148) = (a132 + a148, a132 - a148);
    let (a164, a180) = (a164 + a180, a164 - a180);
    let (a68, a100) = (a68 + a100, a68 - a100);
    let (a84, a116) = (a84 + a116, a84 - a116);
    let a116 = (a116 << 48);
    let (a68, a84) = (a68 + a84, a68 - a84);
    let (a100, a116) = (a100 + a116, a100 - a116);
    let (a196, a228) = (a196 + a228, a196 - a228);
    let (a212, a244) = (a212 + a244, a212 - a244);
    let a244 = (a244 << 48);
    let (a196, a212) = (a196 + a212, a196 - a212);
    let (a228, a244) = (a228 + a244, a228 - a244);
    let (a5, a133) = (a5 + a133, a5 - a133);
    let (a69, a197) = (a69 + a197, a69 - a197);
    let a197 = (a197 << 48);
    let (a5, a69) = (a5 + a69, a5 - a69);
    let (a133, a197) = (a133 + a197, a133 - a197);
    let (a21, a149) = (a21 + a149, a21 - a149);
    let (a85, a213) = (a85 + a213, a85 - a213);
    let a213 = (a213 << 48);
    let (a21, a85) = (a21 + a85, a21 - a85);
    let (a149, a213) = (a149 + a213, a149 - a213);
    let (a37, a165) = (a37 + a165, a37 - a165);
    let (a101, a229) = (a101 + a229, a101 - a229);
    let a229 = (a229 << 48);
    let (a37, a101) = (a37 + a101, a37 - a101);
    let (a165, a229) = (a165 + a229, a165 - a229);
    let (a53, a181) = (a53 + a181, a53 - a181);
    let (a117, a245) = (a117 + a245, a117 - a245);
    let a245 = (a245 << 48);
    let (a53, a117) = (a53 + a117, a53 - a117);
    let (a181, a245) = (a181 + a245, a181 - a245);
    let a149 = (a149 << 12);
    let a85 = (a85 << 24);
    let a213 = (a213 << 36);
    let a165 = (a165 << 24);
    let a101 = (a101 << 48);
    let a229 = (a229 << 72);
    let a181 = (a181 << 36);
    let a117 = (a117 << 72);
    let a245 = (-(a245 << 12));
    let (a5, a37) = (a5 + a37, a5 - a37);
    let (a21, a53) = (a21 + a53, a21 - a53);
    let a53 = (a53 << 48);
    let (a5, a21) = (a5 + a21, a5 - a21);
    let (a37, a53) = (a37 + a53, a37 - a53);
    let (a133, a165) = (a133 + a165, a133 - a165);
    let (a149, a181) = (a149 + a181, a149 - a181);
    let a181 = (a181 << 48);
    let (a133, a149) = (a133 + a149, a133 - a149);
    let (a165, a181) = (a165 + a181, a165 - a181);
    let (a69, a101) = (a69 + a101, a69 - a101);
    let (a85, a117) = (a85 + a117, a85 - a117);
    let a117 = (a117 << 48);
    let (a69, a85) = (a69 + a85, a69 - a85);
    let (a101, a117) = (a101 + a117, a101 - a117);
    let (a197, a229) = (a197 + a229, a197 - a229);
    let (a213, a245) = (a213 + a245, a213 - a245);
    let a245 = (a245 << 48);
    let (a197, a213) = (a197 + a213, a197 - a213);
    let (a229, a245) = (a229 + a245, a229 - a245);
    let (a6, a134) = (a6 + a134, a6 - a134);
    let (a70, a198) = (a70 + a198, a70 - a198);
    let a198 = (a198 << 48);
    let (a6, a70) = (a6 + a70, a6 - a70);
    let (a134, a198) = (a134 + a198, a134 - a198);
    let (a22, a150) = (a22 + a150, a22 - a150);
    let (a86, a214) = (a86 + a214, a86 - a214);
    let a214 = (a214 << 48);
    let (a22, a86) = (a22 + a86, a22 - a86);
    let (a150, a214) = (a150 + a214, a150 - a214);
    let (a38, a166) = (a38 + a166, a38 - a166);
    let (a102, a230) = (a102 + a230, a102 - a230);
    let a230 = (a230 << 48);
    let (a38, a102) = (a38 + a102, a38 - a102);
    let (a166, a230) = (a166 + a230, a166 - a230);
    let (a54, a182) = (a54 + a182, a54 - a182);
    let (a118, a246) = (a118 + a246, a118 - a246);
    let a246 = (a246 << 48);
    let (a54, a118) = (a54 + a118, a54 - a118);
    let (a182, a246) = (a182 + a246, a182 - a246);
    let a150 = (a150 << 12);
    let a86 = (a86 << 24);
    let a214 = (a214 << 36);
    let a166 = (a166 << 24);
    let a102 = (a102 << 48);
    let a230 = (a230 << 72);
    let a182 = (a182 << 36);
    let a118 = (a118 << 72);
    let a246 = (-(a246 << 12));
    let (a6, a38) = (a6 + a38, a6 - a38);
    let (a22, a54) = (a22 + a54, a22 - a54);
    let a54 = (a54 << 48);
    let (a6, a22) = (a6 + a22, a6 - a22);
    let (a38, a54) = (a38 + a54, a38 - a54);
    let (a134, a166) = (a134 + a166, a134 - a166);
    let (a150, a182) = (a150 + a182, a150 - a182);
    let a182 = (a182 << 48);
    let (a134, a150) = (a134 + a150, a134 - a150);
    let (a166, a182) = (a166 + a182, a166 - a182);
    let (a70, a102) = (a70 + a102, a70 - a102);
    let (a86, a118) = (a86 + a118, a86 - a118);
    let a118 = (a118 << 48);
    let (a70, a86) = (a70 + a86, a70 - a86);
    let (a102, a118) = (a102 + a118, a102 - a118);
    let (a198, a230) = (a198 + a230, a198 - a230);
    let (a214, a246) = (a214 + a246, a214 - a246);
    let a246 = (a246 << 48);
    let (a198, a214) = (a198 + a214, a198 - a214);
    let (a230, a246) = (a230 + a246, a230 - a246);
    let (a7, a135) = (a7 + a135, a7 - a135);
    let (a71, a199) = (a71 + a199, a71 - a199);
    let a199 = (a199 << 48);
    let (a7, a71) = (a7 + a71, a7 - a71);
    let (a135, a199) = (a135 + a199, a135 - a199);
    let (a23, a151) = (a23 + a151, a23 - a151);
    let (a87, a215) = (a87 + a215, a87 - a215);
    let a215 = (a215 << 48);
    let (a23, a87) = (a23 + a87, a23 - a87);
    let (a151, a215) = (a151 + a215, a151 - a215);
    let (a39, a167) = (a39 + a167, a39 - a167);
    let (a103, a231) = (a103 + a231, a103 - a231);
    let a231 = (a231 << 48);
    let (a39, a103) = (a39 + a103, a39 - a103);
    let (a167, a231) = (a167 + a231, a167 - a231);
    let (a55, a183) = (a55 + a183, a55 - a183);
    let (a119, a247) = (a119 + a247, a119 - a247);
    let a247 = (a247 << 48);
    let (a55, a119) = (a55 + a119, a55 - a119);
    let (a183, a247) = (a183 + a247, a183 - a247);
    let a151 = (a151 << 12);
    let a87 = (a87 << 24);
    let a215 = (a215 << 36);
    let a167 = (a167 << 24);
    let a103 = (a103 << 48);
    let a231 = (a231 << 72);
    let a183 = (a183 << 36);
    let a119 = (a119 << 72);
    let a247 = (-(a247 << 12));
    let (a7, a39) = (a7 + a39, a7 - a39);
    let (a23, a55) = (a23 + a55, a23 - a55);
    let a55 = (a55 << 48);
    let (a7, a23) = (a7 + a23, a7 - a23);
    let (a39, a55) = (a39 + a55, a39 - a55);
    let (a135, a167) = (a135 + a167, a135 - a167);
    let (a151, a183) = (a151 + a183, a151 - a183);
    let a183 = (a183 << 48);
    let (a135, a151) = (a135 + a151, a135 - a151);
    let (a167, a183) = (a167 + a183, a167 - a183);
    let (a71, a103) = (a71 + a103, a71 - a103);
    let (a87, a119) = (a87 + a119, a87 - a119);
    let a119 = (a119 << 48);
    let (a71, a87) = (a71 + a87, a71 - a87);
    let (a103, a119) = (a103 + a119, a103 - a119);
    let (a199, a231) = (a199 + a231, a199 - a231);
    let (a215, a247) = (a215 + a247, a215 - a247);
    let a247 = (a247 << 48);
    let (a199, a215) = (a199 + a215, a199 - a215);
    let (a231, a247) = (a231 + a247, a231 - a247);
    let (a8, a136) = (a8 + a136, a8 - a136);
    let (a72, a200) = (a72 + a200, a72 - a200);
    let a200 = (a200 << 48);
    let (a8, a72) = (a8 + a72, a8 - a72);
    let (a136, a200) = (a136 + a200, a136 - a200);
    let (a24, a152) = (a24 + a152, a24 - a152);
    let (a88, a216) = (a88 + a216, a88 - a216);
    let a216 = (a216 << 48);
    let (a24, a88) = (a24 + a88, a24 - a88);
    let (a152, a216) = (a152 + a216, a152 - a216);
    let (a40, a168) = (a40 + a168, a40 - a168);
    let (a104, a232) = (a104 + a232, a104 - a232);
    let a232 = (a232 << 48);
    let (a40, a104) = (a40 + a104, a40 - a104);
    let (a168, a232) = (a168 + a232, a168 - a232);
    let (a56, a184) = (a56 + a184, a56 - a184);
    let (a120, a248) = (a120 + a248, a120 - a248);
    let a248 = (a248 << 48);
    let (a56, a120) = (a56 + a120, a56 - a120);
    let (a184, a248) = (a184 + a248, a184 - a248);
    let a152 = (a152 << 12);
    let a88 = (a88 << 24);
    let a216 = (a216 << 36);
    let a168 = (a168 << 24);
    let a104 = (a104 << 48);
    let a232 = (a232 << 72);
    let a184 = (a184 << 36);
    let a120 = (a120 << 72);
    let a248 = (-(a248 << 12));
    let (a8, a40) = (a8 + a40, a8 - a40);
    let (a24, a56) = (a24 + a56, a24 - a56);
    let a56 = (a56 << 48);
    let (a8, a24) = (a8 + a24, a8 - a24);
    let (a40, a56) = (a40 + a56, a40 - a56);
    let (a136, a168) = (a136 + a168, a136 - a168);
    let (a152, a184) = (a152 + a184, a152 - a184);
    let a184 = (a184 << 48);
    let (a136, a152) = (a136 + a152, a136 - a152);
    let (a168, a184) = (a168 + a184, a168 - a184);
    let (a72, a104) = (a72 + a104, a72 - a104);
    let (a88, a120) = (a88 + a120, a88 - a120);
    let a120 = (a120 << 48);
    let (a72, a88) = (a72 + a88, a72 - a88);
    let (a104, a120) = (a104 + a120, a104 - a120);
    let (a200, a232) = (a200 + a232, a200 - a232);
    let (a216, a248) = (a216 + a248, a216 - a248);
    let a248 = (a248 << 48);
    let (a200, a216) = (a200 + a216, a200 - a216);
    let (a232, a248) = (a232 + a248, a232 - a248);
    let (a9, a137) = (a9 + a137, a9 - a137);
    let (a73, a201) = (a73 + a201, a73 - a201);
    let a201 = (a201 << 48);
    let (a9, a73) = (a9 + a73, a9 - a73);
    let (a137, a201) = (a137 + a201, a137 - a201);
    let (a25, a153) = (a25 + a153, a25 - a153);
    let (a89, a217) = (a89 + a217, a89 - a217);
    let a217 = (a217 << 48);
    let (a25, a89) = (a25 + a89, a25 - a89);
    let (a153, a217) = (a153 + a217, a153 - a217);
    let (a41, a169) = (a41 + a169, a41 - a169);
    let (a105, a233) = (a105 + a233, a105 - a233);
    let a233 = (a233 << 48);
    let (a41, a105) = (a41 + a105, a41 - a105);
    let (a169, a233) = (a169 + a233, a169 - a233);
    let (a57, a185) = (a57 + a185, a57 - a185);
    let (a121, a249) = (a121 + a249, a121 - a249);
    let a249 = (a249 << 48);
    let (a57, a121) = (a57 + a121, a57 - a121);
    let (a185, a249) = (a185 + a249, a185 - a249);
    let a153 = (a153 << 12);
    let a89 = (a89 << 24);
    let a217 = (a217 << 36);
    let a169 = (a169 << 24);
    let a105 = (a105 << 48);
    let a233 = (a233 << 72);
    let a185 = (a185 << 36);
    let a121 = (a121 << 72);
    let a249 = (-(a249 << 12));
    let (a9, a41) = (a9 + a41, a9 - a41);
    let (a25, a57) = (a25 + a57, a25 - a57);
    let a57 = (a57 << 48);
    let (a9, a25) = (a9 + a25, a9 - a25);
    let (a41, a57) = (a41 + a57, a41 - a57);
    let (a137, a169) = (a137 + a169, a137 - a169);
    let (a153, a185) = (a153 + a185, a153 - a185);
    let a185 = (a185 << 48);
    let (a137, a153) = (a137 + a153, a137 - a153);
    let (a169, a185) = (a169 + a185, a169 - a185);
    let (a73, a105) = (a73 + a105, a73 - a105);
    let (a89, a121) = (a89 + a121, a89 - a121);
    let a121 = (a121 << 48);
    let (a73, a89) = (a73 + a89, a73 - a89);
    let (a105, a121) = (a105 + a121, a105 - a121);
    let (a201, a233) = (a201 + a233, a201 - a233);
    let (a217, a249) = (a217 + a249, a217 - a249);
    let a249 = (a249 << 48);
    let (a201, a217) = (a201 + a217, a201 - a217);
    let (a233, a249) = (a233 + a249, a233 - a249);
    let (a10, a138) = (a10 + a138, a10 - a138);
    let (a74, a202) = (a74 + a202, a74 - a202);
    let a202 = (a202 << 48);
    let (a10, a74) = (a10 + a74, a10 - a74);
    let (a138, a202) = (a138 + a202, a138 - a202);
    let (a26, a154) = (a26 + a154, a26 - a154);
    let (a90, a218) = (a90 + a218, a90 - a218);
    let a218 = (a218 << 48);
    let (a26, a90) = (a26 + a90, a26 - a90);
    let (a154, a218) = (a154 + a218, a154 - a218);
    let (a42, a170) = (a42 + a170, a42 - a170);
    let (a106, a234) = (a106 + a234, a106 - a234);
    let a234 = (a234 << 48);
    let (a42, a106) = (a42 + a106, a42 - a106);
    let (a170, a234) = (a170 + a234, a170 - a234);
    let (a58, a186) = (a58 + a186, a58 - a186);
    let (a122, a250) = (a122 + a250, a122 - a250);
    let a250 = (a250 << 48);
    let (a58, a122) = (a58 + a122, a58 - a122);
    let (a186, a250) = (a186 + a250, a186 - a250);
    let a154 = (a154 << 12);
    let a90 = (a90 << 24);
    let a218 = (a218 << 36);
    let a170 = (a170 << 24);
    let a106 = (a106 << 48);
    let a234 = (a234 << 72);
    let a186 = (a186 << 36);
    let a122 = (a122 << 72);
    let a250 = (-(a250 << 12));
    let (a10, a42) = (a10 + a42, a10 - a42);
    let (a26, a58) = (a26 + a58, a26 - a58);
    let a58 = (a58 << 48);
    let (a10, a26) = (a10 + a26, a10 - a26);
    let (a42, a58) = (a42 + a58, a42 - a58);
    let (a138, a170) = (a138 + a170, a138 - a170);
    let (a154, a186) = (a154 + a186, a154 - a186);
    let a186 = (a186 << 48);
    let (a138, a154) = (a138 + a154, a138 - a154);
    let (a170, a186) = (a170 + a186, a170 - a186);
    let (a74, a106) = (a74 + a106, a74 - a106);
    let (a90, a122) = (a90 + a122, a90 - a122);
    let a122 = (a122 << 48);
    let (a74, a90) = (a74 + a90, a74 - a90);
    let (a106, a122) = (a106 + a122, a106 - a122);
    let (a202, a234) = (a202 + a234, a202 - a234);
    let (a218, a250) = (a218 + a250, a218 - a250);
    let a250 = (a250 << 48);
    let (a202, a218) = (a202 + a218, a202 - a218);
    let (a234, a250) = (a234 + a250, a234 - a250);
    let (a11, a139) = (a11 + a139, a11 - a139);
    let (a75, a203) = (a75 + a203, a75 - a203);
    let a203 = (a203 << 48);
    let (a11, a75) = (a11 + a75, a11 - a75);
    let (a139, a203) = (a139 + a203, a139 - a203);
    let (a27, a155) = (a27 + a155, a27 - a155);
    let (a91, a219) = (a91 + a219, a91 - a219);
    let a219 = (a219 << 48);
    let (a27, a91) = (a27 + a91, a27 - a91);
    let (a155, a219) = (a155 + a219, a155 - a219);
    let (a43, a171) = (a43 + a171, a43 - a171);
    let (a107, a235) = (a107 + a235, a107 - a235);
    let a235 = (a235 << 48);
    let (a43, a107) = (a43 + a107, a43 - a107);
    let (a171, a235) = (a171 + a235, a171 - a235);
    let (a59, a187) = (a59 + a187, a59 - a187);
    let (a123, a251) = (a123 + a251, a123 - a251);
    let a251 = (a251 << 48);
    let (a59, a123) = (a59 + a123, a59 - a123);
    let (a187, a251) = (a187 + a251, a187 - a251);
    let a155 = (a155 << 12);
    let a91 = (a91 << 24);
    let a219 = (a219 << 36);
    let a171 = (a171 << 24);
    let a107 = (a107 << 48);
    let a235 = (a235 << 72);
    let a187 = (a187 << 36);
    let a123 = (a123 << 72);
    let a251 = (-(a251 << 12));
    let (a11, a43) = (a11 + a43, a11 - a43);
    let (a27, a59) = (a27 + a59, a27 - a59);
    let a59 = (a59 << 48);
    let (a11, a27) = (a11 + a27, a11 - a27);
    let (a43, a59) = (a43 + a59, a43 - a59);
    let (a139, a171) = (a139 + a171, a139 - a171);
    let (a155, a187) = (a155 + a187, a155 - a187);
    let a187 = (a187 << 48);
    let (a139, a155) = (a139 + a155, a139 - a155);
    let (a171, a187) = (a171 + a187, a171 - a187);
    let (a75, a107) = (a75 + a107, a75 - a107);
    let (a91, a123) = (a91 + a123, a91 - a123);
    let a123 = (a123 << 48);
    let (a75, a91) = (a75 + a91, a75 - a91);
    let (a107, a123) = (a107 + a123, a107 - a123);
    let (a203, a235) = (a203 + a235, a203 - a235);
    let (a219, a251) = (a219 + a251, a219 - a251);
    let a251 = (a251 << 48);
    let (a203, a219) = (a203 + a219, a203 - a219);
    let (a235, a251) = (a235 + a251, a235 - a251);
    let (a12, a140) = (a12 + a140, a12 - a140);
    let (a76, a204) = (a76 + a204, a76 - a204);
    let a204 = (a204 << 48);
    let (a12, a76) = (a12 + a76, a12 - a76);
    let (a140, a204) = (a140 + a204, a140 - a204);
    let (a28, a156) = (a28 + a156, a28 - a156);
    let (a92, a220) = (a92 + a220, a92 - a220);
    let a220 = (a220 << 48);
    let (a28, a92) = (a28 + a92, a28 - a92);
    let (a156, a220) = (a156 + a220, a156 - a220);
    let (a44, a172) = (a44 + a172, a44 - a172);
    let (a108, a236) = (a108 + a236, a108 - a236);
    let a236 = (a236 << 48);
    let (a44, a108) = (a44 + a108, a44 - a108);
    let (a172, a236) = (a172 + a236, a172 - a236);
    let (a60, a188) = (a60 + a188, a60 - a188);
    let (a124, a252) = (a124 + a252, a124 - a252);
    let a252 = (a252 << 48);
    let (a60, a124) = (a60 + a124, a60 - a124);
    let (a188, a252) = (a188 + a252, a188 - a252);
    let a156 = (a156 << 12);
    let a92 = (a92 << 24);
    let a220 = (a220 << 36);
    let a172 = (a172 << 24);
    let a108 = (a108 << 48);
    let a236 = (a236 << 72);
    let a188 = (a188 << 36);
    let a124 = (a124 << 72);
    let a252 = (-(a252 << 12));
    let (a12, a44) = (a12 + a44, a12 - a44);
    let (a28, a60) = (a28 + a60, a28 - a60);
    let a60 = (a60 << 48);
    let (a12, a28) = (a12 + a28, a12 - a28);
    let (a44, a60) = (a44 + a60, a44 - a60);
    let (a140, a172) = (a140 + a172, a140 - a172);
    let (a156, a188) = (a156 + a188, a156 - a188);
    let a188 = (a188 << 48);
    let (a140, a156) = (a140 + a156, a140 - a156);
    let (a172, a188) = (a172 + a188, a172 - a188);
    let (a76, a108) = (a76 + a108, a76 - a108);
    let (a92, a124) = (a92 + a124, a92 - a124);
    let a124 = (a124 << 48);
    let (a76, a92) = (a76 + a92, a76 - a92);
    let (a108, a124) = (a108 + a124, a108 - a124);
    let (a204, a236) = (a204 + a236, a204 - a236);
    let (a220, a252) = (a220 + a252, a220 - a252);
    let a252 = (a252 << 48);
    let (a204, a220) = (a204 + a220, a204 - a220);
    let (a236, a252) = (a236 + a252, a236 - a252);
    let (a13, a141) = (a13 + a141, a13 - a141);
    let (a77, a205) = (a77 + a205, a77 - a205);
    let a205 = (a205 << 48);
    let (a13, a77) = (a13 + a77, a13 - a77);
    let (a141, a205) = (a141 + a205, a141 - a205);
    let (a29, a157) = (a29 + a157, a29 - a157);
    let (a93, a221) = (a93 + a221, a93 - a221);
    let a221 = (a221 << 48);
    let (a29, a93) = (a29 + a93, a29 - a93);
    let (a157, a221) = (a157 + a221, a157 - a221);
    let (a45, a173) = (a45 + a173, a45 - a173);
    let (a109, a237) = (a109 + a237, a109 - a237);
    let a237 = (a237 << 48);
    let (a45, a109) = (a45 + a109, a45 - a109);
    let (a173, a237) = (a173 + a237, a173 - a237);
    let (a61, a189) = (a61 + a189, a61 - a189);
    let (a125, a253) = (a125 + a253, a125 - a253);
    let a253 = (a253 << 48);
    let (a61, a125) = (a61 + a125, a61 - a125);
    let (a189, a253) = (a189 + a253, a189 - a253);
    let a157 = (a157 << 12);
    let a93 = (a93 << 24);
    let a221 = (a221 << 36);
    let a173 = (a173 << 24);
    let a109 = (a109 << 48);
    let a237 = (a237 << 72);
    let a189 = (a189 << 36);
    let a125 = (a125 << 72);
    let a253 = (-(a253 << 12));
    let (a13, a45) = (a13 + a45, a13 - a45);
    let (a29, a61) = (a29 + a61, a29 - a61);
    let a61 = (a61 << 48);
    let (a13, a29) = (a13 + a29, a13 - a29);
    let (a45, a61) = (a45 + a61, a45 - a61);
    let (a141, a173) = (a141 + a173, a141 - a173);
    let (a157, a189) = (a157 + a189, a157 - a189);
    let a189 = (a189 << 48);
    let (a141, a157) = (a141 + a157, a141 - a157);
    let (a173, a189) = (a173 + a189, a173 - a189);
    let (a77, a109) = (a77 + a109, a77 - a109);
    let (a93, a125) = (a93 + a125, a93 - a125);
    let a125 = (a125 << 48);
    let (a77, a93) = (a77 + a93, a77 - a93);
    let (a109, a125) = (a109 + a125, a109 - a125);
    let (a205, a237) = (a205 + a237, a205 - a237);
    let (a221, a253) = (a221 + a253, a221 - a253);
    let a253 = (a253 << 48);
    let (a205, a221) = (a205 + a221, a205 - a221);
    let (a237, a253) = (a237 + a253, a237 - a253);
    let (a14, a142) = (a14 + a142, a14 - a142);
    let (a78, a206) = (a78 + a206, a78 - a206);
    let a206 = (a206 << 48);
    let (a14, a78) = (a14 + a78, a14 - a78);
    let (a142, a206) = (a142 + a206, a142 - a206);
    let (a30, a158) = (a30 + a158, a30 - a158);
    let (a94, a222) = (a94 + a222, a94 - a222);
    let a222 = (a222 << 48);
    let (a30, a94) = (a30 + a94, a30 - a94);
    let (a158, a222) = (a158 + a222, a158 - a222);
    let (a46, a174) = (a46 + a174, a46 - a174);
    let (a110, a238) = (a110 + a238, a110 - a238);
    let a238 = (a238 << 48);
    let (a46, a110) = (a46 + a110, a46 - a110);
    let (a174, a238) = (a174 + a238, a174 - a238);
    let (a62, a190) = (a62 + a190, a62 - a190);
    let (a126, a254) = (a126 + a254, a126 - a254);
    let a254 = (a254 << 48);
    let (a62, a126) = (a62 + a126, a62 - a126);
    let (a190, a254) = (a190 + a254, a190 - a254);
    let a158 = (a158 << 12);
    let a94 = (a94 << 24);
    let a222 = (a222 << 36);
    let a174 = (a174 << 24);
    let a110 = (a110 << 48);
    let a238 = (a238 << 72);
    let a190 = (a190 << 36);
    let a126 = (a126 << 72);
    let a254 = (-(a254 << 12));
    let (a14, a46) = (a14 + a46, a14 - a46);
    let (a30, a62) = (a30 + a62, a30 - a62);
    let a62 = (a62 << 48);
    let (a14, a30) = (a14 + a30, a14 - a30);
    let (a46, a62) = (a46 + a62, a46 - a62);
    let (a142, a174) = (a142 + a174, a142 - a174);
    let (a158, a190) = (a158 + a190, a158 - a190);
    let a190 = (a190 << 48);
    let (a142, a158) = (a142 + a158, a142 - a158);
    let (a174, a190) = (a174 + a190, a174 - a190);
    let (a78, a110) = (a78 + a110, a78 - a110);
    let (a94, a126) = (a94 + a126, a94 - a126);
    let a126 = (a126 << 48);
    let (a78, a94) = (a78 + a94, a78 - a94);
    let (a110, a126) = (a110 + a126, a110 - a126);
    let (a206, a238) = (a206 + a238, a206 - a238);
    let (a222, a254) = (a222 + a254, a222 - a254);
    let a254 = (a254 << 48);
    let (a206, a222) = (a206 + a222, a206 - a222);
    let (a238, a254) = (a238 + a254, a238 - a254);
    let (a15, a143) = (a15 + a143, a15 - a143);
    let (a79, a207) = (a79 + a207, a79 - a207);
    let a207 = (a207 << 48);
    let (a15, a79) = (a15 + a79, a15 - a79);
    let (a143, a207) = (a143 + a207, a143 - a207);
    let (a31, a159) = (a31 + a159, a31 - a159);
    let (a95, a223) = (a95 + a223, a95 - a223);
    let a223 = (a223 << 48);
    let (a31, a95) = (a31 + a95, a31 - a95);
    let (a159, a223) = (a159 + a223, a159 - a223);
    let (a47, a175) = (a47 + a175, a47 - a175);
    let (a111, a239) = (a111 + a239, a111 - a239);
    let a239 = (a239 << 48);
    let (a47, a111) = (a47 + a111, a47 - a111);
    let (a175, a239) = (a175 + a239, a175 - a239);
    let (a63, a191) = (a63 + a191, a63 - a191);
    let (a127, a255) = (a127 + a255, a127 - a255);
    let a255 = (a255 << 48);
    let (a63, a127) = (a63 + a127, a63 - a127);
    let (a191, a255) = (a191 + a255, a191 - a255);
    let a159 = (a159 << 12);
    let a95 = (a95 << 24);
    let a223 = (a223 << 36);
    let a175 = (a175 << 24);
    let a111 = (a111 << 48);
    let a239 = (a239 << 72);
    let a191 = (a191 << 36);
    let a127 = (a127 << 72);
    let a255 = (-(a255 << 12));
    let (a15, a47) = (a15 + a47, a15 - a47);
    let (a31, a63) = (a31 + a63, a31 - a63);
    let a63 = (a63 << 48);
    let (a15, a31) = (a15 + a31, a15 - a31);
    let (a47, a63) = (a47 + a63, a47 - a63);
    let (a143, a175) = (a143 + a175, a143 - a175);
    let (a159, a191) = (a159 + a191, a159 - a191);
    let a191 = (a191 << 48);
    let (a143, a159) = (a143 + a159, a143 - a159);
    let (a175, a191) = (a175 + a191, a175 - a191);
    let (a79, a111) = (a79 + a111, a79 - a111);
    let (a95, a127) = (a95 + a127, a95 - a127);
    let a127 = (a127 << 48);
    let (a79, a95) = (a79 + a95, a79 - a95);
    let (a111, a127) = (a111 + a127, a111 - a127);
    let (a207, a239) = (a207 + a239, a207 - a239);
    let (a223, a255) = (a223 + a255, a223 - a255);
    let a255 = (a255 << 48);
    let (a207, a223) = (a207 + a223, a207 - a223);
    let (a239, a255) = (a239 + a255, a239 - a255);
    let a129 = a129 * Field::new(5575382163818481237);
    let a65 = (a65 << 25) + (-(a65 << 73));
    let a193 = a193 * Field::new(3328437340319972906);
    let a33 = (a33 << 3);
    let a161 = a161 * Field::new(7709569171718681254);
    let a97 = (a97 << 28) + (-(a97 << 76));
    let a225 = a225 * Field::new(8180754653145198927);
    let a17 = (a17 << 6);
    let a145 = a145 * Field::new(6336321165505697069);
    let a81 = (a81 << 31) + (-(a81 << 79));
    let a209 = a209 * Field::new(10105805016917838453);
    let a49 = (a49 << 9);
    let a177 = a177 * Field::new(13797081185216407910);
    let a113 = (a113 << 34) + (-(a113 << 82));
    let a241 = a241 * Field::new(7059463857684370340);
    let a130 = (a130 << 25) + (-(a130 << 73));
    let a66 = (a66 << 3);
    let a194 = (a194 << 28) + (-(a194 << 76));
    let a34 = (a34 << 6);
    let a162 = (a162 << 31) + (-(a162 << 79));
    let a98 = (a98 << 9);
    let a226 = (a226 << 34) + (-(a226 << 82));
    let a18 = (a18 << 12);
    let a146 = (a146 << 37) + (-(a146 << 85));
    let a82 = (a82 << 15);
    let a210 = (a210 << 40) + (-(a210 << 88));
    let a50 = (a50 << 18);
    let a178 = (a178 << 43) + (-(a178 << 91));
    let a114 = (a114 << 21);
    let a242 = (a242 << 46) + (-(a242 << 94));
    let a131 = a131 * Field::new(3328437340319972906);
    let a67 = (a67 << 28) + (-(a67 << 76));
    let a195 = a195 * Field::new(6336321165505697069);
    let a35 = (a35 << 9);
    let a163 = a163 * Field::new(7059463857684370340);
    let a99 = (a99 << 37) + (-(a99 << 85));
    let a227 = a227 * Field::new(16016224591364643153);
    let a19 = (a19 << 18);
    let a147 = a147 * Field::new(17330401598553671485);
    let a83 = (a83 << 46) + (-(a83 << 94));
    let a211 = a211 * Field::new(9952623958621855812);
    let a51 = (a51 << 27);
    let a179 = a179 * Field::new(281721071064741919);
    let a115 = (a115 << 55) + (a115 << 7);
    let a243 = a243 * Field::new(4442103655964903148);
    let a132 = (a132 << 3);
    let a68 = (a68 << 6);
    let a196 = (a196 << 9);
    let a36 = (a36 << 12);
    let a164 = (a164 << 15);
    let a100 = (a100 << 18);
    let a228 = (a228 << 21);
    let a20 = (a20 << 24);
    let a148 = (a148 << 27);
    let a84 = (a84 << 30);
    let a212 = (a212 << 33);
    let a52 = (a52 << 36);
    let a180 = (a180 << 39);
    let a116 = (a116 << 42);
    let a244 = (a244 << 45);
    let a133 = a133 * Field::new(7709569171718681254);
    let a69 = (a69 << 31) + (-(a69 << 79));
    let a197 = a197 * Field::new(7059463857684370340);
    let a37 = (a37 << 15);
    let a165 = a165 * Field::new(17449332314429639298);
    let a101 = (a101 << 46) + (-(a101 << 94));
    let a229 = a229 * Field::new(2341058142559915780);
    let a21 = (a21 << 30);
    let a149 = a149 * Field::new(4442103655964903148);
    let a85 = (a85 << 61) + (a85 << 13);
    let a213 = a213 * Field::new(10231374777478672322);
    let a53 = (a53 << 45);
    let a181 = a181 * Field::new(14041890976876060974);
    let a117 = (a117 << 76) + (a117 << 28);
    let a245 = a245 * Field::new(10561990880479197442);
    let a134 = (a134 << 28) + (-(a134 << 76));
    let a70 = (a70 << 9);
    let a198 = (a198 << 37) + (-(a198 << 85));
    let a38 = (a38 << 18);
    let a166 = (a166 << 46) + (-(a166 << 94));
    let a102 = (a102 << 27);
    let a230 = (a230 << 55) + (a230 << 7);
    let a22 = (a22 << 36);
    let a150 = (a150 << 64) + (a150 << 16);
    let a86 = (a86 << 45);
    let a214 = (a214 << 73) + (a214 << 25);
    let a54 = (a54 << 54);
    let a182 = (a182 << 82) + (a182 << 34);
    let a118 = (a118 << 63);
    let a246 = (a246 << 91) + (a246 << 43);
    let a135 = a135 * Field::new(8180754653145198927);
    let a71 = (a71 << 34) + (-(a71 << 82));
    let a199 = a199 * Field::new(16016224591364643153);
    let a39 = (a39 << 21);
    let a167 = a167 * Field::new(2341058142559915780);
    let a103 = (a103 << 55) + (a103 << 7);
    let a231 = a231 * Field::new(17090085178304640863);
    let a23 = (a23 << 42);
    let a151 = a151 * Field::new(9171943329124577373);
    let a87 = (a87 << 76) + (a87 << 28);
    let a215 = a215 * Field::new(13664737158269917819);
    let a55 = (a55 << 63);
    let a183 = a183 * Field::new(4299803665592489687);
    let a119 = (-(a119 << 1)) + (a119 << 49);
    let a247 = a247 * Field::new(13949104517951277988);
    let a136 = (a136 << 6);
    let a72 = (a72 << 12);
    let a200 = (a200 << 18);
    let a40 = (a40 << 24);
    let a168 = (a168 << 30);
    let a104 = (a104 << 36);
    let a232 = (a232 << 42);
    let a24 = (a24 << 48);
    let a152 = (a152 << 54);
    let a88 = (a88 << 60);
    let a216 = (a216 << 66);
    let a56 = (a56 << 72);
    let a184 = (a184 << 78);
    let a120 = (a120 << 84);
    let a248 = (a248 << 90);
    let a137 = a137 * Field::new(6336321165505697069);
    let a73 = (a73 << 37) + (-(a73 << 85));
    let a201 = a201 * Field::new(17330401598553671485);
    let a41 = (a41 << 27);
    let a169 = a169 * Field::new(4442103655964903148);
    let a105 = (a105 << 64) + (a105 << 16);
    let a233 = a233 * Field::new(9171943329124577373);
    let a25 = (a25 << 54);
    let a153 = a153 * Field::new(17084176919086420947);
    let a89 = (a89 << 91) + (a89 << 43);
    let a217 = a217 * Field::new(16933017626115159474);
    let a57 = (a57 << 81);
    let a185 = a185 * Field::new(3051558327610197629);
    let a121 = (-(a121 << 22)) + (a121 << 70);
    let a249 = a249 * Field::new(10265989416269385394);
    let a138 = (a138 << 31) + (-(a138 << 79));
    let a74 = (a74 << 15);
    let a202 = (a202 << 46) + (-(a202 << 94));
    let a42 = (a42 << 30);
    let a170 = (a170 << 61) + (a170 << 13);
    let a106 = (a106 << 45);
    let a234 = (a234 << 76) + (a234 << 28);
    let a26 = (a26 << 60);
    let a154 = (a154 << 91) + (a154 << 43);
    let a90 = (a90 << 75);
    let a218 = (-(a218 << 10)) + (a218 << 58);
    let a58 = (a58 << 90);
    let a186 = (-(a186 << 25)) + (a186 << 73);
    let a122 = (-(a122 << 9));
    let a250 = (-(a250 << 40)) + (a250 << 88);
    let a139 = a139 * Field::new(10105805016917838453);
    let a75 = (a75 << 40) + (-(a75 << 88));
    let a203 = a203 * Field::new(9952623958621855812);
    let a43 = (a43 << 33);
    let a171 = a171 * Field::new(10231374777478672322);
    let a107 = (a107 << 73) + (a107 << 25);
    let a235 = a235 * Field::new(13664737158269917819);
    let a27 = (a27 << 66);
    let a155 = a155 * Field::new(16933017626115159474);
    let a91 = (-(a91 << 10)) + (a91 << 58);
    let a219 = a219 * Field::new(5965722551466996711);
    let a59 = (-(a59 << 3));
    let a187 = a187 * Field::new(11387280211730213981);
    let a123 = (-(a123 << 43)) + (a123 << 91);
    let a251 = a251 * Field::new(12612728678098075109);
    let a140 = (a140 << 9);
    let a76 = (a76 << 18);
    let a204 = (a204 << 27);
    let a44 = (a44 << 36);
    let a172 = (a172 << 45);
    let a108 = (a108 << 54);
    let a236 = (a236 << 63);
    let a28 = (a28 << 72);
    let a156 = (a156 << 81);
    let a92 = (a92 << 90);
    let a220 = (-(a220 << 3));
    let a60 = (-(a60 << 12));
    let a188 = (-(a188 << 21));
    let a124 = (-(a124 << 30));
    let a252 = (-(a252 << 39));
    let a141 = a141 * Field::new(13797081185216407910);
    let a77 = (a77 << 43) + (-(a77 << 91));
    let a205 = a205 * Field::new(281721071064741919);
    let a45 = (a45 << 39);
    let a173 = a173 * Field::new(14041890976876060974);
    let a109 = (a109 << 82) + (a109 << 34);
    let a237 = a237 * Field::new(4299803665592489687);
    let a29 = (a29 << 78);
    let a157 = a157 * Field::new(3051558327610197629);
    let a93 = (-(a93 << 25)) + (a93 << 73);
    let a221 = a221 * Field::new(11387280211730213981);
    let a61 = (-(a61 << 21));
    let a189 = a189 * Field::new(8668109077711679267);
    let a125 = (-(a125 << 64)) + (-(a125 << 16));
    let a253 = a253 * Field::new(411429644661718300);
    let a142 = (a142 << 34) + (-(a142 << 82));
    let a78 = (a78 << 21);
    let a206 = (a206 << 55) + (a206 << 7);
    let a46 = (a46 << 42);
    let a174 = (a174 << 76) + (a174 << 28);
    let a110 = (a110 << 63);
    let a238 = (-(a238 << 1)) + (a238 << 49);
    let a30 = (a30 << 84);
    let a158 = (-(a158 << 22)) + (a158 << 70);
    let a94 = (-(a94 << 9));
    let a222 = (-(a222 << 43)) + (a222 << 91);
    let a62 = (-(a62 << 30));
    let a190 = (-(a190 << 64)) + (-(a190 << 16));
    let a126 = (-(a126 << 51));
    let a254 = (-(a254 << 85)) + (-(a254 << 37));
    let a143 = a143 * Field::new(7059463857684370340);
    let a79 = (a79 << 46) + (-(a79 << 94));
    let a207 = a207 * Field::new(4442103655964903148);
    let a47 = (a47 << 45);
    let a175 = a175 * Field::new(10561990880479197442);
    let a111 = (a111 << 91) + (a111 << 43);
    let a239 = a239 * Field::new(13949104517951277988);
    let a31 = (a31 << 90);
    let a159 = a159 * Field::new(10265989416269385394);
    let a95 = (-(a95 << 40)) + (a95 << 88);
    let a223 = a223 * Field::new(12612728678098075109);
    let a63 = (-(a63 << 39));
    let a191 = a191 * Field::new(411429644661718300);
    let a127 = (-(a127 << 85)) + (-(a127 << 37));
    let a255 = a255 * Field::new(10158338780952714962);
    let (a0, a8) = (a0 + a8, a0 - a8);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let a12 = (a12 << 48);
    let (a0, a4) = (a0 + a4, a0 - a4);
    let (a8, a12) = (a8 + a12, a8 - a12);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let (a5, a13) = (a5 + a13, a5 - a13);
    let a13 = (a13 << 48);
    let (a1, a5) = (a1 + a5, a1 - a5);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a2, a10) = (a2 + a10, a2 - a10);
    let (a6, a14) = (a6 + a14, a6 - a14);
    let a14 = (a14 << 48);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a10, a14) = (a10 + a14, a10 - a14);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let (a7, a15) = (a7 + a15, a7 - a15);
    let a15 = (a15 << 48);
    let (a3, a7) = (a3 + a7, a3 - a7);
    let (a11, a15) = (a11 + a15, a11 - a15);
    let a9 = (a9 << 12);
    let a5 = (a5 << 24);
    let a13 = (a13 << 36);
    let a10 = (a10 << 24);
    let a6 = (a6 << 48);
    let a14 = (a14 << 72);
    let a11 = (a11 << 36);
    let a7 = (a7 << 72);
    let a15 = (-(a15 << 12));
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = (a3 << 48);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a8, a10) = (a8 + a10, a8 - a10);
    let (a9, a11) = (a9 + a11, a9 - a11);
    let a11 = (a11 << 48);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = (a7 << 48);
    let (a4, a5) = (a4 + a5, a4 - a5);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a15 = (a15 << 48);
    let (a12, a13) = (a12 + a13, a12 - a13);
    let (a14, a15) = (a14 + a15, a14 - a15);
    let (a128, a136) = (a128 + a136, a128 - a136);
    let (a132, a140) = (a132 + a140, a132 - a140);
    let a140 = (a140 << 48);
    let (a128, a132) = (a128 + a132, a128 - a132);
    let (a136, a140) = (a136 + a140, a136 - a140);
    let (a129, a137) = (a129 + a137, a129 - a137);
    let (a133, a141) = (a133 + a141, a133 - a141);
    let a141 = (a141 << 48);
    let (a129, a133) = (a129 + a133, a129 - a133);
    let (a137, a141) = (a137 + a141, a137 - a141);
    let (a130, a138) = (a130 + a138, a130 - a138);
    let (a134, a142) = (a134 + a142, a134 - a142);
    let a142 = (a142 << 48);
    let (a130, a134) = (a130 + a134, a130 - a134);
    let (a138, a142) = (a138 + a142, a138 - a142);
    let (a131, a139) = (a131 + a139, a131 - a139);
    let (a135, a143) = (a135 + a143, a135 - a143);
    let a143 = (a143 << 48);
    let (a131, a135) = (a131 + a135, a131 - a135);
    let (a139, a143) = (a139 + a143, a139 - a143);
    let a137 = (a137 << 12);
    let a133 = (a133 << 24);
    let a141 = (a141 << 36);
    let a138 = (a138 << 24);
    let a134 = (a134 << 48);
    let a142 = (a142 << 72);
    let a139 = (a139 << 36);
    let a135 = (a135 << 72);
    let a143 = (-(a143 << 12));
    let (a128, a130) = (a128 + a130, a128 - a130);
    let (a129, a131) = (a129 + a131, a129 - a131);
    let a131 = (a131 << 48);
    let (a128, a129) = (a128 + a129, a128 - a129);
    let (a130, a131) = (a130 + a131, a130 - a131);
    let (a136, a138) = (a136 + a138, a136 - a138);
    let (a137, a139) = (a137 + a139, a137 - a139);
    let a139 = (a139 << 48);
    let (a136, a137) = (a136 + a137, a136 - a137);
    let (a138, a139) = (a138 + a139, a138 - a139);
    let (a132, a134) = (a132 + a134, a132 - a134);
    let (a133, a135) = (a133 + a135, a133 - a135);
    let a135 = (a135 << 48);
    let (a132, a133) = (a132 + a133, a132 - a133);
    let (a134, a135) = (a134 + a135, a134 - a135);
    let (a140, a142) = (a140 + a142, a140 - a142);
    let (a141, a143) = (a141 + a143, a141 - a143);
    let a143 = (a143 << 48);
    let (a140, a141) = (a140 + a141, a140 - a141);
    let (a142, a143) = (a142 + a143, a142 - a143);
    let (a64, a72) = (a64 + a72, a64 - a72);
    let (a68, a76) = (a68 + a76, a68 - a76);
    let a76 = (a76 << 48);
    let (a64, a68) = (a64 + a68, a64 - a68);
    let (a72, a76) = (a72 + a76, a72 - a76);
    let (a65, a73) = (a65 + a73, a65 - a73);
    let (a69, a77) = (a69 + a77, a69 - a77);
    let a77 = (a77 << 48);
    let (a65, a69) = (a65 + a69, a65 - a69);
    let (a73, a77) = (a73 + a77, a73 - a77);
    let (a66, a74) = (a66 + a74, a66 - a74);
    let (a70, a78) = (a70 + a78, a70 - a78);
    let a78 = (a78 << 48);
    let (a66, a70) = (a66 + a70, a66 - a70);
    let (a74, a78) = (a74 + a78, a74 - a78);
    let (a67, a75) = (a67 + a75, a67 - a75);
    let (a71, a79) = (a71 + a79, a71 - a79);
    let a79 = (a79 << 48);
    let (a67, a71) = (a67 + a71, a67 - a71);
    let (a75, a79) = (a75 + a79, a75 - a79);
    let a73 = (a73 << 12);
    let a69 = (a69 << 24);
    let a77 = (a77 << 36);
    let a74 = (a74 << 24);
    let a70 = (a70 << 48);
    let a78 = (a78 << 72);
    let a75 = (a75 << 36);
    let a71 = (a71 << 72);
    let a79 = (-(a79 << 12));
    let (a64, a66) = (a64 + a66, a64 - a66);
    let (a65, a67) = (a65 + a67, a65 - a67);
    let a67 = (a67 << 48);
    let (a64, a65) = (a64 + a65, a64 - a65);
    let (a66, a67) = (a66 + a67, a66 - a67);
    let (a72, a74) = (a72 + a74, a72 - a74);
    let (a73, a75) = (a73 + a75, a73 - a75);
    let a75 = (a75 << 48);
    let (a72, a73) = (a72 + a73, a72 - a73);
    let (a74, a75) = (a74 + a75, a74 - a75);
    let (a68, a70) = (a68 + a70, a68 - a70);
    let (a69, a71) = (a69 + a71, a69 - a71);
    let a71 = (a71 << 48);
    let (a68, a69) = (a68 + a69, a68 - a69);
    let (a70, a71) = (a70 + a71, a70 - a71);
    let (a76, a78) = (a76 + a78, a76 - a78);
    let (a77, a79) = (a77 + a79, a77 - a79);
    let a79 = (a79 << 48);
    let (a76, a77) = (a76 + a77, a76 - a77);
    let (a78, a79) = (a78 + a79, a78 - a79);
    let (a192, a200) = (a192 + a200, a192 - a200);
    let (a196, a204) = (a196 + a204, a196 - a204);
    let a204 = (a204 << 48);
    let (a192, a196) = (a192 + a196, a192 - a196);
    let (a200, a204) = (a200 + a204, a200 - a204);
    let (a193, a201) = (a193 + a201, a193 - a201);
    let (a197, a205) = (a197 + a205, a197 - a205);
    let a205 = (a205 << 48);
    let (a193, a197) = (a193 + a197, a193 - a197);
    let (a201, a205) = (a201 + a205, a201 - a205);
    let (a194, a202) = (a194 + a202, a194 - a202);
    let (a198, a206) = (a198 + a206, a198 - a206);
    let a206 = (a206 << 48);
    let (a194, a198) = (a194 + a198, a194 - a198);
    let (a202, a206) = (a202 + a206, a202 - a206);
    let (a195, a203) = (a195 + a203, a195 - a203);
    let (a199, a207) = (a199 + a207, a199 - a207);
    let a207 = (a207 << 48);
    let (a195, a199) = (a195 + a199, a195 - a199);
    let (a203, a207) = (a203 + a207, a203 - a207);
    let a201 = (a201 << 12);
    let a197 = (a197 << 24);
    let a205 = (a205 << 36);
    let a202 = (a202 << 24);
    let a198 = (a198 << 48);
    let a206 = (a206 << 72);
    let a203 = (a203 << 36);
    let a199 = (a199 << 72);
    let a207 = (-(a207 << 12));
    let (a192, a194) = (a192 + a194, a192 - a194);
    let (a193, a195) = (a193 + a195, a193 - a195);
    let a195 = (a195 << 48);
    let (a192, a193) = (a192 + a193, a192 - a193);
    let (a194, a195) = (a194 + a195, a194 - a195);
    let (a200, a202) = (a200 + a202, a200 - a202);
    let (a201, a203) = (a201 + a203, a201 - a203);
    let a203 = (a203 << 48);
    let (a200, a201) = (a200 + a201, a200 - a201);
    let (a202, a203) = (a202 + a203, a202 - a203);
    let (a196, a198) = (a196 + a198, a196 - a198);
    let (a197, a199) = (a197 + a199, a197 - a199);
    let a199 = (a199 << 48);
    let (a196, a197) = (a196 + a197, a196 - a197);
    let (a198, a199) = (a198 + a199, a198 - a199);
    let (a204, a206) = (a204 + a206, a204 - a206);
    let (a205, a207) = (a205 + a207, a205 - a207);
    let a207 = (a207 << 48);
    let (a204, a205) = (a204 + a205, a204 - a205);
    let (a206, a207) = (a206 + a207, a206 - a207);
    let (a32, a40) = (a32 + a40, a32 - a40);
    let (a36, a44) = (a36 + a44, a36 - a44);
    let a44 = (a44 << 48);
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a40, a44) = (a40 + a44, a40 - a44);
    let (a33, a41) = (a33 + a41, a33 - a41);
    let (a37, a45) = (a37 + a45, a37 - a45);
    let a45 = (a45 << 48);
    let (a33, a37) = (a33 + a37, a33 - a37);
    let (a41, a45) = (a41 + a45, a41 - a45);
    let (a34, a42) = (a34 + a42, a34 - a42);
    let (a38, a46) = (a38 + a46, a38 - a46);
    let a46 = (a46 << 48);
    let (a34, a38) = (a34 + a38, a34 - a38);
    let (a42, a46) = (a42 + a46, a42 - a46);
    let (a35, a43) = (a35 + a43, a35 - a43);
    let (a39, a47) = (a39 + a47, a39 - a47);
    let a47 = (a47 << 48);
    let (a35, a39) = (a35 + a39, a35 - a39);
    let (a43, a47) = (a43 + a47, a43 - a47);
    let a41 = (a41 << 12);
    let a37 = (a37 << 24);
    let a45 = (a45 << 36);
    let a42 = (a42 << 24);
    let a38 = (a38 << 48);
    let a46 = (a46 << 72);
    let a43 = (a43 << 36);
    let a39 = (a39 << 72);
    let a47 = (-(a47 << 12));
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a33, a35) = (a33 + a35, a33 - a35);
    let a35 = (a35 << 48);
    let (a32, a33) = (a32 + a33, a32 - a33);
    let (a34, a35) = (a34 + a35, a34 - a35);
    let (a40, a42) = (a40 + a42, a40 - a42);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let a43 = (a43 << 48);
    let (a40, a41) = (a40 + a41, a40 - a41);
    let (a42, a43) = (a42 + a43, a42 - a43);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let a39 = (a39 << 48);
    let (a36, a37) = (a36 + a37, a36 - a37);
    let (a38, a39) = (a38 + a39, a38 - a39);
    let (a44, a46) = (a44 + a46, a44 - a46);
    let (a45, a47) = (a45 + a47, a45 - a47);
    let a47 = (a47 << 48);
    let (a44, a45) = (a44 + a45, a44 - a45);
    let (a46, a47) = (a46 + a47, a46 - a47);
    let (a160, a168) = (a160 + a168, a160 - a168);
    let (a164, a172) = (a164 + a172, a164 - a172);
    let a172 = (a172 << 48);
    let (a160, a164) = (a160 + a164, a160 - a164);
    let (a168, a172) = (a168 + a172, a168 - a172);
    let (a161, a169) = (a161 + a169, a161 - a169);
    let (a165, a173) = (a165 + a173, a165 - a173);
    let a173 = (a173 << 48);
    let (a161, a165) = (a161 + a165, a161 - a165);
    let (a169, a173) = (a169 + a173, a169 - a173);
    let (a162, a170) = (a162 + a170, a162 - a170);
    let (a166, a174) = (a166 + a174, a166 - a174);
    let a174 = (a174 << 48);
    let (a162, a166) = (a162 + a166, a162 - a166);
    let (a170, a174) = (a170 + a174, a170 - a174);
    let (a163, a171) = (a163 + a171, a163 - a171);
    let (a167, a175) = (a167 + a175, a167 - a175);
    let a175 = (a175 << 48);
    let (a163, a167) = (a163 + a167, a163 - a167);
    let (a171, a175) = (a171 + a175, a171 - a175);
    let a169 = (a169 << 12);
    let a165 = (a165 << 24);
    let a173 = (a173 << 36);
    let a170 = (a170 << 24);
    let a166 = (a166 << 48);
    let a174 = (a174 << 72);
    let a171 = (a171 << 36);
    let a167 = (a167 << 72);
    let a175 = (-(a175 << 12));
    let (a160, a162) = (a160 + a162, a160 - a162);
    let (a161, a163) = (a161 + a163, a161 - a163);
    let a163 = (a163 << 48);
    let (a160, a161) = (a160 + a161, a160 - a161);
    let (a162, a163) = (a162 + a163, a162 - a163);
    let (a168, a170) = (a168 + a170, a168 - a170);
    let (a169, a171) = (a169 + a171, a169 - a171);
    let a171 = (a171 << 48);
    let (a168, a169) = (a168 + a169, a168 - a169);
    let (a170, a171) = (a170 + a171, a170 - a171);
    let (a164, a166) = (a164 + a166, a164 - a166);
    let (a165, a167) = (a165 + a167, a165 - a167);
    let a167 = (a167 << 48);
    let (a164, a165) = (a164 + a165, a164 - a165);
    let (a166, a167) = (a166 + a167, a166 - a167);
    let (a172, a174) = (a172 + a174, a172 - a174);
    let (a173, a175) = (a173 + a175, a173 - a175);
    let a175 = (a175 << 48);
    let (a172, a173) = (a172 + a173, a172 - a173);
    let (a174, a175) = (a174 + a175, a174 - a175);
    let (a96, a104) = (a96 + a104, a96 - a104);
    let (a100, a108) = (a100 + a108, a100 - a108);
    let a108 = (a108 << 48);
    let (a96, a100) = (a96 + a100, a96 - a100);
    let (a104, a108) = (a104 + a108, a104 - a108);
    let (a97, a105) = (a97 + a105, a97 - a105);
    let (a101, a109) = (a101 + a109, a101 - a109);
    let a109 = (a109 << 48);
    let (a97, a101) = (a97 + a101, a97 - a101);
    let (a105, a109) = (a105 + a109, a105 - a109);
    let (a98, a106) = (a98 + a106, a98 - a106);
    let (a102, a110) = (a102 + a110, a102 - a110);
    let a110 = (a110 << 48);
    let (a98, a102) = (a98 + a102, a98 - a102);
    let (a106, a110) = (a106 + a110, a106 - a110);
    let (a99, a107) = (a99 + a107, a99 - a107);
    let (a103, a111) = (a103 + a111, a103 - a111);
    let a111 = (a111 << 48);
    let (a99, a103) = (a99 + a103, a99 - a103);
    let (a107, a111) = (a107 + a111, a107 - a111);
    let a105 = (a105 << 12);
    let a101 = (a101 << 24);
    let a109 = (a109 << 36);
    let a106 = (a106 << 24);
    let a102 = (a102 << 48);
    let a110 = (a110 << 72);
    let a107 = (a107 << 36);
    let a103 = (a103 << 72);
    let a111 = (-(a111 << 12));
    let (a96, a98) = (a96 + a98, a96 - a98);
    let (a97, a99) = (a97 + a99, a97 - a99);
    let a99 = (a99 << 48);
    let (a96, a97) = (a96 + a97, a96 - a97);
    let (a98, a99) = (a98 + a99, a98 - a99);
    let (a104, a106) = (a104 + a106, a104 - a106);
    let (a105, a107) = (a105 + a107, a105 - a107);
    let a107 = (a107 << 48);
    let (a104, a105) = (a104 + a105, a104 - a105);
    let (a106, a107) = (a106 + a107, a106 - a107);
    let (a100, a102) = (a100 + a102, a100 - a102);
    let (a101, a103) = (a101 + a103, a101 - a103);
    let a103 = (a103 << 48);
    let (a100, a101) = (a100 + a101, a100 - a101);
    let (a102, a103) = (a102 + a103, a102 - a103);
    let (a108, a110) = (a108 + a110, a108 - a110);
    let (a109, a111) = (a109 + a111, a109 - a111);
    let a111 = (a111 << 48);
    let (a108, a109) = (a108 + a109, a108 - a109);
    let (a110, a111) = (a110 + a111, a110 - a111);
    let (a224, a232) = (a224 + a232, a224 - a232);
    let (a228, a236) = (a228 + a236, a228 - a236);
    let a236 = (a236 << 48);
    let (a224, a228) = (a224 + a228, a224 - a228);
    let (a232, a236) = (a232 + a236, a232 - a236);
    let (a225, a233) = (a225 + a233, a225 - a233);
    let (a229, a237) = (a229 + a237, a229 - a237);
    let a237 = (a237 << 48);
    let (a225, a229) = (a225 + a229, a225 - a229);
    let (a233, a237) = (a233 + a237, a233 - a237);
    let (a226, a234) = (a226 + a234, a226 - a234);
    let (a230, a238) = (a230 + a238, a230 - a238);
    let a238 = (a238 << 48);
    let (a226, a230) = (a226 + a230, a226 - a230);
    let (a234, a238) = (a234 + a238, a234 - a238);
    let (a227, a235) = (a227 + a235, a227 - a235);
    let (a231, a239) = (a231 + a239, a231 - a239);
    let a239 = (a239 << 48);
    let (a227, a231) = (a227 + a231, a227 - a231);
    let (a235, a239) = (a235 + a239, a235 - a239);
    let a233 = (a233 << 12);
    let a229 = (a229 << 24);
    let a237 = (a237 << 36);
    let a234 = (a234 << 24);
    let a230 = (a230 << 48);
    let a238 = (a238 << 72);
    let a235 = (a235 << 36);
    let a231 = (a231 << 72);
    let a239 = (-(a239 << 12));
    let (a224, a226) = (a224 + a226, a224 - a226);
    let (a225, a227) = (a225 + a227, a225 - a227);
    let a227 = (a227 << 48);
    let (a224, a225) = (a224 + a225, a224 - a225);
    let (a226, a227) = (a226 + a227, a226 - a227);
    let (a232, a234) = (a232 + a234, a232 - a234);
    let (a233, a235) = (a233 + a235, a233 - a235);
    let a235 = (a235 << 48);
    let (a232, a233) = (a232 + a233, a232 - a233);
    let (a234, a235) = (a234 + a235, a234 - a235);
    let (a228, a230) = (a228 + a230, a228 - a230);
    let (a229, a231) = (a229 + a231, a229 - a231);
    let a231 = (a231 << 48);
    let (a228, a229) = (a228 + a229, a228 - a229);
    let (a230, a231) = (a230 + a231, a230 - a231);
    let (a236, a238) = (a236 + a238, a236 - a238);
    let (a237, a239) = (a237 + a239, a237 - a239);
    let a239 = (a239 << 48);
    let (a236, a237) = (a236 + a237, a236 - a237);
    let (a238, a239) = (a238 + a239, a238 - a239);
    let (a16, a24) = (a16 + a24, a16 - a24);
    let (a20, a28) = (a20 + a28, a20 - a28);
    let a28 = (a28 << 48);
    let (a16, a20) = (a16 + a20, a16 - a20);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a17, a25) = (a17 + a25, a17 - a25);
    let (a21, a29) = (a21 + a29, a21 - a29);
    let a29 = (a29 << 48);
    let (a17, a21) = (a17 + a21, a17 - a21);
    let (a25, a29) = (a25 + a29, a25 - a29);
    let (a18, a26) = (a18 + a26, a18 - a26);
    let (a22, a30) = (a22 + a30, a22 - a30);
    let a30 = (a30 << 48);
    let (a18, a22) = (a18 + a22, a18 - a22);
    let (a26, a30) = (a26 + a30, a26 - a30);
    let (a19, a27) = (a19 + a27, a19 - a27);
    let (a23, a31) = (a23 + a31, a23 - a31);
    let a31 = (a31 << 48);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let (a27, a31) = (a27 + a31, a27 - a31);
    let a25 = (a25 << 12);
    let a21 = (a21 << 24);
    let a29 = (a29 << 36);
    let a26 = (a26 << 24);
    let a22 = (a22 << 48);
    let a30 = (a30 << 72);
    let a27 = (a27 << 36);
    let a23 = (a23 << 72);
    let a31 = (-(a31 << 12));
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let a19 = (a19 << 48);
    let (a16, a17) = (a16 + a17, a16 - a17);
    let (a18, a19) = (a18 + a19, a18 - a19);
    let (a24, a26) = (a24 + a26, a24 - a26);
    let (a25, a27) = (a25 + a27, a25 - a27);
    let a27 = (a27 << 48);
    let (a24, a25) = (a24 + a25, a24 - a25);
    let (a26, a27) = (a26 + a27, a26 - a27);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let a23 = (a23 << 48);
    let (a20, a21) = (a20 + a21, a20 - a21);
    let (a22, a23) = (a22 + a23, a22 - a23);
    let (a28, a30) = (a28 + a30, a28 - a30);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let a31 = (a31 << 48);
    let (a28, a29) = (a28 + a29, a28 - a29);
    let (a30, a31) = (a30 + a31, a30 - a31);
    let (a144, a152) = (a144 + a152, a144 - a152);
    let (a148, a156) = (a148 + a156, a148 - a156);
    let a156 = (a156 << 48);
    let (a144, a148) = (a144 + a148, a144 - a148);
    let (a152, a156) = (a152 + a156, a152 - a156);
    let (a145, a153) = (a145 + a153, a145 - a153);
    let (a149, a157) = (a149 + a157, a149 - a157);
    let a157 = (a157 << 48);
    let (a145, a149) = (a145 + a149, a145 - a149);
    let (a153, a157) = (a153 + a157, a153 - a157);
    let (a146, a154) = (a146 + a154, a146 - a154);
    let (a150, a158) = (a150 + a158, a150 - a158);
    let a158 = (a158 << 48);
    let (a146, a150) = (a146 + a150, a146 - a150);
    let (a154, a158) = (a154 + a158, a154 - a158);
    let (a147, a155) = (a147 + a155, a147 - a155);
    let (a151, a159) = (a151 + a159, a151 - a159);
    let a159 = (a159 << 48);
    let (a147, a151) = (a147 + a151, a147 - a151);
    let (a155, a159) = (a155 + a159, a155 - a159);
    let a153 = (a153 << 12);
    let a149 = (a149 << 24);
    let a157 = (a157 << 36);
    let a154 = (a154 << 24);
    let a150 = (a150 << 48);
    let a158 = (a158 << 72);
    let a155 = (a155 << 36);
    let a151 = (a151 << 72);
    let a159 = (-(a159 << 12));
    let (a144, a146) = (a144 + a146, a144 - a146);
    let (a145, a147) = (a145 + a147, a145 - a147);
    let a147 = (a147 << 48);
    let (a144, a145) = (a144 + a145, a144 - a145);
    let (a146, a147) = (a146 + a147, a146 - a147);
    let (a152, a154) = (a152 + a154, a152 - a154);
    let (a153, a155) = (a153 + a155, a153 - a155);
    let a155 = (a155 << 48);
    let (a152, a153) = (a152 + a153, a152 - a153);
    let (a154, a155) = (a154 + a155, a154 - a155);
    let (a148, a150) = (a148 + a150, a148 - a150);
    let (a149, a151) = (a149 + a151, a149 - a151);
    let a151 = (a151 << 48);
    let (a148, a149) = (a148 + a149, a148 - a149);
    let (a150, a151) = (a150 + a151, a150 - a151);
    let (a156, a158) = (a156 + a158, a156 - a158);
    let (a157, a159) = (a157 + a159, a157 - a159);
    let a159 = (a159 << 48);
    let (a156, a157) = (a156 + a157, a156 - a157);
    let (a158, a159) = (a158 + a159, a158 - a159);
    let (a80, a88) = (a80 + a88, a80 - a88);
    let (a84, a92) = (a84 + a92, a84 - a92);
    let a92 = (a92 << 48);
    let (a80, a84) = (a80 + a84, a80 - a84);
    let (a88, a92) = (a88 + a92, a88 - a92);
    let (a81, a89) = (a81 + a89, a81 - a89);
    let (a85, a93) = (a85 + a93, a85 - a93);
    let a93 = (a93 << 48);
    let (a81, a85) = (a81 + a85, a81 - a85);
    let (a89, a93) = (a89 + a93, a89 - a93);
    let (a82, a90) = (a82 + a90, a82 - a90);
    let (a86, a94) = (a86 + a94, a86 - a94);
    let a94 = (a94 << 48);
    let (a82, a86) = (a82 + a86, a82 - a86);
    let (a90, a94) = (a90 + a94, a90 - a94);
    let (a83, a91) = (a83 + a91, a83 - a91);
    let (a87, a95) = (a87 + a95, a87 - a95);
    let a95 = (a95 << 48);
    let (a83, a87) = (a83 + a87, a83 - a87);
    let (a91, a95) = (a91 + a95, a91 - a95);
    let a89 = (a89 << 12);
    let a85 = (a85 << 24);
    let a93 = (a93 << 36);
    let a90 = (a90 << 24);
    let a86 = (a86 << 48);
    let a94 = (a94 << 72);
    let a91 = (a91 << 36);
    let a87 = (a87 << 72);
    let a95 = (-(a95 << 12));
    let (a80, a82) = (a80 + a82, a80 - a82);
    let (a81, a83) = (a81 + a83, a81 - a83);
    let a83 = (a83 << 48);
    let (a80, a81) = (a80 + a81, a80 - a81);
    let (a82, a83) = (a82 + a83, a82 - a83);
    let (a88, a90) = (a88 + a90, a88 - a90);
    let (a89, a91) = (a89 + a91, a89 - a91);
    let a91 = (a91 << 48);
    let (a88, a89) = (a88 + a89, a88 - a89);
    let (a90, a91) = (a90 + a91, a90 - a91);
    let (a84, a86) = (a84 + a86, a84 - a86);
    let (a85, a87) = (a85 + a87, a85 - a87);
    let a87 = (a87 << 48);
    let (a84, a85) = (a84 + a85, a84 - a85);
    let (a86, a87) = (a86 + a87, a86 - a87);
    let (a92, a94) = (a92 + a94, a92 - a94);
    let (a93, a95) = (a93 + a95, a93 - a95);
    let a95 = (a95 << 48);
    let (a92, a93) = (a92 + a93, a92 - a93);
    let (a94, a95) = (a94 + a95, a94 - a95);
    let (a208, a216) = (a208 + a216, a208 - a216);
    let (a212, a220) = (a212 + a220, a212 - a220);
    let a220 = (a220 << 48);
    let (a208, a212) = (a208 + a212, a208 - a212);
    let (a216, a220) = (a216 + a220, a216 - a220);
    let (a209, a217) = (a209 + a217, a209 - a217);
    let (a213, a221) = (a213 + a221, a213 - a221);
    let a221 = (a221 << 48);
    let (a209, a213) = (a209 + a213, a209 - a213);
    let (a217, a221) = (a217 + a221, a217 - a221);
    let (a210, a218) = (a210 + a218, a210 - a218);
    let (a214, a222) = (a214 + a222, a214 - a222);
    let a222 = (a222 << 48);
    let (a210, a214) = (a210 + a214, a210 - a214);
    let (a218, a222) = (a218 + a222, a218 - a222);
    let (a211, a219) = (a211 + a219, a211 - a219);
    let (a215, a223) = (a215 + a223, a215 - a223);
    let a223 = (a223 << 48);
    let (a211, a215) = (a211 + a215, a211 - a215);
    let (a219, a223) = (a219 + a223, a219 - a223);
    let a217 = (a217 << 12);
    let a213 = (a213 << 24);
    let a221 = (a221 << 36);
    let a218 = (a218 << 24);
    let a214 = (a214 << 48);
    let a222 = (a222 << 72);
    let a219 = (a219 << 36);
    let a215 = (a215 << 72);
    let a223 = (-(a223 << 12));
    let (a208, a210) = (a208 + a210, a208 - a210);
    let (a209, a211) = (a209 + a211, a209 - a211);
    let a211 = (a211 << 48);
    let (a208, a209) = (a208 + a209, a208 - a209);
    let (a210, a211) = (a210 + a211, a210 - a211);
    let (a216, a218) = (a216 + a218, a216 - a218);
    let (a217, a219) = (a217 + a219, a217 - a219);
    let a219 = (a219 << 48);
    let (a216, a217) = (a216 + a217, a216 - a217);
    let (a218, a219) = (a218 + a219, a218 - a219);
    let (a212, a214) = (a212 + a214, a212 - a214);
    let (a213, a215) = (a213 + a215, a213 - a215);
    let a215 = (a215 << 48);
    let (a212, a213) = (a212 + a213, a212 - a213);
    let (a214, a215) = (a214 + a215, a214 - a215);
    let (a220, a222) = (a220 + a222, a220 - a222);
    let (a221, a223) = (a221 + a223, a221 - a223);
    let a223 = (a223 << 48);
    let (a220, a221) = (a220 + a221, a220 - a221);
    let (a222, a223) = (a222 + a223, a222 - a223);
    let (a48, a56) = (a48 + a56, a48 - a56);
    let (a52, a60) = (a52 + a60, a52 - a60);
    let a60 = (a60 << 48);
    let (a48, a52) = (a48 + a52, a48 - a52);
    let (a56, a60) = (a56 + a60, a56 - a60);
    let (a49, a57) = (a49 + a57, a49 - a57);
    let (a53, a61) = (a53 + a61, a53 - a61);
    let a61 = (a61 << 48);
    let (a49, a53) = (a49 + a53, a49 - a53);
    let (a57, a61) = (a57 + a61, a57 - a61);
    let (a50, a58) = (a50 + a58, a50 - a58);
    let (a54, a62) = (a54 + a62, a54 - a62);
    let a62 = (a62 << 48);
    let (a50, a54) = (a50 + a54, a50 - a54);
    let (a58, a62) = (a58 + a62, a58 - a62);
    let (a51, a59) = (a51 + a59, a51 - a59);
    let (a55, a63) = (a55 + a63, a55 - a63);
    let a63 = (a63 << 48);
    let (a51, a55) = (a51 + a55, a51 - a55);
    let (a59, a63) = (a59 + a63, a59 - a63);
    let a57 = (a57 << 12);
    let a53 = (a53 << 24);
    let a61 = (a61 << 36);
    let a58 = (a58 << 24);
    let a54 = (a54 << 48);
    let a62 = (a62 << 72);
    let a59 = (a59 << 36);
    let a55 = (a55 << 72);
    let a63 = (-(a63 << 12));
    let (a48, a50) = (a48 + a50, a48 - a50);
    let (a49, a51) = (a49 + a51, a49 - a51);
    let a51 = (a51 << 48);
    let (a48, a49) = (a48 + a49, a48 - a49);
    let (a50, a51) = (a50 + a51, a50 - a51);
    let (a56, a58) = (a56 + a58, a56 - a58);
    let (a57, a59) = (a57 + a59, a57 - a59);
    let a59 = (a59 << 48);
    let (a56, a57) = (a56 + a57, a56 - a57);
    let (a58, a59) = (a58 + a59, a58 - a59);
    let (a52, a54) = (a52 + a54, a52 - a54);
    let (a53, a55) = (a53 + a55, a53 - a55);
    let a55 = (a55 << 48);
    let (a52, a53) = (a52 + a53, a52 - a53);
    let (a54, a55) = (a54 + a55, a54 - a55);
    let (a60, a62) = (a60 + a62, a60 - a62);
    let (a61, a63) = (a61 + a63, a61 - a63);
    let a63 = (a63 << 48);
    let (a60, a61) = (a60 + a61, a60 - a61);
    let (a62, a63) = (a62 + a63, a62 - a63);
    let (a176, a184) = (a176 + a184, a176 - a184);
    let (a180, a188) = (a180 + a188, a180 - a188);
    let a188 = (a188 << 48);
    let (a176, a180) = (a176 + a180, a176 - a180);
    let (a184, a188) = (a184 + a188, a184 - a188);
    let (a177, a185) = (a177 + a185, a177 - a185);
    let (a181, a189) = (a181 + a189, a181 - a189);
    let a189 = (a189 << 48);
    let (a177, a181) = (a177 + a181, a177 - a181);
    let (a185, a189) = (a185 + a189, a185 - a189);
    let (a178, a186) = (a178 + a186, a178 - a186);
    let (a182, a190) = (a182 + a190, a182 - a190);
    let a190 = (a190 << 48);
    let (a178, a182) = (a178 + a182, a178 - a182);
    let (a186, a190) = (a186 + a190, a186 - a190);
    let (a179, a187) = (a179 + a187, a179 - a187);
    let (a183, a191) = (a183 + a191, a183 - a191);
    let a191 = (a191 << 48);
    let (a179, a183) = (a179 + a183, a179 - a183);
    let (a187, a191) = (a187 + a191, a187 - a191);
    let a185 = (a185 << 12);
    let a181 = (a181 << 24);
    let a189 = (a189 << 36);
    let a186 = (a186 << 24);
    let a182 = (a182 << 48);
    let a190 = (a190 << 72);
    let a187 = (a187 << 36);
    let a183 = (a183 << 72);
    let a191 = (-(a191 << 12));
    let (a176, a178) = (a176 + a178, a176 - a178);
    let (a177, a179) = (a177 + a179, a177 - a179);
    let a179 = (a179 << 48);
    let (a176, a177) = (a176 + a177, a176 - a177);
    let (a178, a179) = (a178 + a179, a178 - a179);
    let (a184, a186) = (a184 + a186, a184 - a186);
    let (a185, a187) = (a185 + a187, a185 - a187);
    let a187 = (a187 << 48);
    let (a184, a185) = (a184 + a185, a184 - a185);
    let (a186, a187) = (a186 + a187, a186 - a187);
    let (a180, a182) = (a180 + a182, a180 - a182);
    let (a181, a183) = (a181 + a183, a181 - a183);
    let a183 = (a183 << 48);
    let (a180, a181) = (a180 + a181, a180 - a181);
    let (a182, a183) = (a182 + a183, a182 - a183);
    let (a188, a190) = (a188 + a190, a188 - a190);
    let (a189, a191) = (a189 + a191, a189 - a191);
    let a191 = (a191 << 48);
    let (a188, a189) = (a188 + a189, a188 - a189);
    let (a190, a191) = (a190 + a191, a190 - a191);
    let (a112, a120) = (a112 + a120, a112 - a120);
    let (a116, a124) = (a116 + a124, a116 - a124);
    let a124 = (a124 << 48);
    let (a112, a116) = (a112 + a116, a112 - a116);
    let (a120, a124) = (a120 + a124, a120 - a124);
    let (a113, a121) = (a113 + a121, a113 - a121);
    let (a117, a125) = (a117 + a125, a117 - a125);
    let a125 = (a125 << 48);
    let (a113, a117) = (a113 + a117, a113 - a117);
    let (a121, a125) = (a121 + a125, a121 - a125);
    let (a114, a122) = (a114 + a122, a114 - a122);
    let (a118, a126) = (a118 + a126, a118 - a126);
    let a126 = (a126 << 48);
    let (a114, a118) = (a114 + a118, a114 - a118);
    let (a122, a126) = (a122 + a126, a122 - a126);
    let (a115, a123) = (a115 + a123, a115 - a123);
    let (a119, a127) = (a119 + a127, a119 - a127);
    let a127 = (a127 << 48);
    let (a115, a119) = (a115 + a119, a115 - a119);
    let (a123, a127) = (a123 + a127, a123 - a127);
    let a121 = (a121 << 12);
    let a117 = (a117 << 24);
    let a125 = (a125 << 36);
    let a122 = (a122 << 24);
    let a118 = (a118 << 48);
    let a126 = (a126 << 72);
    let a123 = (a123 << 36);
    let a119 = (a119 << 72);
    let a127 = (-(a127 << 12));
    let (a112, a114) = (a112 + a114, a112 - a114);
    let (a113, a115) = (a113 + a115, a113 - a115);
    let a115 = (a115 << 48);
    let (a112, a113) = (a112 + a113, a112 - a113);
    let (a114, a115) = (a114 + a115, a114 - a115);
    let (a120, a122) = (a120 + a122, a120 - a122);
    let (a121, a123) = (a121 + a123, a121 - a123);
    let a123 = (a123 << 48);
    let (a120, a121) = (a120 + a121, a120 - a121);
    let (a122, a123) = (a122 + a123, a122 - a123);
    let (a116, a118) = (a116 + a118, a116 - a118);
    let (a117, a119) = (a117 + a119, a117 - a119);
    let a119 = (a119 << 48);
    let (a116, a117) = (a116 + a117, a116 - a117);
    let (a118, a119) = (a118 + a119, a118 - a119);
    let (a124, a126) = (a124 + a126, a124 - a126);
    let (a125, a127) = (a125 + a127, a125 - a127);
    let a127 = (a127 << 48);
    let (a124, a125) = (a124 + a125, a124 - a125);
    let (a126, a127) = (a126 + a127, a126 - a127);
    let (a240, a248) = (a240 + a248, a240 - a248);
    let (a244, a252) = (a244 + a252, a244 - a252);
    let a252 = (a252 << 48);
    let (a240, a244) = (a240 + a244, a240 - a244);
    let (a248, a252) = (a248 + a252, a248 - a252);
    let (a241, a249) = (a241 + a249, a241 - a249);
    let (a245, a253) = (a245 + a253, a245 - a253);
    let a253 = (a253 << 48);
    let (a241, a245) = (a241 + a245, a241 - a245);
    let (a249, a253) = (a249 + a253, a249 - a253);
    let (a242, a250) = (a242 + a250, a242 - a250);
    let (a246, a254) = (a246 + a254, a246 - a254);
    let a254 = (a254 << 48);
    let (a242, a246) = (a242 + a246, a242 - a246);
    let (a250, a254) = (a250 + a254, a250 - a254);
    let (a243, a251) = (a243 + a251, a243 - a251);
    let (a247, a255) = (a247 + a255, a247 - a255);
    let a255 = (a255 << 48);
    let (a243, a247) = (a243 + a247, a243 - a247);
    let (a251, a255) = (a251 + a255, a251 - a255);
    let a249 = (a249 << 12);
    let a245 = (a245 << 24);
    let a253 = (a253 << 36);
    let a250 = (a250 << 24);
    let a246 = (a246 << 48);
    let a254 = (a254 << 72);
    let a251 = (a251 << 36);
    let a247 = (a247 << 72);
    let a255 = (-(a255 << 12));
    let (a240, a242) = (a240 + a242, a240 - a242);
    let (a241, a243) = (a241 + a243, a241 - a243);
    let a243 = (a243 << 48);
    let (a240, a241) = (a240 + a241, a240 - a241);
    let (a242, a243) = (a242 + a243, a242 - a243);
    let (a248, a250) = (a248 + a250, a248 - a250);
    let (a249, a251) = (a249 + a251, a249 - a251);
    let a251 = (a251 << 48);
    let (a248, a249) = (a248 + a249, a248 - a249);
    let (a250, a251) = (a250 + a251, a250 - a251);
    let (a244, a246) = (a244 + a246, a244 - a246);
    let (a245, a247) = (a245 + a247, a245 - a247);
    let a247 = (a247 << 48);
    let (a244, a245) = (a244 + a245, a244 - a245);
    let (a246, a247) = (a246 + a247, a246 - a247);
    let (a252, a254) = (a252 + a254, a252 - a254);
    let (a253, a255) = (a253 + a255, a253 - a255);
    let a255 = (a255 << 48);
    let (a252, a253) = (a252 + a253, a252 - a253);
    let (a254, a255) = (a254 + a255, a254 - a255);
    values[0] = a0;
    values[1] = a128;
    values[2] = a64;
    values[3] = a192;
    values[4] = a32;
    values[5] = a160;
    values[6] = a96;
    values[7] = a224;
    values[8] = a16;
    values[9] = a144;
    values[10] = a80;
    values[11] = a208;
    values[12] = a48;
    values[13] = a176;
    values[14] = a112;
    values[15] = a240;
    values[16] = a8;
    values[17] = a136;
    values[18] = a72;
    values[19] = a200;
    values[20] = a40;
    values[21] = a168;
    values[22] = a104;
    values[23] = a232;
    values[24] = a24;
    values[25] = a152;
    values[26] = a88;
    values[27] = a216;
    values[28] = a56;
    values[29] = a184;
    values[30] = a120;
    values[31] = a248;
    values[32] = a4;
    values[33] = a132;
    values[34] = a68;
    values[35] = a196;
    values[36] = a36;
    values[37] = a164;
    values[38] = a100;
    values[39] = a228;
    values[40] = a20;
    values[41] = a148;
    values[42] = a84;
    values[43] = a212;
    values[44] = a52;
    values[45] = a180;
    values[46] = a116;
    values[47] = a244;
    values[48] = a12;
    values[49] = a140;
    values[50] = a76;
    values[51] = a204;
    values[52] = a44;
    values[53] = a172;
    values[54] = a108;
    values[55] = a236;
    values[56] = a28;
    values[57] = a156;
    values[58] = a92;
    values[59] = a220;
    values[60] = a60;
    values[61] = a188;
    values[62] = a124;
    values[63] = a252;
    values[64] = a2;
    values[65] = a130;
    values[66] = a66;
    values[67] = a194;
    values[68] = a34;
    values[69] = a162;
    values[70] = a98;
    values[71] = a226;
    values[72] = a18;
    values[73] = a146;
    values[74] = a82;
    values[75] = a210;
    values[76] = a50;
    values[77] = a178;
    values[78] = a114;
    values[79] = a242;
    values[80] = a10;
    values[81] = a138;
    values[82] = a74;
    values[83] = a202;
    values[84] = a42;
    values[85] = a170;
    values[86] = a106;
    values[87] = a234;
    values[88] = a26;
    values[89] = a154;
    values[90] = a90;
    values[91] = a218;
    values[92] = a58;
    values[93] = a186;
    values[94] = a122;
    values[95] = a250;
    values[96] = a6;
    values[97] = a134;
    values[98] = a70;
    values[99] = a198;
    values[100] = a38;
    values[101] = a166;
    values[102] = a102;
    values[103] = a230;
    values[104] = a22;
    values[105] = a150;
    values[106] = a86;
    values[107] = a214;
    values[108] = a54;
    values[109] = a182;
    values[110] = a118;
    values[111] = a246;
    values[112] = a14;
    values[113] = a142;
    values[114] = a78;
    values[115] = a206;
    values[116] = a46;
    values[117] = a174;
    values[118] = a110;
    values[119] = a238;
    values[120] = a30;
    values[121] = a158;
    values[122] = a94;
    values[123] = a222;
    values[124] = a62;
    values[125] = a190;
    values[126] = a126;
    values[127] = a254;
    values[128] = a1;
    values[129] = a129;
    values[130] = a65;
    values[131] = a193;
    values[132] = a33;
    values[133] = a161;
    values[134] = a97;
    values[135] = a225;
    values[136] = a17;
    values[137] = a145;
    values[138] = a81;
    values[139] = a209;
    values[140] = a49;
    values[141] = a177;
    values[142] = a113;
    values[143] = a241;
    values[144] = a9;
    values[145] = a137;
    values[146] = a73;
    values[147] = a201;
    values[148] = a41;
    values[149] = a169;
    values[150] = a105;
    values[151] = a233;
    values[152] = a25;
    values[153] = a153;
    values[154] = a89;
    values[155] = a217;
    values[156] = a57;
    values[157] = a185;
    values[158] = a121;
    values[159] = a249;
    values[160] = a5;
    values[161] = a133;
    values[162] = a69;
    values[163] = a197;
    values[164] = a37;
    values[165] = a165;
    values[166] = a101;
    values[167] = a229;
    values[168] = a21;
    values[169] = a149;
    values[170] = a85;
    values[171] = a213;
    values[172] = a53;
    values[173] = a181;
    values[174] = a117;
    values[175] = a245;
    values[176] = a13;
    values[177] = a141;
    values[178] = a77;
    values[179] = a205;
    values[180] = a45;
    values[181] = a173;
    values[182] = a109;
    values[183] = a237;
    values[184] = a29;
    values[185] = a157;
    values[186] = a93;
    values[187] = a221;
    values[188] = a61;
    values[189] = a189;
    values[190] = a125;
    values[191] = a253;
    values[192] = a3;
    values[193] = a131;
    values[194] = a67;
    values[195] = a195;
    values[196] = a35;
    values[197] = a163;
    values[198] = a99;
    values[199] = a227;
    values[200] = a19;
    values[201] = a147;
    values[202] = a83;
    values[203] = a211;
    values[204] = a51;
    values[205] = a179;
    values[206] = a115;
    values[207] = a243;
    values[208] = a11;
    values[209] = a139;
    values[210] = a75;
    values[211] = a203;
    values[212] = a43;
    values[213] = a171;
    values[214] = a107;
    values[215] = a235;
    values[216] = a27;
    values[217] = a155;
    values[218] = a91;
    values[219] = a219;
    values[220] = a59;
    values[221] = a187;
    values[222] = a123;
    values[223] = a251;
    values[224] = a7;
    values[225] = a135;
    values[226] = a71;
    values[227] = a199;
    values[228] = a39;
    values[229] = a167;
    values[230] = a103;
    values[231] = a231;
    values[232] = a23;
    values[233] = a151;
    values[234] = a87;
    values[235] = a215;
    values[236] = a55;
    values[237] = a183;
    values[238] = a119;
    values[239] = a247;
    values[240] = a15;
    values[241] = a143;
    values[242] = a79;
    values[243] = a207;
    values[244] = a47;
    values[245] = a175;
    values[246] = a111;
    values[247] = a239;
    values[248] = a31;
    values[249] = a159;
    values[250] = a95;
    values[251] = a223;
    values[252] = a63;
    values[253] = a191;
    values[254] = a127;
    values[255] = a255;
}

#[cfg(test)]
mod tests {
    use super::{super::ntt_naive, *};

    #[test]
    fn test_small_ntt() {
        for size in [
            0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 16, 20, 24, 30, 32, 40, 48, 60, 64, 80, 96, 120,
            128, 160, 192, 240, 256,
        ] {
            let input = (0..size).map(Field::from).collect::<Vec<_>>();
            let mut output = input.clone();
            let supported = small_ntt(output.as_mut_slice());
            assert!(supported);
            let mut output_ref = input;
            ntt_naive(output_ref.as_mut_slice());
            assert_eq!(output, output_ref);
        }
    }

    #[test]
    fn test_ntt_2() {
        let size = 2;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_2(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_3() {
        let size = 3;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_3(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_4() {
        let size = 4;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_4(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_5() {
        let size = 5;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_5(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_6() {
        let size = 6;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_6(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_8() {
        let size = 8;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_8(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_10() {
        let size = 10;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_10(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_12() {
        let size = 12;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_12(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_15() {
        let size = 15;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_15(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_16() {
        let size = 16;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_16(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_20() {
        let size = 20;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_20(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_24() {
        let size = 24;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_24(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_30() {
        let size = 30;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_30(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_32() {
        let size = 32;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_32(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_40() {
        let size = 40;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_40(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_48() {
        let size = 48;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_48(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_60() {
        let size = 60;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_60(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_64() {
        let size = 64;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_64(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_80() {
        let size = 80;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_80(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_96() {
        let size = 96;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_96(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_120() {
        let size = 120;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_120(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_128() {
        let size = 128;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_128(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_160() {
        let size = 160;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_160(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_192() {
        let size = 192;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_192(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_240() {
        let size = 240;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_240(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

    #[test]
    fn test_ntt_256() {
        let size = 256;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_256(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }
}
