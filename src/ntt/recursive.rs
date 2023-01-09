use super::small::small_ntt;
use crate::{ntt::ntt_naive, permute::transpose_copy, Field};

/// Recursive Number Theoretic Transform.
///
/// Cooley–Tukey Mixed-Radix "six-step" FFT algorithm with [`small_ntt`] as
/// base case.
///
/// * David H. Bailey (1989). "FFTs in External or Hierarchical Memory". <https://www.davidhbailey.com/dhbpapers/fftq.pdf>
///
/// * <https://dl.acm.org/doi/10.1145/1464291.1464352>
///
/// * <https://users.ece.cmu.edu/~franzf/papers/fft-enc11.pdf>
pub fn four_step(value: &mut [Field]) {
    let n = value.len();
    if n <= 1 {
        return;
    }

    // Try an optimized small NTT.
    // if small_ntt(value) {
    //     return;
    // }

    // Base case for prime sizes.
    if n == 2 || n == 3 || n == 5 || n == 17 || n == 257 || n == 65537 {
        ntt_naive(value);
        return;
    }

    // Interpret `value` as an approximately square matrix.
    let a = divisor_split(n);
    if a <= 1 {
        panic!("Unimplemented four-step NTT for {}", n);
    }
    let b = n / a;
    // eprintln!("{} = {a} × {b}", n);
    // Reinterpret as a × b matrix.

    // Transpose to b × a matrix.
    transpose_copy(value, (a, b));

    // Compute `a`-sized FFTs.
    value.chunks_exact_mut(a).for_each(four_step);

    // Apply twiddle factors.
    twiddle(value, (b, a));

    // Transpose to a × b matrix.
    transpose_copy(value, (b, a));

    // Compute `b`-sized FFTs.
    value.chunks_exact_mut(b).for_each(four_step);

    // Transpose back to get results in order.
    transpose_copy(value, (a, b));
}

/// Goldilocks divisor split.
///
/// Given composite `n` that divides the multiplicative group order,
/// return `(a, b)` such that `a <= b`, `a != 1`, `n = a * b` and `a` and `b`
/// are as close as possible.
#[must_use]
pub fn divisor_split(n: usize) -> usize {
    if n == 0 || (Field::MODULUS - 1) % (n as u64) != 0 {
        panic!("{} does not divide multiplicative group order.", n);
    }
    let shift = n.trailing_zeros() / 2;
    let k = match n >> (2 * shift) {
        1..=3 => 1,
        5 => 1,
        6 => 2,
        10 => 2,
        _ => panic!("Unimplemented divisor split for {}", n),
    };
    k << shift
}

fn twiddle(value: &mut [Field], (rows, cols): (usize, usize)) {
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
    use super::{super::ntt_naive, *};

    #[test]
    fn test_divisor_split() {
        assert_eq!(divisor_split(1), 1);
        assert_eq!(divisor_split(2), 1);
        assert_eq!(divisor_split(3), 1);
        assert_eq!(divisor_split(4), 2);
        assert_eq!(divisor_split(5), 1);
        assert_eq!(divisor_split(6), 2);
        assert_eq!(divisor_split(8), 2);
        assert_eq!(divisor_split(16), 4);
        assert_eq!(divisor_split(10), 2);
        // TODO assert_eq!(divisor_split(20), 4);
    }
    #[test]
    fn test_four_step_8() {
        let input = (0..8).rev().map(Field::from).collect::<Vec<_>>();
        let mut output = input.clone();
        four_step(&mut output);
        let mut reference = input;
        ntt_naive(&mut reference);
        assert_eq!(output, reference);
    }

    #[test]
    fn test_four_step_square() {
        for twos in 0..10 {
            let size = 1 << twos;
            let size = size * size;
            let input = (0..size).map(Field::from).collect::<Vec<_>>();
            let mut output = input.clone();
            four_step(&mut output);
            let mut reference = input;
            ntt_naive(&mut reference);
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
            four_step(&mut output);
            let mut reference = input;
            ntt_naive(&mut reference);
            assert_eq!(output, reference);
        }
    }
}
