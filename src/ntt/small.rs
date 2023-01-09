//! Generated using `cargo run --bin codegen`
use crate::Field;

/// Apply a small NTT to `values`, or return `false` if the size is not supported.
pub fn small_ntt(values: &mut [Field]) -> bool {
    match values.len() {
        ..=1 => return true,
        2 => ntt_2(values),
        4 => ntt_4(values),
        8 => ntt_8(values),
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

/// Size 4 NTT.
fn ntt_4(values: &mut [Field]) {
    debug_assert_eq!(values.len(), 4);
    let a0 = values[0];
    let a1 = values[1];
    let a2 = values[2];
    let a3 = values[3];
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = a3.mul_root_384(96);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    values[0] = a0;
    values[1] = a2;
    values[2] = a1;
    values[3] = a3;
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
    let a5 = a5.mul_root_384(48);
    let a2 = a2.mul_root_384(96);
    let a6 = a6.mul_root_384(144);
    let (a0, a2) = (a0 + a2, a0 - a2);
    let (a1, a3) = (a1 + a3, a1 - a3);
    let a3 = a3.mul_root_384(96);
    let (a0, a1) = (a0 + a1, a0 - a1);
    let (a2, a3) = (a2 + a3, a2 - a3);
    let (a4, a6) = (a4 + a6, a4 - a6);
    let (a5, a7) = (a5 + a7, a5 - a7);
    let a7 = a7.mul_root_384(96);
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

#[cfg(test)]
mod tests {
    use super::{super::ntt_naive, *};

    #[test]
    fn test_small_ntt() {
        for size in [0, 1, 2, 4, 8] {
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
    fn test_ntt_8() {
        let size = 8;
        let input = (0..size).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_8(output.as_mut_slice());
        let mut output_ref = input;
        ntt_naive(output_ref.as_mut_slice());
        assert_eq!(output, output_ref);
    }

}
