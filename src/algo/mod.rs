mod aarch64;
mod generic;
mod utils;

pub use self::generic::{add, inv, mul, pow, reduce_128, reduce_64, root, shift};

// OPT: Use a super optimizer to find optimal sequences for add, mul, neg, sub,
// and shift.

#[cfg(feature = "criterion")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        generic::bench::group(criterion);
        aarch64::bench::group(criterion);
    }
}
