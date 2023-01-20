use crate::{divisors::split, permute::transpose, Field};
use rayon::prelude::*;

/// Cooley–Tukey $\sqrt{n}$ recursive NTT.
pub fn ntt(values: &mut [Field]) {
    const PAR_THRESHOLD: usize = 1 << 15;
    let n = values.len();

    // Interpret `values` as an approximately square matrix.
    let a = split(n);
    assert!(a > 1, "Unimplemented Cooley-Tukey split for for {n}");
    let b = n / a;

    if n < PAR_THRESHOLD {
        recurse(values, super::ntt, (a, b));
    } else {
        par_recurse(values, super::ntt, (a, b));
    }
}

/// Cooley-Tukey NTT.
pub fn recurse_copy(value: &mut [Field], inner: impl Fn(&mut [Field]), (a, b): (usize, usize)) {
    assert_eq!(value.len(), a * b);
    let root = Field::root(value.len() as u64)
        .expect("Vector length does not divide multiplicative group order.");

    // Operate over columns by copying
    let mut buffer = vec![Field::new(0); a];
    let mut omega_row = root;
    for j in 0..b {
        // Copy column into buffer
        for (i, buffer) in buffer.iter_mut().enumerate() {
            *buffer = value[i * b + j];
        }

        // Transform column
        inner(buffer.as_mut_slice());

        // Apply twiddles
        if j > 0 {
            let mut omega_col = omega_row;
            for value in buffer.iter_mut().skip(1) {
                *value *= omega_col;
                omega_col *= omega_row;
            }
            omega_row *= root;
        }

        // Copy buffer into column
        for (i, buffer) in buffer.iter().enumerate() {
            value[i * b + j] = *buffer;
        }
    }

    // Operate over rows directly
    value.chunks_exact_mut(b).for_each(&inner);

    // Do the final transpose to get the result in the right order.
    transpose(value, (a, b));
}

/// Cooley-Tukey six-step NTT.
pub fn recurse(value: &mut [Field], inner: impl Fn(&mut [Field]), (a, b): (usize, usize)) {
    debug_assert_eq!(value.len(), a * b);
    let root = Field::root(value.len() as u64)
        .expect("Vector length does not divide multiplicative group order.");

    transpose(value, (a, b));
    let mut omega_col = root;
    value.chunks_exact_mut(a).enumerate().for_each(|(i, row)| {
        inner(row);
        if i > 0 {
            let mut omega_row = omega_col;
            for value in row.iter_mut().skip(1) {
                *value *= omega_row;
                omega_row *= omega_col;
            }
            omega_col *= root;
        }
    });
    transpose(value, (b, a));
    value.chunks_exact_mut(b).for_each(&inner);
    transpose(value, (a, b));
}

/// Parallel Cooley-Tukey six-step NTT.
pub fn par_recurse(
    value: &mut [Field],
    inner: impl Fn(&mut [Field]) + Sync,
    (a, b): (usize, usize),
) {
    debug_assert_eq!(value.len(), a * b);
    let root = Field::root(value.len() as u64)
        .expect("Vector length does not divide multiplicative group order.");

    transpose(value, (a, b));

    value
        .par_chunks_exact_mut(a)
        .enumerate()
        .for_each_with(None, |omega_col, (row, values)| {
            inner(values);
            if row > 0 {
                let omega_col = omega_col.get_or_insert_with(|| root.pow(row as u64));
                let mut omega_row = *omega_col;
                for value in values.iter_mut().skip(1) {
                    *value *= omega_row;
                    omega_row *= *omega_col;
                }
                *omega_col *= root;
            }
        });
    transpose(value, (b, a));
    value.par_chunks_exact_mut(b).for_each(&inner);
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
            omega_row *= omega_col;
        }
        omega_col *= root;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{naive, tests::test_ntt_fn},
        *,
    };

    #[test]
    fn test_ntt_16() {
        test_ntt_fn(ntt, 16);
    }

    #[test]
    fn test_ntt_256() {
        test_ntt_fn(ntt, 256);
    }

    #[test]
    #[ignore]
    fn test_ntt_65536() {
        test_ntt_fn(ntt, 65536);
    }

    #[test]
    fn test_ntt_1024() {
        test_ntt_fn(ntt, 1024);
    }

    #[test]
    fn test_ntt_384() {
        test_ntt_fn(ntt, 384);
    }

    #[test]
    fn test_recurse_256_16_16() {
        test_ntt_fn(|values| recurse(values, naive::ntt, (16, 16)), 256);
    }

    #[test]
    fn test_recurse_256_4_64() {
        test_ntt_fn(|values| recurse(values, naive::ntt, (4, 64)), 256);
    }

    #[test]
    fn test_par_recurse_256_16_16() {
        test_ntt_fn(|values| par_recurse(values, naive::ntt, (16, 16)), 256);
    }

    #[test]
    fn test_par_recurse_256_4_64() {
        test_ntt_fn(|values| par_recurse(values, naive::ntt, (4, 64)), 256);
    }
}
