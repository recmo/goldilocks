
pub unsafe fn transpose_square(a: *mut u64, stride: usize, n: usize) {
    assert!(n >= 8);
    assert!(n.is_power_of_two());
    if n == 8 {
        todo!(); // transpose_8(a, stride);
    } else {
        let n = n >> 1;
        transpose_square(a, stride, n);
        transpose_swap_square(a.add(n), a.add(n * stride), stride, n);
        transpose_swap_square(a.add(n * stride), a.add(n), stride, n);
        transpose_square(a.add(n * stride + n), stride, n);
    }
}

pub unsafe fn transpose_swap_square(a: *mut u64, b: *mut u64, stride: usize, n: usize) {
    assert!(n >= 8);
    assert!(n.is_power_of_two());
    if n == 8 {
        todo!(); // transpose_swap_8(a, b, stride);
    } else {
        let n = n >> 1;
        transpose_swap_square(a, b, stride, n);
        transpose_swap_square(a.add(n), b.add(n * stride), stride, n);
        transpose_swap_square(a.add(n * stride), b.add(n), stride, n);
        transpose_swap_square(a.add(n * stride + n), b.add(n * stride + n), stride, n);
    }
}
