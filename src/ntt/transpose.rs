use super::prefetch::PrefetchIndex;
use tracing::{instrument, trace};
use rayon::prelude::*;

// TODO: See https://github.com/ejmahler/transpose/blob/master/src/in_place.rs
// TODO: See https://github.com/preiter93/transpose/blob/master/src/inplace.rs

// https://link.springer.com/chapter/10.1007/978-3-642-55195-6_10
// https://hal.inria.fr/hal-02960539/file/IEEE_TC_GodardLoechnerBastoul.pdf


// TODO: Outer cache-oblivious layer for mmap-backed.
// TODO: Parallel transpose.

/// Transpose a square matrix of stretches.
///
/// The matrix is composed of `size` x `size` stretches of length `stretch`.
///
/// `stretch` can only be `1` or `2`.
#[instrument(skip(matrix))]
pub fn transpose_square_stretch<T>(matrix: &mut [T], size: usize, stretch: usize) {
    trace!(
        "Transposing {} x {} square matrix of {} stretches",
        size,
        size,
        stretch
    );
    trace!("BEGIN Transpose");
    assert_eq!(matrix.len(), size * size * stretch);
    match stretch {
        1 => transpose_square_1(matrix, size),
        2 => transpose_square_2(matrix, size),
        _ => unimplemented!("Only stretch sizes 1 and 2 are supported"),
    }
    trace!("END Transpose");
}

// TODO: Handle odd sizes
fn transpose_square_1<T>(matrix: &mut [T], size: usize) {
    const CACHE_LINE_SIZE: usize = 64;
    const PREFETCH_STRIDE: usize = 0;
    debug_assert_eq!(matrix.len(), size * size);
    if size % 2 != 0 {
        unimplemented!("Odd sizes are not supported");
    }
    // Number of elements to fetch at a time.
    let min_fetch = std::cmp::max(1, CACHE_LINE_SIZE / std::mem::size_of::<T>());

    // Iterate over upper-left triangle, working in 2x2 blocks
    // Stretches of two are useful because they span a 64B cache line when T is 32
    // bytes.
    for row in (0..size).step_by(2) {
        let i = row * size + row;
        matrix.swap(i + 1, i + size);
        for col in (row..size).step_by(2).skip(1) {
            let i = row * size + col;
            let j = col * size + row;
            if PREFETCH_STRIDE > 0 && col + PREFETCH_STRIDE * 2 < size {
                matrix.prefetch_index_write(i + PREFETCH_STRIDE * 2);
                matrix.prefetch_index_write(i + PREFETCH_STRIDE * 2 + size);
                matrix.prefetch_index_write(j + PREFETCH_STRIDE * 2 * size);
                matrix.prefetch_index_write(j + PREFETCH_STRIDE * 2 * size + size);
            }
            matrix.swap(i, j);
            matrix.swap(i + 1, j + size);
            matrix.swap(i + size, j + 1);
            matrix.swap(i + size + 1, j + size + 1);
        }
    }
}

fn transpose_square_2<T>(matrix: &mut [T], size: usize) {
    const PREFETCH_STRIDE: usize = 8;
    debug_assert_eq!(matrix.len(), 2 * size * size);

    // Iterate over upper-left triangle, working in 1x2 blocks
    for row in 0..size {
        for col in (row..size).skip(1) {
            let i = (row * size + col) * 2;
            let j = (col * size + row) * 2;
            if PREFETCH_STRIDE > 0 && col + PREFETCH_STRIDE < size {
                // Hardware prefetcher picks up on the first one.
                // matrix.prefetch_index_write(i + PREFETCH_STRIDE * 2);
                matrix.prefetch_index_write(i + PREFETCH_STRIDE * 2 * size);
            }
            matrix.swap(i, j);
            matrix.swap(i + 1, j + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{
        prop_assert_eq, prop_assume, proptest,
        strategy::{Just, Strategy},
    };

    fn reference(matrix: &[u32], size: usize, stretch: usize) -> Vec<u32> {
        assert_eq!(matrix.len(), size * size * stretch);
        let mut result = matrix.to_vec();
        for i in 0..size {
            for j in 0..size {
                for k in 0..stretch {
                    let a = (i * size + j) * stretch + k;
                    let b = (j * size + i) * stretch + k;
                    result[b] = matrix[a];
                }
            }
        }
        result
    }

    fn arb_matrix_sized(size: usize, stretch: usize) -> impl Strategy<Value = Vec<u32>> {
        #[allow(clippy::cast_possible_truncation)]
        Just((0..size * size * stretch).map(|i| i as u32).collect())
    }

    fn arb_matrix() -> impl Strategy<Value = (Vec<u32>, usize, usize)> {
        (0_usize..=100, 1_usize..=2).prop_flat_map(|(size, stretch)| {
            (arb_matrix_sized(size, stretch), Just(size), Just(stretch))
        })
    }

    /// Reference transpose is its own inverse
    #[test]
    fn reference_involutory() {
        proptest!(|((orig, size, stretch) in arb_matrix())| {
            let transposed = reference(&orig, size, stretch);
            let result = reference(&transposed, size ,stretch);
            prop_assert_eq!(result, orig);
        });
    }

    /// Transpose matches reference
    #[test]
    fn compare_reference() {
        proptest!(|((mut matrix, size, stretch) in arb_matrix())| {
            prop_assume!(stretch == 2 || size % 2 == 0);
            let expected = reference(&matrix, size, stretch);
            transpose_square_stretch(&mut matrix, size, stretch);
            prop_assert_eq!(matrix, expected);
        });
    }
}

#[cfg(feature = "criterion")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use crate::Field;
    use rayon::prelude::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        bench_transpose_square_1(criterion);
        bench_lib_transpose_inplace(criterion);
        bench_lib_transpose(criterion);
        bench_lib_transpose2(criterion);
        bench_lib_transpose2_oop(criterion);
    }

    fn rand_vec(size: usize) -> Vec<Field> {
        let mut result = vec![Field::from(0); size];
        result.par_iter_mut().for_each(|x| *x = rand::random());
        result
    }

    pub fn bench_transpose_square_1(c: &mut Criterion) {
        let mut group = c.benchmark_group("transpose_square_1");
        group.sample_size(10);

        for i in 10..=16 {
            let size = 1_usize << i;
            group.bench_function(format!("{size}x{size}"), |b| {
                let mut matrix = rand_vec(size * size);
                b.iter(|| transpose_square_1(&mut matrix, size))
            });
        }
        group.finish();
    }

    pub fn bench_lib_transpose_inplace(c: &mut Criterion) {
        use ::transpose1::transpose_inplace;
        let mut group = c.benchmark_group("transpose1_ip");
        group.sample_size(10);

        for i in 10..=14 {
            let size = 1_usize << i;
            group.bench_function(format!("{size}x{size}"), |b| {
                let mut matrix = rand_vec(size * size);
                let mut scratch = vec![Field::from(0); size];
                b.iter(|| transpose_inplace(&mut matrix, &mut scratch, size, size))
            });
        }
        group.finish();
    }

    pub fn bench_lib_transpose(c: &mut Criterion) {
        use ::transpose1::transpose;
        let mut group = c.benchmark_group("transpose1_oop");
        group.sample_size(10);

        for i in 10..=15 {
            let size = 1_usize << i;
            group.bench_function(format!("{size}x{size}"), |b| {
                let mut matrix = rand_vec(size * size);
                let mut out = vec![Field::from(0); size * size];
                b.iter(|| transpose(&matrix, &mut out, size, size))
            });
        }
        group.finish();
    }

    pub fn bench_lib_transpose2(c: &mut Criterion) {
        use ::transpose2::ip_transpose;
        let mut group = c.benchmark_group("transpose2_ip");
        group.sample_size(10);

        for i in 10..=16 {
            let size = 1_usize << i;
            group.bench_function(format!("{size}x{size}"), |b| {
                let mut matrix = rand_vec(size * size);
                let mut scratch = vec![Field::from(0); size];
                b.iter(|| ip_transpose(&mut matrix, &mut scratch, size, size))
            });
        }
        group.finish();
    }

    pub fn bench_lib_transpose2_oop(c: &mut Criterion) {
        use ::transpose2::oop_transpose;
        let mut group = c.benchmark_group("transpose2_oop");
        group.sample_size(10);

        for i in 10..=16 {
            let size = 1_usize << i;
            group.bench_function(format!("{size}x{size}"), |b| {
                let mut matrix = rand_vec(size * size);
                let mut out = vec![Field::from(0); size * size];
                b.iter(|| oop_transpose(&matrix, &mut out, size, size))
            });
        }
        group.finish();
    }
}
