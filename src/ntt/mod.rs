pub mod cooley_tukey;
pub mod good_thomas;
pub mod naive;
pub mod rader;
pub mod small;

use crate::Field;

pub fn ntt(values: &mut [Field]) {
    cooley_tukey::ntt(values);
}

pub fn intt(values: &mut [Field]) {
    naive::ntt(values);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    /// Test `f` by comparing to naive implementation
    #[track_caller]
    pub fn test_ntt_fn(f: impl Fn(&mut [Field]), size: usize) {
        let mut rng = StdRng::seed_from_u64(Field::MODULUS);
        let mut values = (0..size).map(|_| rng.gen()).collect::<Vec<_>>();
        let mut expected = values.clone();
        naive::ntt(&mut expected);
        f(&mut values);
        assert_eq!(values, expected);
    }
}
