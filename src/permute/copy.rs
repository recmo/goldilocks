use rayon::prelude::*;
use std::{
    slice,
    sync::atomic::{AtomicPtr, Ordering},
};

pub fn transpose_copy_wo(matrix: &mut [u64], width: usize, height: usize) {
    let n = width * height;
    if n == 0 {
        return;
    }
    let q = n - 1;
    let inv_permute = |i| if i == q { q } else { i * width % q };
    let copy = matrix.to_vec();
    permute_wo_oop(&copy, matrix, 1, inv_permute);
}

pub fn transpose_copy_ro(matrix: &mut [u64], width: usize, height: usize) {
    let n = width * height;
    if n == 0 {
        return;
    }
    let q = n - 1;
    let permute = |i| if i == q { q } else { i * height % q };
    let copy = matrix.to_vec();
    unsafe {
        // Safety: `permute` is a permutation.
        permute_ro_oop(&copy, matrix, 1, permute);
    }
}

/// Parallel permuted copy in write order.
///
/// `inv_permute` is the *inverse* of the target permutation.
///
/// # Panics
///
/// Panics if `src` and `dst` have different lengths or if `chunk` does not
/// exactly divide the length.
pub fn permute_wo_oop<T: Copy + Send + Sync>(
    src: &[T],
    dst: &mut [T],
    chunk: usize,
    inv_permute: impl Fn(usize) -> usize + Send + Sync,
) {
    // Note: We could special case `chunk == 1`, but benchmarking showed no
    // improvement.

    // Threshold when to switch to parallel algorithm.
    const PAR_THRESHOLD: usize = 1_usize << 19;

    assert_eq!(src.len(), dst.len());
    assert_eq!(src.len() % chunk, 0);
    let n = src.len() / chunk;
    if src.len() < PAR_THRESHOLD {
        for (i, dst) in dst.chunks_exact_mut(chunk).enumerate() {
            let i = inv_permute(i);
            assert!(i < n, "Permutation exceeds range");
            let src = &src[i * chunk..(i + 1) * chunk];
            dst.copy_from_slice(src);
        }
    } else {
        dst.par_chunks_exact_mut(chunk)
            .enumerate()
            .for_each(|(i, dst)| {
                let i = inv_permute(i);
                assert!(i < n, "Permutation exceeds range");
                let src = &src[i * chunk..(i + 1) * chunk];
                dst.copy_from_slice(src);
            });
    }
}

/// Parallel permuted copy in read order.
///
/// # Safety
///
/// `permute` must be a permutation. If it returns the same value twice, the
/// result is undefined.
///
/// # Panics
///
/// Panics if `src` and `dst` have different lengths or if `chunk` does not
/// exactly divide the length.
pub unsafe fn permute_ro_oop<T: Copy + Send + Sync>(
    src: &[T],
    dst: &mut [T],
    chunk: usize,
    permute: impl Fn(usize) -> usize + Send + Sync,
) {
    // Note: Performance drops compared to `wo`, before the parallelization
    // threshold but it is still faster to run serial.
    const PAR_THRESHOLD: usize = 1_usize << 19;

    assert_eq!(src.len(), dst.len());
    assert_eq!(src.len() % chunk, 0);
    let n = src.len() / chunk;

    if src.len() < PAR_THRESHOLD {
        for (i, src) in src.chunks_exact(chunk).enumerate() {
            let i = permute(i);
            assert!(i < n, "Permutation exceeds range");
            let dst = &mut dst[i * chunk..(i + 1) * chunk];
            dst.copy_from_slice(src);
        }
    } else {
        let ptr = AtomicPtr::new(dst.as_mut_ptr());
        src.par_chunks_exact(chunk)
            .enumerate()
            .for_each(|(i, src)| {
                let i = permute(i);
                let dst: &mut [T] = unsafe {
                    // Safety: Since `permute` is a permutation of `0..n`, we
                    // are guaranteed that each `i` value occurs exactly once.
                    // So none of the slices we make here can overlap.
                    slice::from_raw_parts_mut(ptr.load(Ordering::Relaxed).add(i * chunk), chunk)
                };
                dst.copy_from_slice(src);
            });
    }
}

#[cfg(test)]
mod tests {
    use super::{super::transpose_copy, *};
    use proptest::prelude::*;

    #[test]
    #[rustfmt::skip]
    fn test_transpose_copy_wo() {
        let mut matrix = [
            0, 1, 2, 
            3, 4, 5
        ];
        transpose_copy_wo(&mut matrix, 3, 2);
        assert_eq!(matrix, [
            0, 3, 
            1, 4, 
            2, 5
        ]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_transpose_copy_ro() {
        let mut matrix = [
            0, 1, 2, 
            3, 4, 5
        ];
        transpose_copy_ro(&mut matrix, 3, 2);
        assert_eq!(matrix, [
            0, 3, 
            1, 4, 
            2, 5
        ]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_transpose_copy_wo_ref() {
        proptest!(|(width in 0_usize..100, height in 0_usize..100)| {
            let n = width * height;
            let mut matrix = (0_u64..n as u64).collect::<Vec<_>>();
            let mut reference = matrix.clone();
            transpose_copy_wo(&mut matrix, width, height);
            transpose_copy(&mut reference, (height, width));
            assert_eq!(matrix, reference);
        });
    }

    #[test]
    #[rustfmt::skip]
    fn test_transpose_copy_ro_ref() {
        proptest!(|(width in 0_usize..100, height in 0_usize..100)| {
            let n = width * height;
            let mut matrix = (0_u64..n as u64).collect::<Vec<_>>();
            let mut reference = matrix.clone();
            transpose_copy_ro(&mut matrix, width, height);
            transpose_copy(&mut reference, (height, width));
            assert_eq!(matrix, reference);
        });
    }
}
