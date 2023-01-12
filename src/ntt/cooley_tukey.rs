use super::small::small_ntt;
use crate::{divisors::split, ntt::naive, permute::gw18::transpose, Field};
use rayon::prelude::*;

/// Cooley–Tukey Mixed-Radix "six-step" FFT algorithm.
pub fn ntt(value: &mut [Field]) {
    const PAR_THRESHOLD: usize = 1 << 15;

    let n = value.len();
    if n <= 1 {
        return;
    }

    // Try an optimized small NTT.
    if !cfg!(test) {
        if small_ntt(value) {
            return;
        }
    }

    // Base case for prime sizes.
    if n == 2 || n == 3 || n == 5 || n == 17 || n == 257 || n == 65537 {
        naive::ntt(value);
        return;
    }

    // Interpret `value` as an approximately square matrix.
    let a = split(n);
    if a <= 1 {
        panic!("Unimplemented four-step NTT for {}", n);
    }
    let b = n / a;
    // Reinterpret as a × b matrix.

    // Transpose to b × a matrix.
    transpose(value, (a, b));

    // Compute `a`-sized FFTs.
    if n < PAR_THRESHOLD {
        value.chunks_exact_mut(a).for_each(ntt);
    } else {
        value.par_chunks_exact_mut(a).for_each(ntt);
    }

    // Apply twiddle factors.
    twiddle(value, (b, a));

    // Transpose to a × b matrix.
    transpose(value, (b, a));

    // Compute `b`-sized FFTs.
    if n < PAR_THRESHOLD {
        value.chunks_exact_mut(b).for_each(ntt);
    } else {
        value.par_chunks_exact_mut(b).for_each(ntt);
    }

    // Transpose back to get results in order.
    transpose(value, (a, b));
}

pub fn twiddle(value: &mut [Field], (rows, cols): (usize, usize)) {
    assert_eq!(value.len(), rows * cols);

    let root = Field::root(value.len() as u64)
        .expect("Vector length does not divide multiplicative group order.");
    debug_assert_eq!(root.pow((value.len() / 2) as u64), -Field::from(1));
    debug_assert_eq!(root.pow(value.len() as u64), Field::from(1));

    let mut omega_col = root;
    for j in 1..rows {
        let mut omega_row = omega_col;
        for i in 1..cols {
            value[j * cols + i] *= omega_row;
            if i < cols - 1 {
                omega_row *= omega_col;
            }
        }
        if j < rows - 1 {
            omega_col *= root;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::naive, *};

    #[test]
    fn test_four_step_8() {
        let input = (0..8).rev().map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        ntt(&mut output);
        let mut reference = input;
        naive::ntt(&mut reference);
        assert_eq!(output, reference);
    }

    #[test]
    fn test_four_step_square() {
        for twos in 0..10 {
            let size = 1 << twos;
            let size = size * size;
            let input = (0..size).map(Field::from).collect::<Vec<_>>();
            let mut output = input.clone();
            ntt(&mut output);
            let mut reference = input;
            naive::ntt(&mut reference);
            assert_eq!(output, reference);
        }
    }

    #[test]
    fn test_four_step_rectangular() {
        for twos in 0..10 {
            let size = 1 << twos;
            let size = size * size;
            let input = (0..size).map(Field::from).collect::<Vec<_>>();
            let mut output = input.clone();
            ntt(&mut output);
            let mut reference = input;
            naive::ntt(&mut reference);
            assert_eq!(output, reference);
        }
    }
}
