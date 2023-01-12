use crate::Field;
use crate::ntt::{ntt, intt};

/// In-place circular convolution.
/// 
/// Expects `b` to be in NTT form.
/// 
/// # Panics
/// 
/// Panics if `a` and `b` have different lengths.
pub fn circular(a: &mut [Field], b: &[Field]) {
    assert_eq!(a.len(), b.len());
    dbg!(&a);
    dbg!(&b);
    ntt(a);
    dbg!(&a);
    for (a, b) in a.iter_mut().zip(b) {
        *a *= *b;
    }
    dbg!(&a);
    intt(a);
    dbg!(&a);
}

pub fn circular_naive(a: &[Field], b: &[Field]) -> Vec<Field> {
    assert_eq!(a.len(), b.len());
    let n = a.len();
    let mut result = vec![Field::default(); n];
    for i in 0..n {
        for j in 0..n {
            result[i] += a[j] * b[(i + n - j) % n];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_naive() {
        let a = [Field::from(1), Field::from(2), Field::from(3)];
        let b = [Field::from(4), Field::from(5), Field::from(6)];
        let c = circular_naive(&a, &b);
        assert_eq!(c, [Field::from(31), Field::from(31), Field::from(28)]);
    }

    #[test]
    fn test_circular_ntt() {
        let a = [Field::from(1), Field::from(2), Field::from(3)];
        let mut b = [Field::from(4), Field::from(5), Field::from(6)];
        ntt(&mut b);
        let mut c = a.clone();
        circular(&mut c, &b);
        assert_eq!(c, [Field::from(31), Field::from(31), Field::from(28)]);
    }
}
