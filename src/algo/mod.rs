mod aarch64;
mod generic;

pub use self::{
    aarch64::mul,
    generic::{add, inv, pow, reduce_128, reduce_64, root, shift},
};

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
