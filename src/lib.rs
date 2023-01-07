#![doc = include_str!("../Readme.md")]
#![doc(issue_tracker_base_url = "https://github.com/recmo/goldilocks-ntt/")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::inline_always,
    clippy::doc_markdown // Unfortunately many false positives on Latex.
)]
#![cfg_attr(
    any(test, feature = "criterion"),
    allow(clippy::wildcard_imports, clippy::cognitive_complexity)
)]
// See <https://stackoverflow.com/questions/61417452/how-to-get-a-feature-requirement-tag-in-the-documentation-generated-by-cargo-do>
#![cfg_attr(has_doc_cfg, feature(doc_cfg))]
#![feature(slice_swap_unchecked)]

mod algo;
mod field;
pub mod ntt;
mod rand;
mod transpose;
mod utils;

pub use field::Field;

#[cfg(test)]
mod tests {
    use super::*;
}

#[cfg(feature = "criterion")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use ::rand::{thread_rng, Fill, Rng};
    use criterion::Criterion;
    use rayon::prelude::*;

    pub fn group(criterion: &mut Criterion) {
        algo::bench::group(criterion);
        ntt::bench::group(criterion);
    }

    pub(crate) fn rand_vec<T>(size: usize) -> Vec<T>
    where
        T: Clone + Copy + Default + Send + Sync,
        [T]: Fill,
    {
        let mut result = vec![T::default(); size];
        result
            .par_chunks_mut(1000)
            .for_each_init(|| thread_rng(), |rng, chunk| rng.fill(chunk));
        result
    }
}
