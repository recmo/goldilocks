#![doc = include_str!("../Readme.md")]
#![doc(issue_tracker_base_url = "https://github.com/recmo/goldilocks-ntt/")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::inline_always,
    clippy::doc_markdown // Unfortunately many false positives on Latex.
)]
#![cfg_attr(
    any(test, feature = "bench"),
    allow(clippy::wildcard_imports, clippy::cognitive_complexity)
)]
// See <https://stackoverflow.com/questions/61417452/how-to-get-a-feature-requirement-tag-in-the-documentation-generated-by-cargo-do>
#![cfg_attr(has_doc_cfg, feature(doc_cfg))]
#![feature(slice_swap_unchecked)]

pub mod divisors;
mod field;
pub mod ntt;
pub mod ntt_old;
pub mod permute;
mod rand;
pub mod utils;

pub use field::Field;

#[cfg(test)]
mod tests {}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use ::rand::{
        distributions::{Distribution, Standard},
        thread_rng, Rng,
    };
    use criterion::Criterion;
    use rayon::prelude::*;
    use std::{hint::black_box, time::Instant};

    pub fn group(criterion: &mut Criterion) {
        field::bench::group(criterion);
        ntt_old::bench::group(criterion);
    }

    pub fn rand_vec<T>(size: usize) -> Vec<T>
    where
        T: Clone + Copy + Default + Send + Sync,
        Standard: Distribution<T>,
    {
        let mut result = vec![T::default(); size];
        result
            .par_chunks_mut(1000)
            .for_each_init(thread_rng, |rng, chunk| {
                chunk.iter_mut().for_each(|v| *v = rng.gen());
            });
        result
    }

    pub fn time<O>(mut f: impl FnMut() -> O) -> f64 {
        let mut duration = 0.0;
        let mut count = 0;
        while duration < 5.0 {
            let start = Instant::now();
            let out = black_box(f());
            let end = Instant::now();
            drop(out);
            duration += end.duration_since(start).as_secs_f64();
            count += 1;
        }
        duration / f64::from(count)
    }
}
