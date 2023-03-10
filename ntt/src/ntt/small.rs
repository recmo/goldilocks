//! Generated using `cargo run --bin codegen`
#![allow(
    unused_parens,
    clippy::similar_names,
    clippy::unreadable_literal,
    clippy::too_many_lines
)]
use super::{Ntt, NttFn};
use crate::Field;
use std::sync::Arc;

pub fn ntt(size: usize) -> Option<Arc<dyn Ntt>> {
    let f = match size {
        0 | 1 => ntt_01,
        2 => ntt_2,
        3 => ntt_3,
        4 => ntt_4,
        5 => ntt_5,
        6 => ntt_6,
        8 => ntt_8,
        10 => ntt_10,
        12 => ntt_12,
        15 => ntt_15,
        16 => ntt_16,
        17 => ntt_17,
        20 => ntt_20,
        24 => ntt_24,
        30 => ntt_30,
        32 => ntt_32,
        34 => ntt_34,
        40 => ntt_40,
        48 => ntt_48,
        51 => ntt_51,
        60 => ntt_60,
        64 => ntt_64,
        68 => ntt_68,
        80 => ntt_80,
        85 => ntt_85,
        96 => ntt_96,
        102 => ntt_102,
        120 => ntt_120,
        128 => ntt_128,
        _ => return None,
    };
    Some(Arc::new(NttFn::new(size, f)))
}

pub fn ntt_01(_values: &mut [Field]) {}

/// Size 2 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 2`.
pub fn ntt_2(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 2, 0);
    for values in values.chunks_exact_mut(2) {
        let a0 = values[0];
        let a1 = values[1];
        let (a0, a1) = (a0 + a1, a0 - a1);
        values[0] = a0;
        values[1] = a1;
    }
}

/// Size 3 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 3`.
pub fn ntt_3(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 3, 0);
    for values in values.chunks_exact_mut(3) {
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
}

/// Size 4 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 4`.
pub fn ntt_4(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 4, 0);
    for values in values.chunks_exact_mut(4) {
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
}

/// Size 5 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 5`.
pub fn ntt_5(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 5, 0);
    for values in values.chunks_exact_mut(5) {
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
}

/// Size 6 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 6`.
pub fn ntt_6(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 6, 0);
    for values in values.chunks_exact_mut(6) {
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
}

/// Size 8 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 8`.
pub fn ntt_8(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 8, 0);
    for values in values.chunks_exact_mut(8) {
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
}

/// Size 10 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 10`.
pub fn ntt_10(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 10, 0);
    for values in values.chunks_exact_mut(10) {
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
}

/// Size 12 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 12`.
pub fn ntt_12(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 12, 0);
    for values in values.chunks_exact_mut(12) {
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
}

/// Size 15 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 15`.
pub fn ntt_15(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 15, 0);
    for values in values.chunks_exact_mut(15) {
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
}

/// Size 16 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 16`.
pub fn ntt_16(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 16, 0);
    for values in values.chunks_exact_mut(16) {
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
}

/// Size 17 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 17`.
pub fn ntt_17(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 17, 0);
    for values in values.chunks_exact_mut(17) {
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
}

/// Size 20 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 20`.
pub fn ntt_20(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 20, 0);
    for values in values.chunks_exact_mut(20) {
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
}

/// Size 24 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 24`.
pub fn ntt_24(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 24, 0);
    for values in values.chunks_exact_mut(24) {
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
}

/// Size 30 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 30`.
pub fn ntt_30(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 30, 0);
    for values in values.chunks_exact_mut(30) {
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
}

/// Size 32 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 32`.
pub fn ntt_32(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 32, 0);
    for values in values.chunks_exact_mut(32) {
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
}

/// Size 34 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 34`.
pub fn ntt_34(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 34, 0);
    for values in values.chunks_exact_mut(34) {
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
}

/// Size 40 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 40`.
pub fn ntt_40(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 40, 0);
    for values in values.chunks_exact_mut(40) {
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
}

/// Size 48 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 48`.
pub fn ntt_48(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 48, 0);
    for values in values.chunks_exact_mut(48) {
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
}

/// Size 51 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 51`.
pub fn ntt_51(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 51, 0);
    for values in values.chunks_exact_mut(51) {
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
}

/// Size 60 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 60`.
pub fn ntt_60(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 60, 0);
    for values in values.chunks_exact_mut(60) {
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
}

/// Size 64 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 64`.
pub fn ntt_64(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 64, 0);
    for values in values.chunks_exact_mut(64) {
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
}

/// Size 68 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 68`.
pub fn ntt_68(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 68, 0);
    for values in values.chunks_exact_mut(68) {
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
        let (a4, a64) = (a4 + a64, a4 - a64);
        let (a16, a52) = (a16 + a52, a16 - a52);
        let a52 = (a52 << 48);
        let (a4, a16) = (a4 + a16, a4 - a16);
        let (a64, a52) = (a64 + a52, a64 - a52);
        let (a24, a44) = (a24 + a44, a24 - a44);
        let (a28, a40) = (a28 + a40, a28 - a40);
        let a40 = (a40 << 48);
        let (a24, a28) = (a24 + a28, a24 - a28);
        let (a44, a40) = (a44 + a40, a44 - a40);
        let (a8, a60) = (a8 + a60, a8 - a60);
        let (a32, a36) = (a32 + a36, a32 - a36);
        let a36 = (a36 << 48);
        let (a8, a32) = (a8 + a32, a8 - a32);
        let (a60, a36) = (a60 + a36, a60 - a36);
        let (a48, a20) = (a48 + a20, a48 - a20);
        let (a56, a12) = (a56 + a12, a56 - a12);
        let a12 = (a12 << 48);
        let (a48, a56) = (a48 + a56, a48 - a56);
        let (a20, a12) = (a20 + a12, a20 - a12);
        let a44 = (a44 << 12);
        let a28 = (a28 << 24);
        let a40 = (a40 << 36);
        let a60 = (a60 << 24);
        let a32 = (a32 << 48);
        let a36 = (a36 << 72);
        let a20 = (a20 << 36);
        let a56 = (a56 << 72);
        let a12 = (-(a12 << 12));
        let (a4, a8) = (a4 + a8, a4 - a8);
        let (a24, a48) = (a24 + a48, a24 - a48);
        let a48 = (a48 << 48);
        let (a4, a24) = (a4 + a24, a4 - a24);
        let (a8, a48) = (a8 + a48, a8 - a48);
        let (a64, a60) = (a64 + a60, a64 - a60);
        let (a44, a20) = (a44 + a20, a44 - a20);
        let a20 = (a20 << 48);
        let (a64, a44) = (a64 + a44, a64 - a44);
        let (a60, a20) = (a60 + a20, a60 - a20);
        let (a16, a32) = (a16 + a32, a16 - a32);
        let (a28, a56) = (a28 + a56, a28 - a56);
        let a56 = (a56 << 48);
        let (a16, a28) = (a16 + a28, a16 - a28);
        let (a32, a56) = (a32 + a56, a32 - a56);
        let (a52, a36) = (a52 + a36, a52 - a36);
        let (a40, a12) = (a40 + a12, a40 - a12);
        let a12 = (a12 << 48);
        let (a52, a40) = (a52 + a40, a52 - a40);
        let (a36, a12) = (a36 + a12, a36 - a12);
        let t = a0;
        let a0 = a0 + a4;
        let a4 = a4 * Field::new(1152921504338411520);
        let a64 = a64 * Field::new(6259776822214049175);
        let a16 = a16 * Field::new(9380094172986290191);
        let a52 = a52 * Field::new(891943630314919127);
        let a8 = a8 * Field::new(17228171707553225791);
        let a60 = a60 * Field::new(12855743360534130886);
        let a32 = a32 * Field::new(6167687396920564837);
        let a36 = a36 * Field::new(17201834061724655524);
        let a24 = a24 * Field::new(15308299771656910737);
        let a44 = a44 * Field::new(18186005861103657533);
        let a28 = a28 * Field::new(53595491891823545);
        let a40 = a40 * Field::new(1906638201581172103);
        let a48 = a48 * Field::new(18303651001328874822);
        let a20 = a20 * Field::new(3077466521755967626);
        let a56 = a56 * Field::new(12423593102987598328);
        let a12 = a12 * Field::new(18361513053649472048);
        let a4 = a4 + t;
        let (a4, a24) = (a4 + a24, a4 - a24);
        let (a48, a8) = (a48 + a8, a48 - a8);
        let a8 = (a8 << 48);
        let (a4, a48) = (a4 + a48, a4 - a48);
        let (a24, a8) = (a24 + a8, a24 - a8);
        let (a12, a36) = (a12 + a36, a12 - a36);
        let (a40, a52) = (a40 + a52, a40 - a52);
        let a52 = (a52 << 48);
        let (a12, a40) = (a12 + a40, a12 - a40);
        let (a36, a52) = (a36 + a52, a36 - a52);
        let (a56, a32) = (a56 + a32, a56 - a32);
        let (a28, a16) = (a28 + a16, a28 - a16);
        let a16 = (a16 << 48);
        let (a56, a28) = (a56 + a28, a56 - a28);
        let (a32, a16) = (a32 + a16, a32 - a16);
        let (a20, a60) = (a20 + a60, a20 - a60);
        let (a44, a64) = (a44 + a64, a44 - a64);
        let a64 = (a64 << 48);
        let (a20, a44) = (a20 + a44, a20 - a44);
        let (a60, a64) = (a60 + a64, a60 - a64);
        let a36 = (a36 << 12);
        let a40 = (a40 << 24);
        let a52 = (a52 << 36);
        let a32 = (a32 << 24);
        let a28 = (a28 << 48);
        let a16 = (a16 << 72);
        let a60 = (a60 << 36);
        let a44 = (a44 << 72);
        let a64 = (-(a64 << 12));
        let (a4, a56) = (a4 + a56, a4 - a56);
        let (a12, a20) = (a12 + a20, a12 - a20);
        let a20 = (a20 << 48);
        let (a4, a12) = (a4 + a12, a4 - a12);
        let (a56, a20) = (a56 + a20, a56 - a20);
        let (a24, a32) = (a24 + a32, a24 - a32);
        let (a36, a60) = (a36 + a60, a36 - a60);
        let a60 = (a60 << 48);
        let (a24, a36) = (a24 + a36, a24 - a36);
        let (a32, a60) = (a32 + a60, a32 - a60);
        let (a48, a28) = (a48 + a28, a48 - a28);
        let (a40, a44) = (a40 + a44, a40 - a44);
        let a44 = (a44 << 48);
        let (a48, a40) = (a48 + a40, a48 - a40);
        let (a28, a44) = (a28 + a44, a28 - a44);
        let (a8, a16) = (a8 + a16, a8 - a16);
        let (a52, a64) = (a52 + a64, a52 - a64);
        let a64 = (a64 << 48);
        let (a8, a52) = (a8 + a52, a8 - a52);
        let (a16, a64) = (a16 + a64, a16 - a64);
        let (a21, a13) = (a21 + a13, a21 - a13);
        let (a33, a1) = (a33 + a1, a33 - a1);
        let a1 = (a1 << 48);
        let (a21, a33) = (a21 + a33, a21 - a33);
        let (a13, a1) = (a13 + a1, a13 - a1);
        let (a41, a61) = (a41 + a61, a41 - a61);
        let (a45, a57) = (a45 + a57, a45 - a57);
        let a57 = (a57 << 48);
        let (a41, a45) = (a41 + a45, a41 - a45);
        let (a61, a57) = (a61 + a57, a61 - a57);
        let (a25, a9) = (a25 + a9, a25 - a9);
        let (a49, a53) = (a49 + a53, a49 - a53);
        let a53 = (a53 << 48);
        let (a25, a49) = (a25 + a49, a25 - a49);
        let (a9, a53) = (a9 + a53, a9 - a53);
        let (a65, a37) = (a65 + a37, a65 - a37);
        let (a5, a29) = (a5 + a29, a5 - a29);
        let a29 = (a29 << 48);
        let (a65, a5) = (a65 + a5, a65 - a5);
        let (a37, a29) = (a37 + a29, a37 - a29);
        let a61 = (a61 << 12);
        let a45 = (a45 << 24);
        let a57 = (a57 << 36);
        let a9 = (a9 << 24);
        let a49 = (a49 << 48);
        let a53 = (a53 << 72);
        let a37 = (a37 << 36);
        let a5 = (a5 << 72);
        let a29 = (-(a29 << 12));
        let (a21, a25) = (a21 + a25, a21 - a25);
        let (a41, a65) = (a41 + a65, a41 - a65);
        let a65 = (a65 << 48);
        let (a21, a41) = (a21 + a41, a21 - a41);
        let (a25, a65) = (a25 + a65, a25 - a65);
        let (a13, a9) = (a13 + a9, a13 - a9);
        let (a61, a37) = (a61 + a37, a61 - a37);
        let a37 = (a37 << 48);
        let (a13, a61) = (a13 + a61, a13 - a61);
        let (a9, a37) = (a9 + a37, a9 - a37);
        let (a33, a49) = (a33 + a49, a33 - a49);
        let (a45, a5) = (a45 + a5, a45 - a5);
        let a5 = (a5 << 48);
        let (a33, a45) = (a33 + a45, a33 - a45);
        let (a49, a5) = (a49 + a5, a49 - a5);
        let (a1, a53) = (a1 + a53, a1 - a53);
        let (a57, a29) = (a57 + a29, a57 - a29);
        let a29 = (a29 << 48);
        let (a1, a57) = (a1 + a57, a1 - a57);
        let (a53, a29) = (a53 + a29, a53 - a29);
        let t = a17;
        let a17 = a17 + a21;
        let a21 = a21 * Field::new(1152921504338411520);
        let a13 = a13 * Field::new(6259776822214049175);
        let a33 = a33 * Field::new(9380094172986290191);
        let a1 = a1 * Field::new(891943630314919127);
        let a25 = a25 * Field::new(17228171707553225791);
        let a9 = a9 * Field::new(12855743360534130886);
        let a49 = a49 * Field::new(6167687396920564837);
        let a53 = a53 * Field::new(17201834061724655524);
        let a41 = a41 * Field::new(15308299771656910737);
        let a61 = a61 * Field::new(18186005861103657533);
        let a45 = a45 * Field::new(53595491891823545);
        let a57 = a57 * Field::new(1906638201581172103);
        let a65 = a65 * Field::new(18303651001328874822);
        let a37 = a37 * Field::new(3077466521755967626);
        let a5 = a5 * Field::new(12423593102987598328);
        let a29 = a29 * Field::new(18361513053649472048);
        let a21 = a21 + t;
        let (a21, a41) = (a21 + a41, a21 - a41);
        let (a65, a25) = (a65 + a25, a65 - a25);
        let a25 = (a25 << 48);
        let (a21, a65) = (a21 + a65, a21 - a65);
        let (a41, a25) = (a41 + a25, a41 - a25);
        let (a29, a53) = (a29 + a53, a29 - a53);
        let (a57, a1) = (a57 + a1, a57 - a1);
        let a1 = (a1 << 48);
        let (a29, a57) = (a29 + a57, a29 - a57);
        let (a53, a1) = (a53 + a1, a53 - a1);
        let (a5, a49) = (a5 + a49, a5 - a49);
        let (a45, a33) = (a45 + a33, a45 - a33);
        let a33 = (a33 << 48);
        let (a5, a45) = (a5 + a45, a5 - a45);
        let (a49, a33) = (a49 + a33, a49 - a33);
        let (a37, a9) = (a37 + a9, a37 - a9);
        let (a61, a13) = (a61 + a13, a61 - a13);
        let a13 = (a13 << 48);
        let (a37, a61) = (a37 + a61, a37 - a61);
        let (a9, a13) = (a9 + a13, a9 - a13);
        let a53 = (a53 << 12);
        let a57 = (a57 << 24);
        let a1 = (a1 << 36);
        let a49 = (a49 << 24);
        let a45 = (a45 << 48);
        let a33 = (a33 << 72);
        let a9 = (a9 << 36);
        let a61 = (a61 << 72);
        let a13 = (-(a13 << 12));
        let (a21, a5) = (a21 + a5, a21 - a5);
        let (a29, a37) = (a29 + a37, a29 - a37);
        let a37 = (a37 << 48);
        let (a21, a29) = (a21 + a29, a21 - a29);
        let (a5, a37) = (a5 + a37, a5 - a37);
        let (a41, a49) = (a41 + a49, a41 - a49);
        let (a53, a9) = (a53 + a9, a53 - a9);
        let a9 = (a9 << 48);
        let (a41, a53) = (a41 + a53, a41 - a53);
        let (a49, a9) = (a49 + a9, a49 - a9);
        let (a65, a45) = (a65 + a45, a65 - a45);
        let (a57, a61) = (a57 + a61, a57 - a61);
        let a61 = (a61 << 48);
        let (a65, a57) = (a65 + a57, a65 - a57);
        let (a45, a61) = (a45 + a61, a45 - a61);
        let (a25, a33) = (a25 + a33, a25 - a33);
        let (a1, a13) = (a1 + a13, a1 - a13);
        let a13 = (a13 << 48);
        let (a25, a1) = (a25 + a1, a25 - a1);
        let (a33, a13) = (a33 + a13, a33 - a13);
        let (a38, a30) = (a38 + a30, a38 - a30);
        let (a50, a18) = (a50 + a18, a50 - a18);
        let a18 = (a18 << 48);
        let (a38, a50) = (a38 + a50, a38 - a50);
        let (a30, a18) = (a30 + a18, a30 - a18);
        let (a58, a10) = (a58 + a10, a58 - a10);
        let (a62, a6) = (a62 + a6, a62 - a6);
        let a6 = (a6 << 48);
        let (a58, a62) = (a58 + a62, a58 - a62);
        let (a10, a6) = (a10 + a6, a10 - a6);
        let (a42, a26) = (a42 + a26, a42 - a26);
        let (a66, a2) = (a66 + a2, a66 - a2);
        let a2 = (a2 << 48);
        let (a42, a66) = (a42 + a66, a42 - a66);
        let (a26, a2) = (a26 + a2, a26 - a2);
        let (a14, a54) = (a14 + a54, a14 - a54);
        let (a22, a46) = (a22 + a46, a22 - a46);
        let a46 = (a46 << 48);
        let (a14, a22) = (a14 + a22, a14 - a22);
        let (a54, a46) = (a54 + a46, a54 - a46);
        let a10 = (a10 << 12);
        let a62 = (a62 << 24);
        let a6 = (a6 << 36);
        let a26 = (a26 << 24);
        let a66 = (a66 << 48);
        let a2 = (a2 << 72);
        let a54 = (a54 << 36);
        let a22 = (a22 << 72);
        let a46 = (-(a46 << 12));
        let (a38, a42) = (a38 + a42, a38 - a42);
        let (a58, a14) = (a58 + a14, a58 - a14);
        let a14 = (a14 << 48);
        let (a38, a58) = (a38 + a58, a38 - a58);
        let (a42, a14) = (a42 + a14, a42 - a14);
        let (a30, a26) = (a30 + a26, a30 - a26);
        let (a10, a54) = (a10 + a54, a10 - a54);
        let a54 = (a54 << 48);
        let (a30, a10) = (a30 + a10, a30 - a10);
        let (a26, a54) = (a26 + a54, a26 - a54);
        let (a50, a66) = (a50 + a66, a50 - a66);
        let (a62, a22) = (a62 + a22, a62 - a22);
        let a22 = (a22 << 48);
        let (a50, a62) = (a50 + a62, a50 - a62);
        let (a66, a22) = (a66 + a22, a66 - a22);
        let (a18, a2) = (a18 + a2, a18 - a2);
        let (a6, a46) = (a6 + a46, a6 - a46);
        let a46 = (a46 << 48);
        let (a18, a6) = (a18 + a6, a18 - a6);
        let (a2, a46) = (a2 + a46, a2 - a46);
        let t = a34;
        let a34 = a34 + a38;
        let a38 = a38 * Field::new(1152921504338411520);
        let a30 = a30 * Field::new(6259776822214049175);
        let a50 = a50 * Field::new(9380094172986290191);
        let a18 = a18 * Field::new(891943630314919127);
        let a42 = a42 * Field::new(17228171707553225791);
        let a26 = a26 * Field::new(12855743360534130886);
        let a66 = a66 * Field::new(6167687396920564837);
        let a2 = a2 * Field::new(17201834061724655524);
        let a58 = a58 * Field::new(15308299771656910737);
        let a10 = a10 * Field::new(18186005861103657533);
        let a62 = a62 * Field::new(53595491891823545);
        let a6 = a6 * Field::new(1906638201581172103);
        let a14 = a14 * Field::new(18303651001328874822);
        let a54 = a54 * Field::new(3077466521755967626);
        let a22 = a22 * Field::new(12423593102987598328);
        let a46 = a46 * Field::new(18361513053649472048);
        let a38 = a38 + t;
        let (a38, a58) = (a38 + a58, a38 - a58);
        let (a14, a42) = (a14 + a42, a14 - a42);
        let a42 = (a42 << 48);
        let (a38, a14) = (a38 + a14, a38 - a14);
        let (a58, a42) = (a58 + a42, a58 - a42);
        let (a46, a2) = (a46 + a2, a46 - a2);
        let (a6, a18) = (a6 + a18, a6 - a18);
        let a18 = (a18 << 48);
        let (a46, a6) = (a46 + a6, a46 - a6);
        let (a2, a18) = (a2 + a18, a2 - a18);
        let (a22, a66) = (a22 + a66, a22 - a66);
        let (a62, a50) = (a62 + a50, a62 - a50);
        let a50 = (a50 << 48);
        let (a22, a62) = (a22 + a62, a22 - a62);
        let (a66, a50) = (a66 + a50, a66 - a50);
        let (a54, a26) = (a54 + a26, a54 - a26);
        let (a10, a30) = (a10 + a30, a10 - a30);
        let a30 = (a30 << 48);
        let (a54, a10) = (a54 + a10, a54 - a10);
        let (a26, a30) = (a26 + a30, a26 - a30);
        let a2 = (a2 << 12);
        let a6 = (a6 << 24);
        let a18 = (a18 << 36);
        let a66 = (a66 << 24);
        let a62 = (a62 << 48);
        let a50 = (a50 << 72);
        let a26 = (a26 << 36);
        let a10 = (a10 << 72);
        let a30 = (-(a30 << 12));
        let (a38, a22) = (a38 + a22, a38 - a22);
        let (a46, a54) = (a46 + a54, a46 - a54);
        let a54 = (a54 << 48);
        let (a38, a46) = (a38 + a46, a38 - a46);
        let (a22, a54) = (a22 + a54, a22 - a54);
        let (a58, a66) = (a58 + a66, a58 - a66);
        let (a2, a26) = (a2 + a26, a2 - a26);
        let a26 = (a26 << 48);
        let (a58, a2) = (a58 + a2, a58 - a2);
        let (a66, a26) = (a66 + a26, a66 - a26);
        let (a14, a62) = (a14 + a62, a14 - a62);
        let (a6, a10) = (a6 + a10, a6 - a10);
        let a10 = (a10 << 48);
        let (a14, a6) = (a14 + a6, a14 - a6);
        let (a62, a10) = (a62 + a10, a62 - a10);
        let (a42, a50) = (a42 + a50, a42 - a50);
        let (a18, a30) = (a18 + a30, a18 - a30);
        let a30 = (a30 << 48);
        let (a42, a18) = (a42 + a18, a42 - a18);
        let (a50, a30) = (a50 + a30, a50 - a30);
        let (a55, a47) = (a55 + a47, a55 - a47);
        let (a67, a35) = (a67 + a35, a67 - a35);
        let a35 = (a35 << 48);
        let (a55, a67) = (a55 + a67, a55 - a67);
        let (a47, a35) = (a47 + a35, a47 - a35);
        let (a7, a27) = (a7 + a27, a7 - a27);
        let (a11, a23) = (a11 + a23, a11 - a23);
        let a23 = (a23 << 48);
        let (a7, a11) = (a7 + a11, a7 - a11);
        let (a27, a23) = (a27 + a23, a27 - a23);
        let (a59, a43) = (a59 + a43, a59 - a43);
        let (a15, a19) = (a15 + a19, a15 - a19);
        let a19 = (a19 << 48);
        let (a59, a15) = (a59 + a15, a59 - a15);
        let (a43, a19) = (a43 + a19, a43 - a19);
        let (a31, a3) = (a31 + a3, a31 - a3);
        let (a39, a63) = (a39 + a63, a39 - a63);
        let a63 = (a63 << 48);
        let (a31, a39) = (a31 + a39, a31 - a39);
        let (a3, a63) = (a3 + a63, a3 - a63);
        let a27 = (a27 << 12);
        let a11 = (a11 << 24);
        let a23 = (a23 << 36);
        let a43 = (a43 << 24);
        let a15 = (a15 << 48);
        let a19 = (a19 << 72);
        let a3 = (a3 << 36);
        let a39 = (a39 << 72);
        let a63 = (-(a63 << 12));
        let (a55, a59) = (a55 + a59, a55 - a59);
        let (a7, a31) = (a7 + a31, a7 - a31);
        let a31 = (a31 << 48);
        let (a55, a7) = (a55 + a7, a55 - a7);
        let (a59, a31) = (a59 + a31, a59 - a31);
        let (a47, a43) = (a47 + a43, a47 - a43);
        let (a27, a3) = (a27 + a3, a27 - a3);
        let a3 = (a3 << 48);
        let (a47, a27) = (a47 + a27, a47 - a27);
        let (a43, a3) = (a43 + a3, a43 - a3);
        let (a67, a15) = (a67 + a15, a67 - a15);
        let (a11, a39) = (a11 + a39, a11 - a39);
        let a39 = (a39 << 48);
        let (a67, a11) = (a67 + a11, a67 - a11);
        let (a15, a39) = (a15 + a39, a15 - a39);
        let (a35, a19) = (a35 + a19, a35 - a19);
        let (a23, a63) = (a23 + a63, a23 - a63);
        let a63 = (a63 << 48);
        let (a35, a23) = (a35 + a23, a35 - a23);
        let (a19, a63) = (a19 + a63, a19 - a63);
        let t = a51;
        let a51 = a51 + a55;
        let a55 = a55 * Field::new(1152921504338411520);
        let a47 = a47 * Field::new(6259776822214049175);
        let a67 = a67 * Field::new(9380094172986290191);
        let a35 = a35 * Field::new(891943630314919127);
        let a59 = a59 * Field::new(17228171707553225791);
        let a43 = a43 * Field::new(12855743360534130886);
        let a15 = a15 * Field::new(6167687396920564837);
        let a19 = a19 * Field::new(17201834061724655524);
        let a7 = a7 * Field::new(15308299771656910737);
        let a27 = a27 * Field::new(18186005861103657533);
        let a11 = a11 * Field::new(53595491891823545);
        let a23 = a23 * Field::new(1906638201581172103);
        let a31 = a31 * Field::new(18303651001328874822);
        let a3 = a3 * Field::new(3077466521755967626);
        let a39 = a39 * Field::new(12423593102987598328);
        let a63 = a63 * Field::new(18361513053649472048);
        let a55 = a55 + t;
        let (a55, a7) = (a55 + a7, a55 - a7);
        let (a31, a59) = (a31 + a59, a31 - a59);
        let a59 = (a59 << 48);
        let (a55, a31) = (a55 + a31, a55 - a31);
        let (a7, a59) = (a7 + a59, a7 - a59);
        let (a63, a19) = (a63 + a19, a63 - a19);
        let (a23, a35) = (a23 + a35, a23 - a35);
        let a35 = (a35 << 48);
        let (a63, a23) = (a63 + a23, a63 - a23);
        let (a19, a35) = (a19 + a35, a19 - a35);
        let (a39, a15) = (a39 + a15, a39 - a15);
        let (a11, a67) = (a11 + a67, a11 - a67);
        let a67 = (a67 << 48);
        let (a39, a11) = (a39 + a11, a39 - a11);
        let (a15, a67) = (a15 + a67, a15 - a67);
        let (a3, a43) = (a3 + a43, a3 - a43);
        let (a27, a47) = (a27 + a47, a27 - a47);
        let a47 = (a47 << 48);
        let (a3, a27) = (a3 + a27, a3 - a27);
        let (a43, a47) = (a43 + a47, a43 - a47);
        let a19 = (a19 << 12);
        let a23 = (a23 << 24);
        let a35 = (a35 << 36);
        let a15 = (a15 << 24);
        let a11 = (a11 << 48);
        let a67 = (a67 << 72);
        let a43 = (a43 << 36);
        let a27 = (a27 << 72);
        let a47 = (-(a47 << 12));
        let (a55, a39) = (a55 + a39, a55 - a39);
        let (a63, a3) = (a63 + a3, a63 - a3);
        let a3 = (a3 << 48);
        let (a55, a63) = (a55 + a63, a55 - a63);
        let (a39, a3) = (a39 + a3, a39 - a3);
        let (a7, a15) = (a7 + a15, a7 - a15);
        let (a19, a43) = (a19 + a43, a19 - a43);
        let a43 = (a43 << 48);
        let (a7, a19) = (a7 + a19, a7 - a19);
        let (a15, a43) = (a15 + a43, a15 - a43);
        let (a31, a11) = (a31 + a11, a31 - a11);
        let (a23, a27) = (a23 + a27, a23 - a27);
        let a27 = (a27 << 48);
        let (a31, a23) = (a31 + a23, a31 - a23);
        let (a11, a27) = (a11 + a27, a11 - a27);
        let (a59, a67) = (a59 + a67, a59 - a67);
        let (a35, a47) = (a35 + a47, a35 - a47);
        let a47 = (a47 << 48);
        let (a59, a35) = (a59 + a35, a59 - a35);
        let (a67, a47) = (a67 + a47, a67 - a47);
        let (a0, a34) = (a0 + a34, a0 - a34);
        let (a17, a51) = (a17 + a51, a17 - a51);
        let a51 = (a51 << 48);
        let (a0, a17) = (a0 + a17, a0 - a17);
        let (a34, a51) = (a34 + a51, a34 - a51);
        let (a4, a38) = (a4 + a38, a4 - a38);
        let (a21, a55) = (a21 + a55, a21 - a55);
        let a55 = (a55 << 48);
        let (a4, a21) = (a4 + a21, a4 - a21);
        let (a38, a55) = (a38 + a55, a38 - a55);
        let (a44, a10) = (a44 + a10, a44 - a10);
        let (a61, a27) = (a61 + a27, a61 - a27);
        let a27 = (a27 << 48);
        let (a44, a61) = (a44 + a61, a44 - a61);
        let (a10, a27) = (a10 + a27, a10 - a27);
        let (a24, a58) = (a24 + a58, a24 - a58);
        let (a41, a7) = (a41 + a7, a41 - a7);
        let a7 = (a7 << 48);
        let (a24, a41) = (a24 + a41, a24 - a41);
        let (a58, a7) = (a58 + a7, a58 - a7);
        let (a20, a54) = (a20 + a54, a20 - a54);
        let (a37, a3) = (a37 + a3, a37 - a3);
        let a3 = (a3 << 48);
        let (a20, a37) = (a20 + a37, a20 - a37);
        let (a54, a3) = (a54 + a3, a54 - a3);
        let (a32, a66) = (a32 + a66, a32 - a66);
        let (a49, a15) = (a49 + a15, a49 - a15);
        let a15 = (a15 << 48);
        let (a32, a49) = (a32 + a49, a32 - a49);
        let (a66, a15) = (a66 + a15, a66 - a15);
        let (a64, a30) = (a64 + a30, a64 - a30);
        let (a13, a47) = (a13 + a47, a13 - a47);
        let a47 = (a47 << 48);
        let (a64, a13) = (a64 + a13, a64 - a13);
        let (a30, a47) = (a30 + a47, a30 - a47);
        let (a52, a18) = (a52 + a18, a52 - a18);
        let (a1, a35) = (a1 + a35, a1 - a35);
        let a35 = (a35 << 48);
        let (a52, a1) = (a52 + a1, a52 - a1);
        let (a18, a35) = (a18 + a35, a18 - a35);
        let (a40, a6) = (a40 + a6, a40 - a6);
        let (a57, a23) = (a57 + a23, a57 - a23);
        let a23 = (a23 << 48);
        let (a40, a57) = (a40 + a57, a40 - a57);
        let (a6, a23) = (a6 + a23, a6 - a23);
        let (a48, a14) = (a48 + a14, a48 - a14);
        let (a65, a31) = (a65 + a31, a65 - a31);
        let a31 = (a31 << 48);
        let (a48, a65) = (a48 + a65, a48 - a65);
        let (a14, a31) = (a14 + a31, a14 - a31);
        let (a8, a42) = (a8 + a42, a8 - a42);
        let (a25, a59) = (a25 + a59, a25 - a59);
        let a59 = (a59 << 48);
        let (a8, a25) = (a8 + a25, a8 - a25);
        let (a42, a59) = (a42 + a59, a42 - a59);
        let (a16, a50) = (a16 + a50, a16 - a50);
        let (a33, a67) = (a33 + a67, a33 - a67);
        let a67 = (a67 << 48);
        let (a16, a33) = (a16 + a33, a16 - a33);
        let (a50, a67) = (a50 + a67, a50 - a67);
        let (a60, a26) = (a60 + a26, a60 - a26);
        let (a9, a43) = (a9 + a43, a9 - a43);
        let a43 = (a43 << 48);
        let (a60, a9) = (a60 + a9, a60 - a9);
        let (a26, a43) = (a26 + a43, a26 - a43);
        let (a56, a22) = (a56 + a22, a56 - a22);
        let (a5, a39) = (a5 + a39, a5 - a39);
        let a39 = (a39 << 48);
        let (a56, a5) = (a56 + a5, a56 - a5);
        let (a22, a39) = (a22 + a39, a22 - a39);
        let (a36, a2) = (a36 + a2, a36 - a2);
        let (a53, a19) = (a53 + a19, a53 - a19);
        let a19 = (a19 << 48);
        let (a36, a53) = (a36 + a53, a36 - a53);
        let (a2, a19) = (a2 + a19, a2 - a19);
        let (a28, a62) = (a28 + a62, a28 - a62);
        let (a45, a11) = (a45 + a11, a45 - a11);
        let a11 = (a11 << 48);
        let (a28, a45) = (a28 + a45, a28 - a45);
        let (a62, a11) = (a62 + a11, a62 - a11);
        let (a12, a46) = (a12 + a46, a12 - a46);
        let (a29, a63) = (a29 + a63, a29 - a63);
        let a63 = (a63 << 48);
        let (a12, a29) = (a12 + a29, a12 - a29);
        let (a46, a63) = (a46 + a63, a46 - a63);
        values[0] = a0;
        values[1] = a38;
        values[2] = a61;
        values[3] = a7;
        values[4] = a20;
        values[5] = a66;
        values[6] = a13;
        values[7] = a35;
        values[8] = a40;
        values[9] = a14;
        values[10] = a25;
        values[11] = a67;
        values[12] = a60;
        values[13] = a22;
        values[14] = a53;
        values[15] = a11;
        values[16] = a12;
        values[17] = a34;
        values[18] = a21;
        values[19] = a27;
        values[20] = a24;
        values[21] = a54;
        values[22] = a49;
        values[23] = a47;
        values[24] = a52;
        values[25] = a6;
        values[26] = a65;
        values[27] = a59;
        values[28] = a16;
        values[29] = a26;
        values[30] = a5;
        values[31] = a19;
        values[32] = a28;
        values[33] = a46;
        values[34] = a17;
        values[35] = a55;
        values[36] = a44;
        values[37] = a58;
        values[38] = a37;
        values[39] = a15;
        values[40] = a64;
        values[41] = a18;
        values[42] = a57;
        values[43] = a31;
        values[44] = a8;
        values[45] = a50;
        values[46] = a9;
        values[47] = a39;
        values[48] = a36;
        values[49] = a62;
        values[50] = a29;
        values[51] = a51;
        values[52] = a4;
        values[53] = a10;
        values[54] = a41;
        values[55] = a3;
        values[56] = a32;
        values[57] = a30;
        values[58] = a1;
        values[59] = a23;
        values[60] = a48;
        values[61] = a42;
        values[62] = a33;
        values[63] = a43;
        values[64] = a56;
        values[65] = a2;
        values[66] = a45;
        values[67] = a63;
    }
}

/// Size 80 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 80`.
pub fn ntt_80(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 80, 0);
    for values in values.chunks_exact_mut(80) {
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
        let (a62, a68) = (a62 + a68, a62 - a68);
        let (a66, a64) = (a66 + a64, a66 - a64);
        let a64 = (a64 << 48);
        let (a62, a66) = (a62 + a66, a62 - a66);
        let (a68, a64) = (a68 + a64, a68 - a64);
        let t = a60;
        let a60 = a60 + a62;
        let a62 = a62 * Field::new(4611686017353646080);
        let a68 = a68 * Field::new(16181989089180173841);
        let a66 = a66 * Field::new(5818851782451133869);
        let a64 = a64 * Field::new(11322249509082494407);
        let a62 = a62 + t;
        let (a62, a66) = (a62 + a66, a62 - a66);
        let (a64, a68) = (a64 + a68, a64 - a68);
        let a68 = (a68 << 48);
        let (a62, a64) = (a62 + a64, a62 - a64);
        let (a66, a68) = (a66 + a68, a66 - a68);
        let (a67, a63) = (a67 + a63, a67 - a63);
        let (a61, a69) = (a61 + a69, a61 - a69);
        let a69 = (a69 << 48);
        let (a67, a61) = (a67 + a61, a67 - a61);
        let (a63, a69) = (a63 + a69, a63 - a69);
        let t = a65;
        let a65 = a65 + a67;
        let a67 = a67 * Field::new(4611686017353646080);
        let a63 = a63 * Field::new(16181989089180173841);
        let a61 = a61 * Field::new(5818851782451133869);
        let a69 = a69 * Field::new(11322249509082494407);
        let a67 = a67 + t;
        let (a67, a61) = (a67 + a61, a67 - a61);
        let (a69, a63) = (a69 + a63, a69 - a63);
        let a63 = (a63 << 48);
        let (a67, a69) = (a67 + a69, a67 - a69);
        let (a61, a63) = (a61 + a63, a61 - a63);
        let (a60, a65) = (a60 + a65, a60 - a65);
        let (a62, a67) = (a62 + a67, a62 - a67);
        let (a66, a61) = (a66 + a61, a66 - a61);
        let (a68, a63) = (a68 + a63, a68 - a63);
        let (a64, a69) = (a64 + a69, a64 - a69);
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
        let (a72, a78) = (a72 + a78, a72 - a78);
        let (a76, a74) = (a76 + a74, a76 - a74);
        let a74 = (a74 << 48);
        let (a72, a76) = (a72 + a76, a72 - a76);
        let (a78, a74) = (a78 + a74, a78 - a74);
        let t = a70;
        let a70 = a70 + a72;
        let a72 = a72 * Field::new(4611686017353646080);
        let a78 = a78 * Field::new(16181989089180173841);
        let a76 = a76 * Field::new(5818851782451133869);
        let a74 = a74 * Field::new(11322249509082494407);
        let a72 = a72 + t;
        let (a72, a76) = (a72 + a76, a72 - a76);
        let (a74, a78) = (a74 + a78, a74 - a78);
        let a78 = (a78 << 48);
        let (a72, a74) = (a72 + a74, a72 - a74);
        let (a76, a78) = (a76 + a78, a76 - a78);
        let (a77, a73) = (a77 + a73, a77 - a73);
        let (a71, a79) = (a71 + a79, a71 - a79);
        let a79 = (a79 << 48);
        let (a77, a71) = (a77 + a71, a77 - a71);
        let (a73, a79) = (a73 + a79, a73 - a79);
        let t = a75;
        let a75 = a75 + a77;
        let a77 = a77 * Field::new(4611686017353646080);
        let a73 = a73 * Field::new(16181989089180173841);
        let a71 = a71 * Field::new(5818851782451133869);
        let a79 = a79 * Field::new(11322249509082494407);
        let a77 = a77 + t;
        let (a77, a71) = (a77 + a71, a77 - a71);
        let (a79, a73) = (a79 + a73, a79 - a73);
        let a73 = (a73 << 48);
        let (a77, a79) = (a77 + a79, a77 - a79);
        let (a71, a73) = (a71 + a73, a71 - a73);
        let (a70, a75) = (a70 + a75, a70 - a75);
        let (a72, a77) = (a72 + a77, a72 - a77);
        let (a76, a71) = (a76 + a71, a76 - a71);
        let (a78, a73) = (a78 + a73, a78 - a73);
        let (a74, a79) = (a74 + a79, a74 - a79);
        values[0] = a0;
        values[1] = a40;
        values[2] = a20;
        values[3] = a60;
        values[4] = a10;
        values[5] = a50;
        values[6] = a30;
        values[7] = a70;
        values[8] = a7;
        values[9] = a47;
        values[10] = a27;
        values[11] = a67;
        values[12] = a17;
        values[13] = a57;
        values[14] = a37;
        values[15] = a77;
        values[16] = a6;
        values[17] = a46;
        values[18] = a26;
        values[19] = a66;
        values[20] = a16;
        values[21] = a56;
        values[22] = a36;
        values[23] = a76;
        values[24] = a3;
        values[25] = a43;
        values[26] = a23;
        values[27] = a63;
        values[28] = a13;
        values[29] = a53;
        values[30] = a33;
        values[31] = a73;
        values[32] = a4;
        values[33] = a44;
        values[34] = a24;
        values[35] = a64;
        values[36] = a14;
        values[37] = a54;
        values[38] = a34;
        values[39] = a74;
        values[40] = a5;
        values[41] = a45;
        values[42] = a25;
        values[43] = a65;
        values[44] = a15;
        values[45] = a55;
        values[46] = a35;
        values[47] = a75;
        values[48] = a2;
        values[49] = a42;
        values[50] = a22;
        values[51] = a62;
        values[52] = a12;
        values[53] = a52;
        values[54] = a32;
        values[55] = a72;
        values[56] = a1;
        values[57] = a41;
        values[58] = a21;
        values[59] = a61;
        values[60] = a11;
        values[61] = a51;
        values[62] = a31;
        values[63] = a71;
        values[64] = a8;
        values[65] = a48;
        values[66] = a28;
        values[67] = a68;
        values[68] = a18;
        values[69] = a58;
        values[70] = a38;
        values[71] = a78;
        values[72] = a9;
        values[73] = a49;
        values[74] = a29;
        values[75] = a69;
        values[76] = a19;
        values[77] = a59;
        values[78] = a39;
        values[79] = a79;
    }
}

/// Size 85 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 85`.
pub fn ntt_85(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 85, 0);
    for values in values.chunks_exact_mut(85) {
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
        let (a5, a80) = (a5 + a80, a5 - a80);
        let (a20, a65) = (a20 + a65, a20 - a65);
        let a65 = (a65 << 48);
        let (a5, a20) = (a5 + a20, a5 - a20);
        let (a80, a65) = (a80 + a65, a80 - a65);
        let (a30, a55) = (a30 + a55, a30 - a55);
        let (a35, a50) = (a35 + a50, a35 - a50);
        let a50 = (a50 << 48);
        let (a30, a35) = (a30 + a35, a30 - a35);
        let (a55, a50) = (a55 + a50, a55 - a50);
        let (a10, a75) = (a10 + a75, a10 - a75);
        let (a40, a45) = (a40 + a45, a40 - a45);
        let a45 = (a45 << 48);
        let (a10, a40) = (a10 + a40, a10 - a40);
        let (a75, a45) = (a75 + a45, a75 - a45);
        let (a60, a25) = (a60 + a25, a60 - a25);
        let (a70, a15) = (a70 + a15, a70 - a15);
        let a15 = (a15 << 48);
        let (a60, a70) = (a60 + a70, a60 - a70);
        let (a25, a15) = (a25 + a15, a25 - a15);
        let a55 = (a55 << 12);
        let a35 = (a35 << 24);
        let a50 = (a50 << 36);
        let a75 = (a75 << 24);
        let a40 = (a40 << 48);
        let a45 = (a45 << 72);
        let a25 = (a25 << 36);
        let a70 = (a70 << 72);
        let a15 = (-(a15 << 12));
        let (a5, a10) = (a5 + a10, a5 - a10);
        let (a30, a60) = (a30 + a60, a30 - a60);
        let a60 = (a60 << 48);
        let (a5, a30) = (a5 + a30, a5 - a30);
        let (a10, a60) = (a10 + a60, a10 - a60);
        let (a80, a75) = (a80 + a75, a80 - a75);
        let (a55, a25) = (a55 + a25, a55 - a25);
        let a25 = (a25 << 48);
        let (a80, a55) = (a80 + a55, a80 - a55);
        let (a75, a25) = (a75 + a25, a75 - a25);
        let (a20, a40) = (a20 + a40, a20 - a40);
        let (a35, a70) = (a35 + a70, a35 - a70);
        let a70 = (a70 << 48);
        let (a20, a35) = (a20 + a35, a20 - a35);
        let (a40, a70) = (a40 + a70, a40 - a70);
        let (a65, a45) = (a65 + a45, a65 - a45);
        let (a50, a15) = (a50 + a15, a50 - a15);
        let a15 = (a15 << 48);
        let (a65, a50) = (a65 + a50, a65 - a50);
        let (a45, a15) = (a45 + a15, a45 - a15);
        let t = a0;
        let a0 = a0 + a5;
        let a5 = a5 * Field::new(1152921504338411520);
        let a80 = a80 * Field::new(6259776822214049175);
        let a20 = a20 * Field::new(9380094172986290191);
        let a65 = a65 * Field::new(891943630314919127);
        let a10 = a10 * Field::new(17228171707553225791);
        let a75 = a75 * Field::new(12855743360534130886);
        let a40 = a40 * Field::new(6167687396920564837);
        let a45 = a45 * Field::new(17201834061724655524);
        let a30 = a30 * Field::new(15308299771656910737);
        let a55 = a55 * Field::new(18186005861103657533);
        let a35 = a35 * Field::new(53595491891823545);
        let a50 = a50 * Field::new(1906638201581172103);
        let a60 = a60 * Field::new(18303651001328874822);
        let a25 = a25 * Field::new(3077466521755967626);
        let a70 = a70 * Field::new(12423593102987598328);
        let a15 = a15 * Field::new(18361513053649472048);
        let a5 = a5 + t;
        let (a5, a30) = (a5 + a30, a5 - a30);
        let (a60, a10) = (a60 + a10, a60 - a10);
        let a10 = (a10 << 48);
        let (a5, a60) = (a5 + a60, a5 - a60);
        let (a30, a10) = (a30 + a10, a30 - a10);
        let (a15, a45) = (a15 + a45, a15 - a45);
        let (a50, a65) = (a50 + a65, a50 - a65);
        let a65 = (a65 << 48);
        let (a15, a50) = (a15 + a50, a15 - a50);
        let (a45, a65) = (a45 + a65, a45 - a65);
        let (a70, a40) = (a70 + a40, a70 - a40);
        let (a35, a20) = (a35 + a20, a35 - a20);
        let a20 = (a20 << 48);
        let (a70, a35) = (a70 + a35, a70 - a35);
        let (a40, a20) = (a40 + a20, a40 - a20);
        let (a25, a75) = (a25 + a75, a25 - a75);
        let (a55, a80) = (a55 + a80, a55 - a80);
        let a80 = (a80 << 48);
        let (a25, a55) = (a25 + a55, a25 - a55);
        let (a75, a80) = (a75 + a80, a75 - a80);
        let a45 = (a45 << 12);
        let a50 = (a50 << 24);
        let a65 = (a65 << 36);
        let a40 = (a40 << 24);
        let a35 = (a35 << 48);
        let a20 = (a20 << 72);
        let a75 = (a75 << 36);
        let a55 = (a55 << 72);
        let a80 = (-(a80 << 12));
        let (a5, a70) = (a5 + a70, a5 - a70);
        let (a15, a25) = (a15 + a25, a15 - a25);
        let a25 = (a25 << 48);
        let (a5, a15) = (a5 + a15, a5 - a15);
        let (a70, a25) = (a70 + a25, a70 - a25);
        let (a30, a40) = (a30 + a40, a30 - a40);
        let (a45, a75) = (a45 + a75, a45 - a75);
        let a75 = (a75 << 48);
        let (a30, a45) = (a30 + a45, a30 - a45);
        let (a40, a75) = (a40 + a75, a40 - a75);
        let (a60, a35) = (a60 + a35, a60 - a35);
        let (a50, a55) = (a50 + a55, a50 - a55);
        let a55 = (a55 << 48);
        let (a60, a50) = (a60 + a50, a60 - a50);
        let (a35, a55) = (a35 + a55, a35 - a55);
        let (a10, a20) = (a10 + a20, a10 - a20);
        let (a65, a80) = (a65 + a80, a65 - a80);
        let a80 = (a80 << 48);
        let (a10, a65) = (a10 + a65, a10 - a65);
        let (a20, a80) = (a20 + a80, a20 - a80);
        let (a22, a12) = (a22 + a12, a22 - a12);
        let (a37, a82) = (a37 + a82, a37 - a82);
        let a82 = (a82 << 48);
        let (a22, a37) = (a22 + a37, a22 - a37);
        let (a12, a82) = (a12 + a82, a12 - a82);
        let (a47, a72) = (a47 + a72, a47 - a72);
        let (a52, a67) = (a52 + a67, a52 - a67);
        let a67 = (a67 << 48);
        let (a47, a52) = (a47 + a52, a47 - a52);
        let (a72, a67) = (a72 + a67, a72 - a67);
        let (a27, a7) = (a27 + a7, a27 - a7);
        let (a57, a62) = (a57 + a62, a57 - a62);
        let a62 = (a62 << 48);
        let (a27, a57) = (a27 + a57, a27 - a57);
        let (a7, a62) = (a7 + a62, a7 - a62);
        let (a77, a42) = (a77 + a42, a77 - a42);
        let (a2, a32) = (a2 + a32, a2 - a32);
        let a32 = (a32 << 48);
        let (a77, a2) = (a77 + a2, a77 - a2);
        let (a42, a32) = (a42 + a32, a42 - a32);
        let a72 = (a72 << 12);
        let a52 = (a52 << 24);
        let a67 = (a67 << 36);
        let a7 = (a7 << 24);
        let a57 = (a57 << 48);
        let a62 = (a62 << 72);
        let a42 = (a42 << 36);
        let a2 = (a2 << 72);
        let a32 = (-(a32 << 12));
        let (a22, a27) = (a22 + a27, a22 - a27);
        let (a47, a77) = (a47 + a77, a47 - a77);
        let a77 = (a77 << 48);
        let (a22, a47) = (a22 + a47, a22 - a47);
        let (a27, a77) = (a27 + a77, a27 - a77);
        let (a12, a7) = (a12 + a7, a12 - a7);
        let (a72, a42) = (a72 + a42, a72 - a42);
        let a42 = (a42 << 48);
        let (a12, a72) = (a12 + a72, a12 - a72);
        let (a7, a42) = (a7 + a42, a7 - a42);
        let (a37, a57) = (a37 + a57, a37 - a57);
        let (a52, a2) = (a52 + a2, a52 - a2);
        let a2 = (a2 << 48);
        let (a37, a52) = (a37 + a52, a37 - a52);
        let (a57, a2) = (a57 + a2, a57 - a2);
        let (a82, a62) = (a82 + a62, a82 - a62);
        let (a67, a32) = (a67 + a32, a67 - a32);
        let a32 = (a32 << 48);
        let (a82, a67) = (a82 + a67, a82 - a67);
        let (a62, a32) = (a62 + a32, a62 - a32);
        let t = a17;
        let a17 = a17 + a22;
        let a22 = a22 * Field::new(1152921504338411520);
        let a12 = a12 * Field::new(6259776822214049175);
        let a37 = a37 * Field::new(9380094172986290191);
        let a82 = a82 * Field::new(891943630314919127);
        let a27 = a27 * Field::new(17228171707553225791);
        let a7 = a7 * Field::new(12855743360534130886);
        let a57 = a57 * Field::new(6167687396920564837);
        let a62 = a62 * Field::new(17201834061724655524);
        let a47 = a47 * Field::new(15308299771656910737);
        let a72 = a72 * Field::new(18186005861103657533);
        let a52 = a52 * Field::new(53595491891823545);
        let a67 = a67 * Field::new(1906638201581172103);
        let a77 = a77 * Field::new(18303651001328874822);
        let a42 = a42 * Field::new(3077466521755967626);
        let a2 = a2 * Field::new(12423593102987598328);
        let a32 = a32 * Field::new(18361513053649472048);
        let a22 = a22 + t;
        let (a22, a47) = (a22 + a47, a22 - a47);
        let (a77, a27) = (a77 + a27, a77 - a27);
        let a27 = (a27 << 48);
        let (a22, a77) = (a22 + a77, a22 - a77);
        let (a47, a27) = (a47 + a27, a47 - a27);
        let (a32, a62) = (a32 + a62, a32 - a62);
        let (a67, a82) = (a67 + a82, a67 - a82);
        let a82 = (a82 << 48);
        let (a32, a67) = (a32 + a67, a32 - a67);
        let (a62, a82) = (a62 + a82, a62 - a82);
        let (a2, a57) = (a2 + a57, a2 - a57);
        let (a52, a37) = (a52 + a37, a52 - a37);
        let a37 = (a37 << 48);
        let (a2, a52) = (a2 + a52, a2 - a52);
        let (a57, a37) = (a57 + a37, a57 - a37);
        let (a42, a7) = (a42 + a7, a42 - a7);
        let (a72, a12) = (a72 + a12, a72 - a12);
        let a12 = (a12 << 48);
        let (a42, a72) = (a42 + a72, a42 - a72);
        let (a7, a12) = (a7 + a12, a7 - a12);
        let a62 = (a62 << 12);
        let a67 = (a67 << 24);
        let a82 = (a82 << 36);
        let a57 = (a57 << 24);
        let a52 = (a52 << 48);
        let a37 = (a37 << 72);
        let a7 = (a7 << 36);
        let a72 = (a72 << 72);
        let a12 = (-(a12 << 12));
        let (a22, a2) = (a22 + a2, a22 - a2);
        let (a32, a42) = (a32 + a42, a32 - a42);
        let a42 = (a42 << 48);
        let (a22, a32) = (a22 + a32, a22 - a32);
        let (a2, a42) = (a2 + a42, a2 - a42);
        let (a47, a57) = (a47 + a57, a47 - a57);
        let (a62, a7) = (a62 + a7, a62 - a7);
        let a7 = (a7 << 48);
        let (a47, a62) = (a47 + a62, a47 - a62);
        let (a57, a7) = (a57 + a7, a57 - a7);
        let (a77, a52) = (a77 + a52, a77 - a52);
        let (a67, a72) = (a67 + a72, a67 - a72);
        let a72 = (a72 << 48);
        let (a77, a67) = (a77 + a67, a77 - a67);
        let (a52, a72) = (a52 + a72, a52 - a72);
        let (a27, a37) = (a27 + a37, a27 - a37);
        let (a82, a12) = (a82 + a12, a82 - a12);
        let a12 = (a12 << 48);
        let (a27, a82) = (a27 + a82, a27 - a82);
        let (a37, a12) = (a37 + a12, a37 - a12);
        let (a39, a29) = (a39 + a29, a39 - a29);
        let (a54, a14) = (a54 + a14, a54 - a14);
        let a14 = (a14 << 48);
        let (a39, a54) = (a39 + a54, a39 - a54);
        let (a29, a14) = (a29 + a14, a29 - a14);
        let (a64, a4) = (a64 + a4, a64 - a4);
        let (a69, a84) = (a69 + a84, a69 - a84);
        let a84 = (a84 << 48);
        let (a64, a69) = (a64 + a69, a64 - a69);
        let (a4, a84) = (a4 + a84, a4 - a84);
        let (a44, a24) = (a44 + a24, a44 - a24);
        let (a74, a79) = (a74 + a79, a74 - a79);
        let a79 = (a79 << 48);
        let (a44, a74) = (a44 + a74, a44 - a74);
        let (a24, a79) = (a24 + a79, a24 - a79);
        let (a9, a59) = (a9 + a59, a9 - a59);
        let (a19, a49) = (a19 + a49, a19 - a49);
        let a49 = (a49 << 48);
        let (a9, a19) = (a9 + a19, a9 - a19);
        let (a59, a49) = (a59 + a49, a59 - a49);
        let a4 = (a4 << 12);
        let a69 = (a69 << 24);
        let a84 = (a84 << 36);
        let a24 = (a24 << 24);
        let a74 = (a74 << 48);
        let a79 = (a79 << 72);
        let a59 = (a59 << 36);
        let a19 = (a19 << 72);
        let a49 = (-(a49 << 12));
        let (a39, a44) = (a39 + a44, a39 - a44);
        let (a64, a9) = (a64 + a9, a64 - a9);
        let a9 = (a9 << 48);
        let (a39, a64) = (a39 + a64, a39 - a64);
        let (a44, a9) = (a44 + a9, a44 - a9);
        let (a29, a24) = (a29 + a24, a29 - a24);
        let (a4, a59) = (a4 + a59, a4 - a59);
        let a59 = (a59 << 48);
        let (a29, a4) = (a29 + a4, a29 - a4);
        let (a24, a59) = (a24 + a59, a24 - a59);
        let (a54, a74) = (a54 + a74, a54 - a74);
        let (a69, a19) = (a69 + a19, a69 - a19);
        let a19 = (a19 << 48);
        let (a54, a69) = (a54 + a69, a54 - a69);
        let (a74, a19) = (a74 + a19, a74 - a19);
        let (a14, a79) = (a14 + a79, a14 - a79);
        let (a84, a49) = (a84 + a49, a84 - a49);
        let a49 = (a49 << 48);
        let (a14, a84) = (a14 + a84, a14 - a84);
        let (a79, a49) = (a79 + a49, a79 - a49);
        let t = a34;
        let a34 = a34 + a39;
        let a39 = a39 * Field::new(1152921504338411520);
        let a29 = a29 * Field::new(6259776822214049175);
        let a54 = a54 * Field::new(9380094172986290191);
        let a14 = a14 * Field::new(891943630314919127);
        let a44 = a44 * Field::new(17228171707553225791);
        let a24 = a24 * Field::new(12855743360534130886);
        let a74 = a74 * Field::new(6167687396920564837);
        let a79 = a79 * Field::new(17201834061724655524);
        let a64 = a64 * Field::new(15308299771656910737);
        let a4 = a4 * Field::new(18186005861103657533);
        let a69 = a69 * Field::new(53595491891823545);
        let a84 = a84 * Field::new(1906638201581172103);
        let a9 = a9 * Field::new(18303651001328874822);
        let a59 = a59 * Field::new(3077466521755967626);
        let a19 = a19 * Field::new(12423593102987598328);
        let a49 = a49 * Field::new(18361513053649472048);
        let a39 = a39 + t;
        let (a39, a64) = (a39 + a64, a39 - a64);
        let (a9, a44) = (a9 + a44, a9 - a44);
        let a44 = (a44 << 48);
        let (a39, a9) = (a39 + a9, a39 - a9);
        let (a64, a44) = (a64 + a44, a64 - a44);
        let (a49, a79) = (a49 + a79, a49 - a79);
        let (a84, a14) = (a84 + a14, a84 - a14);
        let a14 = (a14 << 48);
        let (a49, a84) = (a49 + a84, a49 - a84);
        let (a79, a14) = (a79 + a14, a79 - a14);
        let (a19, a74) = (a19 + a74, a19 - a74);
        let (a69, a54) = (a69 + a54, a69 - a54);
        let a54 = (a54 << 48);
        let (a19, a69) = (a19 + a69, a19 - a69);
        let (a74, a54) = (a74 + a54, a74 - a54);
        let (a59, a24) = (a59 + a24, a59 - a24);
        let (a4, a29) = (a4 + a29, a4 - a29);
        let a29 = (a29 << 48);
        let (a59, a4) = (a59 + a4, a59 - a4);
        let (a24, a29) = (a24 + a29, a24 - a29);
        let a79 = (a79 << 12);
        let a84 = (a84 << 24);
        let a14 = (a14 << 36);
        let a74 = (a74 << 24);
        let a69 = (a69 << 48);
        let a54 = (a54 << 72);
        let a24 = (a24 << 36);
        let a4 = (a4 << 72);
        let a29 = (-(a29 << 12));
        let (a39, a19) = (a39 + a19, a39 - a19);
        let (a49, a59) = (a49 + a59, a49 - a59);
        let a59 = (a59 << 48);
        let (a39, a49) = (a39 + a49, a39 - a49);
        let (a19, a59) = (a19 + a59, a19 - a59);
        let (a64, a74) = (a64 + a74, a64 - a74);
        let (a79, a24) = (a79 + a24, a79 - a24);
        let a24 = (a24 << 48);
        let (a64, a79) = (a64 + a79, a64 - a79);
        let (a74, a24) = (a74 + a24, a74 - a24);
        let (a9, a69) = (a9 + a69, a9 - a69);
        let (a84, a4) = (a84 + a4, a84 - a4);
        let a4 = (a4 << 48);
        let (a9, a84) = (a9 + a84, a9 - a84);
        let (a69, a4) = (a69 + a4, a69 - a4);
        let (a44, a54) = (a44 + a54, a44 - a54);
        let (a14, a29) = (a14 + a29, a14 - a29);
        let a29 = (a29 << 48);
        let (a44, a14) = (a44 + a14, a44 - a14);
        let (a54, a29) = (a54 + a29, a54 - a29);
        let (a56, a46) = (a56 + a46, a56 - a46);
        let (a71, a31) = (a71 + a31, a71 - a31);
        let a31 = (a31 << 48);
        let (a56, a71) = (a56 + a71, a56 - a71);
        let (a46, a31) = (a46 + a31, a46 - a31);
        let (a81, a21) = (a81 + a21, a81 - a21);
        let (a1, a16) = (a1 + a16, a1 - a16);
        let a16 = (a16 << 48);
        let (a81, a1) = (a81 + a1, a81 - a1);
        let (a21, a16) = (a21 + a16, a21 - a16);
        let (a61, a41) = (a61 + a41, a61 - a41);
        let (a6, a11) = (a6 + a11, a6 - a11);
        let a11 = (a11 << 48);
        let (a61, a6) = (a61 + a6, a61 - a6);
        let (a41, a11) = (a41 + a11, a41 - a11);
        let (a26, a76) = (a26 + a76, a26 - a76);
        let (a36, a66) = (a36 + a66, a36 - a66);
        let a66 = (a66 << 48);
        let (a26, a36) = (a26 + a36, a26 - a36);
        let (a76, a66) = (a76 + a66, a76 - a66);
        let a21 = (a21 << 12);
        let a1 = (a1 << 24);
        let a16 = (a16 << 36);
        let a41 = (a41 << 24);
        let a6 = (a6 << 48);
        let a11 = (a11 << 72);
        let a76 = (a76 << 36);
        let a36 = (a36 << 72);
        let a66 = (-(a66 << 12));
        let (a56, a61) = (a56 + a61, a56 - a61);
        let (a81, a26) = (a81 + a26, a81 - a26);
        let a26 = (a26 << 48);
        let (a56, a81) = (a56 + a81, a56 - a81);
        let (a61, a26) = (a61 + a26, a61 - a26);
        let (a46, a41) = (a46 + a41, a46 - a41);
        let (a21, a76) = (a21 + a76, a21 - a76);
        let a76 = (a76 << 48);
        let (a46, a21) = (a46 + a21, a46 - a21);
        let (a41, a76) = (a41 + a76, a41 - a76);
        let (a71, a6) = (a71 + a6, a71 - a6);
        let (a1, a36) = (a1 + a36, a1 - a36);
        let a36 = (a36 << 48);
        let (a71, a1) = (a71 + a1, a71 - a1);
        let (a6, a36) = (a6 + a36, a6 - a36);
        let (a31, a11) = (a31 + a11, a31 - a11);
        let (a16, a66) = (a16 + a66, a16 - a66);
        let a66 = (a66 << 48);
        let (a31, a16) = (a31 + a16, a31 - a16);
        let (a11, a66) = (a11 + a66, a11 - a66);
        let t = a51;
        let a51 = a51 + a56;
        let a56 = a56 * Field::new(1152921504338411520);
        let a46 = a46 * Field::new(6259776822214049175);
        let a71 = a71 * Field::new(9380094172986290191);
        let a31 = a31 * Field::new(891943630314919127);
        let a61 = a61 * Field::new(17228171707553225791);
        let a41 = a41 * Field::new(12855743360534130886);
        let a6 = a6 * Field::new(6167687396920564837);
        let a11 = a11 * Field::new(17201834061724655524);
        let a81 = a81 * Field::new(15308299771656910737);
        let a21 = a21 * Field::new(18186005861103657533);
        let a1 = a1 * Field::new(53595491891823545);
        let a16 = a16 * Field::new(1906638201581172103);
        let a26 = a26 * Field::new(18303651001328874822);
        let a76 = a76 * Field::new(3077466521755967626);
        let a36 = a36 * Field::new(12423593102987598328);
        let a66 = a66 * Field::new(18361513053649472048);
        let a56 = a56 + t;
        let (a56, a81) = (a56 + a81, a56 - a81);
        let (a26, a61) = (a26 + a61, a26 - a61);
        let a61 = (a61 << 48);
        let (a56, a26) = (a56 + a26, a56 - a26);
        let (a81, a61) = (a81 + a61, a81 - a61);
        let (a66, a11) = (a66 + a11, a66 - a11);
        let (a16, a31) = (a16 + a31, a16 - a31);
        let a31 = (a31 << 48);
        let (a66, a16) = (a66 + a16, a66 - a16);
        let (a11, a31) = (a11 + a31, a11 - a31);
        let (a36, a6) = (a36 + a6, a36 - a6);
        let (a1, a71) = (a1 + a71, a1 - a71);
        let a71 = (a71 << 48);
        let (a36, a1) = (a36 + a1, a36 - a1);
        let (a6, a71) = (a6 + a71, a6 - a71);
        let (a76, a41) = (a76 + a41, a76 - a41);
        let (a21, a46) = (a21 + a46, a21 - a46);
        let a46 = (a46 << 48);
        let (a76, a21) = (a76 + a21, a76 - a21);
        let (a41, a46) = (a41 + a46, a41 - a46);
        let a11 = (a11 << 12);
        let a16 = (a16 << 24);
        let a31 = (a31 << 36);
        let a6 = (a6 << 24);
        let a1 = (a1 << 48);
        let a71 = (a71 << 72);
        let a41 = (a41 << 36);
        let a21 = (a21 << 72);
        let a46 = (-(a46 << 12));
        let (a56, a36) = (a56 + a36, a56 - a36);
        let (a66, a76) = (a66 + a76, a66 - a76);
        let a76 = (a76 << 48);
        let (a56, a66) = (a56 + a66, a56 - a66);
        let (a36, a76) = (a36 + a76, a36 - a76);
        let (a81, a6) = (a81 + a6, a81 - a6);
        let (a11, a41) = (a11 + a41, a11 - a41);
        let a41 = (a41 << 48);
        let (a81, a11) = (a81 + a11, a81 - a11);
        let (a6, a41) = (a6 + a41, a6 - a41);
        let (a26, a1) = (a26 + a1, a26 - a1);
        let (a16, a21) = (a16 + a21, a16 - a21);
        let a21 = (a21 << 48);
        let (a26, a16) = (a26 + a16, a26 - a16);
        let (a1, a21) = (a1 + a21, a1 - a21);
        let (a61, a71) = (a61 + a71, a61 - a71);
        let (a31, a46) = (a31 + a46, a31 - a46);
        let a46 = (a46 << 48);
        let (a61, a31) = (a61 + a31, a61 - a31);
        let (a71, a46) = (a71 + a46, a71 - a46);
        let (a73, a63) = (a73 + a63, a73 - a63);
        let (a3, a48) = (a3 + a48, a3 - a48);
        let a48 = (a48 << 48);
        let (a73, a3) = (a73 + a3, a73 - a3);
        let (a63, a48) = (a63 + a48, a63 - a48);
        let (a13, a38) = (a13 + a38, a13 - a38);
        let (a18, a33) = (a18 + a33, a18 - a33);
        let a33 = (a33 << 48);
        let (a13, a18) = (a13 + a18, a13 - a18);
        let (a38, a33) = (a38 + a33, a38 - a33);
        let (a78, a58) = (a78 + a58, a78 - a58);
        let (a23, a28) = (a23 + a28, a23 - a28);
        let a28 = (a28 << 48);
        let (a78, a23) = (a78 + a23, a78 - a23);
        let (a58, a28) = (a58 + a28, a58 - a28);
        let (a43, a8) = (a43 + a8, a43 - a8);
        let (a53, a83) = (a53 + a83, a53 - a83);
        let a83 = (a83 << 48);
        let (a43, a53) = (a43 + a53, a43 - a53);
        let (a8, a83) = (a8 + a83, a8 - a83);
        let a38 = (a38 << 12);
        let a18 = (a18 << 24);
        let a33 = (a33 << 36);
        let a58 = (a58 << 24);
        let a23 = (a23 << 48);
        let a28 = (a28 << 72);
        let a8 = (a8 << 36);
        let a53 = (a53 << 72);
        let a83 = (-(a83 << 12));
        let (a73, a78) = (a73 + a78, a73 - a78);
        let (a13, a43) = (a13 + a43, a13 - a43);
        let a43 = (a43 << 48);
        let (a73, a13) = (a73 + a13, a73 - a13);
        let (a78, a43) = (a78 + a43, a78 - a43);
        let (a63, a58) = (a63 + a58, a63 - a58);
        let (a38, a8) = (a38 + a8, a38 - a8);
        let a8 = (a8 << 48);
        let (a63, a38) = (a63 + a38, a63 - a38);
        let (a58, a8) = (a58 + a8, a58 - a8);
        let (a3, a23) = (a3 + a23, a3 - a23);
        let (a18, a53) = (a18 + a53, a18 - a53);
        let a53 = (a53 << 48);
        let (a3, a18) = (a3 + a18, a3 - a18);
        let (a23, a53) = (a23 + a53, a23 - a53);
        let (a48, a28) = (a48 + a28, a48 - a28);
        let (a33, a83) = (a33 + a83, a33 - a83);
        let a83 = (a83 << 48);
        let (a48, a33) = (a48 + a33, a48 - a33);
        let (a28, a83) = (a28 + a83, a28 - a83);
        let t = a68;
        let a68 = a68 + a73;
        let a73 = a73 * Field::new(1152921504338411520);
        let a63 = a63 * Field::new(6259776822214049175);
        let a3 = a3 * Field::new(9380094172986290191);
        let a48 = a48 * Field::new(891943630314919127);
        let a78 = a78 * Field::new(17228171707553225791);
        let a58 = a58 * Field::new(12855743360534130886);
        let a23 = a23 * Field::new(6167687396920564837);
        let a28 = a28 * Field::new(17201834061724655524);
        let a13 = a13 * Field::new(15308299771656910737);
        let a38 = a38 * Field::new(18186005861103657533);
        let a18 = a18 * Field::new(53595491891823545);
        let a33 = a33 * Field::new(1906638201581172103);
        let a43 = a43 * Field::new(18303651001328874822);
        let a8 = a8 * Field::new(3077466521755967626);
        let a53 = a53 * Field::new(12423593102987598328);
        let a83 = a83 * Field::new(18361513053649472048);
        let a73 = a73 + t;
        let (a73, a13) = (a73 + a13, a73 - a13);
        let (a43, a78) = (a43 + a78, a43 - a78);
        let a78 = (a78 << 48);
        let (a73, a43) = (a73 + a43, a73 - a43);
        let (a13, a78) = (a13 + a78, a13 - a78);
        let (a83, a28) = (a83 + a28, a83 - a28);
        let (a33, a48) = (a33 + a48, a33 - a48);
        let a48 = (a48 << 48);
        let (a83, a33) = (a83 + a33, a83 - a33);
        let (a28, a48) = (a28 + a48, a28 - a48);
        let (a53, a23) = (a53 + a23, a53 - a23);
        let (a18, a3) = (a18 + a3, a18 - a3);
        let a3 = (a3 << 48);
        let (a53, a18) = (a53 + a18, a53 - a18);
        let (a23, a3) = (a23 + a3, a23 - a3);
        let (a8, a58) = (a8 + a58, a8 - a58);
        let (a38, a63) = (a38 + a63, a38 - a63);
        let a63 = (a63 << 48);
        let (a8, a38) = (a8 + a38, a8 - a38);
        let (a58, a63) = (a58 + a63, a58 - a63);
        let a28 = (a28 << 12);
        let a33 = (a33 << 24);
        let a48 = (a48 << 36);
        let a23 = (a23 << 24);
        let a18 = (a18 << 48);
        let a3 = (a3 << 72);
        let a58 = (a58 << 36);
        let a38 = (a38 << 72);
        let a63 = (-(a63 << 12));
        let (a73, a53) = (a73 + a53, a73 - a53);
        let (a83, a8) = (a83 + a8, a83 - a8);
        let a8 = (a8 << 48);
        let (a73, a83) = (a73 + a83, a73 - a83);
        let (a53, a8) = (a53 + a8, a53 - a8);
        let (a13, a23) = (a13 + a23, a13 - a23);
        let (a28, a58) = (a28 + a58, a28 - a58);
        let a58 = (a58 << 48);
        let (a13, a28) = (a13 + a28, a13 - a28);
        let (a23, a58) = (a23 + a58, a23 - a58);
        let (a43, a18) = (a43 + a18, a43 - a18);
        let (a33, a38) = (a33 + a38, a33 - a38);
        let a38 = (a38 << 48);
        let (a43, a33) = (a43 + a33, a43 - a33);
        let (a18, a38) = (a18 + a38, a18 - a38);
        let (a78, a3) = (a78 + a3, a78 - a3);
        let (a48, a63) = (a48 + a63, a48 - a63);
        let a63 = (a63 << 48);
        let (a78, a48) = (a78 + a48, a78 - a48);
        let (a3, a63) = (a3 + a63, a3 - a63);
        let (a17, a68) = (a17 + a68, a17 - a68);
        let (a51, a34) = (a51 + a34, a51 - a34);
        let a34 = (a34 << 48);
        let (a17, a51) = (a17 + a51, a17 - a51);
        let (a68, a34) = (a68 + a34, a68 - a34);
        let t = a0;
        let a0 = a0 + a17;
        let a17 = a17 * Field::new(4611686017353646080);
        let a68 = a68 * Field::new(16181989089180173841);
        let a51 = a51 * Field::new(5818851782451133869);
        let a34 = a34 * Field::new(11322249509082494407);
        let a17 = a17 + t;
        let (a17, a51) = (a17 + a51, a17 - a51);
        let (a34, a68) = (a34 + a68, a34 - a68);
        let a68 = (a68 << 48);
        let (a17, a34) = (a17 + a34, a17 - a34);
        let (a51, a68) = (a51 + a68, a51 - a68);
        let (a22, a73) = (a22 + a73, a22 - a73);
        let (a56, a39) = (a56 + a39, a56 - a39);
        let a39 = (a39 << 48);
        let (a22, a56) = (a22 + a56, a22 - a56);
        let (a73, a39) = (a73 + a39, a73 - a39);
        let t = a5;
        let a5 = a5 + a22;
        let a22 = a22 * Field::new(4611686017353646080);
        let a73 = a73 * Field::new(16181989089180173841);
        let a56 = a56 * Field::new(5818851782451133869);
        let a39 = a39 * Field::new(11322249509082494407);
        let a22 = a22 + t;
        let (a22, a56) = (a22 + a56, a22 - a56);
        let (a39, a73) = (a39 + a73, a39 - a73);
        let a73 = (a73 << 48);
        let (a22, a39) = (a22 + a39, a22 - a39);
        let (a56, a73) = (a56 + a73, a56 - a73);
        let (a72, a38) = (a72 + a38, a72 - a38);
        let (a21, a4) = (a21 + a4, a21 - a4);
        let a4 = (a4 << 48);
        let (a72, a21) = (a72 + a21, a72 - a21);
        let (a38, a4) = (a38 + a4, a38 - a4);
        let t = a55;
        let a55 = a55 + a72;
        let a72 = a72 * Field::new(4611686017353646080);
        let a38 = a38 * Field::new(16181989089180173841);
        let a21 = a21 * Field::new(5818851782451133869);
        let a4 = a4 * Field::new(11322249509082494407);
        let a72 = a72 + t;
        let (a72, a21) = (a72 + a21, a72 - a21);
        let (a4, a38) = (a4 + a38, a4 - a38);
        let a38 = (a38 << 48);
        let (a72, a4) = (a72 + a4, a72 - a4);
        let (a21, a38) = (a21 + a38, a21 - a38);
        let (a47, a13) = (a47 + a13, a47 - a13);
        let (a81, a64) = (a81 + a64, a81 - a64);
        let a64 = (a64 << 48);
        let (a47, a81) = (a47 + a81, a47 - a81);
        let (a13, a64) = (a13 + a64, a13 - a64);
        let t = a30;
        let a30 = a30 + a47;
        let a47 = a47 * Field::new(4611686017353646080);
        let a13 = a13 * Field::new(16181989089180173841);
        let a81 = a81 * Field::new(5818851782451133869);
        let a64 = a64 * Field::new(11322249509082494407);
        let a47 = a47 + t;
        let (a47, a81) = (a47 + a81, a47 - a81);
        let (a64, a13) = (a64 + a13, a64 - a13);
        let a13 = (a13 << 48);
        let (a47, a64) = (a47 + a64, a47 - a64);
        let (a81, a13) = (a81 + a13, a81 - a13);
        let (a42, a8) = (a42 + a8, a42 - a8);
        let (a76, a59) = (a76 + a59, a76 - a59);
        let a59 = (a59 << 48);
        let (a42, a76) = (a42 + a76, a42 - a76);
        let (a8, a59) = (a8 + a59, a8 - a59);
        let t = a25;
        let a25 = a25 + a42;
        let a42 = a42 * Field::new(4611686017353646080);
        let a8 = a8 * Field::new(16181989089180173841);
        let a76 = a76 * Field::new(5818851782451133869);
        let a59 = a59 * Field::new(11322249509082494407);
        let a42 = a42 + t;
        let (a42, a76) = (a42 + a76, a42 - a76);
        let (a59, a8) = (a59 + a8, a59 - a8);
        let a8 = (a8 << 48);
        let (a42, a59) = (a42 + a59, a42 - a59);
        let (a76, a8) = (a76 + a8, a76 - a8);
        let (a57, a23) = (a57 + a23, a57 - a23);
        let (a6, a74) = (a6 + a74, a6 - a74);
        let a74 = (a74 << 48);
        let (a57, a6) = (a57 + a6, a57 - a6);
        let (a23, a74) = (a23 + a74, a23 - a74);
        let t = a40;
        let a40 = a40 + a57;
        let a57 = a57 * Field::new(4611686017353646080);
        let a23 = a23 * Field::new(16181989089180173841);
        let a6 = a6 * Field::new(5818851782451133869);
        let a74 = a74 * Field::new(11322249509082494407);
        let a57 = a57 + t;
        let (a57, a6) = (a57 + a6, a57 - a6);
        let (a74, a23) = (a74 + a23, a74 - a23);
        let a23 = (a23 << 48);
        let (a57, a74) = (a57 + a74, a57 - a74);
        let (a6, a23) = (a6 + a23, a6 - a23);
        let (a12, a63) = (a12 + a63, a12 - a63);
        let (a46, a29) = (a46 + a29, a46 - a29);
        let a29 = (a29 << 48);
        let (a12, a46) = (a12 + a46, a12 - a46);
        let (a63, a29) = (a63 + a29, a63 - a29);
        let t = a80;
        let a80 = a80 + a12;
        let a12 = a12 * Field::new(4611686017353646080);
        let a63 = a63 * Field::new(16181989089180173841);
        let a46 = a46 * Field::new(5818851782451133869);
        let a29 = a29 * Field::new(11322249509082494407);
        let a12 = a12 + t;
        let (a12, a46) = (a12 + a46, a12 - a46);
        let (a29, a63) = (a29 + a63, a29 - a63);
        let a63 = (a63 << 48);
        let (a12, a29) = (a12 + a29, a12 - a29);
        let (a46, a63) = (a46 + a63, a46 - a63);
        let (a82, a48) = (a82 + a48, a82 - a48);
        let (a31, a14) = (a31 + a14, a31 - a14);
        let a14 = (a14 << 48);
        let (a82, a31) = (a82 + a31, a82 - a31);
        let (a48, a14) = (a48 + a14, a48 - a14);
        let t = a65;
        let a65 = a65 + a82;
        let a82 = a82 * Field::new(4611686017353646080);
        let a48 = a48 * Field::new(16181989089180173841);
        let a31 = a31 * Field::new(5818851782451133869);
        let a14 = a14 * Field::new(11322249509082494407);
        let a82 = a82 + t;
        let (a82, a31) = (a82 + a31, a82 - a31);
        let (a14, a48) = (a14 + a48, a14 - a48);
        let a48 = (a48 << 48);
        let (a82, a14) = (a82 + a14, a82 - a14);
        let (a31, a48) = (a31 + a48, a31 - a48);
        let (a67, a33) = (a67 + a33, a67 - a33);
        let (a16, a84) = (a16 + a84, a16 - a84);
        let a84 = (a84 << 48);
        let (a67, a16) = (a67 + a16, a67 - a16);
        let (a33, a84) = (a33 + a84, a33 - a84);
        let t = a50;
        let a50 = a50 + a67;
        let a67 = a67 * Field::new(4611686017353646080);
        let a33 = a33 * Field::new(16181989089180173841);
        let a16 = a16 * Field::new(5818851782451133869);
        let a84 = a84 * Field::new(11322249509082494407);
        let a67 = a67 + t;
        let (a67, a16) = (a67 + a16, a67 - a16);
        let (a84, a33) = (a84 + a33, a84 - a33);
        let a33 = (a33 << 48);
        let (a67, a84) = (a67 + a84, a67 - a84);
        let (a16, a33) = (a16 + a33, a16 - a33);
        let (a77, a43) = (a77 + a43, a77 - a43);
        let (a26, a9) = (a26 + a9, a26 - a9);
        let a9 = (a9 << 48);
        let (a77, a26) = (a77 + a26, a77 - a26);
        let (a43, a9) = (a43 + a9, a43 - a9);
        let t = a60;
        let a60 = a60 + a77;
        let a77 = a77 * Field::new(4611686017353646080);
        let a43 = a43 * Field::new(16181989089180173841);
        let a26 = a26 * Field::new(5818851782451133869);
        let a9 = a9 * Field::new(11322249509082494407);
        let a77 = a77 + t;
        let (a77, a26) = (a77 + a26, a77 - a26);
        let (a9, a43) = (a9 + a43, a9 - a43);
        let a43 = (a43 << 48);
        let (a77, a9) = (a77 + a9, a77 - a9);
        let (a26, a43) = (a26 + a43, a26 - a43);
        let (a27, a78) = (a27 + a78, a27 - a78);
        let (a61, a44) = (a61 + a44, a61 - a44);
        let a44 = (a44 << 48);
        let (a27, a61) = (a27 + a61, a27 - a61);
        let (a78, a44) = (a78 + a44, a78 - a44);
        let t = a10;
        let a10 = a10 + a27;
        let a27 = a27 * Field::new(4611686017353646080);
        let a78 = a78 * Field::new(16181989089180173841);
        let a61 = a61 * Field::new(5818851782451133869);
        let a44 = a44 * Field::new(11322249509082494407);
        let a27 = a27 + t;
        let (a27, a61) = (a27 + a61, a27 - a61);
        let (a44, a78) = (a44 + a78, a44 - a78);
        let a78 = (a78 << 48);
        let (a27, a44) = (a27 + a44, a27 - a44);
        let (a61, a78) = (a61 + a78, a61 - a78);
        let (a37, a3) = (a37 + a3, a37 - a3);
        let (a71, a54) = (a71 + a54, a71 - a54);
        let a54 = (a54 << 48);
        let (a37, a71) = (a37 + a71, a37 - a71);
        let (a3, a54) = (a3 + a54, a3 - a54);
        let t = a20;
        let a20 = a20 + a37;
        let a37 = a37 * Field::new(4611686017353646080);
        let a3 = a3 * Field::new(16181989089180173841);
        let a71 = a71 * Field::new(5818851782451133869);
        let a54 = a54 * Field::new(11322249509082494407);
        let a37 = a37 + t;
        let (a37, a71) = (a37 + a71, a37 - a71);
        let (a54, a3) = (a54 + a3, a54 - a3);
        let a3 = (a3 << 48);
        let (a37, a54) = (a37 + a54, a37 - a54);
        let (a71, a3) = (a71 + a3, a71 - a3);
        let (a7, a58) = (a7 + a58, a7 - a58);
        let (a41, a24) = (a41 + a24, a41 - a24);
        let a24 = (a24 << 48);
        let (a7, a41) = (a7 + a41, a7 - a41);
        let (a58, a24) = (a58 + a24, a58 - a24);
        let t = a75;
        let a75 = a75 + a7;
        let a7 = a7 * Field::new(4611686017353646080);
        let a58 = a58 * Field::new(16181989089180173841);
        let a41 = a41 * Field::new(5818851782451133869);
        let a24 = a24 * Field::new(11322249509082494407);
        let a7 = a7 + t;
        let (a7, a41) = (a7 + a41, a7 - a41);
        let (a24, a58) = (a24 + a58, a24 - a58);
        let a58 = (a58 << 48);
        let (a7, a24) = (a7 + a24, a7 - a24);
        let (a41, a58) = (a41 + a58, a41 - a58);
        let (a2, a53) = (a2 + a53, a2 - a53);
        let (a36, a19) = (a36 + a19, a36 - a19);
        let a19 = (a19 << 48);
        let (a2, a36) = (a2 + a36, a2 - a36);
        let (a53, a19) = (a53 + a19, a53 - a19);
        let t = a70;
        let a70 = a70 + a2;
        let a2 = a2 * Field::new(4611686017353646080);
        let a53 = a53 * Field::new(16181989089180173841);
        let a36 = a36 * Field::new(5818851782451133869);
        let a19 = a19 * Field::new(11322249509082494407);
        let a2 = a2 + t;
        let (a2, a36) = (a2 + a36, a2 - a36);
        let (a19, a53) = (a19 + a53, a19 - a53);
        let a53 = (a53 << 48);
        let (a2, a19) = (a2 + a19, a2 - a19);
        let (a36, a53) = (a36 + a53, a36 - a53);
        let (a62, a28) = (a62 + a28, a62 - a28);
        let (a11, a79) = (a11 + a79, a11 - a79);
        let a79 = (a79 << 48);
        let (a62, a11) = (a62 + a11, a62 - a11);
        let (a28, a79) = (a28 + a79, a28 - a79);
        let t = a45;
        let a45 = a45 + a62;
        let a62 = a62 * Field::new(4611686017353646080);
        let a28 = a28 * Field::new(16181989089180173841);
        let a11 = a11 * Field::new(5818851782451133869);
        let a79 = a79 * Field::new(11322249509082494407);
        let a62 = a62 + t;
        let (a62, a11) = (a62 + a11, a62 - a11);
        let (a79, a28) = (a79 + a28, a79 - a28);
        let a28 = (a28 << 48);
        let (a62, a79) = (a62 + a79, a62 - a79);
        let (a11, a28) = (a11 + a28, a11 - a28);
        let (a52, a18) = (a52 + a18, a52 - a18);
        let (a1, a69) = (a1 + a69, a1 - a69);
        let a69 = (a69 << 48);
        let (a52, a1) = (a52 + a1, a52 - a1);
        let (a18, a69) = (a18 + a69, a18 - a69);
        let t = a35;
        let a35 = a35 + a52;
        let a52 = a52 * Field::new(4611686017353646080);
        let a18 = a18 * Field::new(16181989089180173841);
        let a1 = a1 * Field::new(5818851782451133869);
        let a69 = a69 * Field::new(11322249509082494407);
        let a52 = a52 + t;
        let (a52, a1) = (a52 + a1, a52 - a1);
        let (a69, a18) = (a69 + a18, a69 - a18);
        let a18 = (a18 << 48);
        let (a52, a69) = (a52 + a69, a52 - a69);
        let (a1, a18) = (a1 + a18, a1 - a18);
        let (a32, a83) = (a32 + a83, a32 - a83);
        let (a66, a49) = (a66 + a49, a66 - a49);
        let a49 = (a49 << 48);
        let (a32, a66) = (a32 + a66, a32 - a66);
        let (a83, a49) = (a83 + a49, a83 - a49);
        let t = a15;
        let a15 = a15 + a32;
        let a32 = a32 * Field::new(4611686017353646080);
        let a83 = a83 * Field::new(16181989089180173841);
        let a66 = a66 * Field::new(5818851782451133869);
        let a49 = a49 * Field::new(11322249509082494407);
        let a32 = a32 + t;
        let (a32, a66) = (a32 + a66, a32 - a66);
        let (a49, a83) = (a49 + a83, a49 - a83);
        let a83 = (a83 << 48);
        let (a32, a49) = (a32 + a49, a32 - a49);
        let (a66, a83) = (a66 + a83, a66 - a83);
        values[0] = a0;
        values[1] = a22;
        values[2] = a21;
        values[3] = a13;
        values[4] = a59;
        values[5] = a40;
        values[6] = a12;
        values[7] = a31;
        values[8] = a33;
        values[9] = a9;
        values[10] = a10;
        values[11] = a37;
        values[12] = a41;
        values[13] = a53;
        values[14] = a79;
        values[15] = a35;
        values[16] = a32;
        values[17] = a51;
        values[18] = a73;
        values[19] = a4;
        values[20] = a30;
        values[21] = a42;
        values[22] = a6;
        values[23] = a63;
        values[24] = a14;
        values[25] = a50;
        values[26] = a77;
        values[27] = a61;
        values[28] = a3;
        values[29] = a24;
        values[30] = a70;
        values[31] = a62;
        values[32] = a1;
        values[33] = a83;
        values[34] = a34;
        values[35] = a5;
        values[36] = a72;
        values[37] = a81;
        values[38] = a8;
        values[39] = a74;
        values[40] = a80;
        values[41] = a82;
        values[42] = a16;
        values[43] = a43;
        values[44] = a44;
        values[45] = a20;
        values[46] = a7;
        values[47] = a36;
        values[48] = a28;
        values[49] = a69;
        values[50] = a15;
        values[51] = a17;
        values[52] = a56;
        values[53] = a38;
        values[54] = a64;
        values[55] = a25;
        values[56] = a57;
        values[57] = a46;
        values[58] = a48;
        values[59] = a84;
        values[60] = a60;
        values[61] = a27;
        values[62] = a71;
        values[63] = a58;
        values[64] = a19;
        values[65] = a45;
        values[66] = a52;
        values[67] = a66;
        values[68] = a68;
        values[69] = a39;
        values[70] = a55;
        values[71] = a47;
        values[72] = a76;
        values[73] = a23;
        values[74] = a29;
        values[75] = a65;
        values[76] = a67;
        values[77] = a26;
        values[78] = a78;
        values[79] = a54;
        values[80] = a75;
        values[81] = a2;
        values[82] = a11;
        values[83] = a18;
        values[84] = a49;
    }
}

/// Size 96 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 96`.
pub fn ntt_96(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 96, 0);
    for values in values.chunks_exact_mut(96) {
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
        let (a48, a54) = (a48 + a54, a48 - a54);
        let (a51, a57) = (a51 + a57, a51 - a57);
        let a57 = (a57 << 48);
        let (a48, a51) = (a48 + a51, a48 - a51);
        let (a54, a57) = (a54 + a57, a54 - a57);
        let (a52, a58) = (a52 + a58, a52 - a58);
        let (a55, a49) = (a55 + a49, a55 - a49);
        let a49 = (a49 << 48);
        let (a52, a55) = (a52 + a55, a52 - a55);
        let (a58, a49) = (a58 + a49, a58 - a49);
        let (a56, a50) = (a56 + a50, a56 - a50);
        let (a59, a53) = (a59 + a53, a59 - a53);
        let a53 = (a53 << 48);
        let (a56, a59) = (a56 + a59, a56 - a59);
        let (a50, a53) = (a50 + a53, a50 - a53);
        let (a48, a52, a56) = (
            a48 + a52 + a56,
            a48 + (a52 << 64) - (a56 << 32),
            a48 - (a52 << 32) + (a56 << 64),
        );
        let (a54, a58, a50) = (
            a54 + a58 + a50,
            a54 + (a58 << 64) - (a50 << 32),
            a54 - (a58 << 32) + (a50 << 64),
        );
        let (a51, a55, a59) = (
            a51 + a55 + a59,
            a51 + (a55 << 64) - (a59 << 32),
            a51 - (a55 << 32) + (a59 << 64),
        );
        let (a57, a49, a53) = (
            a57 + a49 + a53,
            a57 + (a49 << 64) - (a53 << 32),
            a57 - (a49 << 32) + (a53 << 64),
        );
        let (a24, a30) = (a24 + a30, a24 - a30);
        let (a27, a33) = (a27 + a33, a27 - a33);
        let a33 = (a33 << 48);
        let (a24, a27) = (a24 + a27, a24 - a27);
        let (a30, a33) = (a30 + a33, a30 - a33);
        let (a28, a34) = (a28 + a34, a28 - a34);
        let (a31, a25) = (a31 + a25, a31 - a25);
        let a25 = (a25 << 48);
        let (a28, a31) = (a28 + a31, a28 - a31);
        let (a34, a25) = (a34 + a25, a34 - a25);
        let (a32, a26) = (a32 + a26, a32 - a26);
        let (a35, a29) = (a35 + a29, a35 - a29);
        let a29 = (a29 << 48);
        let (a32, a35) = (a32 + a35, a32 - a35);
        let (a26, a29) = (a26 + a29, a26 - a29);
        let (a24, a28, a32) = (
            a24 + a28 + a32,
            a24 + (a28 << 64) - (a32 << 32),
            a24 - (a28 << 32) + (a32 << 64),
        );
        let (a30, a34, a26) = (
            a30 + a34 + a26,
            a30 + (a34 << 64) - (a26 << 32),
            a30 - (a34 << 32) + (a26 << 64),
        );
        let (a27, a31, a35) = (
            a27 + a31 + a35,
            a27 + (a31 << 64) - (a35 << 32),
            a27 - (a31 << 32) + (a35 << 64),
        );
        let (a33, a25, a29) = (
            a33 + a25 + a29,
            a33 + (a25 << 64) - (a29 << 32),
            a33 - (a25 << 32) + (a29 << 64),
        );
        let (a72, a78) = (a72 + a78, a72 - a78);
        let (a75, a81) = (a75 + a81, a75 - a81);
        let a81 = (a81 << 48);
        let (a72, a75) = (a72 + a75, a72 - a75);
        let (a78, a81) = (a78 + a81, a78 - a81);
        let (a76, a82) = (a76 + a82, a76 - a82);
        let (a79, a73) = (a79 + a73, a79 - a73);
        let a73 = (a73 << 48);
        let (a76, a79) = (a76 + a79, a76 - a79);
        let (a82, a73) = (a82 + a73, a82 - a73);
        let (a80, a74) = (a80 + a74, a80 - a74);
        let (a83, a77) = (a83 + a77, a83 - a77);
        let a77 = (a77 << 48);
        let (a80, a83) = (a80 + a83, a80 - a83);
        let (a74, a77) = (a74 + a77, a74 - a77);
        let (a72, a76, a80) = (
            a72 + a76 + a80,
            a72 + (a76 << 64) - (a80 << 32),
            a72 - (a76 << 32) + (a80 << 64),
        );
        let (a78, a82, a74) = (
            a78 + a82 + a74,
            a78 + (a82 << 64) - (a74 << 32),
            a78 - (a82 << 32) + (a74 << 64),
        );
        let (a75, a79, a83) = (
            a75 + a79 + a83,
            a75 + (a79 << 64) - (a83 << 32),
            a75 - (a79 << 32) + (a83 << 64),
        );
        let (a81, a73, a77) = (
            a81 + a73 + a77,
            a81 + (a73 << 64) - (a77 << 32),
            a81 - (a73 << 32) + (a77 << 64),
        );
        let (a12, a18) = (a12 + a18, a12 - a18);
        let (a15, a21) = (a15 + a21, a15 - a21);
        let a21 = (a21 << 48);
        let (a12, a15) = (a12 + a15, a12 - a15);
        let (a18, a21) = (a18 + a21, a18 - a21);
        let (a16, a22) = (a16 + a22, a16 - a22);
        let (a19, a13) = (a19 + a13, a19 - a13);
        let a13 = (a13 << 48);
        let (a16, a19) = (a16 + a19, a16 - a19);
        let (a22, a13) = (a22 + a13, a22 - a13);
        let (a20, a14) = (a20 + a14, a20 - a14);
        let (a23, a17) = (a23 + a17, a23 - a17);
        let a17 = (a17 << 48);
        let (a20, a23) = (a20 + a23, a20 - a23);
        let (a14, a17) = (a14 + a17, a14 - a17);
        let (a12, a16, a20) = (
            a12 + a16 + a20,
            a12 + (a16 << 64) - (a20 << 32),
            a12 - (a16 << 32) + (a20 << 64),
        );
        let (a18, a22, a14) = (
            a18 + a22 + a14,
            a18 + (a22 << 64) - (a14 << 32),
            a18 - (a22 << 32) + (a14 << 64),
        );
        let (a15, a19, a23) = (
            a15 + a19 + a23,
            a15 + (a19 << 64) - (a23 << 32),
            a15 - (a19 << 32) + (a23 << 64),
        );
        let (a21, a13, a17) = (
            a21 + a13 + a17,
            a21 + (a13 << 64) - (a17 << 32),
            a21 - (a13 << 32) + (a17 << 64),
        );
        let (a60, a66) = (a60 + a66, a60 - a66);
        let (a63, a69) = (a63 + a69, a63 - a69);
        let a69 = (a69 << 48);
        let (a60, a63) = (a60 + a63, a60 - a63);
        let (a66, a69) = (a66 + a69, a66 - a69);
        let (a64, a70) = (a64 + a70, a64 - a70);
        let (a67, a61) = (a67 + a61, a67 - a61);
        let a61 = (a61 << 48);
        let (a64, a67) = (a64 + a67, a64 - a67);
        let (a70, a61) = (a70 + a61, a70 - a61);
        let (a68, a62) = (a68 + a62, a68 - a62);
        let (a71, a65) = (a71 + a65, a71 - a65);
        let a65 = (a65 << 48);
        let (a68, a71) = (a68 + a71, a68 - a71);
        let (a62, a65) = (a62 + a65, a62 - a65);
        let (a60, a64, a68) = (
            a60 + a64 + a68,
            a60 + (a64 << 64) - (a68 << 32),
            a60 - (a64 << 32) + (a68 << 64),
        );
        let (a66, a70, a62) = (
            a66 + a70 + a62,
            a66 + (a70 << 64) - (a62 << 32),
            a66 - (a70 << 32) + (a62 << 64),
        );
        let (a63, a67, a71) = (
            a63 + a67 + a71,
            a63 + (a67 << 64) - (a71 << 32),
            a63 - (a67 << 32) + (a71 << 64),
        );
        let (a69, a61, a65) = (
            a69 + a61 + a65,
            a69 + (a61 << 64) - (a65 << 32),
            a69 - (a61 << 32) + (a65 << 64),
        );
        let (a36, a42) = (a36 + a42, a36 - a42);
        let (a39, a45) = (a39 + a45, a39 - a45);
        let a45 = (a45 << 48);
        let (a36, a39) = (a36 + a39, a36 - a39);
        let (a42, a45) = (a42 + a45, a42 - a45);
        let (a40, a46) = (a40 + a46, a40 - a46);
        let (a43, a37) = (a43 + a37, a43 - a37);
        let a37 = (a37 << 48);
        let (a40, a43) = (a40 + a43, a40 - a43);
        let (a46, a37) = (a46 + a37, a46 - a37);
        let (a44, a38) = (a44 + a38, a44 - a38);
        let (a47, a41) = (a47 + a41, a47 - a41);
        let a41 = (a41 << 48);
        let (a44, a47) = (a44 + a47, a44 - a47);
        let (a38, a41) = (a38 + a41, a38 - a41);
        let (a36, a40, a44) = (
            a36 + a40 + a44,
            a36 + (a40 << 64) - (a44 << 32),
            a36 - (a40 << 32) + (a44 << 64),
        );
        let (a42, a46, a38) = (
            a42 + a46 + a38,
            a42 + (a46 << 64) - (a38 << 32),
            a42 - (a46 << 32) + (a38 << 64),
        );
        let (a39, a43, a47) = (
            a39 + a43 + a47,
            a39 + (a43 << 64) - (a47 << 32),
            a39 - (a43 << 32) + (a47 << 64),
        );
        let (a45, a37, a41) = (
            a45 + a37 + a41,
            a45 + (a37 << 64) - (a41 << 32),
            a45 - (a37 << 32) + (a41 << 64),
        );
        let (a84, a90) = (a84 + a90, a84 - a90);
        let (a87, a93) = (a87 + a93, a87 - a93);
        let a93 = (a93 << 48);
        let (a84, a87) = (a84 + a87, a84 - a87);
        let (a90, a93) = (a90 + a93, a90 - a93);
        let (a88, a94) = (a88 + a94, a88 - a94);
        let (a91, a85) = (a91 + a85, a91 - a85);
        let a85 = (a85 << 48);
        let (a88, a91) = (a88 + a91, a88 - a91);
        let (a94, a85) = (a94 + a85, a94 - a85);
        let (a92, a86) = (a92 + a86, a92 - a86);
        let (a95, a89) = (a95 + a89, a95 - a89);
        let a89 = (a89 << 48);
        let (a92, a95) = (a92 + a95, a92 - a95);
        let (a86, a89) = (a86 + a89, a86 - a89);
        let (a84, a88, a92) = (
            a84 + a88 + a92,
            a84 + (a88 << 64) - (a92 << 32),
            a84 - (a88 << 32) + (a92 << 64),
        );
        let (a90, a94, a86) = (
            a90 + a94 + a86,
            a90 + (a94 << 64) - (a86 << 32),
            a90 - (a94 << 32) + (a86 << 64),
        );
        let (a87, a91, a95) = (
            a87 + a91 + a95,
            a87 + (a91 << 64) - (a95 << 32),
            a87 - (a91 << 32) + (a95 << 64),
        );
        let (a93, a85, a89) = (
            a93 + a85 + a89,
            a93 + (a85 << 64) - (a89 << 32),
            a93 - (a85 << 32) + (a89 << 64),
        );
        values[0] = a0;
        values[1] = a48;
        values[2] = a24;
        values[3] = a72;
        values[4] = a12;
        values[5] = a60;
        values[6] = a36;
        values[7] = a84;
        values[8] = a10;
        values[9] = a58;
        values[10] = a34;
        values[11] = a82;
        values[12] = a22;
        values[13] = a70;
        values[14] = a46;
        values[15] = a94;
        values[16] = a11;
        values[17] = a59;
        values[18] = a35;
        values[19] = a83;
        values[20] = a23;
        values[21] = a71;
        values[22] = a47;
        values[23] = a95;
        values[24] = a9;
        values[25] = a57;
        values[26] = a33;
        values[27] = a81;
        values[28] = a21;
        values[29] = a69;
        values[30] = a45;
        values[31] = a93;
        values[32] = a4;
        values[33] = a52;
        values[34] = a28;
        values[35] = a76;
        values[36] = a16;
        values[37] = a64;
        values[38] = a40;
        values[39] = a88;
        values[40] = a2;
        values[41] = a50;
        values[42] = a26;
        values[43] = a74;
        values[44] = a14;
        values[45] = a62;
        values[46] = a38;
        values[47] = a86;
        values[48] = a3;
        values[49] = a51;
        values[50] = a27;
        values[51] = a75;
        values[52] = a15;
        values[53] = a63;
        values[54] = a39;
        values[55] = a87;
        values[56] = a1;
        values[57] = a49;
        values[58] = a25;
        values[59] = a73;
        values[60] = a13;
        values[61] = a61;
        values[62] = a37;
        values[63] = a85;
        values[64] = a8;
        values[65] = a56;
        values[66] = a32;
        values[67] = a80;
        values[68] = a20;
        values[69] = a68;
        values[70] = a44;
        values[71] = a92;
        values[72] = a6;
        values[73] = a54;
        values[74] = a30;
        values[75] = a78;
        values[76] = a18;
        values[77] = a66;
        values[78] = a42;
        values[79] = a90;
        values[80] = a7;
        values[81] = a55;
        values[82] = a31;
        values[83] = a79;
        values[84] = a19;
        values[85] = a67;
        values[86] = a43;
        values[87] = a91;
        values[88] = a5;
        values[89] = a53;
        values[90] = a29;
        values[91] = a77;
        values[92] = a17;
        values[93] = a65;
        values[94] = a41;
        values[95] = a89;
    }
}

/// Size 102 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 102`.
pub fn ntt_102(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 102, 0);
    for values in values.chunks_exact_mut(102) {
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
        let (a6, a96) = (a6 + a96, a6 - a96);
        let (a24, a78) = (a24 + a78, a24 - a78);
        let a78 = (a78 << 48);
        let (a6, a24) = (a6 + a24, a6 - a24);
        let (a96, a78) = (a96 + a78, a96 - a78);
        let (a36, a66) = (a36 + a66, a36 - a66);
        let (a42, a60) = (a42 + a60, a42 - a60);
        let a60 = (a60 << 48);
        let (a36, a42) = (a36 + a42, a36 - a42);
        let (a66, a60) = (a66 + a60, a66 - a60);
        let (a12, a90) = (a12 + a90, a12 - a90);
        let (a48, a54) = (a48 + a54, a48 - a54);
        let a54 = (a54 << 48);
        let (a12, a48) = (a12 + a48, a12 - a48);
        let (a90, a54) = (a90 + a54, a90 - a54);
        let (a72, a30) = (a72 + a30, a72 - a30);
        let (a84, a18) = (a84 + a18, a84 - a18);
        let a18 = (a18 << 48);
        let (a72, a84) = (a72 + a84, a72 - a84);
        let (a30, a18) = (a30 + a18, a30 - a18);
        let a66 = (a66 << 12);
        let a42 = (a42 << 24);
        let a60 = (a60 << 36);
        let a90 = (a90 << 24);
        let a48 = (a48 << 48);
        let a54 = (a54 << 72);
        let a30 = (a30 << 36);
        let a84 = (a84 << 72);
        let a18 = (-(a18 << 12));
        let (a6, a12) = (a6 + a12, a6 - a12);
        let (a36, a72) = (a36 + a72, a36 - a72);
        let a72 = (a72 << 48);
        let (a6, a36) = (a6 + a36, a6 - a36);
        let (a12, a72) = (a12 + a72, a12 - a72);
        let (a96, a90) = (a96 + a90, a96 - a90);
        let (a66, a30) = (a66 + a30, a66 - a30);
        let a30 = (a30 << 48);
        let (a96, a66) = (a96 + a66, a96 - a66);
        let (a90, a30) = (a90 + a30, a90 - a30);
        let (a24, a48) = (a24 + a48, a24 - a48);
        let (a42, a84) = (a42 + a84, a42 - a84);
        let a84 = (a84 << 48);
        let (a24, a42) = (a24 + a42, a24 - a42);
        let (a48, a84) = (a48 + a84, a48 - a84);
        let (a78, a54) = (a78 + a54, a78 - a54);
        let (a60, a18) = (a60 + a18, a60 - a18);
        let a18 = (a18 << 48);
        let (a78, a60) = (a78 + a60, a78 - a60);
        let (a54, a18) = (a54 + a18, a54 - a18);
        let t = a0;
        let a0 = a0 + a6;
        let a6 = a6 * Field::new(1152921504338411520);
        let a96 = a96 * Field::new(6259776822214049175);
        let a24 = a24 * Field::new(9380094172986290191);
        let a78 = a78 * Field::new(891943630314919127);
        let a12 = a12 * Field::new(17228171707553225791);
        let a90 = a90 * Field::new(12855743360534130886);
        let a48 = a48 * Field::new(6167687396920564837);
        let a54 = a54 * Field::new(17201834061724655524);
        let a36 = a36 * Field::new(15308299771656910737);
        let a66 = a66 * Field::new(18186005861103657533);
        let a42 = a42 * Field::new(53595491891823545);
        let a60 = a60 * Field::new(1906638201581172103);
        let a72 = a72 * Field::new(18303651001328874822);
        let a30 = a30 * Field::new(3077466521755967626);
        let a84 = a84 * Field::new(12423593102987598328);
        let a18 = a18 * Field::new(18361513053649472048);
        let a6 = a6 + t;
        let (a6, a36) = (a6 + a36, a6 - a36);
        let (a72, a12) = (a72 + a12, a72 - a12);
        let a12 = (a12 << 48);
        let (a6, a72) = (a6 + a72, a6 - a72);
        let (a36, a12) = (a36 + a12, a36 - a12);
        let (a18, a54) = (a18 + a54, a18 - a54);
        let (a60, a78) = (a60 + a78, a60 - a78);
        let a78 = (a78 << 48);
        let (a18, a60) = (a18 + a60, a18 - a60);
        let (a54, a78) = (a54 + a78, a54 - a78);
        let (a84, a48) = (a84 + a48, a84 - a48);
        let (a42, a24) = (a42 + a24, a42 - a24);
        let a24 = (a24 << 48);
        let (a84, a42) = (a84 + a42, a84 - a42);
        let (a48, a24) = (a48 + a24, a48 - a24);
        let (a30, a90) = (a30 + a90, a30 - a90);
        let (a66, a96) = (a66 + a96, a66 - a96);
        let a96 = (a96 << 48);
        let (a30, a66) = (a30 + a66, a30 - a66);
        let (a90, a96) = (a90 + a96, a90 - a96);
        let a54 = (a54 << 12);
        let a60 = (a60 << 24);
        let a78 = (a78 << 36);
        let a48 = (a48 << 24);
        let a42 = (a42 << 48);
        let a24 = (a24 << 72);
        let a90 = (a90 << 36);
        let a66 = (a66 << 72);
        let a96 = (-(a96 << 12));
        let (a6, a84) = (a6 + a84, a6 - a84);
        let (a18, a30) = (a18 + a30, a18 - a30);
        let a30 = (a30 << 48);
        let (a6, a18) = (a6 + a18, a6 - a18);
        let (a84, a30) = (a84 + a30, a84 - a30);
        let (a36, a48) = (a36 + a48, a36 - a48);
        let (a54, a90) = (a54 + a90, a54 - a90);
        let a90 = (a90 << 48);
        let (a36, a54) = (a36 + a54, a36 - a54);
        let (a48, a90) = (a48 + a90, a48 - a90);
        let (a72, a42) = (a72 + a42, a72 - a42);
        let (a60, a66) = (a60 + a66, a60 - a66);
        let a66 = (a66 << 48);
        let (a72, a60) = (a72 + a60, a72 - a60);
        let (a42, a66) = (a42 + a66, a42 - a66);
        let (a12, a24) = (a12 + a24, a12 - a24);
        let (a78, a96) = (a78 + a96, a78 - a96);
        let a96 = (a96 << 48);
        let (a12, a78) = (a12 + a78, a12 - a78);
        let (a24, a96) = (a24 + a96, a24 - a96);
        let (a23, a11) = (a23 + a11, a23 - a11);
        let (a41, a95) = (a41 + a95, a41 - a95);
        let a95 = (a95 << 48);
        let (a23, a41) = (a23 + a41, a23 - a41);
        let (a11, a95) = (a11 + a95, a11 - a95);
        let (a53, a83) = (a53 + a83, a53 - a83);
        let (a59, a77) = (a59 + a77, a59 - a77);
        let a77 = (a77 << 48);
        let (a53, a59) = (a53 + a59, a53 - a59);
        let (a83, a77) = (a83 + a77, a83 - a77);
        let (a29, a5) = (a29 + a5, a29 - a5);
        let (a65, a71) = (a65 + a71, a65 - a71);
        let a71 = (a71 << 48);
        let (a29, a65) = (a29 + a65, a29 - a65);
        let (a5, a71) = (a5 + a71, a5 - a71);
        let (a89, a47) = (a89 + a47, a89 - a47);
        let (a101, a35) = (a101 + a35, a101 - a35);
        let a35 = (a35 << 48);
        let (a89, a101) = (a89 + a101, a89 - a101);
        let (a47, a35) = (a47 + a35, a47 - a35);
        let a83 = (a83 << 12);
        let a59 = (a59 << 24);
        let a77 = (a77 << 36);
        let a5 = (a5 << 24);
        let a65 = (a65 << 48);
        let a71 = (a71 << 72);
        let a47 = (a47 << 36);
        let a101 = (a101 << 72);
        let a35 = (-(a35 << 12));
        let (a23, a29) = (a23 + a29, a23 - a29);
        let (a53, a89) = (a53 + a89, a53 - a89);
        let a89 = (a89 << 48);
        let (a23, a53) = (a23 + a53, a23 - a53);
        let (a29, a89) = (a29 + a89, a29 - a89);
        let (a11, a5) = (a11 + a5, a11 - a5);
        let (a83, a47) = (a83 + a47, a83 - a47);
        let a47 = (a47 << 48);
        let (a11, a83) = (a11 + a83, a11 - a83);
        let (a5, a47) = (a5 + a47, a5 - a47);
        let (a41, a65) = (a41 + a65, a41 - a65);
        let (a59, a101) = (a59 + a101, a59 - a101);
        let a101 = (a101 << 48);
        let (a41, a59) = (a41 + a59, a41 - a59);
        let (a65, a101) = (a65 + a101, a65 - a101);
        let (a95, a71) = (a95 + a71, a95 - a71);
        let (a77, a35) = (a77 + a35, a77 - a35);
        let a35 = (a35 << 48);
        let (a95, a77) = (a95 + a77, a95 - a77);
        let (a71, a35) = (a71 + a35, a71 - a35);
        let t = a17;
        let a17 = a17 + a23;
        let a23 = a23 * Field::new(1152921504338411520);
        let a11 = a11 * Field::new(6259776822214049175);
        let a41 = a41 * Field::new(9380094172986290191);
        let a95 = a95 * Field::new(891943630314919127);
        let a29 = a29 * Field::new(17228171707553225791);
        let a5 = a5 * Field::new(12855743360534130886);
        let a65 = a65 * Field::new(6167687396920564837);
        let a71 = a71 * Field::new(17201834061724655524);
        let a53 = a53 * Field::new(15308299771656910737);
        let a83 = a83 * Field::new(18186005861103657533);
        let a59 = a59 * Field::new(53595491891823545);
        let a77 = a77 * Field::new(1906638201581172103);
        let a89 = a89 * Field::new(18303651001328874822);
        let a47 = a47 * Field::new(3077466521755967626);
        let a101 = a101 * Field::new(12423593102987598328);
        let a35 = a35 * Field::new(18361513053649472048);
        let a23 = a23 + t;
        let (a23, a53) = (a23 + a53, a23 - a53);
        let (a89, a29) = (a89 + a29, a89 - a29);
        let a29 = (a29 << 48);
        let (a23, a89) = (a23 + a89, a23 - a89);
        let (a53, a29) = (a53 + a29, a53 - a29);
        let (a35, a71) = (a35 + a71, a35 - a71);
        let (a77, a95) = (a77 + a95, a77 - a95);
        let a95 = (a95 << 48);
        let (a35, a77) = (a35 + a77, a35 - a77);
        let (a71, a95) = (a71 + a95, a71 - a95);
        let (a101, a65) = (a101 + a65, a101 - a65);
        let (a59, a41) = (a59 + a41, a59 - a41);
        let a41 = (a41 << 48);
        let (a101, a59) = (a101 + a59, a101 - a59);
        let (a65, a41) = (a65 + a41, a65 - a41);
        let (a47, a5) = (a47 + a5, a47 - a5);
        let (a83, a11) = (a83 + a11, a83 - a11);
        let a11 = (a11 << 48);
        let (a47, a83) = (a47 + a83, a47 - a83);
        let (a5, a11) = (a5 + a11, a5 - a11);
        let a71 = (a71 << 12);
        let a77 = (a77 << 24);
        let a95 = (a95 << 36);
        let a65 = (a65 << 24);
        let a59 = (a59 << 48);
        let a41 = (a41 << 72);
        let a5 = (a5 << 36);
        let a83 = (a83 << 72);
        let a11 = (-(a11 << 12));
        let (a23, a101) = (a23 + a101, a23 - a101);
        let (a35, a47) = (a35 + a47, a35 - a47);
        let a47 = (a47 << 48);
        let (a23, a35) = (a23 + a35, a23 - a35);
        let (a101, a47) = (a101 + a47, a101 - a47);
        let (a53, a65) = (a53 + a65, a53 - a65);
        let (a71, a5) = (a71 + a5, a71 - a5);
        let a5 = (a5 << 48);
        let (a53, a71) = (a53 + a71, a53 - a71);
        let (a65, a5) = (a65 + a5, a65 - a5);
        let (a89, a59) = (a89 + a59, a89 - a59);
        let (a77, a83) = (a77 + a83, a77 - a83);
        let a83 = (a83 << 48);
        let (a89, a77) = (a89 + a77, a89 - a77);
        let (a59, a83) = (a59 + a83, a59 - a83);
        let (a29, a41) = (a29 + a41, a29 - a41);
        let (a95, a11) = (a95 + a11, a95 - a11);
        let a11 = (a11 << 48);
        let (a29, a95) = (a29 + a95, a29 - a95);
        let (a41, a11) = (a41 + a11, a41 - a11);
        let (a40, a28) = (a40 + a28, a40 - a28);
        let (a58, a10) = (a58 + a10, a58 - a10);
        let a10 = (a10 << 48);
        let (a40, a58) = (a40 + a58, a40 - a58);
        let (a28, a10) = (a28 + a10, a28 - a10);
        let (a70, a100) = (a70 + a100, a70 - a100);
        let (a76, a94) = (a76 + a94, a76 - a94);
        let a94 = (a94 << 48);
        let (a70, a76) = (a70 + a76, a70 - a76);
        let (a100, a94) = (a100 + a94, a100 - a94);
        let (a46, a22) = (a46 + a22, a46 - a22);
        let (a82, a88) = (a82 + a88, a82 - a88);
        let a88 = (a88 << 48);
        let (a46, a82) = (a46 + a82, a46 - a82);
        let (a22, a88) = (a22 + a88, a22 - a88);
        let (a4, a64) = (a4 + a64, a4 - a64);
        let (a16, a52) = (a16 + a52, a16 - a52);
        let a52 = (a52 << 48);
        let (a4, a16) = (a4 + a16, a4 - a16);
        let (a64, a52) = (a64 + a52, a64 - a52);
        let a100 = (a100 << 12);
        let a76 = (a76 << 24);
        let a94 = (a94 << 36);
        let a22 = (a22 << 24);
        let a82 = (a82 << 48);
        let a88 = (a88 << 72);
        let a64 = (a64 << 36);
        let a16 = (a16 << 72);
        let a52 = (-(a52 << 12));
        let (a40, a46) = (a40 + a46, a40 - a46);
        let (a70, a4) = (a70 + a4, a70 - a4);
        let a4 = (a4 << 48);
        let (a40, a70) = (a40 + a70, a40 - a70);
        let (a46, a4) = (a46 + a4, a46 - a4);
        let (a28, a22) = (a28 + a22, a28 - a22);
        let (a100, a64) = (a100 + a64, a100 - a64);
        let a64 = (a64 << 48);
        let (a28, a100) = (a28 + a100, a28 - a100);
        let (a22, a64) = (a22 + a64, a22 - a64);
        let (a58, a82) = (a58 + a82, a58 - a82);
        let (a76, a16) = (a76 + a16, a76 - a16);
        let a16 = (a16 << 48);
        let (a58, a76) = (a58 + a76, a58 - a76);
        let (a82, a16) = (a82 + a16, a82 - a16);
        let (a10, a88) = (a10 + a88, a10 - a88);
        let (a94, a52) = (a94 + a52, a94 - a52);
        let a52 = (a52 << 48);
        let (a10, a94) = (a10 + a94, a10 - a94);
        let (a88, a52) = (a88 + a52, a88 - a52);
        let t = a34;
        let a34 = a34 + a40;
        let a40 = a40 * Field::new(1152921504338411520);
        let a28 = a28 * Field::new(6259776822214049175);
        let a58 = a58 * Field::new(9380094172986290191);
        let a10 = a10 * Field::new(891943630314919127);
        let a46 = a46 * Field::new(17228171707553225791);
        let a22 = a22 * Field::new(12855743360534130886);
        let a82 = a82 * Field::new(6167687396920564837);
        let a88 = a88 * Field::new(17201834061724655524);
        let a70 = a70 * Field::new(15308299771656910737);
        let a100 = a100 * Field::new(18186005861103657533);
        let a76 = a76 * Field::new(53595491891823545);
        let a94 = a94 * Field::new(1906638201581172103);
        let a4 = a4 * Field::new(18303651001328874822);
        let a64 = a64 * Field::new(3077466521755967626);
        let a16 = a16 * Field::new(12423593102987598328);
        let a52 = a52 * Field::new(18361513053649472048);
        let a40 = a40 + t;
        let (a40, a70) = (a40 + a70, a40 - a70);
        let (a4, a46) = (a4 + a46, a4 - a46);
        let a46 = (a46 << 48);
        let (a40, a4) = (a40 + a4, a40 - a4);
        let (a70, a46) = (a70 + a46, a70 - a46);
        let (a52, a88) = (a52 + a88, a52 - a88);
        let (a94, a10) = (a94 + a10, a94 - a10);
        let a10 = (a10 << 48);
        let (a52, a94) = (a52 + a94, a52 - a94);
        let (a88, a10) = (a88 + a10, a88 - a10);
        let (a16, a82) = (a16 + a82, a16 - a82);
        let (a76, a58) = (a76 + a58, a76 - a58);
        let a58 = (a58 << 48);
        let (a16, a76) = (a16 + a76, a16 - a76);
        let (a82, a58) = (a82 + a58, a82 - a58);
        let (a64, a22) = (a64 + a22, a64 - a22);
        let (a100, a28) = (a100 + a28, a100 - a28);
        let a28 = (a28 << 48);
        let (a64, a100) = (a64 + a100, a64 - a100);
        let (a22, a28) = (a22 + a28, a22 - a28);
        let a88 = (a88 << 12);
        let a94 = (a94 << 24);
        let a10 = (a10 << 36);
        let a82 = (a82 << 24);
        let a76 = (a76 << 48);
        let a58 = (a58 << 72);
        let a22 = (a22 << 36);
        let a100 = (a100 << 72);
        let a28 = (-(a28 << 12));
        let (a40, a16) = (a40 + a16, a40 - a16);
        let (a52, a64) = (a52 + a64, a52 - a64);
        let a64 = (a64 << 48);
        let (a40, a52) = (a40 + a52, a40 - a52);
        let (a16, a64) = (a16 + a64, a16 - a64);
        let (a70, a82) = (a70 + a82, a70 - a82);
        let (a88, a22) = (a88 + a22, a88 - a22);
        let a22 = (a22 << 48);
        let (a70, a88) = (a70 + a88, a70 - a88);
        let (a82, a22) = (a82 + a22, a82 - a22);
        let (a4, a76) = (a4 + a76, a4 - a76);
        let (a94, a100) = (a94 + a100, a94 - a100);
        let a100 = (a100 << 48);
        let (a4, a94) = (a4 + a94, a4 - a94);
        let (a76, a100) = (a76 + a100, a76 - a100);
        let (a46, a58) = (a46 + a58, a46 - a58);
        let (a10, a28) = (a10 + a28, a10 - a28);
        let a28 = (a28 << 48);
        let (a46, a10) = (a46 + a10, a46 - a10);
        let (a58, a28) = (a58 + a28, a58 - a28);
        let (a57, a45) = (a57 + a45, a57 - a45);
        let (a75, a27) = (a75 + a27, a75 - a27);
        let a27 = (a27 << 48);
        let (a57, a75) = (a57 + a75, a57 - a75);
        let (a45, a27) = (a45 + a27, a45 - a27);
        let (a87, a15) = (a87 + a15, a87 - a15);
        let (a93, a9) = (a93 + a9, a93 - a9);
        let a9 = (a9 << 48);
        let (a87, a93) = (a87 + a93, a87 - a93);
        let (a15, a9) = (a15 + a9, a15 - a9);
        let (a63, a39) = (a63 + a39, a63 - a39);
        let (a99, a3) = (a99 + a3, a99 - a3);
        let a3 = (a3 << 48);
        let (a63, a99) = (a63 + a99, a63 - a99);
        let (a39, a3) = (a39 + a3, a39 - a3);
        let (a21, a81) = (a21 + a81, a21 - a81);
        let (a33, a69) = (a33 + a69, a33 - a69);
        let a69 = (a69 << 48);
        let (a21, a33) = (a21 + a33, a21 - a33);
        let (a81, a69) = (a81 + a69, a81 - a69);
        let a15 = (a15 << 12);
        let a93 = (a93 << 24);
        let a9 = (a9 << 36);
        let a39 = (a39 << 24);
        let a99 = (a99 << 48);
        let a3 = (a3 << 72);
        let a81 = (a81 << 36);
        let a33 = (a33 << 72);
        let a69 = (-(a69 << 12));
        let (a57, a63) = (a57 + a63, a57 - a63);
        let (a87, a21) = (a87 + a21, a87 - a21);
        let a21 = (a21 << 48);
        let (a57, a87) = (a57 + a87, a57 - a87);
        let (a63, a21) = (a63 + a21, a63 - a21);
        let (a45, a39) = (a45 + a39, a45 - a39);
        let (a15, a81) = (a15 + a81, a15 - a81);
        let a81 = (a81 << 48);
        let (a45, a15) = (a45 + a15, a45 - a15);
        let (a39, a81) = (a39 + a81, a39 - a81);
        let (a75, a99) = (a75 + a99, a75 - a99);
        let (a93, a33) = (a93 + a33, a93 - a33);
        let a33 = (a33 << 48);
        let (a75, a93) = (a75 + a93, a75 - a93);
        let (a99, a33) = (a99 + a33, a99 - a33);
        let (a27, a3) = (a27 + a3, a27 - a3);
        let (a9, a69) = (a9 + a69, a9 - a69);
        let a69 = (a69 << 48);
        let (a27, a9) = (a27 + a9, a27 - a9);
        let (a3, a69) = (a3 + a69, a3 - a69);
        let t = a51;
        let a51 = a51 + a57;
        let a57 = a57 * Field::new(1152921504338411520);
        let a45 = a45 * Field::new(6259776822214049175);
        let a75 = a75 * Field::new(9380094172986290191);
        let a27 = a27 * Field::new(891943630314919127);
        let a63 = a63 * Field::new(17228171707553225791);
        let a39 = a39 * Field::new(12855743360534130886);
        let a99 = a99 * Field::new(6167687396920564837);
        let a3 = a3 * Field::new(17201834061724655524);
        let a87 = a87 * Field::new(15308299771656910737);
        let a15 = a15 * Field::new(18186005861103657533);
        let a93 = a93 * Field::new(53595491891823545);
        let a9 = a9 * Field::new(1906638201581172103);
        let a21 = a21 * Field::new(18303651001328874822);
        let a81 = a81 * Field::new(3077466521755967626);
        let a33 = a33 * Field::new(12423593102987598328);
        let a69 = a69 * Field::new(18361513053649472048);
        let a57 = a57 + t;
        let (a57, a87) = (a57 + a87, a57 - a87);
        let (a21, a63) = (a21 + a63, a21 - a63);
        let a63 = (a63 << 48);
        let (a57, a21) = (a57 + a21, a57 - a21);
        let (a87, a63) = (a87 + a63, a87 - a63);
        let (a69, a3) = (a69 + a3, a69 - a3);
        let (a9, a27) = (a9 + a27, a9 - a27);
        let a27 = (a27 << 48);
        let (a69, a9) = (a69 + a9, a69 - a9);
        let (a3, a27) = (a3 + a27, a3 - a27);
        let (a33, a99) = (a33 + a99, a33 - a99);
        let (a93, a75) = (a93 + a75, a93 - a75);
        let a75 = (a75 << 48);
        let (a33, a93) = (a33 + a93, a33 - a93);
        let (a99, a75) = (a99 + a75, a99 - a75);
        let (a81, a39) = (a81 + a39, a81 - a39);
        let (a15, a45) = (a15 + a45, a15 - a45);
        let a45 = (a45 << 48);
        let (a81, a15) = (a81 + a15, a81 - a15);
        let (a39, a45) = (a39 + a45, a39 - a45);
        let a3 = (a3 << 12);
        let a9 = (a9 << 24);
        let a27 = (a27 << 36);
        let a99 = (a99 << 24);
        let a93 = (a93 << 48);
        let a75 = (a75 << 72);
        let a39 = (a39 << 36);
        let a15 = (a15 << 72);
        let a45 = (-(a45 << 12));
        let (a57, a33) = (a57 + a33, a57 - a33);
        let (a69, a81) = (a69 + a81, a69 - a81);
        let a81 = (a81 << 48);
        let (a57, a69) = (a57 + a69, a57 - a69);
        let (a33, a81) = (a33 + a81, a33 - a81);
        let (a87, a99) = (a87 + a99, a87 - a99);
        let (a3, a39) = (a3 + a39, a3 - a39);
        let a39 = (a39 << 48);
        let (a87, a3) = (a87 + a3, a87 - a3);
        let (a99, a39) = (a99 + a39, a99 - a39);
        let (a21, a93) = (a21 + a93, a21 - a93);
        let (a9, a15) = (a9 + a15, a9 - a15);
        let a15 = (a15 << 48);
        let (a21, a9) = (a21 + a9, a21 - a9);
        let (a93, a15) = (a93 + a15, a93 - a15);
        let (a63, a75) = (a63 + a75, a63 - a75);
        let (a27, a45) = (a27 + a45, a27 - a45);
        let a45 = (a45 << 48);
        let (a63, a27) = (a63 + a27, a63 - a27);
        let (a75, a45) = (a75 + a45, a75 - a45);
        let (a74, a62) = (a74 + a62, a74 - a62);
        let (a92, a44) = (a92 + a44, a92 - a44);
        let a44 = (a44 << 48);
        let (a74, a92) = (a74 + a92, a74 - a92);
        let (a62, a44) = (a62 + a44, a62 - a44);
        let (a2, a32) = (a2 + a32, a2 - a32);
        let (a8, a26) = (a8 + a26, a8 - a26);
        let a26 = (a26 << 48);
        let (a2, a8) = (a2 + a8, a2 - a8);
        let (a32, a26) = (a32 + a26, a32 - a26);
        let (a80, a56) = (a80 + a56, a80 - a56);
        let (a14, a20) = (a14 + a20, a14 - a20);
        let a20 = (a20 << 48);
        let (a80, a14) = (a80 + a14, a80 - a14);
        let (a56, a20) = (a56 + a20, a56 - a20);
        let (a38, a98) = (a38 + a98, a38 - a98);
        let (a50, a86) = (a50 + a86, a50 - a86);
        let a86 = (a86 << 48);
        let (a38, a50) = (a38 + a50, a38 - a50);
        let (a98, a86) = (a98 + a86, a98 - a86);
        let a32 = (a32 << 12);
        let a8 = (a8 << 24);
        let a26 = (a26 << 36);
        let a56 = (a56 << 24);
        let a14 = (a14 << 48);
        let a20 = (a20 << 72);
        let a98 = (a98 << 36);
        let a50 = (a50 << 72);
        let a86 = (-(a86 << 12));
        let (a74, a80) = (a74 + a80, a74 - a80);
        let (a2, a38) = (a2 + a38, a2 - a38);
        let a38 = (a38 << 48);
        let (a74, a2) = (a74 + a2, a74 - a2);
        let (a80, a38) = (a80 + a38, a80 - a38);
        let (a62, a56) = (a62 + a56, a62 - a56);
        let (a32, a98) = (a32 + a98, a32 - a98);
        let a98 = (a98 << 48);
        let (a62, a32) = (a62 + a32, a62 - a32);
        let (a56, a98) = (a56 + a98, a56 - a98);
        let (a92, a14) = (a92 + a14, a92 - a14);
        let (a8, a50) = (a8 + a50, a8 - a50);
        let a50 = (a50 << 48);
        let (a92, a8) = (a92 + a8, a92 - a8);
        let (a14, a50) = (a14 + a50, a14 - a50);
        let (a44, a20) = (a44 + a20, a44 - a20);
        let (a26, a86) = (a26 + a86, a26 - a86);
        let a86 = (a86 << 48);
        let (a44, a26) = (a44 + a26, a44 - a26);
        let (a20, a86) = (a20 + a86, a20 - a86);
        let t = a68;
        let a68 = a68 + a74;
        let a74 = a74 * Field::new(1152921504338411520);
        let a62 = a62 * Field::new(6259776822214049175);
        let a92 = a92 * Field::new(9380094172986290191);
        let a44 = a44 * Field::new(891943630314919127);
        let a80 = a80 * Field::new(17228171707553225791);
        let a56 = a56 * Field::new(12855743360534130886);
        let a14 = a14 * Field::new(6167687396920564837);
        let a20 = a20 * Field::new(17201834061724655524);
        let a2 = a2 * Field::new(15308299771656910737);
        let a32 = a32 * Field::new(18186005861103657533);
        let a8 = a8 * Field::new(53595491891823545);
        let a26 = a26 * Field::new(1906638201581172103);
        let a38 = a38 * Field::new(18303651001328874822);
        let a98 = a98 * Field::new(3077466521755967626);
        let a50 = a50 * Field::new(12423593102987598328);
        let a86 = a86 * Field::new(18361513053649472048);
        let a74 = a74 + t;
        let (a74, a2) = (a74 + a2, a74 - a2);
        let (a38, a80) = (a38 + a80, a38 - a80);
        let a80 = (a80 << 48);
        let (a74, a38) = (a74 + a38, a74 - a38);
        let (a2, a80) = (a2 + a80, a2 - a80);
        let (a86, a20) = (a86 + a20, a86 - a20);
        let (a26, a44) = (a26 + a44, a26 - a44);
        let a44 = (a44 << 48);
        let (a86, a26) = (a86 + a26, a86 - a26);
        let (a20, a44) = (a20 + a44, a20 - a44);
        let (a50, a14) = (a50 + a14, a50 - a14);
        let (a8, a92) = (a8 + a92, a8 - a92);
        let a92 = (a92 << 48);
        let (a50, a8) = (a50 + a8, a50 - a8);
        let (a14, a92) = (a14 + a92, a14 - a92);
        let (a98, a56) = (a98 + a56, a98 - a56);
        let (a32, a62) = (a32 + a62, a32 - a62);
        let a62 = (a62 << 48);
        let (a98, a32) = (a98 + a32, a98 - a32);
        let (a56, a62) = (a56 + a62, a56 - a62);
        let a20 = (a20 << 12);
        let a26 = (a26 << 24);
        let a44 = (a44 << 36);
        let a14 = (a14 << 24);
        let a8 = (a8 << 48);
        let a92 = (a92 << 72);
        let a56 = (a56 << 36);
        let a32 = (a32 << 72);
        let a62 = (-(a62 << 12));
        let (a74, a50) = (a74 + a50, a74 - a50);
        let (a86, a98) = (a86 + a98, a86 - a98);
        let a98 = (a98 << 48);
        let (a74, a86) = (a74 + a86, a74 - a86);
        let (a50, a98) = (a50 + a98, a50 - a98);
        let (a2, a14) = (a2 + a14, a2 - a14);
        let (a20, a56) = (a20 + a56, a20 - a56);
        let a56 = (a56 << 48);
        let (a2, a20) = (a2 + a20, a2 - a20);
        let (a14, a56) = (a14 + a56, a14 - a56);
        let (a38, a8) = (a38 + a8, a38 - a8);
        let (a26, a32) = (a26 + a32, a26 - a32);
        let a32 = (a32 << 48);
        let (a38, a26) = (a38 + a26, a38 - a26);
        let (a8, a32) = (a8 + a32, a8 - a32);
        let (a80, a92) = (a80 + a92, a80 - a92);
        let (a44, a62) = (a44 + a62, a44 - a62);
        let a62 = (a62 << 48);
        let (a80, a44) = (a80 + a44, a80 - a44);
        let (a92, a62) = (a92 + a62, a92 - a62);
        let (a91, a79) = (a91 + a79, a91 - a79);
        let (a7, a61) = (a7 + a61, a7 - a61);
        let a61 = (a61 << 48);
        let (a91, a7) = (a91 + a7, a91 - a7);
        let (a79, a61) = (a79 + a61, a79 - a61);
        let (a19, a49) = (a19 + a49, a19 - a49);
        let (a25, a43) = (a25 + a43, a25 - a43);
        let a43 = (a43 << 48);
        let (a19, a25) = (a19 + a25, a19 - a25);
        let (a49, a43) = (a49 + a43, a49 - a43);
        let (a97, a73) = (a97 + a73, a97 - a73);
        let (a31, a37) = (a31 + a37, a31 - a37);
        let a37 = (a37 << 48);
        let (a97, a31) = (a97 + a31, a97 - a31);
        let (a73, a37) = (a73 + a37, a73 - a37);
        let (a55, a13) = (a55 + a13, a55 - a13);
        let (a67, a1) = (a67 + a1, a67 - a1);
        let a1 = (a1 << 48);
        let (a55, a67) = (a55 + a67, a55 - a67);
        let (a13, a1) = (a13 + a1, a13 - a1);
        let a49 = (a49 << 12);
        let a25 = (a25 << 24);
        let a43 = (a43 << 36);
        let a73 = (a73 << 24);
        let a31 = (a31 << 48);
        let a37 = (a37 << 72);
        let a13 = (a13 << 36);
        let a67 = (a67 << 72);
        let a1 = (-(a1 << 12));
        let (a91, a97) = (a91 + a97, a91 - a97);
        let (a19, a55) = (a19 + a55, a19 - a55);
        let a55 = (a55 << 48);
        let (a91, a19) = (a91 + a19, a91 - a19);
        let (a97, a55) = (a97 + a55, a97 - a55);
        let (a79, a73) = (a79 + a73, a79 - a73);
        let (a49, a13) = (a49 + a13, a49 - a13);
        let a13 = (a13 << 48);
        let (a79, a49) = (a79 + a49, a79 - a49);
        let (a73, a13) = (a73 + a13, a73 - a13);
        let (a7, a31) = (a7 + a31, a7 - a31);
        let (a25, a67) = (a25 + a67, a25 - a67);
        let a67 = (a67 << 48);
        let (a7, a25) = (a7 + a25, a7 - a25);
        let (a31, a67) = (a31 + a67, a31 - a67);
        let (a61, a37) = (a61 + a37, a61 - a37);
        let (a43, a1) = (a43 + a1, a43 - a1);
        let a1 = (a1 << 48);
        let (a61, a43) = (a61 + a43, a61 - a43);
        let (a37, a1) = (a37 + a1, a37 - a1);
        let t = a85;
        let a85 = a85 + a91;
        let a91 = a91 * Field::new(1152921504338411520);
        let a79 = a79 * Field::new(6259776822214049175);
        let a7 = a7 * Field::new(9380094172986290191);
        let a61 = a61 * Field::new(891943630314919127);
        let a97 = a97 * Field::new(17228171707553225791);
        let a73 = a73 * Field::new(12855743360534130886);
        let a31 = a31 * Field::new(6167687396920564837);
        let a37 = a37 * Field::new(17201834061724655524);
        let a19 = a19 * Field::new(15308299771656910737);
        let a49 = a49 * Field::new(18186005861103657533);
        let a25 = a25 * Field::new(53595491891823545);
        let a43 = a43 * Field::new(1906638201581172103);
        let a55 = a55 * Field::new(18303651001328874822);
        let a13 = a13 * Field::new(3077466521755967626);
        let a67 = a67 * Field::new(12423593102987598328);
        let a1 = a1 * Field::new(18361513053649472048);
        let a91 = a91 + t;
        let (a91, a19) = (a91 + a19, a91 - a19);
        let (a55, a97) = (a55 + a97, a55 - a97);
        let a97 = (a97 << 48);
        let (a91, a55) = (a91 + a55, a91 - a55);
        let (a19, a97) = (a19 + a97, a19 - a97);
        let (a1, a37) = (a1 + a37, a1 - a37);
        let (a43, a61) = (a43 + a61, a43 - a61);
        let a61 = (a61 << 48);
        let (a1, a43) = (a1 + a43, a1 - a43);
        let (a37, a61) = (a37 + a61, a37 - a61);
        let (a67, a31) = (a67 + a31, a67 - a31);
        let (a25, a7) = (a25 + a7, a25 - a7);
        let a7 = (a7 << 48);
        let (a67, a25) = (a67 + a25, a67 - a25);
        let (a31, a7) = (a31 + a7, a31 - a7);
        let (a13, a73) = (a13 + a73, a13 - a73);
        let (a49, a79) = (a49 + a79, a49 - a79);
        let a79 = (a79 << 48);
        let (a13, a49) = (a13 + a49, a13 - a49);
        let (a73, a79) = (a73 + a79, a73 - a79);
        let a37 = (a37 << 12);
        let a43 = (a43 << 24);
        let a61 = (a61 << 36);
        let a31 = (a31 << 24);
        let a25 = (a25 << 48);
        let a7 = (a7 << 72);
        let a73 = (a73 << 36);
        let a49 = (a49 << 72);
        let a79 = (-(a79 << 12));
        let (a91, a67) = (a91 + a67, a91 - a67);
        let (a1, a13) = (a1 + a13, a1 - a13);
        let a13 = (a13 << 48);
        let (a91, a1) = (a91 + a1, a91 - a1);
        let (a67, a13) = (a67 + a13, a67 - a13);
        let (a19, a31) = (a19 + a31, a19 - a31);
        let (a37, a73) = (a37 + a73, a37 - a73);
        let a73 = (a73 << 48);
        let (a19, a37) = (a19 + a37, a19 - a37);
        let (a31, a73) = (a31 + a73, a31 - a73);
        let (a55, a25) = (a55 + a25, a55 - a25);
        let (a43, a49) = (a43 + a49, a43 - a49);
        let a49 = (a49 << 48);
        let (a55, a43) = (a55 + a43, a55 - a43);
        let (a25, a49) = (a25 + a49, a25 - a49);
        let (a97, a7) = (a97 + a7, a97 - a7);
        let (a61, a79) = (a61 + a79, a61 - a79);
        let a79 = (a79 << 48);
        let (a97, a61) = (a97 + a61, a97 - a61);
        let (a7, a79) = (a7 + a79, a7 - a79);
        let (a0, a34, a68) = (
            a0 + a34 + a68,
            a0 + (a34 << 64) - (a68 << 32),
            a0 - (a34 << 32) + (a68 << 64),
        );
        let (a51, a85, a17) = (
            a51 + a85 + a17,
            a51 + (a85 << 64) - (a17 << 32),
            a51 - (a85 << 32) + (a17 << 64),
        );
        let (a0, a51) = (a0 + a51, a0 - a51);
        let (a34, a85) = (a34 + a85, a34 - a85);
        let (a68, a17) = (a68 + a17, a68 - a17);
        let (a6, a40, a74) = (
            a6 + a40 + a74,
            a6 + (a40 << 64) - (a74 << 32),
            a6 - (a40 << 32) + (a74 << 64),
        );
        let (a57, a91, a23) = (
            a57 + a91 + a23,
            a57 + (a91 << 64) - (a23 << 32),
            a57 - (a91 << 32) + (a23 << 64),
        );
        let (a6, a57) = (a6 + a57, a6 - a57);
        let (a40, a91) = (a40 + a91, a40 - a91);
        let (a74, a23) = (a74 + a23, a74 - a23);
        let (a66, a100, a32) = (
            a66 + a100 + a32,
            a66 + (a100 << 64) - (a32 << 32),
            a66 - (a100 << 32) + (a32 << 64),
        );
        let (a15, a49, a83) = (
            a15 + a49 + a83,
            a15 + (a49 << 64) - (a83 << 32),
            a15 - (a49 << 32) + (a83 << 64),
        );
        let (a66, a15) = (a66 + a15, a66 - a15);
        let (a100, a49) = (a100 + a49, a100 - a49);
        let (a32, a83) = (a32 + a83, a32 - a83);
        let (a36, a70, a2) = (
            a36 + a70 + a2,
            a36 + (a70 << 64) - (a2 << 32),
            a36 - (a70 << 32) + (a2 << 64),
        );
        let (a87, a19, a53) = (
            a87 + a19 + a53,
            a87 + (a19 << 64) - (a53 << 32),
            a87 - (a19 << 32) + (a53 << 64),
        );
        let (a36, a87) = (a36 + a87, a36 - a87);
        let (a70, a19) = (a70 + a19, a70 - a19);
        let (a2, a53) = (a2 + a53, a2 - a53);
        let (a30, a64, a98) = (
            a30 + a64 + a98,
            a30 + (a64 << 64) - (a98 << 32),
            a30 - (a64 << 32) + (a98 << 64),
        );
        let (a81, a13, a47) = (
            a81 + a13 + a47,
            a81 + (a13 << 64) - (a47 << 32),
            a81 - (a13 << 32) + (a47 << 64),
        );
        let (a30, a81) = (a30 + a81, a30 - a81);
        let (a64, a13) = (a64 + a13, a64 - a13);
        let (a98, a47) = (a98 + a47, a98 - a47);
        let (a48, a82, a14) = (
            a48 + a82 + a14,
            a48 + (a82 << 64) - (a14 << 32),
            a48 - (a82 << 32) + (a14 << 64),
        );
        let (a99, a31, a65) = (
            a99 + a31 + a65,
            a99 + (a31 << 64) - (a65 << 32),
            a99 - (a31 << 32) + (a65 << 64),
        );
        let (a48, a99) = (a48 + a99, a48 - a99);
        let (a82, a31) = (a82 + a31, a82 - a31);
        let (a14, a65) = (a14 + a65, a14 - a65);
        let (a96, a28, a62) = (
            a96 + a28 + a62,
            a96 + (a28 << 64) - (a62 << 32),
            a96 - (a28 << 32) + (a62 << 64),
        );
        let (a45, a79, a11) = (
            a45 + a79 + a11,
            a45 + (a79 << 64) - (a11 << 32),
            a45 - (a79 << 32) + (a11 << 64),
        );
        let (a96, a45) = (a96 + a45, a96 - a45);
        let (a28, a79) = (a28 + a79, a28 - a79);
        let (a62, a11) = (a62 + a11, a62 - a11);
        let (a78, a10, a44) = (
            a78 + a10 + a44,
            a78 + (a10 << 64) - (a44 << 32),
            a78 - (a10 << 32) + (a44 << 64),
        );
        let (a27, a61, a95) = (
            a27 + a61 + a95,
            a27 + (a61 << 64) - (a95 << 32),
            a27 - (a61 << 32) + (a95 << 64),
        );
        let (a78, a27) = (a78 + a27, a78 - a27);
        let (a10, a61) = (a10 + a61, a10 - a61);
        let (a44, a95) = (a44 + a95, a44 - a95);
        let (a60, a94, a26) = (
            a60 + a94 + a26,
            a60 + (a94 << 64) - (a26 << 32),
            a60 - (a94 << 32) + (a26 << 64),
        );
        let (a9, a43, a77) = (
            a9 + a43 + a77,
            a9 + (a43 << 64) - (a77 << 32),
            a9 - (a43 << 32) + (a77 << 64),
        );
        let (a60, a9) = (a60 + a9, a60 - a9);
        let (a94, a43) = (a94 + a43, a94 - a43);
        let (a26, a77) = (a26 + a77, a26 - a77);
        let (a72, a4, a38) = (
            a72 + a4 + a38,
            a72 + (a4 << 64) - (a38 << 32),
            a72 - (a4 << 32) + (a38 << 64),
        );
        let (a21, a55, a89) = (
            a21 + a55 + a89,
            a21 + (a55 << 64) - (a89 << 32),
            a21 - (a55 << 32) + (a89 << 64),
        );
        let (a72, a21) = (a72 + a21, a72 - a21);
        let (a4, a55) = (a4 + a55, a4 - a55);
        let (a38, a89) = (a38 + a89, a38 - a89);
        let (a12, a46, a80) = (
            a12 + a46 + a80,
            a12 + (a46 << 64) - (a80 << 32),
            a12 - (a46 << 32) + (a80 << 64),
        );
        let (a63, a97, a29) = (
            a63 + a97 + a29,
            a63 + (a97 << 64) - (a29 << 32),
            a63 - (a97 << 32) + (a29 << 64),
        );
        let (a12, a63) = (a12 + a63, a12 - a63);
        let (a46, a97) = (a46 + a97, a46 - a97);
        let (a80, a29) = (a80 + a29, a80 - a29);
        let (a24, a58, a92) = (
            a24 + a58 + a92,
            a24 + (a58 << 64) - (a92 << 32),
            a24 - (a58 << 32) + (a92 << 64),
        );
        let (a75, a7, a41) = (
            a75 + a7 + a41,
            a75 + (a7 << 64) - (a41 << 32),
            a75 - (a7 << 32) + (a41 << 64),
        );
        let (a24, a75) = (a24 + a75, a24 - a75);
        let (a58, a7) = (a58 + a7, a58 - a7);
        let (a92, a41) = (a92 + a41, a92 - a41);
        let (a90, a22, a56) = (
            a90 + a22 + a56,
            a90 + (a22 << 64) - (a56 << 32),
            a90 - (a22 << 32) + (a56 << 64),
        );
        let (a39, a73, a5) = (
            a39 + a73 + a5,
            a39 + (a73 << 64) - (a5 << 32),
            a39 - (a73 << 32) + (a5 << 64),
        );
        let (a90, a39) = (a90 + a39, a90 - a39);
        let (a22, a73) = (a22 + a73, a22 - a73);
        let (a56, a5) = (a56 + a5, a56 - a5);
        let (a84, a16, a50) = (
            a84 + a16 + a50,
            a84 + (a16 << 64) - (a50 << 32),
            a84 - (a16 << 32) + (a50 << 64),
        );
        let (a33, a67, a101) = (
            a33 + a67 + a101,
            a33 + (a67 << 64) - (a101 << 32),
            a33 - (a67 << 32) + (a101 << 64),
        );
        let (a84, a33) = (a84 + a33, a84 - a33);
        let (a16, a67) = (a16 + a67, a16 - a67);
        let (a50, a101) = (a50 + a101, a50 - a101);
        let (a54, a88, a20) = (
            a54 + a88 + a20,
            a54 + (a88 << 64) - (a20 << 32),
            a54 - (a88 << 32) + (a20 << 64),
        );
        let (a3, a37, a71) = (
            a3 + a37 + a71,
            a3 + (a37 << 64) - (a71 << 32),
            a3 - (a37 << 32) + (a71 << 64),
        );
        let (a54, a3) = (a54 + a3, a54 - a3);
        let (a88, a37) = (a88 + a37, a88 - a37);
        let (a20, a71) = (a20 + a71, a20 - a71);
        let (a42, a76, a8) = (
            a42 + a76 + a8,
            a42 + (a76 << 64) - (a8 << 32),
            a42 - (a76 << 32) + (a8 << 64),
        );
        let (a93, a25, a59) = (
            a93 + a25 + a59,
            a93 + (a25 << 64) - (a59 << 32),
            a93 - (a25 << 32) + (a59 << 64),
        );
        let (a42, a93) = (a42 + a93, a42 - a93);
        let (a76, a25) = (a76 + a25, a76 - a25);
        let (a8, a59) = (a8 + a59, a8 - a59);
        let (a18, a52, a86) = (
            a18 + a52 + a86,
            a18 + (a52 << 64) - (a86 << 32),
            a18 - (a52 << 32) + (a86 << 64),
        );
        let (a69, a1, a35) = (
            a69 + a1 + a35,
            a69 + (a1 << 64) - (a35 << 32),
            a69 - (a1 << 32) + (a35 << 64),
        );
        let (a18, a69) = (a18 + a69, a18 - a69);
        let (a52, a1) = (a52 + a1, a52 - a1);
        let (a86, a35) = (a86 + a35, a86 - a35);
        values[0] = a0;
        values[1] = a91;
        values[2] = a32;
        values[3] = a87;
        values[4] = a64;
        values[5] = a65;
        values[6] = a96;
        values[7] = a61;
        values[8] = a26;
        values[9] = a21;
        values[10] = a46;
        values[11] = a41;
        values[12] = a90;
        values[13] = a67;
        values[14] = a20;
        values[15] = a93;
        values[16] = a52;
        values[17] = a17;
        values[18] = a6;
        values[19] = a49;
        values[20] = a2;
        values[21] = a81;
        values[22] = a82;
        values[23] = a11;
        values[24] = a78;
        values[25] = a43;
        values[26] = a38;
        values[27] = a63;
        values[28] = a58;
        values[29] = a5;
        values[30] = a84;
        values[31] = a37;
        values[32] = a8;
        values[33] = a69;
        values[34] = a34;
        values[35] = a23;
        values[36] = a66;
        values[37] = a19;
        values[38] = a98;
        values[39] = a99;
        values[40] = a28;
        values[41] = a95;
        values[42] = a60;
        values[43] = a55;
        values[44] = a80;
        values[45] = a75;
        values[46] = a22;
        values[47] = a101;
        values[48] = a54;
        values[49] = a25;
        values[50] = a86;
        values[51] = a51;
        values[52] = a40;
        values[53] = a83;
        values[54] = a36;
        values[55] = a13;
        values[56] = a14;
        values[57] = a45;
        values[58] = a10;
        values[59] = a77;
        values[60] = a72;
        values[61] = a97;
        values[62] = a92;
        values[63] = a39;
        values[64] = a16;
        values[65] = a71;
        values[66] = a42;
        values[67] = a1;
        values[68] = a68;
        values[69] = a57;
        values[70] = a100;
        values[71] = a53;
        values[72] = a30;
        values[73] = a31;
        values[74] = a62;
        values[75] = a27;
        values[76] = a94;
        values[77] = a89;
        values[78] = a12;
        values[79] = a7;
        values[80] = a56;
        values[81] = a33;
        values[82] = a88;
        values[83] = a59;
        values[84] = a18;
        values[85] = a85;
        values[86] = a74;
        values[87] = a15;
        values[88] = a70;
        values[89] = a47;
        values[90] = a48;
        values[91] = a79;
        values[92] = a44;
        values[93] = a9;
        values[94] = a4;
        values[95] = a29;
        values[96] = a24;
        values[97] = a73;
        values[98] = a50;
        values[99] = a3;
        values[100] = a76;
        values[101] = a35;
    }
}

/// Size 120 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 120`.
pub fn ntt_120(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 120, 0);
    for values in values.chunks_exact_mut(120) {
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
        let (a24, a96) = (a24 + a96, a24 - a96);
        let (a72, a48) = (a72 + a48, a72 - a48);
        let a48 = (a48 << 48);
        let (a24, a72) = (a24 + a72, a24 - a72);
        let (a96, a48) = (a96 + a48, a96 - a48);
        let t = a0;
        let a0 = a0 + a24;
        let a24 = a24 * Field::new(4611686017353646080);
        let a96 = a96 * Field::new(16181989089180173841);
        let a72 = a72 * Field::new(5818851782451133869);
        let a48 = a48 * Field::new(11322249509082494407);
        let a24 = a24 + t;
        let (a24, a72) = (a24 + a72, a24 - a72);
        let (a48, a96) = (a48 + a96, a48 - a96);
        let a96 = (a96 << 48);
        let (a24, a48) = (a24 + a48, a24 - a48);
        let (a72, a96) = (a72 + a96, a72 - a96);
        let (a84, a36) = (a84 + a36, a84 - a36);
        let (a12, a108) = (a12 + a108, a12 - a108);
        let a108 = (a108 << 48);
        let (a84, a12) = (a84 + a12, a84 - a12);
        let (a36, a108) = (a36 + a108, a36 - a108);
        let t = a60;
        let a60 = a60 + a84;
        let a84 = a84 * Field::new(4611686017353646080);
        let a36 = a36 * Field::new(16181989089180173841);
        let a12 = a12 * Field::new(5818851782451133869);
        let a108 = a108 * Field::new(11322249509082494407);
        let a84 = a84 + t;
        let (a84, a12) = (a84 + a12, a84 - a12);
        let (a108, a36) = (a108 + a36, a108 - a36);
        let a36 = (a36 << 48);
        let (a84, a108) = (a84 + a108, a84 - a108);
        let (a12, a36) = (a12 + a36, a12 - a36);
        let (a0, a60) = (a0 + a60, a0 - a60);
        let (a24, a84) = (a24 + a84, a24 - a84);
        let (a72, a12) = (a72 + a12, a72 - a12);
        let (a96, a36) = (a96 + a36, a96 - a36);
        let (a48, a108) = (a48 + a108, a48 - a108);
        let (a25, a97) = (a25 + a97, a25 - a97);
        let (a73, a49) = (a73 + a49, a73 - a49);
        let a49 = (a49 << 48);
        let (a25, a73) = (a25 + a73, a25 - a73);
        let (a97, a49) = (a97 + a49, a97 - a49);
        let t = a1;
        let a1 = a1 + a25;
        let a25 = a25 * Field::new(4611686017353646080);
        let a97 = a97 * Field::new(16181989089180173841);
        let a73 = a73 * Field::new(5818851782451133869);
        let a49 = a49 * Field::new(11322249509082494407);
        let a25 = a25 + t;
        let (a25, a73) = (a25 + a73, a25 - a73);
        let (a49, a97) = (a49 + a97, a49 - a97);
        let a97 = (a97 << 48);
        let (a25, a49) = (a25 + a49, a25 - a49);
        let (a73, a97) = (a73 + a97, a73 - a97);
        let (a85, a37) = (a85 + a37, a85 - a37);
        let (a13, a109) = (a13 + a109, a13 - a109);
        let a109 = (a109 << 48);
        let (a85, a13) = (a85 + a13, a85 - a13);
        let (a37, a109) = (a37 + a109, a37 - a109);
        let t = a61;
        let a61 = a61 + a85;
        let a85 = a85 * Field::new(4611686017353646080);
        let a37 = a37 * Field::new(16181989089180173841);
        let a13 = a13 * Field::new(5818851782451133869);
        let a109 = a109 * Field::new(11322249509082494407);
        let a85 = a85 + t;
        let (a85, a13) = (a85 + a13, a85 - a13);
        let (a109, a37) = (a109 + a37, a109 - a37);
        let a37 = (a37 << 48);
        let (a85, a109) = (a85 + a109, a85 - a109);
        let (a13, a37) = (a13 + a37, a13 - a37);
        let (a1, a61) = (a1 + a61, a1 - a61);
        let (a25, a85) = (a25 + a85, a25 - a85);
        let (a73, a13) = (a73 + a13, a73 - a13);
        let (a97, a37) = (a97 + a37, a97 - a37);
        let (a49, a109) = (a49 + a109, a49 - a109);
        let (a26, a98) = (a26 + a98, a26 - a98);
        let (a74, a50) = (a74 + a50, a74 - a50);
        let a50 = (a50 << 48);
        let (a26, a74) = (a26 + a74, a26 - a74);
        let (a98, a50) = (a98 + a50, a98 - a50);
        let t = a2;
        let a2 = a2 + a26;
        let a26 = a26 * Field::new(4611686017353646080);
        let a98 = a98 * Field::new(16181989089180173841);
        let a74 = a74 * Field::new(5818851782451133869);
        let a50 = a50 * Field::new(11322249509082494407);
        let a26 = a26 + t;
        let (a26, a74) = (a26 + a74, a26 - a74);
        let (a50, a98) = (a50 + a98, a50 - a98);
        let a98 = (a98 << 48);
        let (a26, a50) = (a26 + a50, a26 - a50);
        let (a74, a98) = (a74 + a98, a74 - a98);
        let (a86, a38) = (a86 + a38, a86 - a38);
        let (a14, a110) = (a14 + a110, a14 - a110);
        let a110 = (a110 << 48);
        let (a86, a14) = (a86 + a14, a86 - a14);
        let (a38, a110) = (a38 + a110, a38 - a110);
        let t = a62;
        let a62 = a62 + a86;
        let a86 = a86 * Field::new(4611686017353646080);
        let a38 = a38 * Field::new(16181989089180173841);
        let a14 = a14 * Field::new(5818851782451133869);
        let a110 = a110 * Field::new(11322249509082494407);
        let a86 = a86 + t;
        let (a86, a14) = (a86 + a14, a86 - a14);
        let (a110, a38) = (a110 + a38, a110 - a38);
        let a38 = (a38 << 48);
        let (a86, a110) = (a86 + a110, a86 - a110);
        let (a14, a38) = (a14 + a38, a14 - a38);
        let (a2, a62) = (a2 + a62, a2 - a62);
        let (a26, a86) = (a26 + a86, a26 - a86);
        let (a74, a14) = (a74 + a14, a74 - a14);
        let (a98, a38) = (a98 + a38, a98 - a38);
        let (a50, a110) = (a50 + a110, a50 - a110);
        let (a27, a99) = (a27 + a99, a27 - a99);
        let (a75, a51) = (a75 + a51, a75 - a51);
        let a51 = (a51 << 48);
        let (a27, a75) = (a27 + a75, a27 - a75);
        let (a99, a51) = (a99 + a51, a99 - a51);
        let t = a3;
        let a3 = a3 + a27;
        let a27 = a27 * Field::new(4611686017353646080);
        let a99 = a99 * Field::new(16181989089180173841);
        let a75 = a75 * Field::new(5818851782451133869);
        let a51 = a51 * Field::new(11322249509082494407);
        let a27 = a27 + t;
        let (a27, a75) = (a27 + a75, a27 - a75);
        let (a51, a99) = (a51 + a99, a51 - a99);
        let a99 = (a99 << 48);
        let (a27, a51) = (a27 + a51, a27 - a51);
        let (a75, a99) = (a75 + a99, a75 - a99);
        let (a87, a39) = (a87 + a39, a87 - a39);
        let (a15, a111) = (a15 + a111, a15 - a111);
        let a111 = (a111 << 48);
        let (a87, a15) = (a87 + a15, a87 - a15);
        let (a39, a111) = (a39 + a111, a39 - a111);
        let t = a63;
        let a63 = a63 + a87;
        let a87 = a87 * Field::new(4611686017353646080);
        let a39 = a39 * Field::new(16181989089180173841);
        let a15 = a15 * Field::new(5818851782451133869);
        let a111 = a111 * Field::new(11322249509082494407);
        let a87 = a87 + t;
        let (a87, a15) = (a87 + a15, a87 - a15);
        let (a111, a39) = (a111 + a39, a111 - a39);
        let a39 = (a39 << 48);
        let (a87, a111) = (a87 + a111, a87 - a111);
        let (a15, a39) = (a15 + a39, a15 - a39);
        let (a3, a63) = (a3 + a63, a3 - a63);
        let (a27, a87) = (a27 + a87, a27 - a87);
        let (a75, a15) = (a75 + a15, a75 - a15);
        let (a99, a39) = (a99 + a39, a99 - a39);
        let (a51, a111) = (a51 + a111, a51 - a111);
        let (a28, a100) = (a28 + a100, a28 - a100);
        let (a76, a52) = (a76 + a52, a76 - a52);
        let a52 = (a52 << 48);
        let (a28, a76) = (a28 + a76, a28 - a76);
        let (a100, a52) = (a100 + a52, a100 - a52);
        let t = a4;
        let a4 = a4 + a28;
        let a28 = a28 * Field::new(4611686017353646080);
        let a100 = a100 * Field::new(16181989089180173841);
        let a76 = a76 * Field::new(5818851782451133869);
        let a52 = a52 * Field::new(11322249509082494407);
        let a28 = a28 + t;
        let (a28, a76) = (a28 + a76, a28 - a76);
        let (a52, a100) = (a52 + a100, a52 - a100);
        let a100 = (a100 << 48);
        let (a28, a52) = (a28 + a52, a28 - a52);
        let (a76, a100) = (a76 + a100, a76 - a100);
        let (a88, a40) = (a88 + a40, a88 - a40);
        let (a16, a112) = (a16 + a112, a16 - a112);
        let a112 = (a112 << 48);
        let (a88, a16) = (a88 + a16, a88 - a16);
        let (a40, a112) = (a40 + a112, a40 - a112);
        let t = a64;
        let a64 = a64 + a88;
        let a88 = a88 * Field::new(4611686017353646080);
        let a40 = a40 * Field::new(16181989089180173841);
        let a16 = a16 * Field::new(5818851782451133869);
        let a112 = a112 * Field::new(11322249509082494407);
        let a88 = a88 + t;
        let (a88, a16) = (a88 + a16, a88 - a16);
        let (a112, a40) = (a112 + a40, a112 - a40);
        let a40 = (a40 << 48);
        let (a88, a112) = (a88 + a112, a88 - a112);
        let (a16, a40) = (a16 + a40, a16 - a40);
        let (a4, a64) = (a4 + a64, a4 - a64);
        let (a28, a88) = (a28 + a88, a28 - a88);
        let (a76, a16) = (a76 + a16, a76 - a16);
        let (a100, a40) = (a100 + a40, a100 - a40);
        let (a52, a112) = (a52 + a112, a52 - a112);
        let (a29, a101) = (a29 + a101, a29 - a101);
        let (a77, a53) = (a77 + a53, a77 - a53);
        let a53 = (a53 << 48);
        let (a29, a77) = (a29 + a77, a29 - a77);
        let (a101, a53) = (a101 + a53, a101 - a53);
        let t = a5;
        let a5 = a5 + a29;
        let a29 = a29 * Field::new(4611686017353646080);
        let a101 = a101 * Field::new(16181989089180173841);
        let a77 = a77 * Field::new(5818851782451133869);
        let a53 = a53 * Field::new(11322249509082494407);
        let a29 = a29 + t;
        let (a29, a77) = (a29 + a77, a29 - a77);
        let (a53, a101) = (a53 + a101, a53 - a101);
        let a101 = (a101 << 48);
        let (a29, a53) = (a29 + a53, a29 - a53);
        let (a77, a101) = (a77 + a101, a77 - a101);
        let (a89, a41) = (a89 + a41, a89 - a41);
        let (a17, a113) = (a17 + a113, a17 - a113);
        let a113 = (a113 << 48);
        let (a89, a17) = (a89 + a17, a89 - a17);
        let (a41, a113) = (a41 + a113, a41 - a113);
        let t = a65;
        let a65 = a65 + a89;
        let a89 = a89 * Field::new(4611686017353646080);
        let a41 = a41 * Field::new(16181989089180173841);
        let a17 = a17 * Field::new(5818851782451133869);
        let a113 = a113 * Field::new(11322249509082494407);
        let a89 = a89 + t;
        let (a89, a17) = (a89 + a17, a89 - a17);
        let (a113, a41) = (a113 + a41, a113 - a41);
        let a41 = (a41 << 48);
        let (a89, a113) = (a89 + a113, a89 - a113);
        let (a17, a41) = (a17 + a41, a17 - a41);
        let (a5, a65) = (a5 + a65, a5 - a65);
        let (a29, a89) = (a29 + a89, a29 - a89);
        let (a77, a17) = (a77 + a17, a77 - a17);
        let (a101, a41) = (a101 + a41, a101 - a41);
        let (a53, a113) = (a53 + a113, a53 - a113);
        let (a30, a102) = (a30 + a102, a30 - a102);
        let (a78, a54) = (a78 + a54, a78 - a54);
        let a54 = (a54 << 48);
        let (a30, a78) = (a30 + a78, a30 - a78);
        let (a102, a54) = (a102 + a54, a102 - a54);
        let t = a6;
        let a6 = a6 + a30;
        let a30 = a30 * Field::new(4611686017353646080);
        let a102 = a102 * Field::new(16181989089180173841);
        let a78 = a78 * Field::new(5818851782451133869);
        let a54 = a54 * Field::new(11322249509082494407);
        let a30 = a30 + t;
        let (a30, a78) = (a30 + a78, a30 - a78);
        let (a54, a102) = (a54 + a102, a54 - a102);
        let a102 = (a102 << 48);
        let (a30, a54) = (a30 + a54, a30 - a54);
        let (a78, a102) = (a78 + a102, a78 - a102);
        let (a90, a42) = (a90 + a42, a90 - a42);
        let (a18, a114) = (a18 + a114, a18 - a114);
        let a114 = (a114 << 48);
        let (a90, a18) = (a90 + a18, a90 - a18);
        let (a42, a114) = (a42 + a114, a42 - a114);
        let t = a66;
        let a66 = a66 + a90;
        let a90 = a90 * Field::new(4611686017353646080);
        let a42 = a42 * Field::new(16181989089180173841);
        let a18 = a18 * Field::new(5818851782451133869);
        let a114 = a114 * Field::new(11322249509082494407);
        let a90 = a90 + t;
        let (a90, a18) = (a90 + a18, a90 - a18);
        let (a114, a42) = (a114 + a42, a114 - a42);
        let a42 = (a42 << 48);
        let (a90, a114) = (a90 + a114, a90 - a114);
        let (a18, a42) = (a18 + a42, a18 - a42);
        let (a6, a66) = (a6 + a66, a6 - a66);
        let (a30, a90) = (a30 + a90, a30 - a90);
        let (a78, a18) = (a78 + a18, a78 - a18);
        let (a102, a42) = (a102 + a42, a102 - a42);
        let (a54, a114) = (a54 + a114, a54 - a114);
        let (a31, a103) = (a31 + a103, a31 - a103);
        let (a79, a55) = (a79 + a55, a79 - a55);
        let a55 = (a55 << 48);
        let (a31, a79) = (a31 + a79, a31 - a79);
        let (a103, a55) = (a103 + a55, a103 - a55);
        let t = a7;
        let a7 = a7 + a31;
        let a31 = a31 * Field::new(4611686017353646080);
        let a103 = a103 * Field::new(16181989089180173841);
        let a79 = a79 * Field::new(5818851782451133869);
        let a55 = a55 * Field::new(11322249509082494407);
        let a31 = a31 + t;
        let (a31, a79) = (a31 + a79, a31 - a79);
        let (a55, a103) = (a55 + a103, a55 - a103);
        let a103 = (a103 << 48);
        let (a31, a55) = (a31 + a55, a31 - a55);
        let (a79, a103) = (a79 + a103, a79 - a103);
        let (a91, a43) = (a91 + a43, a91 - a43);
        let (a19, a115) = (a19 + a115, a19 - a115);
        let a115 = (a115 << 48);
        let (a91, a19) = (a91 + a19, a91 - a19);
        let (a43, a115) = (a43 + a115, a43 - a115);
        let t = a67;
        let a67 = a67 + a91;
        let a91 = a91 * Field::new(4611686017353646080);
        let a43 = a43 * Field::new(16181989089180173841);
        let a19 = a19 * Field::new(5818851782451133869);
        let a115 = a115 * Field::new(11322249509082494407);
        let a91 = a91 + t;
        let (a91, a19) = (a91 + a19, a91 - a19);
        let (a115, a43) = (a115 + a43, a115 - a43);
        let a43 = (a43 << 48);
        let (a91, a115) = (a91 + a115, a91 - a115);
        let (a19, a43) = (a19 + a43, a19 - a43);
        let (a7, a67) = (a7 + a67, a7 - a67);
        let (a31, a91) = (a31 + a91, a31 - a91);
        let (a79, a19) = (a79 + a19, a79 - a19);
        let (a103, a43) = (a103 + a43, a103 - a43);
        let (a55, a115) = (a55 + a115, a55 - a115);
        let (a32, a104) = (a32 + a104, a32 - a104);
        let (a80, a56) = (a80 + a56, a80 - a56);
        let a56 = (a56 << 48);
        let (a32, a80) = (a32 + a80, a32 - a80);
        let (a104, a56) = (a104 + a56, a104 - a56);
        let t = a8;
        let a8 = a8 + a32;
        let a32 = a32 * Field::new(4611686017353646080);
        let a104 = a104 * Field::new(16181989089180173841);
        let a80 = a80 * Field::new(5818851782451133869);
        let a56 = a56 * Field::new(11322249509082494407);
        let a32 = a32 + t;
        let (a32, a80) = (a32 + a80, a32 - a80);
        let (a56, a104) = (a56 + a104, a56 - a104);
        let a104 = (a104 << 48);
        let (a32, a56) = (a32 + a56, a32 - a56);
        let (a80, a104) = (a80 + a104, a80 - a104);
        let (a92, a44) = (a92 + a44, a92 - a44);
        let (a20, a116) = (a20 + a116, a20 - a116);
        let a116 = (a116 << 48);
        let (a92, a20) = (a92 + a20, a92 - a20);
        let (a44, a116) = (a44 + a116, a44 - a116);
        let t = a68;
        let a68 = a68 + a92;
        let a92 = a92 * Field::new(4611686017353646080);
        let a44 = a44 * Field::new(16181989089180173841);
        let a20 = a20 * Field::new(5818851782451133869);
        let a116 = a116 * Field::new(11322249509082494407);
        let a92 = a92 + t;
        let (a92, a20) = (a92 + a20, a92 - a20);
        let (a116, a44) = (a116 + a44, a116 - a44);
        let a44 = (a44 << 48);
        let (a92, a116) = (a92 + a116, a92 - a116);
        let (a20, a44) = (a20 + a44, a20 - a44);
        let (a8, a68) = (a8 + a68, a8 - a68);
        let (a32, a92) = (a32 + a92, a32 - a92);
        let (a80, a20) = (a80 + a20, a80 - a20);
        let (a104, a44) = (a104 + a44, a104 - a44);
        let (a56, a116) = (a56 + a116, a56 - a116);
        let (a33, a105) = (a33 + a105, a33 - a105);
        let (a81, a57) = (a81 + a57, a81 - a57);
        let a57 = (a57 << 48);
        let (a33, a81) = (a33 + a81, a33 - a81);
        let (a105, a57) = (a105 + a57, a105 - a57);
        let t = a9;
        let a9 = a9 + a33;
        let a33 = a33 * Field::new(4611686017353646080);
        let a105 = a105 * Field::new(16181989089180173841);
        let a81 = a81 * Field::new(5818851782451133869);
        let a57 = a57 * Field::new(11322249509082494407);
        let a33 = a33 + t;
        let (a33, a81) = (a33 + a81, a33 - a81);
        let (a57, a105) = (a57 + a105, a57 - a105);
        let a105 = (a105 << 48);
        let (a33, a57) = (a33 + a57, a33 - a57);
        let (a81, a105) = (a81 + a105, a81 - a105);
        let (a93, a45) = (a93 + a45, a93 - a45);
        let (a21, a117) = (a21 + a117, a21 - a117);
        let a117 = (a117 << 48);
        let (a93, a21) = (a93 + a21, a93 - a21);
        let (a45, a117) = (a45 + a117, a45 - a117);
        let t = a69;
        let a69 = a69 + a93;
        let a93 = a93 * Field::new(4611686017353646080);
        let a45 = a45 * Field::new(16181989089180173841);
        let a21 = a21 * Field::new(5818851782451133869);
        let a117 = a117 * Field::new(11322249509082494407);
        let a93 = a93 + t;
        let (a93, a21) = (a93 + a21, a93 - a21);
        let (a117, a45) = (a117 + a45, a117 - a45);
        let a45 = (a45 << 48);
        let (a93, a117) = (a93 + a117, a93 - a117);
        let (a21, a45) = (a21 + a45, a21 - a45);
        let (a9, a69) = (a9 + a69, a9 - a69);
        let (a33, a93) = (a33 + a93, a33 - a93);
        let (a81, a21) = (a81 + a21, a81 - a21);
        let (a105, a45) = (a105 + a45, a105 - a45);
        let (a57, a117) = (a57 + a117, a57 - a117);
        let (a34, a106) = (a34 + a106, a34 - a106);
        let (a82, a58) = (a82 + a58, a82 - a58);
        let a58 = (a58 << 48);
        let (a34, a82) = (a34 + a82, a34 - a82);
        let (a106, a58) = (a106 + a58, a106 - a58);
        let t = a10;
        let a10 = a10 + a34;
        let a34 = a34 * Field::new(4611686017353646080);
        let a106 = a106 * Field::new(16181989089180173841);
        let a82 = a82 * Field::new(5818851782451133869);
        let a58 = a58 * Field::new(11322249509082494407);
        let a34 = a34 + t;
        let (a34, a82) = (a34 + a82, a34 - a82);
        let (a58, a106) = (a58 + a106, a58 - a106);
        let a106 = (a106 << 48);
        let (a34, a58) = (a34 + a58, a34 - a58);
        let (a82, a106) = (a82 + a106, a82 - a106);
        let (a94, a46) = (a94 + a46, a94 - a46);
        let (a22, a118) = (a22 + a118, a22 - a118);
        let a118 = (a118 << 48);
        let (a94, a22) = (a94 + a22, a94 - a22);
        let (a46, a118) = (a46 + a118, a46 - a118);
        let t = a70;
        let a70 = a70 + a94;
        let a94 = a94 * Field::new(4611686017353646080);
        let a46 = a46 * Field::new(16181989089180173841);
        let a22 = a22 * Field::new(5818851782451133869);
        let a118 = a118 * Field::new(11322249509082494407);
        let a94 = a94 + t;
        let (a94, a22) = (a94 + a22, a94 - a22);
        let (a118, a46) = (a118 + a46, a118 - a46);
        let a46 = (a46 << 48);
        let (a94, a118) = (a94 + a118, a94 - a118);
        let (a22, a46) = (a22 + a46, a22 - a46);
        let (a10, a70) = (a10 + a70, a10 - a70);
        let (a34, a94) = (a34 + a94, a34 - a94);
        let (a82, a22) = (a82 + a22, a82 - a22);
        let (a106, a46) = (a106 + a46, a106 - a46);
        let (a58, a118) = (a58 + a118, a58 - a118);
        let (a35, a107) = (a35 + a107, a35 - a107);
        let (a83, a59) = (a83 + a59, a83 - a59);
        let a59 = (a59 << 48);
        let (a35, a83) = (a35 + a83, a35 - a83);
        let (a107, a59) = (a107 + a59, a107 - a59);
        let t = a11;
        let a11 = a11 + a35;
        let a35 = a35 * Field::new(4611686017353646080);
        let a107 = a107 * Field::new(16181989089180173841);
        let a83 = a83 * Field::new(5818851782451133869);
        let a59 = a59 * Field::new(11322249509082494407);
        let a35 = a35 + t;
        let (a35, a83) = (a35 + a83, a35 - a83);
        let (a59, a107) = (a59 + a107, a59 - a107);
        let a107 = (a107 << 48);
        let (a35, a59) = (a35 + a59, a35 - a59);
        let (a83, a107) = (a83 + a107, a83 - a107);
        let (a95, a47) = (a95 + a47, a95 - a47);
        let (a23, a119) = (a23 + a119, a23 - a119);
        let a119 = (a119 << 48);
        let (a95, a23) = (a95 + a23, a95 - a23);
        let (a47, a119) = (a47 + a119, a47 - a119);
        let t = a71;
        let a71 = a71 + a95;
        let a95 = a95 * Field::new(4611686017353646080);
        let a47 = a47 * Field::new(16181989089180173841);
        let a23 = a23 * Field::new(5818851782451133869);
        let a119 = a119 * Field::new(11322249509082494407);
        let a95 = a95 + t;
        let (a95, a23) = (a95 + a23, a95 - a23);
        let (a119, a47) = (a119 + a47, a119 - a47);
        let a47 = (a47 << 48);
        let (a95, a119) = (a95 + a119, a95 - a119);
        let (a23, a47) = (a23 + a47, a23 - a47);
        let (a11, a71) = (a11 + a71, a11 - a71);
        let (a35, a95) = (a35 + a95, a35 - a95);
        let (a83, a23) = (a83 + a23, a83 - a23);
        let (a107, a47) = (a107 + a47, a107 - a47);
        let (a59, a119) = (a59 + a119, a59 - a119);
        let a85 = a85 * Field::new(12342513394488208227);
        let a73 = a73 * Field::new(5927015354646419725);
        let a37 = a37 * Field::new(9148693690730647261);
        let a49 = a49 * Field::new(6868348408044855211);
        let a61 = (a61 << 8);
        let a25 = a25 * Field::new(5290193119087387221);
        let a13 = a13 * Field::new(4682917097487535278);
        let a97 = a97 * Field::new(17775832080808074370);
        let a109 = a109 * Field::new(5856505865097423521);
        let a86 = a86 * Field::new(5927015354646419725);
        let a74 = a74 * Field::new(6868348408044855211);
        let a38 = a38 * Field::new(5290193119087387221);
        let a50 = a50 * Field::new(17775832080808074370);
        let a62 = (a62 << 16);
        let a26 = a26 * Field::new(18235156514275634624);
        let a14 = a14 * Field::new(5079231842359091375);
        let a98 = a98 * Field::new(9988211933311186582);
        let a110 = a110 * Field::new(8149776168132872528);
        let a87 = a87 * Field::new(9148693690730647261);
        let a75 = a75 * Field::new(5290193119087387221);
        let a39 = a39 * Field::new(5856505865097423521);
        let a51 = a51 * Field::new(18235156514275634624);
        let a63 = (a63 << 24);
        let a27 = a27 * Field::new(8149776168132872528);
        let a15 = a15 * Field::new(11331573348451128694);
        let a99 = a99 * Field::new(1041288259238279555);
        let a111 = a111 * Field::new(4419751934697861046);
        let a88 = a88 * Field::new(6868348408044855211);
        let a76 = a76 * Field::new(17775832080808074370);
        let a40 = a40 * Field::new(18235156514275634624);
        let a52 = a52 * Field::new(9988211933311186582);
        let a64 = (a64 << 32);
        let a28 = a28 * Field::new(1041288259238279555);
        let a16 = a16 * Field::new(15149912995474149095);
        let a100 = a100 * Field::new(6205107048362784195);
        let a112 = a112 * Field::new(17073700798457888299);
        let a89 = (a89 << 8);
        let a77 = (a77 << 16);
        let a41 = (a41 << 24);
        let a53 = (a53 << 32);
        let a65 = (a65 << 40);
        let a29 = (a29 << 48);
        let a17 = (a17 << 56);
        let a101 = (a101 << 64);
        let a113 = (a113 << 72);
        let a90 = a90 * Field::new(5290193119087387221);
        let a78 = a78 * Field::new(18235156514275634624);
        let a42 = a42 * Field::new(8149776168132872528);
        let a54 = a54 * Field::new(1041288259238279555);
        let a66 = (a66 << 48);
        let a30 = a30 * Field::new(17073700798457888299);
        let a18 = a18 * Field::new(17869255328328231396);
        let a102 = a102 * Field::new(15820824984080659046);
        let a114 = a114 * Field::new(2281812832982421726);
        let a91 = a91 * Field::new(4682917097487535278);
        let a79 = a79 * Field::new(5079231842359091375);
        let a43 = a43 * Field::new(11331573348451128694);
        let a55 = a55 * Field::new(15149912995474149095);
        let a67 = (a67 << 56);
        let a31 = a31 * Field::new(17869255328328231396);
        let a19 = a19 * Field::new(2458871528097962065);
        let a103 = a103 * Field::new(7085488865146701717);
        let a115 = a115 * Field::new(9298050378683937060);
        let a92 = a92 * Field::new(17775832080808074370);
        let a80 = a80 * Field::new(9988211933311186582);
        let a44 = a44 * Field::new(1041288259238279555);
        let a56 = a56 * Field::new(6205107048362784195);
        let a68 = (a68 << 64);
        let a32 = a32 * Field::new(15820824984080659046);
        let a20 = a20 * Field::new(7085488865146701717);
        let a104 = a104 * Field::new(11578395661369729110);
        let a116 = a116 * Field::new(211587555138949697);
        let a93 = a93 * Field::new(5856505865097423521);
        let a81 = a81 * Field::new(8149776168132872528);
        let a45 = a45 * Field::new(4419751934697861046);
        let a57 = a57 * Field::new(17073700798457888299);
        let a69 = (a69 << 72);
        let a33 = a33 * Field::new(2281812832982421726);
        let a21 = a21 * Field::new(9298050378683937060);
        let a105 = a105 * Field::new(211587555138949697);
        let a117 = a117 * Field::new(7115170720963455627);
        let a94 = (a94 << 16);
        let a82 = (a82 << 32);
        let a46 = (a46 << 48);
        let a58 = (a58 << 64);
        let a70 = (a70 << 80);
        let a34 = (-a34);
        let a22 = (-(a22 << 16));
        let a106 = (-(a106 << 32));
        let a118 = (-(a118 << 48));
        let a95 = a95 * Field::new(7677121419106473143);
        let a83 = a83 * Field::new(5349526613560066800);
        let a47 = a47 * Field::new(4561472264319460910);
        let a59 = a59 * Field::new(12619683920608008665);
        let a71 = (a71 << 88);
        let a35 = a35 * Field::new(13156550950327197100);
        let a23 = a23 * Field::new(17272925976741953790);
        let a107 = a107 * Field::new(3296831073940435226);
        let a119 = a119 * Field::new(15587202262274831207);
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
        let (a84, a90) = (a84 + a90, a84 - a90);
        let (a87, a93) = (a87 + a93, a87 - a93);
        let a93 = (a93 << 48);
        let (a84, a87) = (a84 + a87, a84 - a87);
        let (a90, a93) = (a90 + a93, a90 - a93);
        let (a88, a94) = (a88 + a94, a88 - a94);
        let (a91, a85) = (a91 + a85, a91 - a85);
        let a85 = (a85 << 48);
        let (a88, a91) = (a88 + a91, a88 - a91);
        let (a94, a85) = (a94 + a85, a94 - a85);
        let (a92, a86) = (a92 + a86, a92 - a86);
        let (a95, a89) = (a95 + a89, a95 - a89);
        let a89 = (a89 << 48);
        let (a92, a95) = (a92 + a95, a92 - a95);
        let (a86, a89) = (a86 + a89, a86 - a89);
        let (a84, a88, a92) = (
            a84 + a88 + a92,
            a84 + (a88 << 64) - (a92 << 32),
            a84 - (a88 << 32) + (a92 << 64),
        );
        let (a90, a94, a86) = (
            a90 + a94 + a86,
            a90 + (a94 << 64) - (a86 << 32),
            a90 - (a94 << 32) + (a86 << 64),
        );
        let (a87, a91, a95) = (
            a87 + a91 + a95,
            a87 + (a91 << 64) - (a95 << 32),
            a87 - (a91 << 32) + (a95 << 64),
        );
        let (a93, a85, a89) = (
            a93 + a85 + a89,
            a93 + (a85 << 64) - (a89 << 32),
            a93 - (a85 << 32) + (a89 << 64),
        );
        let (a72, a78) = (a72 + a78, a72 - a78);
        let (a75, a81) = (a75 + a81, a75 - a81);
        let a81 = (a81 << 48);
        let (a72, a75) = (a72 + a75, a72 - a75);
        let (a78, a81) = (a78 + a81, a78 - a81);
        let (a76, a82) = (a76 + a82, a76 - a82);
        let (a79, a73) = (a79 + a73, a79 - a73);
        let a73 = (a73 << 48);
        let (a76, a79) = (a76 + a79, a76 - a79);
        let (a82, a73) = (a82 + a73, a82 - a73);
        let (a80, a74) = (a80 + a74, a80 - a74);
        let (a83, a77) = (a83 + a77, a83 - a77);
        let a77 = (a77 << 48);
        let (a80, a83) = (a80 + a83, a80 - a83);
        let (a74, a77) = (a74 + a77, a74 - a77);
        let (a72, a76, a80) = (
            a72 + a76 + a80,
            a72 + (a76 << 64) - (a80 << 32),
            a72 - (a76 << 32) + (a80 << 64),
        );
        let (a78, a82, a74) = (
            a78 + a82 + a74,
            a78 + (a82 << 64) - (a74 << 32),
            a78 - (a82 << 32) + (a74 << 64),
        );
        let (a75, a79, a83) = (
            a75 + a79 + a83,
            a75 + (a79 << 64) - (a83 << 32),
            a75 - (a79 << 32) + (a83 << 64),
        );
        let (a81, a73, a77) = (
            a81 + a73 + a77,
            a81 + (a73 << 64) - (a77 << 32),
            a81 - (a73 << 32) + (a77 << 64),
        );
        let (a36, a42) = (a36 + a42, a36 - a42);
        let (a39, a45) = (a39 + a45, a39 - a45);
        let a45 = (a45 << 48);
        let (a36, a39) = (a36 + a39, a36 - a39);
        let (a42, a45) = (a42 + a45, a42 - a45);
        let (a40, a46) = (a40 + a46, a40 - a46);
        let (a43, a37) = (a43 + a37, a43 - a37);
        let a37 = (a37 << 48);
        let (a40, a43) = (a40 + a43, a40 - a43);
        let (a46, a37) = (a46 + a37, a46 - a37);
        let (a44, a38) = (a44 + a38, a44 - a38);
        let (a47, a41) = (a47 + a41, a47 - a41);
        let a41 = (a41 << 48);
        let (a44, a47) = (a44 + a47, a44 - a47);
        let (a38, a41) = (a38 + a41, a38 - a41);
        let (a36, a40, a44) = (
            a36 + a40 + a44,
            a36 + (a40 << 64) - (a44 << 32),
            a36 - (a40 << 32) + (a44 << 64),
        );
        let (a42, a46, a38) = (
            a42 + a46 + a38,
            a42 + (a46 << 64) - (a38 << 32),
            a42 - (a46 << 32) + (a38 << 64),
        );
        let (a39, a43, a47) = (
            a39 + a43 + a47,
            a39 + (a43 << 64) - (a47 << 32),
            a39 - (a43 << 32) + (a47 << 64),
        );
        let (a45, a37, a41) = (
            a45 + a37 + a41,
            a45 + (a37 << 64) - (a41 << 32),
            a45 - (a37 << 32) + (a41 << 64),
        );
        let (a48, a54) = (a48 + a54, a48 - a54);
        let (a51, a57) = (a51 + a57, a51 - a57);
        let a57 = (a57 << 48);
        let (a48, a51) = (a48 + a51, a48 - a51);
        let (a54, a57) = (a54 + a57, a54 - a57);
        let (a52, a58) = (a52 + a58, a52 - a58);
        let (a55, a49) = (a55 + a49, a55 - a49);
        let a49 = (a49 << 48);
        let (a52, a55) = (a52 + a55, a52 - a55);
        let (a58, a49) = (a58 + a49, a58 - a49);
        let (a56, a50) = (a56 + a50, a56 - a50);
        let (a59, a53) = (a59 + a53, a59 - a53);
        let a53 = (a53 << 48);
        let (a56, a59) = (a56 + a59, a56 - a59);
        let (a50, a53) = (a50 + a53, a50 - a53);
        let (a48, a52, a56) = (
            a48 + a52 + a56,
            a48 + (a52 << 64) - (a56 << 32),
            a48 - (a52 << 32) + (a56 << 64),
        );
        let (a54, a58, a50) = (
            a54 + a58 + a50,
            a54 + (a58 << 64) - (a50 << 32),
            a54 - (a58 << 32) + (a50 << 64),
        );
        let (a51, a55, a59) = (
            a51 + a55 + a59,
            a51 + (a55 << 64) - (a59 << 32),
            a51 - (a55 << 32) + (a59 << 64),
        );
        let (a57, a49, a53) = (
            a57 + a49 + a53,
            a57 + (a49 << 64) - (a53 << 32),
            a57 - (a49 << 32) + (a53 << 64),
        );
        let (a60, a66) = (a60 + a66, a60 - a66);
        let (a63, a69) = (a63 + a69, a63 - a69);
        let a69 = (a69 << 48);
        let (a60, a63) = (a60 + a63, a60 - a63);
        let (a66, a69) = (a66 + a69, a66 - a69);
        let (a64, a70) = (a64 + a70, a64 - a70);
        let (a67, a61) = (a67 + a61, a67 - a61);
        let a61 = (a61 << 48);
        let (a64, a67) = (a64 + a67, a64 - a67);
        let (a70, a61) = (a70 + a61, a70 - a61);
        let (a68, a62) = (a68 + a62, a68 - a62);
        let (a71, a65) = (a71 + a65, a71 - a65);
        let a65 = (a65 << 48);
        let (a68, a71) = (a68 + a71, a68 - a71);
        let (a62, a65) = (a62 + a65, a62 - a65);
        let (a60, a64, a68) = (
            a60 + a64 + a68,
            a60 + (a64 << 64) - (a68 << 32),
            a60 - (a64 << 32) + (a68 << 64),
        );
        let (a66, a70, a62) = (
            a66 + a70 + a62,
            a66 + (a70 << 64) - (a62 << 32),
            a66 - (a70 << 32) + (a62 << 64),
        );
        let (a63, a67, a71) = (
            a63 + a67 + a71,
            a63 + (a67 << 64) - (a71 << 32),
            a63 - (a67 << 32) + (a71 << 64),
        );
        let (a69, a61, a65) = (
            a69 + a61 + a65,
            a69 + (a61 << 64) - (a65 << 32),
            a69 - (a61 << 32) + (a65 << 64),
        );
        let (a24, a30) = (a24 + a30, a24 - a30);
        let (a27, a33) = (a27 + a33, a27 - a33);
        let a33 = (a33 << 48);
        let (a24, a27) = (a24 + a27, a24 - a27);
        let (a30, a33) = (a30 + a33, a30 - a33);
        let (a28, a34) = (a28 + a34, a28 - a34);
        let (a31, a25) = (a31 + a25, a31 - a25);
        let a25 = (a25 << 48);
        let (a28, a31) = (a28 + a31, a28 - a31);
        let (a34, a25) = (a34 + a25, a34 - a25);
        let (a32, a26) = (a32 + a26, a32 - a26);
        let (a35, a29) = (a35 + a29, a35 - a29);
        let a29 = (a29 << 48);
        let (a32, a35) = (a32 + a35, a32 - a35);
        let (a26, a29) = (a26 + a29, a26 - a29);
        let (a24, a28, a32) = (
            a24 + a28 + a32,
            a24 + (a28 << 64) - (a32 << 32),
            a24 - (a28 << 32) + (a32 << 64),
        );
        let (a30, a34, a26) = (
            a30 + a34 + a26,
            a30 + (a34 << 64) - (a26 << 32),
            a30 - (a34 << 32) + (a26 << 64),
        );
        let (a27, a31, a35) = (
            a27 + a31 + a35,
            a27 + (a31 << 64) - (a35 << 32),
            a27 - (a31 << 32) + (a35 << 64),
        );
        let (a33, a25, a29) = (
            a33 + a25 + a29,
            a33 + (a25 << 64) - (a29 << 32),
            a33 - (a25 << 32) + (a29 << 64),
        );
        let (a12, a18) = (a12 + a18, a12 - a18);
        let (a15, a21) = (a15 + a21, a15 - a21);
        let a21 = (a21 << 48);
        let (a12, a15) = (a12 + a15, a12 - a15);
        let (a18, a21) = (a18 + a21, a18 - a21);
        let (a16, a22) = (a16 + a22, a16 - a22);
        let (a19, a13) = (a19 + a13, a19 - a13);
        let a13 = (a13 << 48);
        let (a16, a19) = (a16 + a19, a16 - a19);
        let (a22, a13) = (a22 + a13, a22 - a13);
        let (a20, a14) = (a20 + a14, a20 - a14);
        let (a23, a17) = (a23 + a17, a23 - a17);
        let a17 = (a17 << 48);
        let (a20, a23) = (a20 + a23, a20 - a23);
        let (a14, a17) = (a14 + a17, a14 - a17);
        let (a12, a16, a20) = (
            a12 + a16 + a20,
            a12 + (a16 << 64) - (a20 << 32),
            a12 - (a16 << 32) + (a20 << 64),
        );
        let (a18, a22, a14) = (
            a18 + a22 + a14,
            a18 + (a22 << 64) - (a14 << 32),
            a18 - (a22 << 32) + (a14 << 64),
        );
        let (a15, a19, a23) = (
            a15 + a19 + a23,
            a15 + (a19 << 64) - (a23 << 32),
            a15 - (a19 << 32) + (a23 << 64),
        );
        let (a21, a13, a17) = (
            a21 + a13 + a17,
            a21 + (a13 << 64) - (a17 << 32),
            a21 - (a13 << 32) + (a17 << 64),
        );
        let (a96, a102) = (a96 + a102, a96 - a102);
        let (a99, a105) = (a99 + a105, a99 - a105);
        let a105 = (a105 << 48);
        let (a96, a99) = (a96 + a99, a96 - a99);
        let (a102, a105) = (a102 + a105, a102 - a105);
        let (a100, a106) = (a100 + a106, a100 - a106);
        let (a103, a97) = (a103 + a97, a103 - a97);
        let a97 = (a97 << 48);
        let (a100, a103) = (a100 + a103, a100 - a103);
        let (a106, a97) = (a106 + a97, a106 - a97);
        let (a104, a98) = (a104 + a98, a104 - a98);
        let (a107, a101) = (a107 + a101, a107 - a101);
        let a101 = (a101 << 48);
        let (a104, a107) = (a104 + a107, a104 - a107);
        let (a98, a101) = (a98 + a101, a98 - a101);
        let (a96, a100, a104) = (
            a96 + a100 + a104,
            a96 + (a100 << 64) - (a104 << 32),
            a96 - (a100 << 32) + (a104 << 64),
        );
        let (a102, a106, a98) = (
            a102 + a106 + a98,
            a102 + (a106 << 64) - (a98 << 32),
            a102 - (a106 << 32) + (a98 << 64),
        );
        let (a99, a103, a107) = (
            a99 + a103 + a107,
            a99 + (a103 << 64) - (a107 << 32),
            a99 - (a103 << 32) + (a107 << 64),
        );
        let (a105, a97, a101) = (
            a105 + a97 + a101,
            a105 + (a97 << 64) - (a101 << 32),
            a105 - (a97 << 32) + (a101 << 64),
        );
        let (a108, a114) = (a108 + a114, a108 - a114);
        let (a111, a117) = (a111 + a117, a111 - a117);
        let a117 = (a117 << 48);
        let (a108, a111) = (a108 + a111, a108 - a111);
        let (a114, a117) = (a114 + a117, a114 - a117);
        let (a112, a118) = (a112 + a118, a112 - a118);
        let (a115, a109) = (a115 + a109, a115 - a109);
        let a109 = (a109 << 48);
        let (a112, a115) = (a112 + a115, a112 - a115);
        let (a118, a109) = (a118 + a109, a118 - a109);
        let (a116, a110) = (a116 + a110, a116 - a110);
        let (a119, a113) = (a119 + a113, a119 - a113);
        let a113 = (a113 << 48);
        let (a116, a119) = (a116 + a119, a116 - a119);
        let (a110, a113) = (a110 + a113, a110 - a113);
        let (a108, a112, a116) = (
            a108 + a112 + a116,
            a108 + (a112 << 64) - (a116 << 32),
            a108 - (a112 << 32) + (a116 << 64),
        );
        let (a114, a118, a110) = (
            a114 + a118 + a110,
            a114 + (a118 << 64) - (a110 << 32),
            a114 - (a118 << 32) + (a110 << 64),
        );
        let (a111, a115, a119) = (
            a111 + a115 + a119,
            a111 + (a115 << 64) - (a119 << 32),
            a111 - (a115 << 32) + (a119 << 64),
        );
        let (a117, a109, a113) = (
            a117 + a109 + a113,
            a117 + (a109 << 64) - (a113 << 32),
            a117 - (a109 << 32) + (a113 << 64),
        );
        values[0] = a0;
        values[1] = a84;
        values[2] = a72;
        values[3] = a36;
        values[4] = a48;
        values[5] = a60;
        values[6] = a24;
        values[7] = a12;
        values[8] = a96;
        values[9] = a108;
        values[10] = a10;
        values[11] = a94;
        values[12] = a82;
        values[13] = a46;
        values[14] = a58;
        values[15] = a70;
        values[16] = a34;
        values[17] = a22;
        values[18] = a106;
        values[19] = a118;
        values[20] = a11;
        values[21] = a95;
        values[22] = a83;
        values[23] = a47;
        values[24] = a59;
        values[25] = a71;
        values[26] = a35;
        values[27] = a23;
        values[28] = a107;
        values[29] = a119;
        values[30] = a9;
        values[31] = a93;
        values[32] = a81;
        values[33] = a45;
        values[34] = a57;
        values[35] = a69;
        values[36] = a33;
        values[37] = a21;
        values[38] = a105;
        values[39] = a117;
        values[40] = a4;
        values[41] = a88;
        values[42] = a76;
        values[43] = a40;
        values[44] = a52;
        values[45] = a64;
        values[46] = a28;
        values[47] = a16;
        values[48] = a100;
        values[49] = a112;
        values[50] = a2;
        values[51] = a86;
        values[52] = a74;
        values[53] = a38;
        values[54] = a50;
        values[55] = a62;
        values[56] = a26;
        values[57] = a14;
        values[58] = a98;
        values[59] = a110;
        values[60] = a3;
        values[61] = a87;
        values[62] = a75;
        values[63] = a39;
        values[64] = a51;
        values[65] = a63;
        values[66] = a27;
        values[67] = a15;
        values[68] = a99;
        values[69] = a111;
        values[70] = a1;
        values[71] = a85;
        values[72] = a73;
        values[73] = a37;
        values[74] = a49;
        values[75] = a61;
        values[76] = a25;
        values[77] = a13;
        values[78] = a97;
        values[79] = a109;
        values[80] = a8;
        values[81] = a92;
        values[82] = a80;
        values[83] = a44;
        values[84] = a56;
        values[85] = a68;
        values[86] = a32;
        values[87] = a20;
        values[88] = a104;
        values[89] = a116;
        values[90] = a6;
        values[91] = a90;
        values[92] = a78;
        values[93] = a42;
        values[94] = a54;
        values[95] = a66;
        values[96] = a30;
        values[97] = a18;
        values[98] = a102;
        values[99] = a114;
        values[100] = a7;
        values[101] = a91;
        values[102] = a79;
        values[103] = a43;
        values[104] = a55;
        values[105] = a67;
        values[106] = a31;
        values[107] = a19;
        values[108] = a103;
        values[109] = a115;
        values[110] = a5;
        values[111] = a89;
        values[112] = a77;
        values[113] = a41;
        values[114] = a53;
        values[115] = a65;
        values[116] = a29;
        values[117] = a17;
        values[118] = a101;
        values[119] = a113;
    }
}

/// Size 128 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 128`.
pub fn ntt_128(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 128, 0);
    for values in values.chunks_exact_mut(128) {
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
        let a65 = (a65 << 25) - (a65 << 73);
        let a33 = (a33 << 3);
        let a97 = (a97 << 28) - (a97 << 76);
        let a17 = (a17 << 6);
        let a81 = (a81 << 31) - (a81 << 79);
        let a49 = (a49 << 9);
        let a113 = (a113 << 34) - (a113 << 82);
        let a66 = (a66 << 3);
        let a34 = (a34 << 6);
        let a98 = (a98 << 9);
        let a18 = (a18 << 12);
        let a82 = (a82 << 15);
        let a50 = (a50 << 18);
        let a114 = (a114 << 21);
        let a67 = (a67 << 28) - (a67 << 76);
        let a35 = (a35 << 9);
        let a99 = (a99 << 37) - (a99 << 85);
        let a19 = (a19 << 18);
        let a83 = (a83 << 46) - (a83 << 94);
        let a51 = (a51 << 27);
        let a115 = (a115 << 55) + (a115 << 7);
        let a68 = (a68 << 6);
        let a36 = (a36 << 12);
        let a100 = (a100 << 18);
        let a20 = (a20 << 24);
        let a84 = (a84 << 30);
        let a52 = (a52 << 36);
        let a116 = (a116 << 42);
        let a69 = (a69 << 31) - (a69 << 79);
        let a37 = (a37 << 15);
        let a101 = (a101 << 46) - (a101 << 94);
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
        let a71 = (a71 << 34) - (a71 << 82);
        let a39 = (a39 << 21);
        let a103 = (a103 << 55) + (a103 << 7);
        let a23 = (a23 << 42);
        let a87 = (a87 << 76) + (a87 << 28);
        let a55 = (a55 << 63);
        let a119 = (a119 << 49) - (a119 << 1);
        let a72 = (a72 << 12);
        let a40 = (a40 << 24);
        let a104 = (a104 << 36);
        let a24 = (a24 << 48);
        let a88 = (a88 << 60);
        let a56 = (a56 << 72);
        let a120 = (a120 << 84);
        let a73 = (a73 << 37) - (a73 << 85);
        let a41 = (a41 << 27);
        let a105 = (a105 << 64) + (a105 << 16);
        let a25 = (a25 << 54);
        let a89 = (a89 << 91) + (a89 << 43);
        let a57 = (a57 << 81);
        let a121 = (a121 << 70) - (a121 << 22);
        let a74 = (a74 << 15);
        let a42 = (a42 << 30);
        let a106 = (a106 << 45);
        let a26 = (a26 << 60);
        let a90 = (a90 << 75);
        let a58 = (a58 << 90);
        let a122 = (-(a122 << 9));
        let a75 = (a75 << 40) - (a75 << 88);
        let a43 = (a43 << 33);
        let a107 = (a107 << 73) + (a107 << 25);
        let a27 = (a27 << 66);
        let a91 = (a91 << 58) - (a91 << 10);
        let a59 = (-(a59 << 3));
        let a123 = (a123 << 91) - (a123 << 43);
        let a76 = (a76 << 18);
        let a44 = (a44 << 36);
        let a108 = (a108 << 54);
        let a28 = (a28 << 72);
        let a92 = (a92 << 90);
        let a60 = (-(a60 << 12));
        let a124 = (-(a124 << 30));
        let a77 = (a77 << 43) - (a77 << 91);
        let a45 = (a45 << 39);
        let a109 = (a109 << 82) + (a109 << 34);
        let a29 = (a29 << 78);
        let a93 = (a93 << 73) - (a93 << 25);
        let a61 = (-(a61 << 21));
        let a125 = -(a125 << 64) - (a125 << 16);
        let a78 = (a78 << 21);
        let a46 = (a46 << 42);
        let a110 = (a110 << 63);
        let a30 = (a30 << 84);
        let a94 = (-(a94 << 9));
        let a62 = (-(a62 << 30));
        let a126 = (-(a126 << 51));
        let a79 = (a79 << 46) - (a79 << 94);
        let a47 = (a47 << 45);
        let a111 = (a111 << 91) + (a111 << 43);
        let a31 = (a31 << 90);
        let a95 = (a95 << 88) - (a95 << 40);
        let a63 = (-(a63 << 39));
        let a127 = -(a127 << 85) - (a127 << 37);
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
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt, *};

    #[test]
    fn test_ntt_2() {
        test_ntt(ntt(2).unwrap());
    }

    #[test]
    fn test_ntt_3() {
        test_ntt(ntt(3).unwrap());
    }

    #[test]
    fn test_ntt_4() {
        test_ntt(ntt(4).unwrap());
    }

    #[test]
    fn test_ntt_5() {
        test_ntt(ntt(5).unwrap());
    }

    #[test]
    fn test_ntt_6() {
        test_ntt(ntt(6).unwrap());
    }

    #[test]
    fn test_ntt_8() {
        test_ntt(ntt(8).unwrap());
    }

    #[test]
    fn test_ntt_10() {
        test_ntt(ntt(10).unwrap());
    }

    #[test]
    fn test_ntt_12() {
        test_ntt(ntt(12).unwrap());
    }

    #[test]
    fn test_ntt_15() {
        test_ntt(ntt(15).unwrap());
    }

    #[test]
    fn test_ntt_16() {
        test_ntt(ntt(16).unwrap());
    }

    #[test]
    fn test_ntt_17() {
        test_ntt(ntt(17).unwrap());
    }

    #[test]
    fn test_ntt_20() {
        test_ntt(ntt(20).unwrap());
    }

    #[test]
    fn test_ntt_24() {
        test_ntt(ntt(24).unwrap());
    }

    #[test]
    fn test_ntt_30() {
        test_ntt(ntt(30).unwrap());
    }

    #[test]
    fn test_ntt_32() {
        test_ntt(ntt(32).unwrap());
    }

    #[test]
    fn test_ntt_34() {
        test_ntt(ntt(34).unwrap());
    }

    #[test]
    fn test_ntt_40() {
        test_ntt(ntt(40).unwrap());
    }

    #[test]
    fn test_ntt_48() {
        test_ntt(ntt(48).unwrap());
    }

    #[test]
    fn test_ntt_51() {
        test_ntt(ntt(51).unwrap());
    }

    #[test]
    fn test_ntt_60() {
        test_ntt(ntt(60).unwrap());
    }

    #[test]
    fn test_ntt_64() {
        test_ntt(ntt(64).unwrap());
    }

    #[test]
    fn test_ntt_68() {
        test_ntt(ntt(68).unwrap());
    }

    #[test]
    fn test_ntt_80() {
        test_ntt(ntt(80).unwrap());
    }

    #[test]
    fn test_ntt_85() {
        test_ntt(ntt(85).unwrap());
    }

    #[test]
    fn test_ntt_96() {
        test_ntt(ntt(96).unwrap());
    }

    #[test]
    fn test_ntt_102() {
        test_ntt(ntt(102).unwrap());
    }

    #[test]
    fn test_ntt_120() {
        test_ntt(ntt(120).unwrap());
    }

    #[test]
    fn test_ntt_128() {
        test_ntt(ntt(128).unwrap());
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::{super::bench::bench_ntt, *};
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        bench_ntt(criterion, "small", ntt(2).unwrap());
        bench_ntt(criterion, "small", ntt(3).unwrap());
        bench_ntt(criterion, "small", ntt(4).unwrap());
        bench_ntt(criterion, "small", ntt(5).unwrap());
        bench_ntt(criterion, "small", ntt(6).unwrap());
        bench_ntt(criterion, "small", ntt(8).unwrap());
        bench_ntt(criterion, "small", ntt(10).unwrap());
        bench_ntt(criterion, "small", ntt(12).unwrap());
        bench_ntt(criterion, "small", ntt(15).unwrap());
        bench_ntt(criterion, "small", ntt(16).unwrap());
        bench_ntt(criterion, "small", ntt(17).unwrap());
        bench_ntt(criterion, "small", ntt(20).unwrap());
        bench_ntt(criterion, "small", ntt(24).unwrap());
        bench_ntt(criterion, "small", ntt(30).unwrap());
        bench_ntt(criterion, "small", ntt(32).unwrap());
        bench_ntt(criterion, "small", ntt(34).unwrap());
        bench_ntt(criterion, "small", ntt(40).unwrap());
        bench_ntt(criterion, "small", ntt(48).unwrap());
        bench_ntt(criterion, "small", ntt(51).unwrap());
        bench_ntt(criterion, "small", ntt(60).unwrap());
        bench_ntt(criterion, "small", ntt(64).unwrap());
        bench_ntt(criterion, "small", ntt(68).unwrap());
        bench_ntt(criterion, "small", ntt(80).unwrap());
        bench_ntt(criterion, "small", ntt(85).unwrap());
        bench_ntt(criterion, "small", ntt(96).unwrap());
        bench_ntt(criterion, "small", ntt(102).unwrap());
        bench_ntt(criterion, "small", ntt(120).unwrap());
        bench_ntt(criterion, "small", ntt(128).unwrap());
    }
}
