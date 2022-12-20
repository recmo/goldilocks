#![cfg(feature = "rand")]

use crate::Field;
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, UniformSampler},
        Distribution, Standard, Uniform,
    },
    Rng,
};

impl Distribution<Field> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Field {
        Uniform::new(0, Field::MODULUS).sample(rng).into()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UniformField(Uniform<u64>);

impl UniformSampler for UniformField {
    type X = Field;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self(Uniform::<u64>::new::<u64, u64>(
            (*low.borrow()).into(),
            (*high.borrow()).into(),
        ))
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self(Uniform::<u64>::new_inclusive::<u64, u64>(
            (*low.borrow()).into(),
            (*high.borrow()).into(),
        ))
    }
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        self.0.sample(rng).into()
    }
}

impl SampleUniform for Field {
    type Sampler = UniformField;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn test_uniform() {
        let mut rng = StdRng::seed_from_u64(42);
        let dist = Uniform::new(Field::from(0), Field::from(100));
        for _ in 0..100 {
            let x: Field = dist.sample(&mut rng);
            assert!(x >= Field::MIN);
            assert!(x <= Field::MAX);
        }
    }
}
