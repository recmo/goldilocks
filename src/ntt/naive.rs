use crate::Field;

pub fn ntt(v: &mut [Field]) {
    if v.is_empty() {
        return;
    }
    let root = Field::root(v.len() as u64)
        .expect("Vector length does not divide multiplicative group order.");

    let copy = v.to_vec();
    let mut omega_i = Field::from(1);
    for x in v.iter_mut() {
        let mut sum = Field::from(0);
        let mut omega_ij = Field::from(1);
        for y in &copy {
            sum += *y * omega_ij;
            omega_ij *= omega_i;
        }
        *x = sum;
        omega_i *= root;
    }
}

pub fn intt(v: &mut [Field]) {
    if v.is_empty() {
        return;
    }
    // Scale in the inverse
    let scale = Field::from(v.len() as u64).inv();
    for x in v.iter_mut() {
        *x *= scale;
    }

    let root = Field::root(v.len() as u64)
        .expect("Vector length does not divide multiplicative group order.")
        .pow(v.len() as u64 - 1);
    let copy = v.to_vec();
    let mut omega_i = Field::from(1);
    for x in v.iter_mut() {
        let mut sum = Field::from(0);
        let mut omega_ij = Field::from(1);
        for y in &copy {
            sum += *y * omega_ij;
            omega_ij *= omega_i;
        }
        *x = sum;
        omega_i *= root;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[track_caller]
    pub fn test_ntt_intt_inverse(size: usize) {
        let mut rng = StdRng::seed_from_u64(Field::MODULUS);
        let mut values = (0..size).map(|_| rng.gen()).collect::<Vec<_>>();
        let expected = values.clone();
        ntt(&mut values);
        intt(&mut values);
        assert_eq!(values, expected);
    }

    #[test]
    pub fn test_reverse_inverse() {
        let size = 15;
        let mut rng = StdRng::seed_from_u64(Field::MODULUS);
        let mut values = (0..size).map(|_| rng.gen()).collect::<Vec<_>>();
        let mut expected = values.clone();
        values[1..].reverse();
        ntt(&mut values);
        for x in &mut values {
            *x *= Field::from(size as u64).inv();
        }
        intt(&mut expected);
        assert_eq!(values, expected);
    }

    #[test]
    fn test_ntt_intt_inverse_small() {
        test_ntt_intt_inverse(0);
        test_ntt_intt_inverse(1);
        test_ntt_intt_inverse(2);
        test_ntt_intt_inverse(3);
        test_ntt_intt_inverse(4);
        test_ntt_intt_inverse(5);
        test_ntt_intt_inverse(6);
        test_ntt_intt_inverse(8);
    }

    #[test]
    fn test_ntt_intt_inverse_257() {
        test_ntt_intt_inverse(257);
    }

    #[test]
    fn test_ntt_intt_inverse_1024() {
        test_ntt_intt_inverse(1024);
    }
}
