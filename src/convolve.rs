use crate::Field;
use crate::ntt::{ntt, intt};

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

pub fn circular_ntt(a: &[Field], b: &[Field]) -> Vec<Field> {
    assert_eq!(a.len(), b.len());
    let n = a.len();

    let mut a = a.to_vec();
    let mut b = b.to_vec();
    ntt(&mut a);
    ntt(&mut b);
    let mut result = a.iter().zip(b).map(|(&a, b)| a * b).collect::<Vec<_>>();
    intt(&mut result);
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
        let b = [Field::from(4), Field::from(5), Field::from(6)];
        let c = circular_ntt(&a, &b);
        assert_eq!(c, [Field::from(31), Field::from(31), Field::from(28)]);
    }
}
