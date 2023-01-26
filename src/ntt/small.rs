//! Generated using `cargo run --bin codegen`
#![allow(
    unused_parens,
    clippy::similar_names,
    clippy::unreadable_literal,
    clippy::too_many_lines
)]
use crate::Field;

/// Apply a small NTT to `values`, or return `false` if the size is not
/// supported.
pub fn ntt(values: &mut [Field]) -> bool {
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
        17 => ntt_17(values),
        20 => ntt_20(values),
        24 => ntt_24(values),
        30 => ntt_30(values),
        32 => ntt_32(values),
        34 => ntt_34(values),
        40 => ntt_40(values),
        48 => ntt_48(values),
        51 => ntt_51(values),
        60 => ntt_60(values),
        64 => ntt_64(values),
        _ => return false,
    }
    true
}

/// Size 2 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 2`.
pub fn ntt_2(values: &mut [Field]) {
    assert_eq!(values.len(), 2);
    let a0 = values[0];
    let a1 = values[1];
    let (a0, a1) = (a0 + a1, a0 - a1);
    values[0] = a0;
    values[1] = a1;
}

/// Size 3 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 3`.
pub fn ntt_3(values: &mut [Field]) {
    assert_eq!(values.len(), 3);
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
///
/// # Panics
///
/// Panics if `values.len() != 4`.
pub fn ntt_4(values: &mut [Field]) {
    assert_eq!(values.len(), 4);
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
///
/// # Panics
///
/// Panics if `values.len() != 5`.
pub fn ntt_5(values: &mut [Field]) {
    assert_eq!(values.len(), 5);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let (a1, a4) = (a1 + a4, a1 - a4);
    let (a3, a2) = (a3 + a2, a3 - a2);
    let a2 = (a2 << 48);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let (a4, a2) = (a4 + a2, a4 - a2);
    let t = a0;
    let a0 = a0 + a1;
    let a1 = a1 * Field::new(4611686017353646080);
    let a4 = a4 * Field::new(16181989089180173841);
    let a3 = a3 * Field::new(5818851782451133869);
    let a2 = a2 * Field::new(11322249509082494407);
    let a1 = a1 + t;
    let (a1, a3) = (a1 + a3, a1 - a3);
    let (a2, a4) = (a2 + a4, a2 - a4);
    let a4 = (a4 << 48);
    let (a1, a2) = (a1 + a2, a1 - a2);
    let (a3, a4) = (a3 + a4, a3 - a4);
    values[0] = a0;
    values[1] = a1;
    values[2] = a3;
    values[3] = a4;
    values[4] = a2;
}

/// Size 6 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 6`.
pub fn ntt_6(values: &mut [Field]) {
    assert_eq!(values.len(), 6);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let a4 = values[4];
    let a5 = values[5];
    let (a0, a2, a4) = (
        a0 + a2 + a4,
        a0 + (a2 << 64) - (a4 << 32),
        a0 - (a2 << 32) + (a4 << 64),
    );
    let (a3, a5, a1) = (
        a3 + a5 + a1,
        a3 + (a5 << 64) - (a1 << 32),
        a3 - (a5 << 32) + (a1 << 64),
    );
    let (a0, a3) = (a0 + a3, a0 - a3);
    let (a2, a5) = (a2 + a5, a2 - a5);
    let (a4, a1) = (a4 + a1, a4 - a1);
    values[0] = a0;
    values[1] = a5;
    values[2] = a4;
    values[3] = a3;
    values[4] = a2;
    values[5] = a1;
}

/// Size 8 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 8`.
pub fn ntt_8(values: &mut [Field]) {
    assert_eq!(values.len(), 8);
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
///
/// # Panics
///
/// Panics if `values.len() != 10`.
pub fn ntt_10(values: &mut [Field]) {
    assert_eq!(values.len(), 10);
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
    let (a2, a8) = (a2 + a8, a2 - a8);
    let (a6, a4) = (a6 + a4, a6 - a4);
    let a4 = (a4 << 48);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a8, a4) = (a8 + a4, a8 - a4);
    let t = a0;
    let a0 = a0 + a2;
    let a2 = a2 * Field::new(4611686017353646080);
    let a8 = a8 * Field::new(16181989089180173841);
    let a6 = a6 * Field::new(5818851782451133869);
    let a4 = a4 * Field::new(11322249509082494407);
    let a2 = a2 + t;
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a4, a8) = (a4 + a8, a4 - a8);
    let a8 = (a8 << 48);
    let (a2, a4) = (a2 + a4, a2 - a4);
    let (a6, a8) = (a6 + a8, a6 - a8);
    let (a7, a3) = (a7 + a3, a7 - a3);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let a9 = (a9 << 48);
    let (a7, a1) = (a7 + a1, a7 - a1);
    let (a3, a9) = (a3 + a9, a3 - a9);
    let t = a5;
    let a5 = a5 + a7;
    let a7 = a7 * Field::new(4611686017353646080);
    let a3 = a3 * Field::new(16181989089180173841);
    let a1 = a1 * Field::new(5818851782451133869);
    let a9 = a9 * Field::new(11322249509082494407);
    let a7 = a7 + t;
    let (a7, a1) = (a7 + a1, a7 - a1);
    let (a9, a3) = (a9 + a3, a9 - a3);
    let a3 = (a3 << 48);
    let (a7, a9) = (a7 + a9, a7 - a9);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let (a0, a5) = (a0 + a5, a0 - a5);
    let (a2, a7) = (a2 + a7, a2 - a7);
    let (a6, a1) = (a6 + a1, a6 - a1);
    let (a8, a3) = (a8 + a3, a8 - a3);
    let (a4, a9) = (a4 + a9, a4 - a9);
    values[0] = a0;
    values[1] = a7;
    values[2] = a6;
    values[3] = a3;
    values[4] = a4;
    values[5] = a5;
    values[6] = a2;
    values[7] = a1;
    values[8] = a8;
    values[9] = a9;
}

/// Size 12 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 12`.
pub fn ntt_12(values: &mut [Field]) {
    assert_eq!(values.len(), 12);
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
    let (a0, a6) = (a0 + a6, a0 - a6);
    let (a3, a9) = (a3 + a9, a3 - a9);
    let a9 = (a9 << 48);
    let (a0, a3) = (a0 + a3, a0 - a3);
    let (a6, a9) = (a6 + a9, a6 - a9);
    let (a4, a10) = (a4 + a10, a4 - a10);
    let (a7, a1) = (a7 + a1, a7 - a1);
    let a1 = (a1 << 48);
    let (a4, a7) = (a4 + a7, a4 - a7);
    let (a10, a1) = (a10 + a1, a10 - a1);
    let (a8, a2) = (a8 + a2, a8 - a2);
    let (a11, a5) = (a11 + a5, a11 - a5);
    let a5 = (a5 << 48);
    let (a8, a11) = (a8 + a11, a8 - a11);
    let (a2, a5) = (a2 + a5, a2 - a5);
    let (a0, a4, a8) = (
        a0 + a4 + a8,
        a0 + (a4 << 64) - (a8 << 32),
        a0 - (a4 << 32) + (a8 << 64),
    );
    let (a6, a10, a2) = (
        a6 + a10 + a2,
        a6 + (a10 << 64) - (a2 << 32),
        a6 - (a10 << 32) + (a2 << 64),
    );
    let (a3, a7, a11) = (
        a3 + a7 + a11,
        a3 + (a7 << 64) - (a11 << 32),
        a3 - (a7 << 32) + (a11 << 64),
    );
    let (a9, a1, a5) = (
        a9 + a1 + a5,
        a9 + (a1 << 64) - (a5 << 32),
        a9 - (a1 << 32) + (a5 << 64),
    );
    values[0] = a0;
    values[1] = a10;
    values[2] = a11;
    values[3] = a9;
    values[4] = a4;
    values[5] = a2;
    values[6] = a3;
    values[7] = a1;
    values[8] = a8;
    values[9] = a6;
    values[10] = a7;
    values[11] = a5;
}

/// Size 15 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 15`.
pub fn ntt_15(values: &mut [Field]) {
    assert_eq!(values.len(), 15);
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
    let (a3, a12) = (a3 + a12, a3 - a12);
    let (a9, a6) = (a9 + a6, a9 - a6);
    let a6 = (a6 << 48);
    let (a3, a9) = (a3 + a9, a3 - a9);
    let (a12, a6) = (a12 + a6, a12 - a6);
    let t = a0;
    let a0 = a0 + a3;
    let a3 = a3 * Field::new(4611686017353646080);
    let a12 = a12 * Field::new(16181989089180173841);
    let a9 = a9 * Field::new(5818851782451133869);
    let a6 = a6 * Field::new(11322249509082494407);
    let a3 = a3 + t;
    let (a3, a9) = (a3 + a9, a3 - a9);
    let (a6, a12) = (a6 + a12, a6 - a12);
    let a12 = (a12 << 48);
    let (a3, a6) = (a3 + a6, a3 - a6);
    let (a9, a12) = (a9 + a12, a9 - a12);
    let (a8, a2) = (a8 + a2, a8 - a2);
    let (a14, a11) = (a14 + a11, a14 - a11);
    let a11 = (a11 << 48);
    let (a8, a14) = (a8 + a14, a8 - a14);
    let (a2, a11) = (a2 + a11, a2 - a11);
    let t = a5;
    let a5 = a5 + a8;
    let a8 = a8 * Field::new(4611686017353646080);
    let a2 = a2 * Field::new(16181989089180173841);
    let a14 = a14 * Field::new(5818851782451133869);
    let a11 = a11 * Field::new(11322249509082494407);
    let a8 = a8 + t;
    let (a8, a14) = (a8 + a14, a8 - a14);
    let (a11, a2) = (a11 + a2, a11 - a2);
    let a2 = (a2 << 48);
    let (a8, a11) = (a8 + a11, a8 - a11);
    let (a14, a2) = (a14 + a2, a14 - a2);
    let (a13, a7) = (a13 + a7, a13 - a7);
    let (a4, a1) = (a4 + a1, a4 - a1);
    let a1 = (a1 << 48);
    let (a13, a4) = (a13 + a4, a13 - a4);
    let (a7, a1) = (a7 + a1, a7 - a1);
    let t = a10;
    let a10 = a10 + a13;
    let a13 = a13 * Field::new(4611686017353646080);
    let a7 = a7 * Field::new(16181989089180173841);
    let a4 = a4 * Field::new(5818851782451133869);
    let a1 = a1 * Field::new(11322249509082494407);
    let a13 = a13 + t;
    let (a13, a4) = (a13 + a4, a13 - a4);
    let (a1, a7) = (a1 + a7, a1 - a7);
    let a7 = (a7 << 48);
    let (a13, a1) = (a13 + a1, a13 - a1);
    let (a4, a7) = (a4 + a7, a4 - a7);
    let (a0, a5, a10) = (
        a0 + a5 + a10,
        a0 + (a5 << 64) - (a10 << 32),
        a0 - (a5 << 32) + (a10 << 64),
    );
    let (a3, a8, a13) = (
        a3 + a8 + a13,
        a3 + (a8 << 64) - (a13 << 32),
        a3 - (a8 << 32) + (a13 << 64),
    );
    let (a9, a14, a4) = (
        a9 + a14 + a4,
        a9 + (a14 << 64) - (a4 << 32),
        a9 - (a14 << 32) + (a4 << 64),
    );
    let (a12, a2, a7) = (
        a12 + a2 + a7,
        a12 + (a2 << 64) - (a7 << 32),
        a12 - (a2 << 32) + (a7 << 64),
    );
    let (a6, a11, a1) = (
        a6 + a11 + a1,
        a6 + (a11 << 64) - (a1 << 32),
        a6 - (a11 << 32) + (a1 << 64),
    );
    values[0] = a0;
    values[1] = a8;
    values[2] = a4;
    values[3] = a12;
    values[4] = a11;
    values[5] = a10;
    values[6] = a3;
    values[7] = a14;
    values[8] = a7;
    values[9] = a6;
    values[10] = a5;
    values[11] = a13;
    values[12] = a9;
    values[13] = a2;
    values[14] = a1;
}

/// Size 16 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 16`.
pub fn ntt_16(values: &mut [Field]) {
    assert_eq!(values.len(), 16);
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

/// Size 17 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 17`.
pub fn ntt_17(values: &mut [Field]) {
    assert_eq!(values.len(), 17);
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
    let (a1, a16) = (a1 + a16, a1 - a16);
    let (a4, a13) = (a4 + a13, a4 - a13);
    let a13 = (a13 << 48);
    let (a1, a4) = (a1 + a4, a1 - a4);
    let (a16, a13) = (a16 + a13, a16 - a13);
    let (a6, a11) = (a6 + a11, a6 - a11);
    let (a7, a10) = (a7 + a10, a7 - a10);
    let a10 = (a10 << 48);
    let (a6, a7) = (a6 + a7, a6 - a7);
    let (a11, a10) = (a11 + a10, a11 - a10);
    let (a2, a15) = (a2 + a15, a2 - a15);
    let (a8, a9) = (a8 + a9, a8 - a9);
    let a9 = (a9 << 48);
    let (a2, a8) = (a2 + a8, a2 - a8);
    let (a15, a9) = (a15 + a9, a15 - a9);
    let (a12, a5) = (a12 + a5, a12 - a5);
    let (a14, a3) = (a14 + a3, a14 - a3);
    let a3 = (a3 << 48);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a5, a3) = (a5 + a3, a5 - a3);
    let a11 = (a11 << 12);
    let a7 = (a7 << 24);
    let a10 = (a10 << 36);
    let a15 = (a15 << 24);
    let a8 = (a8 << 48);
    let a9 = (a9 << 72);
    let a5 = (a5 << 36);
    let a14 = (a14 << 72);
    let a3 = (-(a3 << 12));
    let (a1, a2) = (a1 + a2, a1 - a2);
    let (a6, a12) = (a6 + a12, a6 - a12);
    let a12 = (a12 << 48);
    let (a1, a6) = (a1 + a6, a1 - a6);
    let (a2, a12) = (a2 + a12, a2 - a12);
    let (a16, a15) = (a16 + a15, a16 - a15);
    let (a11, a5) = (a11 + a5, a11 - a5);
    let a5 = (a5 << 48);
    let (a16, a11) = (a16 + a11, a16 - a11);
    let (a15, a5) = (a15 + a5, a15 - a5);
    let (a4, a8) = (a4 + a8, a4 - a8);
    let (a7, a14) = (a7 + a14, a7 - a14);
    let a14 = (a14 << 48);
    let (a4, a7) = (a4 + a7, a4 - a7);
    let (a8, a14) = (a8 + a14, a8 - a14);
    let (a13, a9) = (a13 + a9, a13 - a9);
    let (a10, a3) = (a10 + a3, a10 - a3);
    let a3 = (a3 << 48);
    let (a13, a10) = (a13 + a10, a13 - a10);
    let (a9, a3) = (a9 + a3, a9 - a3);
    let t = a0;
    let a0 = a0 + a1;
    let a1 = a1 * Field::new(1152921504338411520);
    let a16 = a16 * Field::new(6259776822214049175);
    let a4 = a4 * Field::new(9380094172986290191);
    let a13 = a13 * Field::new(891943630314919127);
    let a2 = a2 * Field::new(17228171707553225791);
    let a15 = a15 * Field::new(12855743360534130886);
    let a8 = a8 * Field::new(6167687396920564837);
    let a9 = a9 * Field::new(17201834061724655524);
    let a6 = a6 * Field::new(15308299771656910737);
    let a11 = a11 * Field::new(18186005861103657533);
    let a7 = a7 * Field::new(53595491891823545);
    let a10 = a10 * Field::new(1906638201581172103);
    let a12 = a12 * Field::new(18303651001328874822);
    let a5 = a5 * Field::new(3077466521755967626);
    let a14 = a14 * Field::new(12423593102987598328);
    let a3 = a3 * Field::new(18361513053649472048);
    let a1 = a1 + t;
    let (a1, a6) = (a1 + a6, a1 - a6);
    let (a12, a2) = (a12 + a2, a12 - a2);
    let a2 = (a2 << 48);
    let (a1, a12) = (a1 + a12, a1 - a12);
    let (a6, a2) = (a6 + a2, a6 - a2);
    let (a3, a9) = (a3 + a9, a3 - a9);
    let (a10, a13) = (a10 + a13, a10 - a13);
    let a13 = (a13 << 48);
    let (a3, a10) = (a3 + a10, a3 - a10);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a14, a8) = (a14 + a8, a14 - a8);
    let (a7, a4) = (a7 + a4, a7 - a4);
    let a4 = (a4 << 48);
    let (a14, a7) = (a14 + a7, a14 - a7);
    let (a8, a4) = (a8 + a4, a8 - a4);
    let (a5, a15) = (a5 + a15, a5 - a15);
    let (a11, a16) = (a11 + a16, a11 - a16);
    let a16 = (a16 << 48);
    let (a5, a11) = (a5 + a11, a5 - a11);
    let (a15, a16) = (a15 + a16, a15 - a16);
    let a9 = (a9 << 12);
    let a10 = (a10 << 24);
    let a13 = (a13 << 36);
    let a8 = (a8 << 24);
    let a7 = (a7 << 48);
    let a4 = (a4 << 72);
    let a15 = (a15 << 36);
    let a11 = (a11 << 72);
    let a16 = (-(a16 << 12));
    let (a1, a14) = (a1 + a14, a1 - a14);
    let (a3, a5) = (a3 + a5, a3 - a5);
    let a5 = (a5 << 48);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let (a14, a5) = (a14 + a5, a14 - a5);
    let (a6, a8) = (a6 + a8, a6 - a8);
    let (a9, a15) = (a9 + a15, a9 - a15);
    let a15 = (a15 << 48);
    let (a6, a9) = (a6 + a9, a6 - a9);
    let (a8, a15) = (a8 + a15, a8 - a15);
    let (a12, a7) = (a12 + a7, a12 - a7);
    let (a10, a11) = (a10 + a11, a10 - a11);
    let a11 = (a11 << 48);
    let (a12, a10) = (a12 + a10, a12 - a10);
    let (a7, a11) = (a7 + a11, a7 - a11);
    let (a2, a4) = (a2 + a4, a2 - a4);
    let (a13, a16) = (a13 + a16, a13 - a16);
    let a16 = (a16 << 48);
    let (a2, a13) = (a2 + a13, a2 - a13);
    let (a4, a16) = (a4 + a16, a4 - a16);
    values[0] = a0;
    values[1] = a1;
    values[2] = a11;
    values[3] = a6;
    values[4] = a5;
    values[5] = a8;
    values[6] = a16;
    values[7] = a13;
    values[8] = a10;
    values[9] = a12;
    values[10] = a2;
    values[11] = a4;
    values[12] = a15;
    values[13] = a14;
    values[14] = a9;
    values[15] = a7;
    values[16] = a3;
}

/// Size 20 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 20`.
pub fn ntt_20(values: &mut [Field]) {
    assert_eq!(values.len(), 20);
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
    let (a4, a16) = (a4 + a16, a4 - a16);
    let (a12, a8) = (a12 + a8, a12 - a8);
    let a8 = (a8 << 48);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let (a16, a8) = (a16 + a8, a16 - a8);
    let t = a0;
    let a0 = a0 + a4;
    let a4 = a4 * Field::new(4611686017353646080);
    let a16 = a16 * Field::new(16181989089180173841);
    let a12 = a12 * Field::new(5818851782451133869);
    let a8 = a8 * Field::new(11322249509082494407);
    let a4 = a4 + t;
    let (a4, a12) = (a4 + a12, a4 - a12);
    let (a8, a16) = (a8 + a16, a8 - a16);
    let a16 = (a16 << 48);
    let (a4, a8) = (a4 + a8, a4 - a8);
    let (a12, a16) = (a12 + a16, a12 - a16);
    let (a9, a1) = (a9 + a1, a9 - a1);
    let (a17, a13) = (a17 + a13, a17 - a13);
    let a13 = (a13 << 48);
    let (a9, a17) = (a9 + a17, a9 - a17);
    let (a1, a13) = (a1 + a13, a1 - a13);
    let t = a5;
    let a5 = a5 + a9;
    let a9 = a9 * Field::new(4611686017353646080);
    let a1 = a1 * Field::new(16181989089180173841);
    let a17 = a17 * Field::new(5818851782451133869);
    let a13 = a13 * Field::new(11322249509082494407);
    let a9 = a9 + t;
    let (a9, a17) = (a9 + a17, a9 - a17);
    let (a13, a1) = (a13 + a1, a13 - a1);
    let a1 = (a1 << 48);
    let (a9, a13) = (a9 + a13, a9 - a13);
    let (a17, a1) = (a17 + a1, a17 - a1);
    let (a14, a6) = (a14 + a6, a14 - a6);
    let (a2, a18) = (a2 + a18, a2 - a18);
    let a18 = (a18 << 48);
    let (a14, a2) = (a14 + a2, a14 - a2);
    let (a6, a18) = (a6 + a18, a6 - a18);
    let t = a10;
    let a10 = a10 + a14;
    let a14 = a14 * Field::new(4611686017353646080);
    let a6 = a6 * Field::new(16181989089180173841);
    let a2 = a2 * Field::new(5818851782451133869);
    let a18 = a18 * Field::new(11322249509082494407);
    let a14 = a14 + t;
    let (a14, a2) = (a14 + a2, a14 - a2);
    let (a18, a6) = (a18 + a6, a18 - a6);
    let a6 = (a6 << 48);
    let (a14, a18) = (a14 + a18, a14 - a18);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a19, a11) = (a19 + a11, a19 - a11);
    let (a7, a3) = (a7 + a3, a7 - a3);
    let a3 = (a3 << 48);
    let (a19, a7) = (a19 + a7, a19 - a7);
    let (a11, a3) = (a11 + a3, a11 - a3);
    let t = a15;
    let a15 = a15 + a19;
    let a19 = a19 * Field::new(4611686017353646080);
    let a11 = a11 * Field::new(16181989089180173841);
    let a7 = a7 * Field::new(5818851782451133869);
    let a3 = a3 * Field::new(11322249509082494407);
    let a19 = a19 + t;
    let (a19, a7) = (a19 + a7, a19 - a7);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let a11 = (a11 << 48);
    let (a19, a3) = (a19 + a3, a19 - a3);
    let (a7, a11) = (a7 + a11, a7 - a11);
    let (a0, a10) = (a0 + a10, a0 - a10);
    let (a5, a15) = (a5 + a15, a5 - a15);
    let a15 = (a15 << 48);
    let (a0, a5) = (a0 + a5, a0 - a5);
    let (a10, a15) = (a10 + a15, a10 - a15);
    let (a4, a14) = (a4 + a14, a4 - a14);
    let (a9, a19) = (a9 + a19, a9 - a19);
    let a19 = (a19 << 48);
    let (a4, a9) = (a4 + a9, a4 - a9);
    let (a14, a19) = (a14 + a19, a14 - a19);
    let (a12, a2) = (a12 + a2, a12 - a2);
    let (a17, a7) = (a17 + a7, a17 - a7);
    let a7 = (a7 << 48);
    let (a12, a17) = (a12 + a17, a12 - a17);
    let (a2, a7) = (a2 + a7, a2 - a7);
    let (a16, a6) = (a16 + a6, a16 - a6);
    let (a1, a11) = (a1 + a11, a1 - a11);
    let a11 = (a11 << 48);
    let (a16, a1) = (a16 + a1, a16 - a1);
    let (a6, a11) = (a6 + a11, a6 - a11);
    let (a8, a18) = (a8 + a18, a8 - a18);
    let (a13, a3) = (a13 + a3, a13 - a3);
    let a3 = (a3 << 48);
    let (a8, a13) = (a8 + a13, a8 - a13);
    let (a18, a3) = (a18 + a3, a18 - a3);
    values[0] = a0;
    values[1] = a14;
    values[2] = a17;
    values[3] = a11;
    values[4] = a8;
    values[5] = a10;
    values[6] = a9;
    values[7] = a7;
    values[8] = a16;
    values[9] = a18;
    values[10] = a5;
    values[11] = a19;
    values[12] = a12;
    values[13] = a6;
    values[14] = a13;
    values[15] = a15;
    values[16] = a4;
    values[17] = a2;
    values[18] = a1;
    values[19] = a3;
}

/// Size 24 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 24`.
pub fn ntt_24(values: &mut [Field]) {
    assert_eq!(values.len(), 24);
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
    let (a0, a2, a4) = (
        a0 + a2 + a4,
        a0 + (a2 << 64) - (a4 << 32),
        a0 - (a2 << 32) + (a4 << 64),
    );
    let (a3, a5, a1) = (
        a3 + a5 + a1,
        a3 + (a5 << 64) - (a1 << 32),
        a3 - (a5 << 32) + (a1 << 64),
    );
    let (a0, a3) = (a0 + a3, a0 - a3);
    let (a2, a5) = (a2 + a5, a2 - a5);
    let (a4, a1) = (a4 + a1, a4 - a1);
    let (a12, a14, a16) = (
        a12 + a14 + a16,
        a12 + (a14 << 64) - (a16 << 32),
        a12 - (a14 << 32) + (a16 << 64),
    );
    let (a15, a17, a13) = (
        a15 + a17 + a13,
        a15 + (a17 << 64) - (a13 << 32),
        a15 - (a17 << 32) + (a13 << 64),
    );
    let (a12, a15) = (a12 + a15, a12 - a15);
    let (a14, a17) = (a14 + a17, a14 - a17);
    let (a16, a13) = (a16 + a13, a16 - a13);
    let (a6, a8, a10) = (
        a6 + a8 + a10,
        a6 + (a8 << 64) - (a10 << 32),
        a6 - (a8 << 32) + (a10 << 64),
    );
    let (a9, a11, a7) = (
        a9 + a11 + a7,
        a9 + (a11 << 64) - (a7 << 32),
        a9 - (a11 << 32) + (a7 << 64),
    );
    let (a6, a9) = (a6 + a9, a6 - a9);
    let (a8, a11) = (a8 + a11, a8 - a11);
    let (a10, a7) = (a10 + a7, a10 - a7);
    let (a18, a20, a22) = (
        a18 + a20 + a22,
        a18 + (a20 << 64) - (a22 << 32),
        a18 - (a20 << 32) + (a22 << 64),
    );
    let (a21, a23, a19) = (
        a21 + a23 + a19,
        a21 + (a23 << 64) - (a19 << 32),
        a21 - (a23 << 32) + (a19 << 64),
    );
    let (a18, a21) = (a18 + a21, a18 - a21);
    let (a20, a23) = (a20 + a23, a20 - a23);
    let (a22, a19) = (a22 + a19, a22 - a19);
    values[0] = a0;
    values[1] = a12;
    values[2] = a6;
    values[3] = a18;
    values[4] = a5;
    values[5] = a17;
    values[6] = a11;
    values[7] = a23;
    values[8] = a4;
    values[9] = a16;
    values[10] = a10;
    values[11] = a22;
    values[12] = a3;
    values[13] = a15;
    values[14] = a9;
    values[15] = a21;
    values[16] = a2;
    values[17] = a14;
    values[18] = a8;
    values[19] = a20;
    values[20] = a1;
    values[21] = a13;
    values[22] = a7;
    values[23] = a19;
}

/// Size 30 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 30`.
pub fn ntt_30(values: &mut [Field]) {
    assert_eq!(values.len(), 30);
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
    let (a0, a10, a20) = (
        a0 + a10 + a20,
        a0 + (a10 << 64) - (a20 << 32),
        a0 - (a10 << 32) + (a20 << 64),
    );
    let (a15, a25, a5) = (
        a15 + a25 + a5,
        a15 + (a25 << 64) - (a5 << 32),
        a15 - (a25 << 32) + (a5 << 64),
    );
    let (a0, a15) = (a0 + a15, a0 - a15);
    let (a10, a25) = (a10 + a25, a10 - a25);
    let (a20, a5) = (a20 + a5, a20 - a5);
    let (a6, a16, a26) = (
        a6 + a16 + a26,
        a6 + (a16 << 64) - (a26 << 32),
        a6 - (a16 << 32) + (a26 << 64),
    );
    let (a21, a1, a11) = (
        a21 + a1 + a11,
        a21 + (a1 << 64) - (a11 << 32),
        a21 - (a1 << 32) + (a11 << 64),
    );
    let (a6, a21) = (a6 + a21, a6 - a21);
    let (a16, a1) = (a16 + a1, a16 - a1);
    let (a26, a11) = (a26 + a11, a26 - a11);
    let (a12, a22, a2) = (
        a12 + a22 + a2,
        a12 + (a22 << 64) - (a2 << 32),
        a12 - (a22 << 32) + (a2 << 64),
    );
    let (a27, a7, a17) = (
        a27 + a7 + a17,
        a27 + (a7 << 64) - (a17 << 32),
        a27 - (a7 << 32) + (a17 << 64),
    );
    let (a12, a27) = (a12 + a27, a12 - a27);
    let (a22, a7) = (a22 + a7, a22 - a7);
    let (a2, a17) = (a2 + a17, a2 - a17);
    let (a18, a28, a8) = (
        a18 + a28 + a8,
        a18 + (a28 << 64) - (a8 << 32),
        a18 - (a28 << 32) + (a8 << 64),
    );
    let (a3, a13, a23) = (
        a3 + a13 + a23,
        a3 + (a13 << 64) - (a23 << 32),
        a3 - (a13 << 32) + (a23 << 64),
    );
    let (a18, a3) = (a18 + a3, a18 - a3);
    let (a28, a13) = (a28 + a13, a28 - a13);
    let (a8, a23) = (a8 + a23, a8 - a23);
    let (a24, a4, a14) = (
        a24 + a4 + a14,
        a24 + (a4 << 64) - (a14 << 32),
        a24 - (a4 << 32) + (a14 << 64),
    );
    let (a9, a19, a29) = (
        a9 + a19 + a29,
        a9 + (a19 << 64) - (a29 << 32),
        a9 - (a19 << 32) + (a29 << 64),
    );
    let (a24, a9) = (a24 + a9, a24 - a9);
    let (a4, a19) = (a4 + a19, a4 - a19);
    let (a14, a29) = (a14 + a29, a14 - a29);
    let (a6, a24) = (a6 + a24, a6 - a24);
    let (a18, a12) = (a18 + a12, a18 - a12);
    let a12 = (a12 << 48);
    let (a6, a18) = (a6 + a18, a6 - a18);
    let (a24, a12) = (a24 + a12, a24 - a12);
    let t = a0;
    let a0 = a0 + a6;
    let a6 = a6 * Field::new(4611686017353646080);
    let a24 = a24 * Field::new(16181989089180173841);
    let a18 = a18 * Field::new(5818851782451133869);
    let a12 = a12 * Field::new(11322249509082494407);
    let a6 = a6 + t;
    let (a6, a18) = (a6 + a18, a6 - a18);
    let (a12, a24) = (a12 + a24, a12 - a24);
    let a24 = (a24 << 48);
    let (a6, a12) = (a6 + a12, a6 - a12);
    let (a18, a24) = (a18 + a24, a18 - a24);
    let (a1, a19) = (a1 + a19, a1 - a19);
    let (a13, a7) = (a13 + a7, a13 - a7);
    let a7 = (a7 << 48);
    let (a1, a13) = (a1 + a13, a1 - a13);
    let (a19, a7) = (a19 + a7, a19 - a7);
    let t = a25;
    let a25 = a25 + a1;
    let a1 = a1 * Field::new(4611686017353646080);
    let a19 = a19 * Field::new(16181989089180173841);
    let a13 = a13 * Field::new(5818851782451133869);
    let a7 = a7 * Field::new(11322249509082494407);
    let a1 = a1 + t;
    let (a1, a13) = (a1 + a13, a1 - a13);
    let (a7, a19) = (a7 + a19, a7 - a19);
    let a19 = (a19 << 48);
    let (a1, a7) = (a1 + a7, a1 - a7);
    let (a13, a19) = (a13 + a19, a13 - a19);
    let (a26, a14) = (a26 + a14, a26 - a14);
    let (a8, a2) = (a8 + a2, a8 - a2);
    let a2 = (a2 << 48);
    let (a26, a8) = (a26 + a8, a26 - a8);
    let (a14, a2) = (a14 + a2, a14 - a2);
    let t = a20;
    let a20 = a20 + a26;
    let a26 = a26 * Field::new(4611686017353646080);
    let a14 = a14 * Field::new(16181989089180173841);
    let a8 = a8 * Field::new(5818851782451133869);
    let a2 = a2 * Field::new(11322249509082494407);
    let a26 = a26 + t;
    let (a26, a8) = (a26 + a8, a26 - a8);
    let (a2, a14) = (a2 + a14, a2 - a14);
    let a14 = (a14 << 48);
    let (a26, a2) = (a26 + a2, a26 - a2);
    let (a8, a14) = (a8 + a14, a8 - a14);
    let (a21, a9) = (a21 + a9, a21 - a9);
    let (a3, a27) = (a3 + a27, a3 - a27);
    let a27 = (a27 << 48);
    let (a21, a3) = (a21 + a3, a21 - a3);
    let (a9, a27) = (a9 + a27, a9 - a27);
    let t = a15;
    let a15 = a15 + a21;
    let a21 = a21 * Field::new(4611686017353646080);
    let a9 = a9 * Field::new(16181989089180173841);
    let a3 = a3 * Field::new(5818851782451133869);
    let a27 = a27 * Field::new(11322249509082494407);
    let a21 = a21 + t;
    let (a21, a3) = (a21 + a3, a21 - a3);
    let (a27, a9) = (a27 + a9, a27 - a9);
    let a9 = (a9 << 48);
    let (a21, a27) = (a21 + a27, a21 - a27);
    let (a3, a9) = (a3 + a9, a3 - a9);
    let (a16, a4) = (a16 + a4, a16 - a4);
    let (a28, a22) = (a28 + a22, a28 - a22);
    let a22 = (a22 << 48);
    let (a16, a28) = (a16 + a28, a16 - a28);
    let (a4, a22) = (a4 + a22, a4 - a22);
    let t = a10;
    let a10 = a10 + a16;
    let a16 = a16 * Field::new(4611686017353646080);
    let a4 = a4 * Field::new(16181989089180173841);
    let a28 = a28 * Field::new(5818851782451133869);
    let a22 = a22 * Field::new(11322249509082494407);
    let a16 = a16 + t;
    let (a16, a28) = (a16 + a28, a16 - a28);
    let (a22, a4) = (a22 + a4, a22 - a4);
    let a4 = (a4 << 48);
    let (a16, a22) = (a16 + a22, a16 - a22);
    let (a28, a4) = (a28 + a4, a28 - a4);
    let (a11, a29) = (a11 + a29, a11 - a29);
    let (a23, a17) = (a23 + a17, a23 - a17);
    let a17 = (a17 << 48);
    let (a11, a23) = (a11 + a23, a11 - a23);
    let (a29, a17) = (a29 + a17, a29 - a17);
    let t = a5;
    let a5 = a5 + a11;
    let a11 = a11 * Field::new(4611686017353646080);
    let a29 = a29 * Field::new(16181989089180173841);
    let a23 = a23 * Field::new(5818851782451133869);
    let a17 = a17 * Field::new(11322249509082494407);
    let a11 = a11 + t;
    let (a11, a23) = (a11 + a23, a11 - a23);
    let (a17, a29) = (a17 + a29, a17 - a29);
    let a29 = (a29 << 48);
    let (a11, a17) = (a11 + a17, a11 - a17);
    let (a23, a29) = (a23 + a29, a23 - a29);
    values[0] = a0;
    values[1] = a1;
    values[2] = a8;
    values[3] = a9;
    values[4] = a22;
    values[5] = a5;
    values[6] = a6;
    values[7] = a13;
    values[8] = a14;
    values[9] = a27;
    values[10] = a10;
    values[11] = a11;
    values[12] = a18;
    values[13] = a19;
    values[14] = a2;
    values[15] = a15;
    values[16] = a16;
    values[17] = a23;
    values[18] = a24;
    values[19] = a7;
    values[20] = a20;
    values[21] = a21;
    values[22] = a28;
    values[23] = a29;
    values[24] = a12;
    values[25] = a25;
    values[26] = a26;
    values[27] = a3;
    values[28] = a4;
    values[29] = a17;
}

/// Size 32 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 32`.
pub fn ntt_32(values: &mut [Field]) {
    assert_eq!(values.len(), 32);
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

/// Size 34 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 34`.
pub fn ntt_34(values: &mut [Field]) {
    assert_eq!(values.len(), 34);
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
    let (a2, a32) = (a2 + a32, a2 - a32);
    let (a8, a26) = (a8 + a26, a8 - a26);
    let a26 = (a26 << 48);
    let (a2, a8) = (a2 + a8, a2 - a8);
    let (a32, a26) = (a32 + a26, a32 - a26);
    let (a12, a22) = (a12 + a22, a12 - a22);
    let (a14, a20) = (a14 + a20, a14 - a20);
    let a20 = (a20 << 48);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a22, a20) = (a22 + a20, a22 - a20);
    let (a4, a30) = (a4 + a30, a4 - a30);
    let (a16, a18) = (a16 + a18, a16 - a18);
    let a18 = (a18 << 48);
    let (a4, a16) = (a4 + a16, a4 - a16);
    let (a30, a18) = (a30 + a18, a30 - a18);
    let (a24, a10) = (a24 + a10, a24 - a10);
    let (a28, a6) = (a28 + a6, a28 - a6);
    let a6 = (a6 << 48);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let (a10, a6) = (a10 + a6, a10 - a6);
    let a22 = (a22 << 12);
    let a14 = (a14 << 24);
    let a20 = (a20 << 36);
    let a30 = (a30 << 24);
    let a16 = (a16 << 48);
    let a18 = (a18 << 72);
    let a10 = (a10 << 36);
    let a28 = (a28 << 72);
    let a6 = (-(a6 << 12));
    let (a2, a4) = (a2 + a4, a2 - a4);
    let (a12, a24) = (a12 + a24, a12 - a24);
    let a24 = (a24 << 48);
    let (a2, a12) = (a2 + a12, a2 - a12);
    let (a4, a24) = (a4 + a24, a4 - a24);
    let (a32, a30) = (a32 + a30, a32 - a30);
    let (a22, a10) = (a22 + a10, a22 - a10);
    let a10 = (a10 << 48);
    let (a32, a22) = (a32 + a22, a32 - a22);
    let (a30, a10) = (a30 + a10, a30 - a10);
    let (a8, a16) = (a8 + a16, a8 - a16);
    let (a14, a28) = (a14 + a28, a14 - a28);
    let a28 = (a28 << 48);
    let (a8, a14) = (a8 + a14, a8 - a14);
    let (a16, a28) = (a16 + a28, a16 - a28);
    let (a26, a18) = (a26 + a18, a26 - a18);
    let (a20, a6) = (a20 + a6, a20 - a6);
    let a6 = (a6 << 48);
    let (a26, a20) = (a26 + a20, a26 - a20);
    let (a18, a6) = (a18 + a6, a18 - a6);
    let t = a0;
    let a0 = a0 + a2;
    let a2 = a2 * Field::new(1152921504338411520);
    let a32 = a32 * Field::new(6259776822214049175);
    let a8 = a8 * Field::new(9380094172986290191);
    let a26 = a26 * Field::new(891943630314919127);
    let a4 = a4 * Field::new(17228171707553225791);
    let a30 = a30 * Field::new(12855743360534130886);
    let a16 = a16 * Field::new(6167687396920564837);
    let a18 = a18 * Field::new(17201834061724655524);
    let a12 = a12 * Field::new(15308299771656910737);
    let a22 = a22 * Field::new(18186005861103657533);
    let a14 = a14 * Field::new(53595491891823545);
    let a20 = a20 * Field::new(1906638201581172103);
    let a24 = a24 * Field::new(18303651001328874822);
    let a10 = a10 * Field::new(3077466521755967626);
    let a28 = a28 * Field::new(12423593102987598328);
    let a6 = a6 * Field::new(18361513053649472048);
    let a2 = a2 + t;
    let (a2, a12) = (a2 + a12, a2 - a12);
    let (a24, a4) = (a24 + a4, a24 - a4);
    let a4 = (a4 << 48);
    let (a2, a24) = (a2 + a24, a2 - a24);
    let (a12, a4) = (a12 + a4, a12 - a4);
    let (a6, a18) = (a6 + a18, a6 - a18);
    let (a20, a26) = (a20 + a26, a20 - a26);
    let a26 = (a26 << 48);
    let (a6, a20) = (a6 + a20, a6 - a20);
    let (a18, a26) = (a18 + a26, a18 - a26);
    let (a28, a16) = (a28 + a16, a28 - a16);
    let (a14, a8) = (a14 + a8, a14 - a8);
    let a8 = (a8 << 48);
    let (a28, a14) = (a28 + a14, a28 - a14);
    let (a16, a8) = (a16 + a8, a16 - a8);
    let (a10, a30) = (a10 + a30, a10 - a30);
    let (a22, a32) = (a22 + a32, a22 - a32);
    let a32 = (a32 << 48);
    let (a10, a22) = (a10 + a22, a10 - a22);
    let (a30, a32) = (a30 + a32, a30 - a32);
    let a18 = (a18 << 12);
    let a20 = (a20 << 24);
    let a26 = (a26 << 36);
    let a16 = (a16 << 24);
    let a14 = (a14 << 48);
    let a8 = (a8 << 72);
    let a30 = (a30 << 36);
    let a22 = (a22 << 72);
    let a32 = (-(a32 << 12));
    let (a2, a28) = (a2 + a28, a2 - a28);
    let (a6, a10) = (a6 + a10, a6 - a10);
    let a10 = (a10 << 48);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a28, a10) = (a28 + a10, a28 - a10);
    let (a12, a16) = (a12 + a16, a12 - a16);
    let (a18, a30) = (a18 + a30, a18 - a30);
    let a30 = (a30 << 48);
    let (a12, a18) = (a12 + a18, a12 - a18);
    let (a16, a30) = (a16 + a30, a16 - a30);
    let (a24, a14) = (a24 + a14, a24 - a14);
    let (a20, a22) = (a20 + a22, a20 - a22);
    let a22 = (a22 << 48);
    let (a24, a20) = (a24 + a20, a24 - a20);
    let (a14, a22) = (a14 + a22, a14 - a22);
    let (a4, a8) = (a4 + a8, a4 - a8);
    let (a26, a32) = (a26 + a32, a26 - a32);
    let a32 = (a32 << 48);
    let (a4, a26) = (a4 + a26, a4 - a26);
    let (a8, a32) = (a8 + a32, a8 - a32);
    let (a19, a15) = (a19 + a15, a19 - a15);
    let (a25, a9) = (a25 + a9, a25 - a9);
    let a9 = (a9 << 48);
    let (a19, a25) = (a19 + a25, a19 - a25);
    let (a15, a9) = (a15 + a9, a15 - a9);
    let (a29, a5) = (a29 + a5, a29 - a5);
    let (a31, a3) = (a31 + a3, a31 - a3);
    let a3 = (a3 << 48);
    let (a29, a31) = (a29 + a31, a29 - a31);
    let (a5, a3) = (a5 + a3, a5 - a3);
    let (a21, a13) = (a21 + a13, a21 - a13);
    let (a33, a1) = (a33 + a1, a33 - a1);
    let a1 = (a1 << 48);
    let (a21, a33) = (a21 + a33, a21 - a33);
    let (a13, a1) = (a13 + a1, a13 - a1);
    let (a7, a27) = (a7 + a27, a7 - a27);
    let (a11, a23) = (a11 + a23, a11 - a23);
    let a23 = (a23 << 48);
    let (a7, a11) = (a7 + a11, a7 - a11);
    let (a27, a23) = (a27 + a23, a27 - a23);
    let a5 = (a5 << 12);
    let a31 = (a31 << 24);
    let a3 = (a3 << 36);
    let a13 = (a13 << 24);
    let a33 = (a33 << 48);
    let a1 = (a1 << 72);
    let a27 = (a27 << 36);
    let a11 = (a11 << 72);
    let a23 = (-(a23 << 12));
    let (a19, a21) = (a19 + a21, a19 - a21);
    let (a29, a7) = (a29 + a7, a29 - a7);
    let a7 = (a7 << 48);
    let (a19, a29) = (a19 + a29, a19 - a29);
    let (a21, a7) = (a21 + a7, a21 - a7);
    let (a15, a13) = (a15 + a13, a15 - a13);
    let (a5, a27) = (a5 + a27, a5 - a27);
    let a27 = (a27 << 48);
    let (a15, a5) = (a15 + a5, a15 - a5);
    let (a13, a27) = (a13 + a27, a13 - a27);
    let (a25, a33) = (a25 + a33, a25 - a33);
    let (a31, a11) = (a31 + a11, a31 - a11);
    let a11 = (a11 << 48);
    let (a25, a31) = (a25 + a31, a25 - a31);
    let (a33, a11) = (a33 + a11, a33 - a11);
    let (a9, a1) = (a9 + a1, a9 - a1);
    let (a3, a23) = (a3 + a23, a3 - a23);
    let a23 = (a23 << 48);
    let (a9, a3) = (a9 + a3, a9 - a3);
    let (a1, a23) = (a1 + a23, a1 - a23);
    let t = a17;
    let a17 = a17 + a19;
    let a19 = a19 * Field::new(1152921504338411520);
    let a15 = a15 * Field::new(6259776822214049175);
    let a25 = a25 * Field::new(9380094172986290191);
    let a9 = a9 * Field::new(891943630314919127);
    let a21 = a21 * Field::new(17228171707553225791);
    let a13 = a13 * Field::new(12855743360534130886);
    let a33 = a33 * Field::new(6167687396920564837);
    let a1 = a1 * Field::new(17201834061724655524);
    let a29 = a29 * Field::new(15308299771656910737);
    let a5 = a5 * Field::new(18186005861103657533);
    let a31 = a31 * Field::new(53595491891823545);
    let a3 = a3 * Field::new(1906638201581172103);
    let a7 = a7 * Field::new(18303651001328874822);
    let a27 = a27 * Field::new(3077466521755967626);
    let a11 = a11 * Field::new(12423593102987598328);
    let a23 = a23 * Field::new(18361513053649472048);
    let a19 = a19 + t;
    let (a19, a29) = (a19 + a29, a19 - a29);
    let (a7, a21) = (a7 + a21, a7 - a21);
    let a21 = (a21 << 48);
    let (a19, a7) = (a19 + a7, a19 - a7);
    let (a29, a21) = (a29 + a21, a29 - a21);
    let (a23, a1) = (a23 + a1, a23 - a1);
    let (a3, a9) = (a3 + a9, a3 - a9);
    let a9 = (a9 << 48);
    let (a23, a3) = (a23 + a3, a23 - a3);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let (a11, a33) = (a11 + a33, a11 - a33);
    let (a31, a25) = (a31 + a25, a31 - a25);
    let a25 = (a25 << 48);
    let (a11, a31) = (a11 + a31, a11 - a31);
    let (a33, a25) = (a33 + a25, a33 - a25);
    let (a27, a13) = (a27 + a13, a27 - a13);
    let (a5, a15) = (a5 + a15, a5 - a15);
    let a15 = (a15 << 48);
    let (a27, a5) = (a27 + a5, a27 - a5);
    let (a13, a15) = (a13 + a15, a13 - a15);
    let a1 = (a1 << 12);
    let a3 = (a3 << 24);
    let a9 = (a9 << 36);
    let a33 = (a33 << 24);
    let a31 = (a31 << 48);
    let a25 = (a25 << 72);
    let a13 = (a13 << 36);
    let a5 = (a5 << 72);
    let a15 = (-(a15 << 12));
    let (a19, a11) = (a19 + a11, a19 - a11);
    let (a23, a27) = (a23 + a27, a23 - a27);
    let a27 = (a27 << 48);
    let (a19, a23) = (a19 + a23, a19 - a23);
    let (a11, a27) = (a11 + a27, a11 - a27);
    let (a29, a33) = (a29 + a33, a29 - a33);
    let (a1, a13) = (a1 + a13, a1 - a13);
    let a13 = (a13 << 48);
    let (a29, a1) = (a29 + a1, a29 - a1);
    let (a33, a13) = (a33 + a13, a33 - a13);
    let (a7, a31) = (a7 + a31, a7 - a31);
    let (a3, a5) = (a3 + a5, a3 - a5);
    let a5 = (a5 << 48);
    let (a7, a3) = (a7 + a3, a7 - a3);
    let (a31, a5) = (a31 + a5, a31 - a5);
    let (a21, a25) = (a21 + a25, a21 - a25);
    let (a9, a15) = (a9 + a15, a9 - a15);
    let a15 = (a15 << 48);
    let (a21, a9) = (a21 + a9, a21 - a9);
    let (a25, a15) = (a25 + a15, a25 - a15);
    let (a0, a17) = (a0 + a17, a0 - a17);
    let (a2, a19) = (a2 + a19, a2 - a19);
    let (a22, a5) = (a22 + a5, a22 - a5);
    let (a12, a29) = (a12 + a29, a12 - a29);
    let (a10, a27) = (a10 + a27, a10 - a27);
    let (a16, a33) = (a16 + a33, a16 - a33);
    let (a32, a15) = (a32 + a15, a32 - a15);
    let (a26, a9) = (a26 + a9, a26 - a9);
    let (a20, a3) = (a20 + a3, a20 - a3);
    let (a24, a7) = (a24 + a7, a24 - a7);
    let (a4, a21) = (a4 + a21, a4 - a21);
    let (a8, a25) = (a8 + a25, a8 - a25);
    let (a30, a13) = (a30 + a13, a30 - a13);
    let (a28, a11) = (a28 + a11, a28 - a11);
    let (a18, a1) = (a18 + a1, a18 - a1);
    let (a14, a31) = (a14 + a31, a14 - a31);
    let (a6, a23) = (a6 + a23, a6 - a23);
    values[0] = a0;
    values[1] = a19;
    values[2] = a22;
    values[3] = a29;
    values[4] = a10;
    values[5] = a33;
    values[6] = a32;
    values[7] = a9;
    values[8] = a20;
    values[9] = a7;
    values[10] = a4;
    values[11] = a25;
    values[12] = a30;
    values[13] = a11;
    values[14] = a18;
    values[15] = a31;
    values[16] = a6;
    values[17] = a17;
    values[18] = a2;
    values[19] = a5;
    values[20] = a12;
    values[21] = a27;
    values[22] = a16;
    values[23] = a15;
    values[24] = a26;
    values[25] = a3;
    values[26] = a24;
    values[27] = a21;
    values[28] = a8;
    values[29] = a13;
    values[30] = a28;
    values[31] = a1;
    values[32] = a14;
    values[33] = a23;
}

/// Size 40 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 40`.
pub fn ntt_40(values: &mut [Field]) {
    assert_eq!(values.len(), 40);
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
    let (a0, a20) = (a0 + a20, a0 - a20);
    let (a5, a25) = (a5 + a25, a5 - a25);
    let (a10, a30) = (a10 + a30, a10 - a30);
    let (a15, a35) = (a15 + a35, a15 - a35);
    let a25 = (a25 << 24);
    let a30 = (a30 << 48);
    let a35 = (a35 << 72);
    let (a0, a10) = (a0 + a10, a0 - a10);
    let (a5, a15) = (a5 + a15, a5 - a15);
    let a15 = (a15 << 48);
    let (a0, a5) = (a0 + a5, a0 - a5);
    let (a10, a15) = (a10 + a15, a10 - a15);
    let (a20, a30) = (a20 + a30, a20 - a30);
    let (a25, a35) = (a25 + a35, a25 - a35);
    let a35 = (a35 << 48);
    let (a20, a25) = (a20 + a25, a20 - a25);
    let (a30, a35) = (a30 + a35, a30 - a35);
    let (a8, a28) = (a8 + a28, a8 - a28);
    let (a13, a33) = (a13 + a33, a13 - a33);
    let (a18, a38) = (a18 + a38, a18 - a38);
    let (a23, a3) = (a23 + a3, a23 - a3);
    let a33 = (a33 << 24);
    let a38 = (a38 << 48);
    let a3 = (a3 << 72);
    let (a8, a18) = (a8 + a18, a8 - a18);
    let (a13, a23) = (a13 + a23, a13 - a23);
    let a23 = (a23 << 48);
    let (a8, a13) = (a8 + a13, a8 - a13);
    let (a18, a23) = (a18 + a23, a18 - a23);
    let (a28, a38) = (a28 + a38, a28 - a38);
    let (a33, a3) = (a33 + a3, a33 - a3);
    let a3 = (a3 << 48);
    let (a28, a33) = (a28 + a33, a28 - a33);
    let (a38, a3) = (a38 + a3, a38 - a3);
    let (a16, a36) = (a16 + a36, a16 - a36);
    let (a21, a1) = (a21 + a1, a21 - a1);
    let (a26, a6) = (a26 + a6, a26 - a6);
    let (a31, a11) = (a31 + a11, a31 - a11);
    let a1 = (a1 << 24);
    let a6 = (a6 << 48);
    let a11 = (a11 << 72);
    let (a16, a26) = (a16 + a26, a16 - a26);
    let (a21, a31) = (a21 + a31, a21 - a31);
    let a31 = (a31 << 48);
    let (a16, a21) = (a16 + a21, a16 - a21);
    let (a26, a31) = (a26 + a31, a26 - a31);
    let (a36, a6) = (a36 + a6, a36 - a6);
    let (a1, a11) = (a1 + a11, a1 - a11);
    let a11 = (a11 << 48);
    let (a36, a1) = (a36 + a1, a36 - a1);
    let (a6, a11) = (a6 + a11, a6 - a11);
    let (a24, a4) = (a24 + a4, a24 - a4);
    let (a29, a9) = (a29 + a9, a29 - a9);
    let (a34, a14) = (a34 + a14, a34 - a14);
    let (a39, a19) = (a39 + a19, a39 - a19);
    let a9 = (a9 << 24);
    let a14 = (a14 << 48);
    let a19 = (a19 << 72);
    let (a24, a34) = (a24 + a34, a24 - a34);
    let (a29, a39) = (a29 + a39, a29 - a39);
    let a39 = (a39 << 48);
    let (a24, a29) = (a24 + a29, a24 - a29);
    let (a34, a39) = (a34 + a39, a34 - a39);
    let (a4, a14) = (a4 + a14, a4 - a14);
    let (a9, a19) = (a9 + a19, a9 - a19);
    let a19 = (a19 << 48);
    let (a4, a9) = (a4 + a9, a4 - a9);
    let (a14, a19) = (a14 + a19, a14 - a19);
    let (a32, a12) = (a32 + a12, a32 - a12);
    let (a37, a17) = (a37 + a17, a37 - a17);
    let (a2, a22) = (a2 + a22, a2 - a22);
    let (a7, a27) = (a7 + a27, a7 - a27);
    let a17 = (a17 << 24);
    let a22 = (a22 << 48);
    let a27 = (a27 << 72);
    let (a32, a2) = (a32 + a2, a32 - a2);
    let (a37, a7) = (a37 + a7, a37 - a7);
    let a7 = (a7 << 48);
    let (a32, a37) = (a32 + a37, a32 - a37);
    let (a2, a7) = (a2 + a7, a2 - a7);
    let (a12, a22) = (a12 + a22, a12 - a22);
    let (a17, a27) = (a17 + a27, a17 - a27);
    let a27 = (a27 << 48);
    let (a12, a17) = (a12 + a17, a12 - a17);
    let (a22, a27) = (a22 + a27, a22 - a27);
    let (a8, a32) = (a8 + a32, a8 - a32);
    let (a24, a16) = (a24 + a16, a24 - a16);
    let a16 = (a16 << 48);
    let (a8, a24) = (a8 + a24, a8 - a24);
    let (a32, a16) = (a32 + a16, a32 - a16);
    let t = a0;
    let a0 = a0 + a8;
    let a8 = a8 * Field::new(4611686017353646080);
    let a32 = a32 * Field::new(16181989089180173841);
    let a24 = a24 * Field::new(5818851782451133869);
    let a16 = a16 * Field::new(11322249509082494407);
    let a8 = a8 + t;
    let (a8, a24) = (a8 + a24, a8 - a24);
    let (a16, a32) = (a16 + a32, a16 - a32);
    let a32 = (a32 << 48);
    let (a8, a16) = (a8 + a16, a8 - a16);
    let (a24, a32) = (a24 + a32, a24 - a32);
    let (a28, a12) = (a28 + a12, a28 - a12);
    let (a4, a36) = (a4 + a36, a4 - a36);
    let a36 = (a36 << 48);
    let (a28, a4) = (a28 + a4, a28 - a4);
    let (a12, a36) = (a12 + a36, a12 - a36);
    let t = a20;
    let a20 = a20 + a28;
    let a28 = a28 * Field::new(4611686017353646080);
    let a12 = a12 * Field::new(16181989089180173841);
    let a4 = a4 * Field::new(5818851782451133869);
    let a36 = a36 * Field::new(11322249509082494407);
    let a28 = a28 + t;
    let (a28, a4) = (a28 + a4, a28 - a4);
    let (a36, a12) = (a36 + a12, a36 - a12);
    let a12 = (a12 << 48);
    let (a28, a36) = (a28 + a36, a28 - a36);
    let (a4, a12) = (a4 + a12, a4 - a12);
    let (a18, a2) = (a18 + a2, a18 - a2);
    let (a34, a26) = (a34 + a26, a34 - a26);
    let a26 = (a26 << 48);
    let (a18, a34) = (a18 + a34, a18 - a34);
    let (a2, a26) = (a2 + a26, a2 - a26);
    let t = a10;
    let a10 = a10 + a18;
    let a18 = a18 * Field::new(4611686017353646080);
    let a2 = a2 * Field::new(16181989089180173841);
    let a34 = a34 * Field::new(5818851782451133869);
    let a26 = a26 * Field::new(11322249509082494407);
    let a18 = a18 + t;
    let (a18, a34) = (a18 + a34, a18 - a34);
    let (a26, a2) = (a26 + a2, a26 - a2);
    let a2 = (a2 << 48);
    let (a18, a26) = (a18 + a26, a18 - a26);
    let (a34, a2) = (a34 + a2, a34 - a2);
    let (a38, a22) = (a38 + a22, a38 - a22);
    let (a14, a6) = (a14 + a6, a14 - a6);
    let a6 = (a6 << 48);
    let (a38, a14) = (a38 + a14, a38 - a14);
    let (a22, a6) = (a22 + a6, a22 - a6);
    let t = a30;
    let a30 = a30 + a38;
    let a38 = a38 * Field::new(4611686017353646080);
    let a22 = a22 * Field::new(16181989089180173841);
    let a14 = a14 * Field::new(5818851782451133869);
    let a6 = a6 * Field::new(11322249509082494407);
    let a38 = a38 + t;
    let (a38, a14) = (a38 + a14, a38 - a14);
    let (a6, a22) = (a6 + a22, a6 - a22);
    let a22 = (a22 << 48);
    let (a38, a6) = (a38 + a6, a38 - a6);
    let (a14, a22) = (a14 + a22, a14 - a22);
    let (a13, a37) = (a13 + a37, a13 - a37);
    let (a29, a21) = (a29 + a21, a29 - a21);
    let a21 = (a21 << 48);
    let (a13, a29) = (a13 + a29, a13 - a29);
    let (a37, a21) = (a37 + a21, a37 - a21);
    let t = a5;
    let a5 = a5 + a13;
    let a13 = a13 * Field::new(4611686017353646080);
    let a37 = a37 * Field::new(16181989089180173841);
    let a29 = a29 * Field::new(5818851782451133869);
    let a21 = a21 * Field::new(11322249509082494407);
    let a13 = a13 + t;
    let (a13, a29) = (a13 + a29, a13 - a29);
    let (a21, a37) = (a21 + a37, a21 - a37);
    let a37 = (a37 << 48);
    let (a13, a21) = (a13 + a21, a13 - a21);
    let (a29, a37) = (a29 + a37, a29 - a37);
    let (a33, a17) = (a33 + a17, a33 - a17);
    let (a9, a1) = (a9 + a1, a9 - a1);
    let a1 = (a1 << 48);
    let (a33, a9) = (a33 + a9, a33 - a9);
    let (a17, a1) = (a17 + a1, a17 - a1);
    let t = a25;
    let a25 = a25 + a33;
    let a33 = a33 * Field::new(4611686017353646080);
    let a17 = a17 * Field::new(16181989089180173841);
    let a9 = a9 * Field::new(5818851782451133869);
    let a1 = a1 * Field::new(11322249509082494407);
    let a33 = a33 + t;
    let (a33, a9) = (a33 + a9, a33 - a9);
    let (a1, a17) = (a1 + a17, a1 - a17);
    let a17 = (a17 << 48);
    let (a33, a1) = (a33 + a1, a33 - a1);
    let (a9, a17) = (a9 + a17, a9 - a17);
    let (a23, a7) = (a23 + a7, a23 - a7);
    let (a39, a31) = (a39 + a31, a39 - a31);
    let a31 = (a31 << 48);
    let (a23, a39) = (a23 + a39, a23 - a39);
    let (a7, a31) = (a7 + a31, a7 - a31);
    let t = a15;
    let a15 = a15 + a23;
    let a23 = a23 * Field::new(4611686017353646080);
    let a7 = a7 * Field::new(16181989089180173841);
    let a39 = a39 * Field::new(5818851782451133869);
    let a31 = a31 * Field::new(11322249509082494407);
    let a23 = a23 + t;
    let (a23, a39) = (a23 + a39, a23 - a39);
    let (a31, a7) = (a31 + a7, a31 - a7);
    let a7 = (a7 << 48);
    let (a23, a31) = (a23 + a31, a23 - a31);
    let (a39, a7) = (a39 + a7, a39 - a7);
    let (a3, a27) = (a3 + a27, a3 - a27);
    let (a19, a11) = (a19 + a11, a19 - a11);
    let a11 = (a11 << 48);
    let (a3, a19) = (a3 + a19, a3 - a19);
    let (a27, a11) = (a27 + a11, a27 - a11);
    let t = a35;
    let a35 = a35 + a3;
    let a3 = a3 * Field::new(4611686017353646080);
    let a27 = a27 * Field::new(16181989089180173841);
    let a19 = a19 * Field::new(5818851782451133869);
    let a11 = a11 * Field::new(11322249509082494407);
    let a3 = a3 + t;
    let (a3, a19) = (a3 + a19, a3 - a19);
    let (a11, a27) = (a11 + a27, a11 - a27);
    let a27 = (a27 << 48);
    let (a3, a11) = (a3 + a11, a3 - a11);
    let (a19, a27) = (a19 + a27, a19 - a27);
    values[0] = a0;
    values[1] = a28;
    values[2] = a34;
    values[3] = a22;
    values[4] = a21;
    values[5] = a25;
    values[6] = a23;
    values[7] = a19;
    values[8] = a32;
    values[9] = a36;
    values[10] = a10;
    values[11] = a38;
    values[12] = a29;
    values[13] = a17;
    values[14] = a31;
    values[15] = a35;
    values[16] = a8;
    values[17] = a4;
    values[18] = a2;
    values[19] = a6;
    values[20] = a5;
    values[21] = a33;
    values[22] = a39;
    values[23] = a27;
    values[24] = a16;
    values[25] = a20;
    values[26] = a18;
    values[27] = a14;
    values[28] = a37;
    values[29] = a1;
    values[30] = a15;
    values[31] = a3;
    values[32] = a24;
    values[33] = a12;
    values[34] = a26;
    values[35] = a30;
    values[36] = a13;
    values[37] = a9;
    values[38] = a7;
    values[39] = a11;
}

/// Size 48 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 48`.
pub fn ntt_48(values: &mut [Field]) {
    assert_eq!(values.len(), 48);
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
    let (a0, a16, a32) = (
        a0 + a16 + a32,
        a0 + (a16 << 64) - (a32 << 32),
        a0 - (a16 << 32) + (a32 << 64),
    );
    let (a24, a40, a8) = (
        a24 + a40 + a8,
        a24 + (a40 << 64) - (a8 << 32),
        a24 - (a40 << 32) + (a8 << 64),
    );
    let (a0, a24) = (a0 + a24, a0 - a24);
    let (a16, a40) = (a16 + a40, a16 - a40);
    let (a32, a8) = (a32 + a8, a32 - a8);
    let (a1, a17, a33) = (
        a1 + a17 + a33,
        a1 + (a17 << 64) - (a33 << 32),
        a1 - (a17 << 32) + (a33 << 64),
    );
    let (a25, a41, a9) = (
        a25 + a41 + a9,
        a25 + (a41 << 64) - (a9 << 32),
        a25 - (a41 << 32) + (a9 << 64),
    );
    let (a1, a25) = (a1 + a25, a1 - a25);
    let (a17, a41) = (a17 + a41, a17 - a41);
    let (a33, a9) = (a33 + a9, a33 - a9);
    let (a2, a18, a34) = (
        a2 + a18 + a34,
        a2 + (a18 << 64) - (a34 << 32),
        a2 - (a18 << 32) + (a34 << 64),
    );
    let (a26, a42, a10) = (
        a26 + a42 + a10,
        a26 + (a42 << 64) - (a10 << 32),
        a26 - (a42 << 32) + (a10 << 64),
    );
    let (a2, a26) = (a2 + a26, a2 - a26);
    let (a18, a42) = (a18 + a42, a18 - a42);
    let (a34, a10) = (a34 + a10, a34 - a10);
    let (a3, a19, a35) = (
        a3 + a19 + a35,
        a3 + (a19 << 64) - (a35 << 32),
        a3 - (a19 << 32) + (a35 << 64),
    );
    let (a27, a43, a11) = (
        a27 + a43 + a11,
        a27 + (a43 << 64) - (a11 << 32),
        a27 - (a43 << 32) + (a11 << 64),
    );
    let (a3, a27) = (a3 + a27, a3 - a27);
    let (a19, a43) = (a19 + a43, a19 - a43);
    let (a35, a11) = (a35 + a11, a35 - a11);
    let (a4, a20, a36) = (
        a4 + a20 + a36,
        a4 + (a20 << 64) - (a36 << 32),
        a4 - (a20 << 32) + (a36 << 64),
    );
    let (a28, a44, a12) = (
        a28 + a44 + a12,
        a28 + (a44 << 64) - (a12 << 32),
        a28 - (a44 << 32) + (a12 << 64),
    );
    let (a4, a28) = (a4 + a28, a4 - a28);
    let (a20, a44) = (a20 + a44, a20 - a44);
    let (a36, a12) = (a36 + a12, a36 - a12);
    let (a5, a21, a37) = (
        a5 + a21 + a37,
        a5 + (a21 << 64) - (a37 << 32),
        a5 - (a21 << 32) + (a37 << 64),
    );
    let (a29, a45, a13) = (
        a29 + a45 + a13,
        a29 + (a45 << 64) - (a13 << 32),
        a29 - (a45 << 32) + (a13 << 64),
    );
    let (a5, a29) = (a5 + a29, a5 - a29);
    let (a21, a45) = (a21 + a45, a21 - a45);
    let (a37, a13) = (a37 + a13, a37 - a13);
    let (a6, a22, a38) = (
        a6 + a22 + a38,
        a6 + (a22 << 64) - (a38 << 32),
        a6 - (a22 << 32) + (a38 << 64),
    );
    let (a30, a46, a14) = (
        a30 + a46 + a14,
        a30 + (a46 << 64) - (a14 << 32),
        a30 - (a46 << 32) + (a14 << 64),
    );
    let (a6, a30) = (a6 + a30, a6 - a30);
    let (a22, a46) = (a22 + a46, a22 - a46);
    let (a38, a14) = (a38 + a14, a38 - a14);
    let (a7, a23, a39) = (
        a7 + a23 + a39,
        a7 + (a23 << 64) - (a39 << 32),
        a7 - (a23 << 32) + (a39 << 64),
    );
    let (a31, a47, a15) = (
        a31 + a47 + a15,
        a31 + (a47 << 64) - (a15 << 32),
        a31 - (a47 << 32) + (a15 << 64),
    );
    let (a7, a31) = (a7 + a31, a7 - a31);
    let (a23, a47) = (a23 + a47, a23 - a47);
    let (a39, a15) = (a39 + a15, a39 - a15);
    let a41 = (a41 << 4);
    let a33 = (a33 << 8);
    let a25 = (a25 << 12);
    let a17 = (a17 << 16);
    let a9 = (a9 << 20);
    let a42 = (a42 << 8);
    let a34 = (a34 << 16);
    let a26 = (a26 << 24);
    let a18 = (a18 << 32);
    let a10 = (a10 << 40);
    let a43 = (a43 << 12);
    let a35 = (a35 << 24);
    let a27 = (a27 << 36);
    let a19 = (a19 << 48);
    let a11 = (a11 << 60);
    let a44 = (a44 << 16);
    let a36 = (a36 << 32);
    let a28 = (a28 << 48);
    let a20 = (a20 << 64);
    let a12 = (a12 << 80);
    let a45 = (a45 << 20);
    let a37 = (a37 << 40);
    let a29 = (a29 << 60);
    let a21 = (a21 << 80);
    let a13 = (-(a13 << 4));
    let a46 = (a46 << 24);
    let a38 = (a38 << 48);
    let a30 = (a30 << 72);
    let a22 = (-a22);
    let a14 = (-(a14 << 24));
    let a47 = (a47 << 28);
    let a39 = (a39 << 56);
    let a31 = (a31 << 84);
    let a23 = (-(a23 << 16));
    let a15 = (-(a15 << 44));
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
    values[0] = a0;
    values[1] = a40;
    values[2] = a32;
    values[3] = a24;
    values[4] = a16;
    values[5] = a8;
    values[6] = a4;
    values[7] = a44;
    values[8] = a36;
    values[9] = a28;
    values[10] = a20;
    values[11] = a12;
    values[12] = a2;
    values[13] = a42;
    values[14] = a34;
    values[15] = a26;
    values[16] = a18;
    values[17] = a10;
    values[18] = a6;
    values[19] = a46;
    values[20] = a38;
    values[21] = a30;
    values[22] = a22;
    values[23] = a14;
    values[24] = a1;
    values[25] = a41;
    values[26] = a33;
    values[27] = a25;
    values[28] = a17;
    values[29] = a9;
    values[30] = a5;
    values[31] = a45;
    values[32] = a37;
    values[33] = a29;
    values[34] = a21;
    values[35] = a13;
    values[36] = a3;
    values[37] = a43;
    values[38] = a35;
    values[39] = a27;
    values[40] = a19;
    values[41] = a11;
    values[42] = a7;
    values[43] = a47;
    values[44] = a39;
    values[45] = a31;
    values[46] = a23;
    values[47] = a15;
}

/// Size 51 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 51`.
pub fn ntt_51(values: &mut [Field]) {
    assert_eq!(values.len(), 51);
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
    let (a3, a48) = (a3 + a48, a3 - a48);
    let (a12, a39) = (a12 + a39, a12 - a39);
    let a39 = (a39 << 48);
    let (a3, a12) = (a3 + a12, a3 - a12);
    let (a48, a39) = (a48 + a39, a48 - a39);
    let (a18, a33) = (a18 + a33, a18 - a33);
    let (a21, a30) = (a21 + a30, a21 - a30);
    let a30 = (a30 << 48);
    let (a18, a21) = (a18 + a21, a18 - a21);
    let (a33, a30) = (a33 + a30, a33 - a30);
    let (a6, a45) = (a6 + a45, a6 - a45);
    let (a24, a27) = (a24 + a27, a24 - a27);
    let a27 = (a27 << 48);
    let (a6, a24) = (a6 + a24, a6 - a24);
    let (a45, a27) = (a45 + a27, a45 - a27);
    let (a36, a15) = (a36 + a15, a36 - a15);
    let (a42, a9) = (a42 + a9, a42 - a9);
    let a9 = (a9 << 48);
    let (a36, a42) = (a36 + a42, a36 - a42);
    let (a15, a9) = (a15 + a9, a15 - a9);
    let a33 = (a33 << 12);
    let a21 = (a21 << 24);
    let a30 = (a30 << 36);
    let a45 = (a45 << 24);
    let a24 = (a24 << 48);
    let a27 = (a27 << 72);
    let a15 = (a15 << 36);
    let a42 = (a42 << 72);
    let a9 = (-(a9 << 12));
    let (a3, a6) = (a3 + a6, a3 - a6);
    let (a18, a36) = (a18 + a36, a18 - a36);
    let a36 = (a36 << 48);
    let (a3, a18) = (a3 + a18, a3 - a18);
    let (a6, a36) = (a6 + a36, a6 - a36);
    let (a48, a45) = (a48 + a45, a48 - a45);
    let (a33, a15) = (a33 + a15, a33 - a15);
    let a15 = (a15 << 48);
    let (a48, a33) = (a48 + a33, a48 - a33);
    let (a45, a15) = (a45 + a15, a45 - a15);
    let (a12, a24) = (a12 + a24, a12 - a24);
    let (a21, a42) = (a21 + a42, a21 - a42);
    let a42 = (a42 << 48);
    let (a12, a21) = (a12 + a21, a12 - a21);
    let (a24, a42) = (a24 + a42, a24 - a42);
    let (a39, a27) = (a39 + a27, a39 - a27);
    let (a30, a9) = (a30 + a9, a30 - a9);
    let a9 = (a9 << 48);
    let (a39, a30) = (a39 + a30, a39 - a30);
    let (a27, a9) = (a27 + a9, a27 - a9);
    let t = a0;
    let a0 = a0 + a3;
    let a3 = a3 * Field::new(1152921504338411520);
    let a48 = a48 * Field::new(6259776822214049175);
    let a12 = a12 * Field::new(9380094172986290191);
    let a39 = a39 * Field::new(891943630314919127);
    let a6 = a6 * Field::new(17228171707553225791);
    let a45 = a45 * Field::new(12855743360534130886);
    let a24 = a24 * Field::new(6167687396920564837);
    let a27 = a27 * Field::new(17201834061724655524);
    let a18 = a18 * Field::new(15308299771656910737);
    let a33 = a33 * Field::new(18186005861103657533);
    let a21 = a21 * Field::new(53595491891823545);
    let a30 = a30 * Field::new(1906638201581172103);
    let a36 = a36 * Field::new(18303651001328874822);
    let a15 = a15 * Field::new(3077466521755967626);
    let a42 = a42 * Field::new(12423593102987598328);
    let a9 = a9 * Field::new(18361513053649472048);
    let a3 = a3 + t;
    let (a3, a18) = (a3 + a18, a3 - a18);
    let (a36, a6) = (a36 + a6, a36 - a6);
    let a6 = (a6 << 48);
    let (a3, a36) = (a3 + a36, a3 - a36);
    let (a18, a6) = (a18 + a6, a18 - a6);
    let (a9, a27) = (a9 + a27, a9 - a27);
    let (a30, a39) = (a30 + a39, a30 - a39);
    let a39 = (a39 << 48);
    let (a9, a30) = (a9 + a30, a9 - a30);
    let (a27, a39) = (a27 + a39, a27 - a39);
    let (a42, a24) = (a42 + a24, a42 - a24);
    let (a21, a12) = (a21 + a12, a21 - a12);
    let a12 = (a12 << 48);
    let (a42, a21) = (a42 + a21, a42 - a21);
    let (a24, a12) = (a24 + a12, a24 - a12);
    let (a15, a45) = (a15 + a45, a15 - a45);
    let (a33, a48) = (a33 + a48, a33 - a48);
    let a48 = (a48 << 48);
    let (a15, a33) = (a15 + a33, a15 - a33);
    let (a45, a48) = (a45 + a48, a45 - a48);
    let a27 = (a27 << 12);
    let a30 = (a30 << 24);
    let a39 = (a39 << 36);
    let a24 = (a24 << 24);
    let a21 = (a21 << 48);
    let a12 = (a12 << 72);
    let a45 = (a45 << 36);
    let a33 = (a33 << 72);
    let a48 = (-(a48 << 12));
    let (a3, a42) = (a3 + a42, a3 - a42);
    let (a9, a15) = (a9 + a15, a9 - a15);
    let a15 = (a15 << 48);
    let (a3, a9) = (a3 + a9, a3 - a9);
    let (a42, a15) = (a42 + a15, a42 - a15);
    let (a18, a24) = (a18 + a24, a18 - a24);
    let (a27, a45) = (a27 + a45, a27 - a45);
    let a45 = (a45 << 48);
    let (a18, a27) = (a18 + a27, a18 - a27);
    let (a24, a45) = (a24 + a45, a24 - a45);
    let (a36, a21) = (a36 + a21, a36 - a21);
    let (a30, a33) = (a30 + a33, a30 - a33);
    let a33 = (a33 << 48);
    let (a36, a30) = (a36 + a30, a36 - a30);
    let (a21, a33) = (a21 + a33, a21 - a33);
    let (a6, a12) = (a6 + a12, a6 - a12);
    let (a39, a48) = (a39 + a48, a39 - a48);
    let a48 = (a48 << 48);
    let (a6, a39) = (a6 + a39, a6 - a39);
    let (a12, a48) = (a12 + a48, a12 - a48);
    let (a20, a14) = (a20 + a14, a20 - a14);
    let (a29, a5) = (a29 + a5, a29 - a5);
    let a5 = (a5 << 48);
    let (a20, a29) = (a20 + a29, a20 - a29);
    let (a14, a5) = (a14 + a5, a14 - a5);
    let (a35, a50) = (a35 + a50, a35 - a50);
    let (a38, a47) = (a38 + a47, a38 - a47);
    let a47 = (a47 << 48);
    let (a35, a38) = (a35 + a38, a35 - a38);
    let (a50, a47) = (a50 + a47, a50 - a47);
    let (a23, a11) = (a23 + a11, a23 - a11);
    let (a41, a44) = (a41 + a44, a41 - a44);
    let a44 = (a44 << 48);
    let (a23, a41) = (a23 + a41, a23 - a41);
    let (a11, a44) = (a11 + a44, a11 - a44);
    let (a2, a32) = (a2 + a32, a2 - a32);
    let (a8, a26) = (a8 + a26, a8 - a26);
    let a26 = (a26 << 48);
    let (a2, a8) = (a2 + a8, a2 - a8);
    let (a32, a26) = (a32 + a26, a32 - a26);
    let a50 = (a50 << 12);
    let a38 = (a38 << 24);
    let a47 = (a47 << 36);
    let a11 = (a11 << 24);
    let a41 = (a41 << 48);
    let a44 = (a44 << 72);
    let a32 = (a32 << 36);
    let a8 = (a8 << 72);
    let a26 = (-(a26 << 12));
    let (a20, a23) = (a20 + a23, a20 - a23);
    let (a35, a2) = (a35 + a2, a35 - a2);
    let a2 = (a2 << 48);
    let (a20, a35) = (a20 + a35, a20 - a35);
    let (a23, a2) = (a23 + a2, a23 - a2);
    let (a14, a11) = (a14 + a11, a14 - a11);
    let (a50, a32) = (a50 + a32, a50 - a32);
    let a32 = (a32 << 48);
    let (a14, a50) = (a14 + a50, a14 - a50);
    let (a11, a32) = (a11 + a32, a11 - a32);
    let (a29, a41) = (a29 + a41, a29 - a41);
    let (a38, a8) = (a38 + a8, a38 - a8);
    let a8 = (a8 << 48);
    let (a29, a38) = (a29 + a38, a29 - a38);
    let (a41, a8) = (a41 + a8, a41 - a8);
    let (a5, a44) = (a5 + a44, a5 - a44);
    let (a47, a26) = (a47 + a26, a47 - a26);
    let a26 = (a26 << 48);
    let (a5, a47) = (a5 + a47, a5 - a47);
    let (a44, a26) = (a44 + a26, a44 - a26);
    let t = a17;
    let a17 = a17 + a20;
    let a20 = a20 * Field::new(1152921504338411520);
    let a14 = a14 * Field::new(6259776822214049175);
    let a29 = a29 * Field::new(9380094172986290191);
    let a5 = a5 * Field::new(891943630314919127);
    let a23 = a23 * Field::new(17228171707553225791);
    let a11 = a11 * Field::new(12855743360534130886);
    let a41 = a41 * Field::new(6167687396920564837);
    let a44 = a44 * Field::new(17201834061724655524);
    let a35 = a35 * Field::new(15308299771656910737);
    let a50 = a50 * Field::new(18186005861103657533);
    let a38 = a38 * Field::new(53595491891823545);
    let a47 = a47 * Field::new(1906638201581172103);
    let a2 = a2 * Field::new(18303651001328874822);
    let a32 = a32 * Field::new(3077466521755967626);
    let a8 = a8 * Field::new(12423593102987598328);
    let a26 = a26 * Field::new(18361513053649472048);
    let a20 = a20 + t;
    let (a20, a35) = (a20 + a35, a20 - a35);
    let (a2, a23) = (a2 + a23, a2 - a23);
    let a23 = (a23 << 48);
    let (a20, a2) = (a20 + a2, a20 - a2);
    let (a35, a23) = (a35 + a23, a35 - a23);
    let (a26, a44) = (a26 + a44, a26 - a44);
    let (a47, a5) = (a47 + a5, a47 - a5);
    let a5 = (a5 << 48);
    let (a26, a47) = (a26 + a47, a26 - a47);
    let (a44, a5) = (a44 + a5, a44 - a5);
    let (a8, a41) = (a8 + a41, a8 - a41);
    let (a38, a29) = (a38 + a29, a38 - a29);
    let a29 = (a29 << 48);
    let (a8, a38) = (a8 + a38, a8 - a38);
    let (a41, a29) = (a41 + a29, a41 - a29);
    let (a32, a11) = (a32 + a11, a32 - a11);
    let (a50, a14) = (a50 + a14, a50 - a14);
    let a14 = (a14 << 48);
    let (a32, a50) = (a32 + a50, a32 - a50);
    let (a11, a14) = (a11 + a14, a11 - a14);
    let a44 = (a44 << 12);
    let a47 = (a47 << 24);
    let a5 = (a5 << 36);
    let a41 = (a41 << 24);
    let a38 = (a38 << 48);
    let a29 = (a29 << 72);
    let a11 = (a11 << 36);
    let a50 = (a50 << 72);
    let a14 = (-(a14 << 12));
    let (a20, a8) = (a20 + a8, a20 - a8);
    let (a26, a32) = (a26 + a32, a26 - a32);
    let a32 = (a32 << 48);
    let (a20, a26) = (a20 + a26, a20 - a26);
    let (a8, a32) = (a8 + a32, a8 - a32);
    let (a35, a41) = (a35 + a41, a35 - a41);
    let (a44, a11) = (a44 + a11, a44 - a11);
    let a11 = (a11 << 48);
    let (a35, a44) = (a35 + a44, a35 - a44);
    let (a41, a11) = (a41 + a11, a41 - a11);
    let (a2, a38) = (a2 + a38, a2 - a38);
    let (a47, a50) = (a47 + a50, a47 - a50);
    let a50 = (a50 << 48);
    let (a2, a47) = (a2 + a47, a2 - a47);
    let (a38, a50) = (a38 + a50, a38 - a50);
    let (a23, a29) = (a23 + a29, a23 - a29);
    let (a5, a14) = (a5 + a14, a5 - a14);
    let a14 = (a14 << 48);
    let (a23, a5) = (a23 + a5, a23 - a5);
    let (a29, a14) = (a29 + a14, a29 - a14);
    let (a37, a31) = (a37 + a31, a37 - a31);
    let (a46, a22) = (a46 + a22, a46 - a22);
    let a22 = (a22 << 48);
    let (a37, a46) = (a37 + a46, a37 - a46);
    let (a31, a22) = (a31 + a22, a31 - a22);
    let (a1, a16) = (a1 + a16, a1 - a16);
    let (a4, a13) = (a4 + a13, a4 - a13);
    let a13 = (a13 << 48);
    let (a1, a4) = (a1 + a4, a1 - a4);
    let (a16, a13) = (a16 + a13, a16 - a13);
    let (a40, a28) = (a40 + a28, a40 - a28);
    let (a7, a10) = (a7 + a10, a7 - a10);
    let a10 = (a10 << 48);
    let (a40, a7) = (a40 + a7, a40 - a7);
    let (a28, a10) = (a28 + a10, a28 - a10);
    let (a19, a49) = (a19 + a49, a19 - a49);
    let (a25, a43) = (a25 + a43, a25 - a43);
    let a43 = (a43 << 48);
    let (a19, a25) = (a19 + a25, a19 - a25);
    let (a49, a43) = (a49 + a43, a49 - a43);
    let a16 = (a16 << 12);
    let a4 = (a4 << 24);
    let a13 = (a13 << 36);
    let a28 = (a28 << 24);
    let a7 = (a7 << 48);
    let a10 = (a10 << 72);
    let a49 = (a49 << 36);
    let a25 = (a25 << 72);
    let a43 = (-(a43 << 12));
    let (a37, a40) = (a37 + a40, a37 - a40);
    let (a1, a19) = (a1 + a19, a1 - a19);
    let a19 = (a19 << 48);
    let (a37, a1) = (a37 + a1, a37 - a1);
    let (a40, a19) = (a40 + a19, a40 - a19);
    let (a31, a28) = (a31 + a28, a31 - a28);
    let (a16, a49) = (a16 + a49, a16 - a49);
    let a49 = (a49 << 48);
    let (a31, a16) = (a31 + a16, a31 - a16);
    let (a28, a49) = (a28 + a49, a28 - a49);
    let (a46, a7) = (a46 + a7, a46 - a7);
    let (a4, a25) = (a4 + a25, a4 - a25);
    let a25 = (a25 << 48);
    let (a46, a4) = (a46 + a4, a46 - a4);
    let (a7, a25) = (a7 + a25, a7 - a25);
    let (a22, a10) = (a22 + a10, a22 - a10);
    let (a13, a43) = (a13 + a43, a13 - a43);
    let a43 = (a43 << 48);
    let (a22, a13) = (a22 + a13, a22 - a13);
    let (a10, a43) = (a10 + a43, a10 - a43);
    let t = a34;
    let a34 = a34 + a37;
    let a37 = a37 * Field::new(1152921504338411520);
    let a31 = a31 * Field::new(6259776822214049175);
    let a46 = a46 * Field::new(9380094172986290191);
    let a22 = a22 * Field::new(891943630314919127);
    let a40 = a40 * Field::new(17228171707553225791);
    let a28 = a28 * Field::new(12855743360534130886);
    let a7 = a7 * Field::new(6167687396920564837);
    let a10 = a10 * Field::new(17201834061724655524);
    let a1 = a1 * Field::new(15308299771656910737);
    let a16 = a16 * Field::new(18186005861103657533);
    let a4 = a4 * Field::new(53595491891823545);
    let a13 = a13 * Field::new(1906638201581172103);
    let a19 = a19 * Field::new(18303651001328874822);
    let a49 = a49 * Field::new(3077466521755967626);
    let a25 = a25 * Field::new(12423593102987598328);
    let a43 = a43 * Field::new(18361513053649472048);
    let a37 = a37 + t;
    let (a37, a1) = (a37 + a1, a37 - a1);
    let (a19, a40) = (a19 + a40, a19 - a40);
    let a40 = (a40 << 48);
    let (a37, a19) = (a37 + a19, a37 - a19);
    let (a1, a40) = (a1 + a40, a1 - a40);
    let (a43, a10) = (a43 + a10, a43 - a10);
    let (a13, a22) = (a13 + a22, a13 - a22);
    let a22 = (a22 << 48);
    let (a43, a13) = (a43 + a13, a43 - a13);
    let (a10, a22) = (a10 + a22, a10 - a22);
    let (a25, a7) = (a25 + a7, a25 - a7);
    let (a4, a46) = (a4 + a46, a4 - a46);
    let a46 = (a46 << 48);
    let (a25, a4) = (a25 + a4, a25 - a4);
    let (a7, a46) = (a7 + a46, a7 - a46);
    let (a49, a28) = (a49 + a28, a49 - a28);
    let (a16, a31) = (a16 + a31, a16 - a31);
    let a31 = (a31 << 48);
    let (a49, a16) = (a49 + a16, a49 - a16);
    let (a28, a31) = (a28 + a31, a28 - a31);
    let a10 = (a10 << 12);
    let a13 = (a13 << 24);
    let a22 = (a22 << 36);
    let a7 = (a7 << 24);
    let a4 = (a4 << 48);
    let a46 = (a46 << 72);
    let a28 = (a28 << 36);
    let a16 = (a16 << 72);
    let a31 = (-(a31 << 12));
    let (a37, a25) = (a37 + a25, a37 - a25);
    let (a43, a49) = (a43 + a49, a43 - a49);
    let a49 = (a49 << 48);
    let (a37, a43) = (a37 + a43, a37 - a43);
    let (a25, a49) = (a25 + a49, a25 - a49);
    let (a1, a7) = (a1 + a7, a1 - a7);
    let (a10, a28) = (a10 + a28, a10 - a28);
    let a28 = (a28 << 48);
    let (a1, a10) = (a1 + a10, a1 - a10);
    let (a7, a28) = (a7 + a28, a7 - a28);
    let (a19, a4) = (a19 + a4, a19 - a4);
    let (a13, a16) = (a13 + a16, a13 - a16);
    let a16 = (a16 << 48);
    let (a19, a13) = (a19 + a13, a19 - a13);
    let (a4, a16) = (a4 + a16, a4 - a16);
    let (a40, a46) = (a40 + a46, a40 - a46);
    let (a22, a31) = (a22 + a31, a22 - a31);
    let a31 = (a31 << 48);
    let (a40, a22) = (a40 + a22, a40 - a22);
    let (a46, a31) = (a46 + a31, a46 - a31);
    let (a0, a17, a34) = (
        a0 + a17 + a34,
        a0 + (a17 << 64) - (a34 << 32),
        a0 - (a17 << 32) + (a34 << 64),
    );
    let (a3, a20, a37) = (
        a3 + a20 + a37,
        a3 + (a20 << 64) - (a37 << 32),
        a3 - (a20 << 32) + (a37 << 64),
    );
    let (a33, a50, a16) = (
        a33 + a50 + a16,
        a33 + (a50 << 64) - (a16 << 32),
        a33 - (a50 << 32) + (a16 << 64),
    );
    let (a18, a35, a1) = (
        a18 + a35 + a1,
        a18 + (a35 << 64) - (a1 << 32),
        a18 - (a35 << 32) + (a1 << 64),
    );
    let (a15, a32, a49) = (
        a15 + a32 + a49,
        a15 + (a32 << 64) - (a49 << 32),
        a15 - (a32 << 32) + (a49 << 64),
    );
    let (a24, a41, a7) = (
        a24 + a41 + a7,
        a24 + (a41 << 64) - (a7 << 32),
        a24 - (a41 << 32) + (a7 << 64),
    );
    let (a48, a14, a31) = (
        a48 + a14 + a31,
        a48 + (a14 << 64) - (a31 << 32),
        a48 - (a14 << 32) + (a31 << 64),
    );
    let (a39, a5, a22) = (
        a39 + a5 + a22,
        a39 + (a5 << 64) - (a22 << 32),
        a39 - (a5 << 32) + (a22 << 64),
    );
    let (a30, a47, a13) = (
        a30 + a47 + a13,
        a30 + (a47 << 64) - (a13 << 32),
        a30 - (a47 << 32) + (a13 << 64),
    );
    let (a36, a2, a19) = (
        a36 + a2 + a19,
        a36 + (a2 << 64) - (a19 << 32),
        a36 - (a2 << 32) + (a19 << 64),
    );
    let (a6, a23, a40) = (
        a6 + a23 + a40,
        a6 + (a23 << 64) - (a40 << 32),
        a6 - (a23 << 32) + (a40 << 64),
    );
    let (a12, a29, a46) = (
        a12 + a29 + a46,
        a12 + (a29 << 64) - (a46 << 32),
        a12 - (a29 << 32) + (a46 << 64),
    );
    let (a45, a11, a28) = (
        a45 + a11 + a28,
        a45 + (a11 << 64) - (a28 << 32),
        a45 - (a11 << 32) + (a28 << 64),
    );
    let (a42, a8, a25) = (
        a42 + a8 + a25,
        a42 + (a8 << 64) - (a25 << 32),
        a42 - (a8 << 32) + (a25 << 64),
    );
    let (a27, a44, a10) = (
        a27 + a44 + a10,
        a27 + (a44 << 64) - (a10 << 32),
        a27 - (a44 << 32) + (a10 << 64),
    );
    let (a21, a38, a4) = (
        a21 + a38 + a4,
        a21 + (a38 << 64) - (a4 << 32),
        a21 - (a38 << 32) + (a4 << 64),
    );
    let (a9, a26, a43) = (
        a9 + a26 + a43,
        a9 + (a26 << 64) - (a43 << 32),
        a9 - (a26 << 32) + (a43 << 64),
    );
    values[0] = a0;
    values[1] = a20;
    values[2] = a16;
    values[3] = a18;
    values[4] = a32;
    values[5] = a7;
    values[6] = a48;
    values[7] = a5;
    values[8] = a13;
    values[9] = a36;
    values[10] = a23;
    values[11] = a46;
    values[12] = a45;
    values[13] = a8;
    values[14] = a10;
    values[15] = a21;
    values[16] = a26;
    values[17] = a34;
    values[18] = a3;
    values[19] = a50;
    values[20] = a1;
    values[21] = a15;
    values[22] = a41;
    values[23] = a31;
    values[24] = a39;
    values[25] = a47;
    values[26] = a19;
    values[27] = a6;
    values[28] = a29;
    values[29] = a28;
    values[30] = a42;
    values[31] = a44;
    values[32] = a4;
    values[33] = a9;
    values[34] = a17;
    values[35] = a37;
    values[36] = a33;
    values[37] = a35;
    values[38] = a49;
    values[39] = a24;
    values[40] = a14;
    values[41] = a22;
    values[42] = a30;
    values[43] = a2;
    values[44] = a40;
    values[45] = a12;
    values[46] = a11;
    values[47] = a25;
    values[48] = a27;
    values[49] = a38;
    values[50] = a43;
}

/// Size 60 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 60`.
pub fn ntt_60(values: &mut [Field]) {
    assert_eq!(values.len(), 60);
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
    let (a0, a20, a40) = (
        a0 + a20 + a40,
        a0 + (a20 << 64) - (a40 << 32),
        a0 - (a20 << 32) + (a40 << 64),
    );
    let (a30, a50, a10) = (
        a30 + a50 + a10,
        a30 + (a50 << 64) - (a10 << 32),
        a30 - (a50 << 32) + (a10 << 64),
    );
    let (a0, a30) = (a0 + a30, a0 - a30);
    let (a20, a50) = (a20 + a50, a20 - a50);
    let (a40, a10) = (a40 + a10, a40 - a10);
    let (a1, a21, a41) = (
        a1 + a21 + a41,
        a1 + (a21 << 64) - (a41 << 32),
        a1 - (a21 << 32) + (a41 << 64),
    );
    let (a31, a51, a11) = (
        a31 + a51 + a11,
        a31 + (a51 << 64) - (a11 << 32),
        a31 - (a51 << 32) + (a11 << 64),
    );
    let (a1, a31) = (a1 + a31, a1 - a31);
    let (a21, a51) = (a21 + a51, a21 - a51);
    let (a41, a11) = (a41 + a11, a41 - a11);
    let (a2, a22, a42) = (
        a2 + a22 + a42,
        a2 + (a22 << 64) - (a42 << 32),
        a2 - (a22 << 32) + (a42 << 64),
    );
    let (a32, a52, a12) = (
        a32 + a52 + a12,
        a32 + (a52 << 64) - (a12 << 32),
        a32 - (a52 << 32) + (a12 << 64),
    );
    let (a2, a32) = (a2 + a32, a2 - a32);
    let (a22, a52) = (a22 + a52, a22 - a52);
    let (a42, a12) = (a42 + a12, a42 - a12);
    let (a3, a23, a43) = (
        a3 + a23 + a43,
        a3 + (a23 << 64) - (a43 << 32),
        a3 - (a23 << 32) + (a43 << 64),
    );
    let (a33, a53, a13) = (
        a33 + a53 + a13,
        a33 + (a53 << 64) - (a13 << 32),
        a33 - (a53 << 32) + (a13 << 64),
    );
    let (a3, a33) = (a3 + a33, a3 - a33);
    let (a23, a53) = (a23 + a53, a23 - a53);
    let (a43, a13) = (a43 + a13, a43 - a13);
    let (a4, a24, a44) = (
        a4 + a24 + a44,
        a4 + (a24 << 64) - (a44 << 32),
        a4 - (a24 << 32) + (a44 << 64),
    );
    let (a34, a54, a14) = (
        a34 + a54 + a14,
        a34 + (a54 << 64) - (a14 << 32),
        a34 - (a54 << 32) + (a14 << 64),
    );
    let (a4, a34) = (a4 + a34, a4 - a34);
    let (a24, a54) = (a24 + a54, a24 - a54);
    let (a44, a14) = (a44 + a14, a44 - a14);
    let (a5, a25, a45) = (
        a5 + a25 + a45,
        a5 + (a25 << 64) - (a45 << 32),
        a5 - (a25 << 32) + (a45 << 64),
    );
    let (a35, a55, a15) = (
        a35 + a55 + a15,
        a35 + (a55 << 64) - (a15 << 32),
        a35 - (a55 << 32) + (a15 << 64),
    );
    let (a5, a35) = (a5 + a35, a5 - a35);
    let (a25, a55) = (a25 + a55, a25 - a55);
    let (a45, a15) = (a45 + a15, a45 - a15);
    let (a6, a26, a46) = (
        a6 + a26 + a46,
        a6 + (a26 << 64) - (a46 << 32),
        a6 - (a26 << 32) + (a46 << 64),
    );
    let (a36, a56, a16) = (
        a36 + a56 + a16,
        a36 + (a56 << 64) - (a16 << 32),
        a36 - (a56 << 32) + (a16 << 64),
    );
    let (a6, a36) = (a6 + a36, a6 - a36);
    let (a26, a56) = (a26 + a56, a26 - a56);
    let (a46, a16) = (a46 + a16, a46 - a16);
    let (a7, a27, a47) = (
        a7 + a27 + a47,
        a7 + (a27 << 64) - (a47 << 32),
        a7 - (a27 << 32) + (a47 << 64),
    );
    let (a37, a57, a17) = (
        a37 + a57 + a17,
        a37 + (a57 << 64) - (a17 << 32),
        a37 - (a57 << 32) + (a17 << 64),
    );
    let (a7, a37) = (a7 + a37, a7 - a37);
    let (a27, a57) = (a27 + a57, a27 - a57);
    let (a47, a17) = (a47 + a17, a47 - a17);
    let (a8, a28, a48) = (
        a8 + a28 + a48,
        a8 + (a28 << 64) - (a48 << 32),
        a8 - (a28 << 32) + (a48 << 64),
    );
    let (a38, a58, a18) = (
        a38 + a58 + a18,
        a38 + (a58 << 64) - (a18 << 32),
        a38 - (a58 << 32) + (a18 << 64),
    );
    let (a8, a38) = (a8 + a38, a8 - a38);
    let (a28, a58) = (a28 + a58, a28 - a58);
    let (a48, a18) = (a48 + a18, a48 - a18);
    let (a9, a29, a49) = (
        a9 + a29 + a49,
        a9 + (a29 << 64) - (a49 << 32),
        a9 - (a29 << 32) + (a49 << 64),
    );
    let (a39, a59, a19) = (
        a39 + a59 + a19,
        a39 + (a59 << 64) - (a19 << 32),
        a39 - (a59 << 32) + (a19 << 64),
    );
    let (a9, a39) = (a9 + a39, a9 - a39);
    let (a29, a59) = (a29 + a59, a29 - a59);
    let (a49, a19) = (a49 + a19, a49 - a19);
    let a51 = a51 * Field::new(5927015354646419725);
    let a41 = a41 * Field::new(6868348408044855211);
    let a31 = a31 * Field::new(5290193119087387221);
    let a21 = a21 * Field::new(17775832080808074370);
    let a11 = (a11 << 16);
    let a52 = a52 * Field::new(6868348408044855211);
    let a42 = a42 * Field::new(17775832080808074370);
    let a32 = a32 * Field::new(18235156514275634624);
    let a22 = a22 * Field::new(9988211933311186582);
    let a12 = (a12 << 32);
    let a53 = a53 * Field::new(5290193119087387221);
    let a43 = a43 * Field::new(18235156514275634624);
    let a33 = a33 * Field::new(8149776168132872528);
    let a23 = a23 * Field::new(1041288259238279555);
    let a13 = (a13 << 48);
    let a54 = a54 * Field::new(17775832080808074370);
    let a44 = a44 * Field::new(9988211933311186582);
    let a34 = a34 * Field::new(1041288259238279555);
    let a24 = a24 * Field::new(6205107048362784195);
    let a14 = (a14 << 64);
    let a55 = (a55 << 16);
    let a45 = (a45 << 32);
    let a35 = (a35 << 48);
    let a25 = (a25 << 64);
    let a15 = (a15 << 80);
    let a56 = a56 * Field::new(18235156514275634624);
    let a46 = a46 * Field::new(1041288259238279555);
    let a36 = a36 * Field::new(17073700798457888299);
    let a26 = a26 * Field::new(15820824984080659046);
    let a16 = (-a16);
    let a57 = a57 * Field::new(5079231842359091375);
    let a47 = a47 * Field::new(15149912995474149095);
    let a37 = a37 * Field::new(17869255328328231396);
    let a27 = a27 * Field::new(7085488865146701717);
    let a17 = (-(a17 << 16));
    let a58 = a58 * Field::new(9988211933311186582);
    let a48 = a48 * Field::new(6205107048362784195);
    let a38 = a38 * Field::new(15820824984080659046);
    let a28 = a28 * Field::new(11578395661369729110);
    let a18 = (-(a18 << 32));
    let a59 = a59 * Field::new(8149776168132872528);
    let a49 = a49 * Field::new(17073700798457888299);
    let a39 = a39 * Field::new(2281812832982421726);
    let a29 = a29 * Field::new(211587555138949697);
    let a19 = (-(a19 << 48));
    let (a2, a8) = (a2 + a8, a2 - a8);
    let (a6, a4) = (a6 + a4, a6 - a4);
    let a4 = (a4 << 48);
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a8, a4) = (a8 + a4, a8 - a4);
    let t = a0;
    let a0 = a0 + a2;
    let a2 = a2 * Field::new(4611686017353646080);
    let a8 = a8 * Field::new(16181989089180173841);
    let a6 = a6 * Field::new(5818851782451133869);
    let a4 = a4 * Field::new(11322249509082494407);
    let a2 = a2 + t;
    let (a2, a6) = (a2 + a6, a2 - a6);
    let (a4, a8) = (a4 + a8, a4 - a8);
    let a8 = (a8 << 48);
    let (a2, a4) = (a2 + a4, a2 - a4);
    let (a6, a8) = (a6 + a8, a6 - a8);
    let (a7, a3) = (a7 + a3, a7 - a3);
    let (a1, a9) = (a1 + a9, a1 - a9);
    let a9 = (a9 << 48);
    let (a7, a1) = (a7 + a1, a7 - a1);
    let (a3, a9) = (a3 + a9, a3 - a9);
    let t = a5;
    let a5 = a5 + a7;
    let a7 = a7 * Field::new(4611686017353646080);
    let a3 = a3 * Field::new(16181989089180173841);
    let a1 = a1 * Field::new(5818851782451133869);
    let a9 = a9 * Field::new(11322249509082494407);
    let a7 = a7 + t;
    let (a7, a1) = (a7 + a1, a7 - a1);
    let (a9, a3) = (a9 + a3, a9 - a3);
    let a3 = (a3 << 48);
    let (a7, a9) = (a7 + a9, a7 - a9);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let (a0, a5) = (a0 + a5, a0 - a5);
    let (a2, a7) = (a2 + a7, a2 - a7);
    let (a6, a1) = (a6 + a1, a6 - a1);
    let (a8, a3) = (a8 + a3, a8 - a3);
    let (a4, a9) = (a4 + a9, a4 - a9);
    let (a52, a58) = (a52 + a58, a52 - a58);
    let (a56, a54) = (a56 + a54, a56 - a54);
    let a54 = (a54 << 48);
    let (a52, a56) = (a52 + a56, a52 - a56);
    let (a58, a54) = (a58 + a54, a58 - a54);
    let t = a50;
    let a50 = a50 + a52;
    let a52 = a52 * Field::new(4611686017353646080);
    let a58 = a58 * Field::new(16181989089180173841);
    let a56 = a56 * Field::new(5818851782451133869);
    let a54 = a54 * Field::new(11322249509082494407);
    let a52 = a52 + t;
    let (a52, a56) = (a52 + a56, a52 - a56);
    let (a54, a58) = (a54 + a58, a54 - a58);
    let a58 = (a58 << 48);
    let (a52, a54) = (a52 + a54, a52 - a54);
    let (a56, a58) = (a56 + a58, a56 - a58);
    let (a57, a53) = (a57 + a53, a57 - a53);
    let (a51, a59) = (a51 + a59, a51 - a59);
    let a59 = (a59 << 48);
    let (a57, a51) = (a57 + a51, a57 - a51);
    let (a53, a59) = (a53 + a59, a53 - a59);
    let t = a55;
    let a55 = a55 + a57;
    let a57 = a57 * Field::new(4611686017353646080);
    let a53 = a53 * Field::new(16181989089180173841);
    let a51 = a51 * Field::new(5818851782451133869);
    let a59 = a59 * Field::new(11322249509082494407);
    let a57 = a57 + t;
    let (a57, a51) = (a57 + a51, a57 - a51);
    let (a59, a53) = (a59 + a53, a59 - a53);
    let a53 = (a53 << 48);
    let (a57, a59) = (a57 + a59, a57 - a59);
    let (a51, a53) = (a51 + a53, a51 - a53);
    let (a50, a55) = (a50 + a55, a50 - a55);
    let (a52, a57) = (a52 + a57, a52 - a57);
    let (a56, a51) = (a56 + a51, a56 - a51);
    let (a58, a53) = (a58 + a53, a58 - a53);
    let (a54, a59) = (a54 + a59, a54 - a59);
    let (a42, a48) = (a42 + a48, a42 - a48);
    let (a46, a44) = (a46 + a44, a46 - a44);
    let a44 = (a44 << 48);
    let (a42, a46) = (a42 + a46, a42 - a46);
    let (a48, a44) = (a48 + a44, a48 - a44);
    let t = a40;
    let a40 = a40 + a42;
    let a42 = a42 * Field::new(4611686017353646080);
    let a48 = a48 * Field::new(16181989089180173841);
    let a46 = a46 * Field::new(5818851782451133869);
    let a44 = a44 * Field::new(11322249509082494407);
    let a42 = a42 + t;
    let (a42, a46) = (a42 + a46, a42 - a46);
    let (a44, a48) = (a44 + a48, a44 - a48);
    let a48 = (a48 << 48);
    let (a42, a44) = (a42 + a44, a42 - a44);
    let (a46, a48) = (a46 + a48, a46 - a48);
    let (a47, a43) = (a47 + a43, a47 - a43);
    let (a41, a49) = (a41 + a49, a41 - a49);
    let a49 = (a49 << 48);
    let (a47, a41) = (a47 + a41, a47 - a41);
    let (a43, a49) = (a43 + a49, a43 - a49);
    let t = a45;
    let a45 = a45 + a47;
    let a47 = a47 * Field::new(4611686017353646080);
    let a43 = a43 * Field::new(16181989089180173841);
    let a41 = a41 * Field::new(5818851782451133869);
    let a49 = a49 * Field::new(11322249509082494407);
    let a47 = a47 + t;
    let (a47, a41) = (a47 + a41, a47 - a41);
    let (a49, a43) = (a49 + a43, a49 - a43);
    let a43 = (a43 << 48);
    let (a47, a49) = (a47 + a49, a47 - a49);
    let (a41, a43) = (a41 + a43, a41 - a43);
    let (a40, a45) = (a40 + a45, a40 - a45);
    let (a42, a47) = (a42 + a47, a42 - a47);
    let (a46, a41) = (a46 + a41, a46 - a41);
    let (a48, a43) = (a48 + a43, a48 - a43);
    let (a44, a49) = (a44 + a49, a44 - a49);
    let (a32, a38) = (a32 + a38, a32 - a38);
    let (a36, a34) = (a36 + a34, a36 - a34);
    let a34 = (a34 << 48);
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a38, a34) = (a38 + a34, a38 - a34);
    let t = a30;
    let a30 = a30 + a32;
    let a32 = a32 * Field::new(4611686017353646080);
    let a38 = a38 * Field::new(16181989089180173841);
    let a36 = a36 * Field::new(5818851782451133869);
    let a34 = a34 * Field::new(11322249509082494407);
    let a32 = a32 + t;
    let (a32, a36) = (a32 + a36, a32 - a36);
    let (a34, a38) = (a34 + a38, a34 - a38);
    let a38 = (a38 << 48);
    let (a32, a34) = (a32 + a34, a32 - a34);
    let (a36, a38) = (a36 + a38, a36 - a38);
    let (a37, a33) = (a37 + a33, a37 - a33);
    let (a31, a39) = (a31 + a39, a31 - a39);
    let a39 = (a39 << 48);
    let (a37, a31) = (a37 + a31, a37 - a31);
    let (a33, a39) = (a33 + a39, a33 - a39);
    let t = a35;
    let a35 = a35 + a37;
    let a37 = a37 * Field::new(4611686017353646080);
    let a33 = a33 * Field::new(16181989089180173841);
    let a31 = a31 * Field::new(5818851782451133869);
    let a39 = a39 * Field::new(11322249509082494407);
    let a37 = a37 + t;
    let (a37, a31) = (a37 + a31, a37 - a31);
    let (a39, a33) = (a39 + a33, a39 - a33);
    let a33 = (a33 << 48);
    let (a37, a39) = (a37 + a39, a37 - a39);
    let (a31, a33) = (a31 + a33, a31 - a33);
    let (a30, a35) = (a30 + a35, a30 - a35);
    let (a32, a37) = (a32 + a37, a32 - a37);
    let (a36, a31) = (a36 + a31, a36 - a31);
    let (a38, a33) = (a38 + a33, a38 - a33);
    let (a34, a39) = (a34 + a39, a34 - a39);
    let (a22, a28) = (a22 + a28, a22 - a28);
    let (a26, a24) = (a26 + a24, a26 - a24);
    let a24 = (a24 << 48);
    let (a22, a26) = (a22 + a26, a22 - a26);
    let (a28, a24) = (a28 + a24, a28 - a24);
    let t = a20;
    let a20 = a20 + a22;
    let a22 = a22 * Field::new(4611686017353646080);
    let a28 = a28 * Field::new(16181989089180173841);
    let a26 = a26 * Field::new(5818851782451133869);
    let a24 = a24 * Field::new(11322249509082494407);
    let a22 = a22 + t;
    let (a22, a26) = (a22 + a26, a22 - a26);
    let (a24, a28) = (a24 + a28, a24 - a28);
    let a28 = (a28 << 48);
    let (a22, a24) = (a22 + a24, a22 - a24);
    let (a26, a28) = (a26 + a28, a26 - a28);
    let (a27, a23) = (a27 + a23, a27 - a23);
    let (a21, a29) = (a21 + a29, a21 - a29);
    let a29 = (a29 << 48);
    let (a27, a21) = (a27 + a21, a27 - a21);
    let (a23, a29) = (a23 + a29, a23 - a29);
    let t = a25;
    let a25 = a25 + a27;
    let a27 = a27 * Field::new(4611686017353646080);
    let a23 = a23 * Field::new(16181989089180173841);
    let a21 = a21 * Field::new(5818851782451133869);
    let a29 = a29 * Field::new(11322249509082494407);
    let a27 = a27 + t;
    let (a27, a21) = (a27 + a21, a27 - a21);
    let (a29, a23) = (a29 + a23, a29 - a23);
    let a23 = (a23 << 48);
    let (a27, a29) = (a27 + a29, a27 - a29);
    let (a21, a23) = (a21 + a23, a21 - a23);
    let (a20, a25) = (a20 + a25, a20 - a25);
    let (a22, a27) = (a22 + a27, a22 - a27);
    let (a26, a21) = (a26 + a21, a26 - a21);
    let (a28, a23) = (a28 + a23, a28 - a23);
    let (a24, a29) = (a24 + a29, a24 - a29);
    let (a12, a18) = (a12 + a18, a12 - a18);
    let (a16, a14) = (a16 + a14, a16 - a14);
    let a14 = (a14 << 48);
    let (a12, a16) = (a12 + a16, a12 - a16);
    let (a18, a14) = (a18 + a14, a18 - a14);
    let t = a10;
    let a10 = a10 + a12;
    let a12 = a12 * Field::new(4611686017353646080);
    let a18 = a18 * Field::new(16181989089180173841);
    let a16 = a16 * Field::new(5818851782451133869);
    let a14 = a14 * Field::new(11322249509082494407);
    let a12 = a12 + t;
    let (a12, a16) = (a12 + a16, a12 - a16);
    let (a14, a18) = (a14 + a18, a14 - a18);
    let a18 = (a18 << 48);
    let (a12, a14) = (a12 + a14, a12 - a14);
    let (a16, a18) = (a16 + a18, a16 - a18);
    let (a17, a13) = (a17 + a13, a17 - a13);
    let (a11, a19) = (a11 + a19, a11 - a19);
    let a19 = (a19 << 48);
    let (a17, a11) = (a17 + a11, a17 - a11);
    let (a13, a19) = (a13 + a19, a13 - a19);
    let t = a15;
    let a15 = a15 + a17;
    let a17 = a17 * Field::new(4611686017353646080);
    let a13 = a13 * Field::new(16181989089180173841);
    let a11 = a11 * Field::new(5818851782451133869);
    let a19 = a19 * Field::new(11322249509082494407);
    let a17 = a17 + t;
    let (a17, a11) = (a17 + a11, a17 - a11);
    let (a19, a13) = (a19 + a13, a19 - a13);
    let a13 = (a13 << 48);
    let (a17, a19) = (a17 + a19, a17 - a19);
    let (a11, a13) = (a11 + a13, a11 - a13);
    let (a10, a15) = (a10 + a15, a10 - a15);
    let (a12, a17) = (a12 + a17, a12 - a17);
    let (a16, a11) = (a16 + a11, a16 - a11);
    let (a18, a13) = (a18 + a13, a18 - a13);
    let (a14, a19) = (a14 + a19, a14 - a19);
    values[0] = a0;
    values[1] = a50;
    values[2] = a40;
    values[3] = a30;
    values[4] = a20;
    values[5] = a10;
    values[6] = a7;
    values[7] = a57;
    values[8] = a47;
    values[9] = a37;
    values[10] = a27;
    values[11] = a17;
    values[12] = a6;
    values[13] = a56;
    values[14] = a46;
    values[15] = a36;
    values[16] = a26;
    values[17] = a16;
    values[18] = a3;
    values[19] = a53;
    values[20] = a43;
    values[21] = a33;
    values[22] = a23;
    values[23] = a13;
    values[24] = a4;
    values[25] = a54;
    values[26] = a44;
    values[27] = a34;
    values[28] = a24;
    values[29] = a14;
    values[30] = a5;
    values[31] = a55;
    values[32] = a45;
    values[33] = a35;
    values[34] = a25;
    values[35] = a15;
    values[36] = a2;
    values[37] = a52;
    values[38] = a42;
    values[39] = a32;
    values[40] = a22;
    values[41] = a12;
    values[42] = a1;
    values[43] = a51;
    values[44] = a41;
    values[45] = a31;
    values[46] = a21;
    values[47] = a11;
    values[48] = a8;
    values[49] = a58;
    values[50] = a48;
    values[51] = a38;
    values[52] = a28;
    values[53] = a18;
    values[54] = a9;
    values[55] = a59;
    values[56] = a49;
    values[57] = a39;
    values[58] = a29;
    values[59] = a19;
}

/// Size 64 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 64`.
pub fn ntt_64(values: &mut [Field]) {
    assert_eq!(values.len(), 64);
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

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt_fn, *};

    #[test]
    fn test_small_ntt() {
        for size in [
            0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 16, 17, 20, 24, 30, 32, 34, 40, 48, 51, 60, 64,
        ] {
            test_ntt_fn(|values| assert!(ntt(values)), size);
        }
    }

    #[test]
    fn test_ntt_2() {
        test_ntt_fn(ntt_2, 2);
    }

    #[test]
    fn test_ntt_3() {
        test_ntt_fn(ntt_3, 3);
    }

    #[test]
    fn test_ntt_4() {
        test_ntt_fn(ntt_4, 4);
    }

    #[test]
    fn test_ntt_5() {
        test_ntt_fn(ntt_5, 5);
    }

    #[test]
    fn test_ntt_6() {
        test_ntt_fn(ntt_6, 6);
    }

    #[test]
    fn test_ntt_8() {
        test_ntt_fn(ntt_8, 8);
    }

    #[test]
    fn test_ntt_10() {
        test_ntt_fn(ntt_10, 10);
    }

    #[test]
    fn test_ntt_12() {
        test_ntt_fn(ntt_12, 12);
    }

    #[test]
    fn test_ntt_15() {
        test_ntt_fn(ntt_15, 15);
    }

    #[test]
    fn test_ntt_16() {
        test_ntt_fn(ntt_16, 16);
    }

    #[test]
    fn test_ntt_17() {
        test_ntt_fn(ntt_17, 17);
    }

    #[test]
    fn test_ntt_20() {
        test_ntt_fn(ntt_20, 20);
    }

    #[test]
    fn test_ntt_24() {
        test_ntt_fn(ntt_24, 24);
    }

    #[test]
    fn test_ntt_30() {
        test_ntt_fn(ntt_30, 30);
    }

    #[test]
    fn test_ntt_32() {
        test_ntt_fn(ntt_32, 32);
    }

    #[test]
    fn test_ntt_34() {
        test_ntt_fn(ntt_34, 34);
    }

    #[test]
    fn test_ntt_40() {
        test_ntt_fn(ntt_40, 40);
    }

    #[test]
    fn test_ntt_48() {
        test_ntt_fn(ntt_48, 48);
    }

    #[test]
    fn test_ntt_51() {
        test_ntt_fn(ntt_51, 51);
    }

    #[test]
    fn test_ntt_60() {
        test_ntt_fn(ntt_60, 60);
    }

    #[test]
    fn test_ntt_64() {
        test_ntt_fn(ntt_64, 64);
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::{super::bench::bench_ntt, *};
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        bench_ntt(criterion, "small", ntt_2, 2);
        bench_ntt(criterion, "small", ntt_3, 3);
        bench_ntt(criterion, "small", ntt_4, 4);
        bench_ntt(criterion, "small", ntt_5, 5);
        bench_ntt(criterion, "small", ntt_6, 6);
        bench_ntt(criterion, "small", ntt_8, 8);
        bench_ntt(criterion, "small", ntt_10, 10);
        bench_ntt(criterion, "small", ntt_12, 12);
        bench_ntt(criterion, "small", ntt_15, 15);
        bench_ntt(criterion, "small", ntt_16, 16);
        bench_ntt(criterion, "small", ntt_17, 17);
        bench_ntt(criterion, "small", ntt_20, 20);
        bench_ntt(criterion, "small", ntt_24, 24);
        bench_ntt(criterion, "small", ntt_30, 30);
        bench_ntt(criterion, "small", ntt_32, 32);
        bench_ntt(criterion, "small", ntt_34, 34);
        bench_ntt(criterion, "small", ntt_40, 40);
        bench_ntt(criterion, "small", ntt_48, 48);
        bench_ntt(criterion, "small", ntt_51, 51);
        bench_ntt(criterion, "small", ntt_60, 60);
        bench_ntt(criterion, "small", ntt_64, 64);
    }
}
