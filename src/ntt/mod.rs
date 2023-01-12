pub mod rader;
pub mod recursive;
pub mod small;

use crate::Field;

pub fn ntt(values: &mut [Field]) {
    recursive::ntt(values);
}

pub fn intt(values: &mut [Field]) {
    intt_naive(values);
}

pub fn ntt_naive(v: &mut [Field]) {
    if v.is_empty() {
        return;
    }
    let root = Field::root(v.len() as u64)
        .expect("Vector length does not divide multiplicative group order.");

    let copy = v.to_vec();
    let mut omega_i = Field::from(1);
    for x in v.iter_mut() {
        let mut sum = Field::from(0);
        let mut omega_ij = Field::from(1);
        for y in &copy {
            sum += *y * omega_ij;
            omega_ij *= omega_i;
        }
        *x = sum;
        omega_i *= root;
    }
}

pub fn intt_naive(v: &mut [Field]) {
    if v.is_empty() {
        return;
    }
    // Scale in the inverse
    let scale = Field::from(v.len() as u64).inv();
    for x in v.iter_mut() {
        *x *= scale;
    }

    let root = Field::root(v.len() as u64)
        .expect("Vector length does not divide multiplicative group order.")
        .pow(v.len() as u64 - 1);
    let copy = v.to_vec();
    let mut omega_i = Field::from(1);
    for x in v.iter_mut() {
        let mut sum = Field::from(0);
        let mut omega_ij = Field::from(1);
        for y in &copy {
            sum += *y * omega_ij;
            omega_ij *= omega_i;
        }
        *x = sum;
        omega_i *= root;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ntt_old::Fft, permute::bit_reverse};

    #[test]
    fn test_ntt_naive_old() {
        let input = (0..4).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_naive(&mut output);
        let mut output_old = input;
        output_old.fft();
        bit_reverse(output_old.as_mut_slice());
        assert_eq!(output, output_old);
    }

    #[test]
    fn test_ntt_intt_naive() {
        let input = (100..108).map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt_naive(&mut output);
        intt_naive(&mut output);
        assert_eq!(input, output);
    }
}
