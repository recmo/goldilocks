mod cooley_tukey;
mod good_thomas;
pub mod gpu;
pub mod naive;
mod rader;
pub mod small;

use self::{cooley_tukey::CooleyTukey, good_thomas::GoodThomas, rader::Rader};
use crate::{
    divisors::{is_divisor, split},
    utils::gcd,
    Field,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

// Minimum work size (in elements) for parallelization.
// const MIN_WORK_SIZE: usize = 1 << 14;
const MIN_WORK_SIZE: usize = 134217728 / 8;

static CACHE: Mutex<BTreeMap<usize, Arc<dyn Ntt>>> = Mutex::new(BTreeMap::new());

pub trait Ntt: Sync + Send {
    fn len(&self) -> usize;

    // OPT: We could also have the stride stored in the structure, and just pass raw
    // pointers down the tree.
    fn ntt(&self, values: &mut [Field]);
}

impl Ntt for Arc<dyn Ntt> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn ntt(&self, values: &mut [Field]) {
        self.as_ref().ntt(values)
    }
}

pub fn clear_cache() {
    let mut lock = CACHE.lock().unwrap();
    lock.clear();
}

pub fn strategy(size: usize) -> Arc<dyn Ntt> {
    let lock = CACHE.lock().unwrap();
    if let Some(ntt) = lock.get(&size) {
        return ntt.clone();
    }
    drop(lock);

    assert!(
        is_divisor(size),
        "{size} is not a supported NTT size (does not divide multiplicative order)"
    );

    let ntt = if size <= 128 {
        small::ntt(size).unwrap()
    } else if size == 17 || size == 257 || size == 65537 {
        Arc::new(Rader::new(size)) as Arc<dyn Ntt>
    } else {
        let a = split(size);
        let b = size / a;
        if gcd(a, b) == 1 {
            Arc::new(GoodThomas::new(a, b)) as Arc<dyn Ntt>
        } else {
            Arc::new(CooleyTukey::new(a, b)) as Arc<dyn Ntt>
        }
    };
    assert_eq!(ntt.len(), size);

    let mut lock = CACHE.lock().unwrap();
    lock.insert(size, ntt.clone());
    ntt
}

pub struct NttFn<F: Fn(&mut [Field]) + Send + Sync>(usize, F);

impl<F: Fn(&mut [Field]) + Send + Sync> NttFn<F> {
    pub fn new(size: usize, f: F) -> Self {
        Self(size, f)
    }
}

impl<F: Fn(&mut [Field]) + Send + Sync> Ntt for NttFn<F> {
    fn len(&self) -> usize {
        self.0
    }

    fn ntt(&self, values: &mut [Field]) {
        (self.1)(values);
    }
}

pub fn ntt(values: &mut [Field]) {
    let strat = strategy(values.len());
    strat.ntt(values);
    return;
}

pub fn intt(values: &mut [Field]) {
    if values.len() <= 1 {
        return;
    }

    // Apply 1/N scaling factor
    let scale = Field::from(values.len() as u64).inv();
    for x in values.iter_mut() {
        *x *= scale;
    }

    // Permute j = N - i mod N
    values[1..].reverse();

    // Apply forward NTT
    ntt(values);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    /// Test `f` by comparing to naive implementation
    #[track_caller]
    pub fn test_ntt(ntt: impl Ntt) {
        let mut rng = StdRng::seed_from_u64(Field::MODULUS);
        let mut values = (0..ntt.len()).map(|_| rng.gen()).collect::<Vec<_>>();
        let mut expected = values.clone();
        naive::ntt(&mut expected);
        ntt.ntt(&mut values);
        assert_eq!(values, expected);
        for (&value, expected) in values.iter().zip(expected) {
            assert_eq!(value, expected);
        }
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use crate::utils::div_round_up;

    use super::*;
    use criterion::{BenchmarkId, Criterion, Throughput};
    use rand::{thread_rng, Rng};

    pub fn group(criterion: &mut Criterion) {
        rader::bench::group(criterion);
        small::bench::group(criterion);
        good_thomas::bench::group(criterion);

        #[cfg(feature = "gpu")]
        gpu::bench::group(criterion);
    }

    pub fn bench_ntt(criterion: &mut Criterion, name: &str, ntt: impl Ntt) {
        // Use a multiple of the NTT size larger than the minimum work size.
        let size = ntt.len();
        let count = div_round_up(MIN_WORK_SIZE, size);
        let total = count * size;

        let mut rng = thread_rng();
        let mut values = (0..total).map(|_| rng.gen()).collect::<Vec<_>>();
        let mut group = criterion.benchmark_group("ntt");
        group.throughput(Throughput::Elements(values.len() as u64));
        group.bench_function(BenchmarkId::new(name, size), move |bencher| {
            bencher.iter(|| ntt.ntt(&mut values));
        });
    }
}
