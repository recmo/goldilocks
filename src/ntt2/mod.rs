pub mod cooley_tukey;
mod matrix;
pub mod naive;
pub mod small;
mod vector;

use self::{matrix::Matrix, vector::Vector};
use crate::Field;
use core::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub trait Ntt: Sync + Send {
    fn len(&self) -> usize;

    // OPT: We could also have the stride stored in the structure, and just pass raw
    // pointers down the tree.
    fn ntt(&self, values: Vector);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    /// Test `f` by comparing to naive implementation
    #[track_caller]
    pub fn test_ntt_fn(f: impl Fn(Vector), size: usize) {
        let mut rng = StdRng::seed_from_u64(Field::MODULUS);
        let mut values = (0..size).map(|_| rng.gen()).collect::<Vec<_>>();
        let mut expected = values.clone();
        crate::ntt::naive::ntt(&mut expected);
        f(Vector::from(values.as_mut_slice()));
        for (&value, expected) in values.iter().zip(expected) {
            assert_eq!(value, expected);
        }
    }

    /// Test `Ntt` object by comparing to naive implementation.
    #[track_caller]
    pub fn test_ntt(ntt: impl Ntt) {
        let size = ntt.len();
        let mut rng = StdRng::seed_from_u64(Field::MODULUS);
        let mut values = (0..size).map(|_| rng.gen()).collect::<Vec<_>>();
        let mut expected = values.clone();
        crate::ntt::naive::ntt(&mut expected);
        ntt.ntt(Vector::from(values.as_mut_slice()));
        for (&value, expected) in values.iter().zip(expected) {
            assert_eq!(value, expected);
        }
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::{BenchmarkId, Criterion, Throughput};
    use rand::{thread_rng, Rng};

    pub fn group(criterion: &mut Criterion) {
        small::bench::group(criterion);
        // rader::bench::group(criterion);
    }

    pub fn bench_ntt(criterion: &mut Criterion, name: &str, ntt: impl Fn(Vector), size: usize) {
        let mut rng = thread_rng();
        let mut values = (0..size).map(|_| rng.gen()).collect::<Vec<_>>();
        let mut vec = Vector::from(values.as_mut_slice());

        let mut group = criterion.benchmark_group("ntt2");
        group.throughput(Throughput::Elements(size as u64));
        group.bench_function(BenchmarkId::new(name, size), move |bencher| {
            bencher.iter(|| ntt(vec.reborrow()));
        });
    }
}
