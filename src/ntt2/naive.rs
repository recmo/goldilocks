use super::{Ntt, Vector};
use crate::Field;
use core::iter;

pub struct Naive {
    twiddles: Vec<Field>,
}

impl Naive {
    pub fn new(len: usize) -> Self {
        if len == 0 {
            return Self {
                twiddles: Vec::new(),
            };
        }
        let root = Field::root(len as u64)
            .expect("Vector length does not divide multiplicative group order.");
        let twiddles = iter::successors(Some(Field::from(1)), |&x| Some(x * root))
            .take(len)
            .collect();
        Self { twiddles }
    }
}

impl Ntt for Naive {
    fn len(&self) -> usize {
        self.twiddles.len()
    }

    fn ntt(&self, mut values: Vector) {
        assert_eq!(values.len(), self.twiddles.len());
        if self.twiddles.is_empty() {
            return;
        }

        let copy = values.to_vec();
        values[0] = copy.iter().copied().sum();
        for (i, x) in values.enumerate().skip(1) {
            let twiddles = self.twiddles.iter().cycle().step_by(i);
            *x = copy
                .iter()
                .zip(twiddles)
                .map(|(&y, &omega)| y * omega)
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[track_caller]
    pub fn test_ntt_nd_agree(size: usize) {
        let algo = Naive::new(size);

        let mut rng = StdRng::seed_from_u64(Field::MODULUS);
        let mut values = (0..size).map(|_| rng.gen()).collect::<Vec<_>>();
        let mut expected = values.clone();
        algo.ntt(Vector::from(values.as_mut_slice()));
        crate::ntt::naive::ntt(&mut expected);
        assert_eq!(values, expected);
    }

    #[test]
    fn test_nttnd() {
        test_ntt_nd_agree(0);
        test_ntt_nd_agree(1);
        test_ntt_nd_agree(2);
        test_ntt_nd_agree(3);
        test_ntt_nd_agree(16);
    }
}
