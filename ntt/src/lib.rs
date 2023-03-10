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
// #![feature(slice_swap_unchecked)]

pub mod convolve;
pub mod divisors;
pub mod field;
pub mod ntt;
pub mod permute;
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
        ntt::bench::group(criterion);
        // ntt2::bench::group(criterion);
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
        let mut total_duration = 0.0;
        let mut count = 0;
        let mut measurements = Vec::new();
        while total_duration < 1.0 {
            // Do a single run first
            let start = Instant::now();
            let out = black_box(f());
            let end = Instant::now();
            drop(out);
            let run_duration = end.duration_since(start).as_secs_f64();

            if run_duration > 0.001 {
                total_duration += run_duration;
                measurements.push(run_duration);
                count += 1;
            } else {
                // Very fast function, run it a lot to get a good measurement
                let start = Instant::now();
                for _ in 0..1000 {
                    black_box(f());
                }
                let end = Instant::now();
                let run_duration = end.duration_since(start).as_secs_f64();
                total_duration += run_duration;
                measurements.push(run_duration / 1000.0);
                count += 1000;
            }
        }
        let _average = total_duration / f64::from(count);
        let middle = measurements.len() / 2;
        let (left, median, right) =
            measurements.select_nth_unstable_by(middle, |a, b| a.partial_cmp(b).unwrap());
        *median
    }
}
