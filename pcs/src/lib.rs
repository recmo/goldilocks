pub mod k12;
pub mod pow;

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        k12::bench::group(criterion);
        pow::bench::group(criterion);
    }
}
