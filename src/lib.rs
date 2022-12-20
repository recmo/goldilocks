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
// Nightly only feature flag to enable the `unlikely` compiler hint.
#![cfg_attr(has_core_intrinsics, feature(core_intrinsics))]

mod algo;
mod field;
mod rand;
// mod ntt;

pub use field::Field;

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        algo::bench::group(criterion);
    }
}
