use super::prefetch::PrefetchIndex;
use rayon::prelude::*;
use std::mem::size_of;
use tracing::{instrument, trace};

// TODO: See https://github.com/ejmahler/transpose/blob/master/src/in_place.rs
// TODO: See https://github.com/preiter93/transpose/blob/master/src/inplace.rs
// See https://github.com/bluss/ndarray-extra/blob/master/src/transpose.rs
// https://athemathmo.github.io/2016/08/29/inplace-transpose.html

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

fn transpose_naive<T>(matrix: &mut [T], size: usize) {
    for row in (0..size).step_by(2) {
        let i = row * size + row;
        unsafe {
            matrix.swap_unchecked(i + 1, i + size);
        }
        for col in (row..size).step_by(2).skip(1) {
            let i = row * size + col;
            let j = col * size + row;
            unsafe {
                matrix.swap_unchecked(i, j);
                matrix.swap_unchecked(i + 1, j + size);
                matrix.swap_unchecked(i + size, j + 1);
                matrix.swap_unchecked(i + size + 1, j + size + 1);
            }
        }
    }
    // for c in 0..size - 1 {
    //     for r in c + 1..size {
    //         let i = r * size + c;
    //         let j = c * size + r;
    //         matrix.swap(i, j);
    //     }
    // }
}

/// Recurse on 2x2 blocks.
///
/// [ A B ] T   [ Aᵀ Cᵀ ]
/// [ C D ]  =  [ Bᵀ Dᵀ ]
///
/// <https://en.algorithmica.org/hpc/external-memory/oblivious/#matrix-transposition>
fn recurse<T>(a: &mut [T], n: usize, N: usize) {
    if n <= 32 {
        // Base case
        for i in 0..n {
            for j in 0..i {
                unsafe {
                    a.swap_unchecked(i * N + j, j * N + i);
                }
            }
        }
    } else {
        let k = n / 2;
        recurse(a, k, N);
        recurse(&mut a[k..], k, N);
        recurse(&mut a[k * N..], k, N);
        recurse(&mut a[k * N + k..], k, N);

        for i in 0..k {
            for j in 0..k {
                unsafe {
                    a.swap_unchecked((i + k) * N + j, i * N + (j + k));
                }
            }
        }

        // TODO: Handle odd n
    }
}

pub fn tiled<T: Sync>(a: &mut [T], n: usize) {
    use std::cmp::min;
    const TILE_SIZE: usize = 32;

    let a = a.as_ptr() as u64;

    (0..n).into_par_iter().step_by(TILE_SIZE).for_each(|i| {
        for j in (i..n).step_by(TILE_SIZE) {
            for ii in i..min(i + TILE_SIZE, n) {
                for jj in j..min(j + TILE_SIZE, n) {
                    unsafe {
                        let a = a as *mut T;
                        let src = a.offset((ii * n + jj) as isize);
                        let dst = a.offset((jj * n + ii) as isize);
                        std::ptr::swap(src, dst);
                    }
                }
            }
        }
    })
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
    fn test_recurse() {
        let size = 256;
        let mut matrix = (0..size * size).map(|i| i as u32).collect::<Vec<_>>();
        let expected = reference(&matrix, size, 1);
        recurse(&mut matrix, size, size);
        assert_eq!(matrix, expected);
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
    use std::mem::transmute;

    use super::*;
    use crate::Field;
    use criterion::{BatchSize, Criterion, Throughput};
    use rayon::prelude::*;

    pub fn group(criterion: &mut Criterion) {
        bench_read(criterion);
        bench_write(criterion);
        bench_update(criterion);

        bench_recurse(criterion);
        bench_tiled(criterion);
        bench_transpose_naive(criterion);
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

    pub fn bench_recurse(c: &mut Criterion) {
        let mut group = c.benchmark_group("transpose/recurse");
        let max = if cfg!(test) { 5 } else { 16 };
        for i in 5..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                b.iter_batched_ref(
                    || rand_vec(size * size),
                    |m| recurse(m.as_mut_slice(), size, size),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_read(c: &mut Criterion) {
        let mut group = c.benchmark_group("mem/read");
        let max = if cfg!(test) { 5 } else { 16 };
        for i in 5..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                b.iter_batched_ref(
                    || unsafe { transmute::<_, Vec<u64>>(rand_vec(size * size)) },
                    |m| {
                        let mut sum = 0_u64;
                        for a in m.iter() {
                            sum = sum.wrapping_add(*a);
                        }
                        sum
                    },
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_write(c: &mut Criterion) {
        let mut group = c.benchmark_group("mem/write");
        let max = if cfg!(test) { 5 } else { 16 };
        for i in 5..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                b.iter_batched_ref(
                    || unsafe { transmute::<_, Vec<u64>>(rand_vec(size * size)) },
                    |m| {
                        for a in m.iter_mut() {
                            *a = 1337;
                        }
                    },
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_update(c: &mut Criterion) {
        let mut group = c.benchmark_group("mem/update");
        let max = if cfg!(test) { 5 } else { 16 };
        for i in 5..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                b.iter_batched_ref(
                    || unsafe { transmute::<_, Vec<u64>>(rand_vec(size * size)) },
                    |m| {
                        for a in m.iter_mut() {
                            *a = a.wrapping_add(1337);
                        }
                    },
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_tiled(c: &mut Criterion) {
        let mut group = c.benchmark_group("transpose/tiled");
        let max = if cfg!(test) { 5 } else { 16 };
        for i in 5..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                b.iter_batched_ref(
                    || rand_vec(size * size),
                    |m| tiled(m.as_mut_slice(), size),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_transpose_naive(c: &mut Criterion) {
        let mut group = c.benchmark_group("transpose/naive");
        let max = if cfg!(test) { 5 } else { 16 };
        for i in 5..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                b.iter_batched_ref(
                    || rand_vec(size * size),
                    |m| transpose_naive(m.as_mut_slice(), size),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_transpose_square_1(c: &mut Criterion) {
        let mut group = c.benchmark_group("transpose/square_1");
        let max = if cfg!(test) { 5 } else { 16 };
        for i in 10..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                b.iter_batched_ref(
                    || rand_vec(size * size),
                    |m| transpose_square_1(m.as_mut_slice(), size),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_lib_transpose_inplace(c: &mut Criterion) {
        use ::transpose1::transpose_inplace;
        let mut group = c.benchmark_group("transpose/lib1_ip");
        let max = if cfg!(test) { 5 } else { 14 };
        for i in 10..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                let mut scratch = vec![Field::from(0); size];
                b.iter_batched_ref(
                    || rand_vec(size * size),
                    |m| transpose_inplace(m.as_mut_slice(), &mut scratch, size, size),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_lib_transpose(c: &mut Criterion) {
        use ::transpose1::transpose;
        let mut group = c.benchmark_group("transpose/lib1_oop");
        let max = if cfg!(test) { 5 } else { 16 };
        for i in 10..=max {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                let mut out = vec![Field::from(0); size * size];
                b.iter_batched_ref(
                    || rand_vec(size * size),
                    |m| transpose(m.as_slice(), &mut out, size, size),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_lib_transpose2(c: &mut Criterion) {
        use ::transpose2::ip_transpose;
        let mut group = c.benchmark_group("transpose/lib2_ip");
        for i in 10..=16 {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                let mut scratch = vec![Field::from(0); size];
                b.iter_batched_ref(
                    || rand_vec(size * size),
                    |m| ip_transpose(m.as_mut_slice(), &mut scratch, size, size),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    pub fn bench_lib_transpose2_oop(c: &mut Criterion) {
        use ::transpose2::oop_transpose;
        let mut group = c.benchmark_group("transpose/lib2_oop");
        for i in 10..=16 {
            let size = 1_usize << i;
            group.throughput(Throughput::Elements((size * size) as u64));
            group.sample_size(if i < 10 { 100 } else { 10 });
            group.bench_function(format!("{size}x{size}"), |b| {
                let mut out = vec![Field::from(0); size * size];
                b.iter_batched_ref(
                    || rand_vec(size * size),
                    |m| oop_transpose(m.as_slice(), &mut out, size, size),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }
}
