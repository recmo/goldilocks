mod generic;

pub use self::generic::{add, inv, mul, pow, reduce_128, reduce_64, root, root_384, shift, sub};

// OPT: Use a super optimizer to find optimal sequences for add, mul, neg, sub,
// and shift.

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        generic::bench::group(criterion);
    }
}
