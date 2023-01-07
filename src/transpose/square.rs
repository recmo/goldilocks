use std::sync::atomic::{AtomicPtr, Ordering};

pub fn transpose_square_pub(a: &mut [u64], width: usize, height: usize) {
    assert_eq!(a.len(), width * height);
    assert_eq!(width, height);
    assert!(width.is_power_of_two());
    unsafe {
        transpose_square(a.as_mut_ptr(), width, width);
    }
}

unsafe fn transpose_square(a: *mut u64, stride: usize, n: usize) {
    const REC_THRESHOLD: usize = 1 << 5;
    const PAR_THRESHOLD: usize = 1 << 10;

    if n < REC_THRESHOLD {
        for i in 0..n {
            for j in 0..i {
                a.add(i * stride + j).swap(a.add(j * stride + i));
            }
        }
    } else if n < PAR_THRESHOLD {
        let n = n >> 1;
        transpose_square(a, stride, n);
        transpose_swap_square(a.add(n), a.add(n * stride), stride, n);
    } else {
        let a = AtomicPtr::new(a);
        let n = n >> 1;
        rayon::join(
            || {
                transpose_swap_square(
                    a.load(Ordering::Relaxed).add(n),
                    a.load(Ordering::Relaxed).add(n * stride),
                    stride,
                    n,
                )
            },
            || {
                rayon::join(
                    || transpose_square(a.load(Ordering::Relaxed), stride, n),
                    || transpose_square(a.load(Ordering::Relaxed).add(n * stride + n), stride, n),
                )
            },
        );
    }
}

unsafe fn transpose_swap_square(a: *mut u64, b: *mut u64, stride: usize, n: usize) {
    const REC_THRESHOLD: usize = 1 << 4;
    const PAR_THRESHOLD: usize = 1 << 10;

    if n < REC_THRESHOLD {
        for i in 0..n {
            for j in 0..n {
                a.add(i * stride + j).swap(a.add(j * stride + i));
            }
        }
    } else if n < PAR_THRESHOLD {
        let n = n >> 1;
        transpose_swap_square(a, b, stride, n);
        transpose_swap_square(a.add(n), b.add(n * stride), stride, n);
        transpose_swap_square(a.add(n * stride), b.add(n), stride, n);
        transpose_swap_square(a.add(n * stride + n), b.add(n * stride + n), stride, n);
    } else {
        let a = AtomicPtr::new(a);
        let b = AtomicPtr::new(b);
        let n = n >> 1;
        rayon::join(
            || {
                rayon::join(
                    || {
                        transpose_swap_square(
                            a.load(Ordering::Relaxed),
                            b.load(Ordering::Relaxed),
                            stride,
                            n,
                        )
                    },
                    || {
                        transpose_swap_square(
                            a.load(Ordering::Relaxed).add(n),
                            b.load(Ordering::Relaxed).add(n * stride),
                            stride,
                            n,
                        )
                    },
                )
            },
            || {
                rayon::join(
                    || {
                        transpose_swap_square(
                            a.load(Ordering::Relaxed).add(n * stride),
                            b.load(Ordering::Relaxed).add(n),
                            stride,
                            n,
                        )
                    },
                    || {
                        transpose_swap_square(
                            a.load(Ordering::Relaxed).add(n * stride + n),
                            b.load(Ordering::Relaxed).add(n * stride + n),
                            stride,
                            n,
                        )
                    },
                )
            },
        );
    }
}
