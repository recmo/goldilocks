use std::sync::Once;

/// Multiplicative group order.
const ORDER: u64 = 0xffff_ffff_0000_0000;

/// Multiplicative group order without large factors (>= 17).
const SMOOTH_ORDER: u64 = 64424509440;

const TWOS: u32 = 32;

/// Factors of the multiplicative group order other than two.
/// All factors occur once.
const FACTORS: [u64; 5] = [3, 5, 17, 257, 65537];

static mut DIVISORS: Vec<u64> = Vec::new();

static DIVISORS_INIT: Once = Once::new();

/// Check if `n` is a divisor of the multiplicative group order.
#[must_use]
pub fn is_divisor(n: usize) -> bool {
    n > 0 && ORDER % (n as u64) == 0
}

pub fn is_smooth(n: usize) -> bool {
    n > 0 && SMOOTH_ORDER % (n as u64) == 0
}

/// Returns sorted list of all divisors of the multiplicative group order.
/// Starting from 1 and ending in the order.
pub fn divisors() -> &'static [u64] {
    unsafe {
        // Safety: DIVISORS is initialized exactly once, and only
        // read from thereafter.
        DIVISORS_INIT.call_once(|| {
            let mut d = (0..=TWOS).map(|i| 1 << i).collect::<Vec<_>>();
            for &f in FACTORS.iter() {
                let n = d.iter().map(|&i| i * f).collect::<Vec<_>>();
                d.extend(n);
            }
            d.sort();
            DIVISORS = d;
        });
        DIVISORS.as_slice()
    }
}

/// Returns the smallest divisor larger than or equal to `n`.
/// Returns zero on zero input.
pub fn next(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let divisors = divisors();
    let i = divisors.binary_search(&(n as u64)).unwrap_or_else(|i| i);
    divisors[i] as usize
}

/// Goldilocks divisor split.
///
/// Given composite `n` that divides the multiplicative group order,
/// return `(a, b)` such that `a <= b`, `a != 1`, `n = a * b` and `a` and `b`
/// are as close as possible.
#[must_use]
pub fn split(n: usize) -> usize {
    if !is_divisor(n) {
        panic!("{} does not divide multiplicative group order.", n);
    }
    let divisors = divisors();

    // Compute integer square root
    let isqrt = (n as f64).sqrt() as usize;
    let isqrt = (isqrt + n / isqrt) / 2;

    let i = divisors
        .binary_search(&(isqrt as u64))
        .unwrap_or_else(|i| i);
    for &d in divisors[..=i].iter().rev() {
        let d = d as usize;
        if n % d == 0 && n / d >= d {
            return d;
        }
    }
    // `1` is a divisor of `n`, so this is unreachable.
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factors() {
        assert_eq!(FACTORS.iter().product::<u64>() << TWOS, ORDER);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors().len(), 1056);
        assert_eq!(divisors()[0], 1);
        assert_eq!(divisors()[1055], ORDER);
        for &d in divisors().iter() {
            assert!(is_divisor(d as usize));
        }
    }
    #[test]
    fn test_next() {
        assert_eq!(next(0), 0);
        assert_eq!(next(1000), 1020);
        assert_eq!(next(1024), 1024);
        assert_eq!(next(1234), 1280);
    }

    #[test]
    fn test_split() {
        for &n in divisors().iter() {
            let n = n as usize;
            let a = split(n);
            let b = n / a;
            // eprintln!("{n} = {a} * {b}");
            assert!(a <= b);
            assert_eq!(a * b, n);
        }
    }
}
