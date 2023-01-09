pub fn transpose(width: usize, height: usize) -> impl Fn(usize) -> usize {
    let q = (width * height).saturating_sub(1);
    move |i| if i == q { q } else { i * width % q }
}

/// Bit-reversal permutation.
///
/// # Panics
///
/// Panics if `size` is not a power of two.
pub fn bit_reverse(size: usize) -> impl Fn(usize) -> usize {
    assert!(size.is_power_of_two());
    let shift = usize::BITS - size.trailing_zeros();
    move |i| i.reverse_bits() >> shift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let transpose = transpose(3, 2);
        assert_eq!(transpose(0), 0);
        assert_eq!(transpose(1), 3);
        assert_eq!(transpose(2), 1);
        assert_eq!(transpose(3), 4);
        assert_eq!(transpose(4), 2);
        assert_eq!(transpose(5), 5);
    }

    #[test]
    fn test_bit_reverse() {
        let bit_reverse = bit_reverse(16);
        assert_eq!(bit_reverse(0), 0);
        assert_eq!(bit_reverse(1), 8);
        assert_eq!(bit_reverse(2), 4);
        assert_eq!(bit_reverse(3), 12);
        assert_eq!(bit_reverse(4), 2);
        assert_eq!(bit_reverse(5), 10);
        assert_eq!(bit_reverse(6), 6);
        assert_eq!(bit_reverse(7), 14);
        assert_eq!(bit_reverse(8), 1);
        assert_eq!(bit_reverse(9), 9);
        assert_eq!(bit_reverse(10), 5);
        assert_eq!(bit_reverse(11), 13);
        assert_eq!(bit_reverse(12), 3);
        assert_eq!(bit_reverse(13), 11);
        assert_eq!(bit_reverse(14), 7);
        assert_eq!(bit_reverse(15), 15);
    }
}
