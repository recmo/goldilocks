//! Generated using `cargo run --bin codegen`
#![allow(
    unused_parens,
    clippy::similar_names,
    clippy::unreadable_literal,
    clippy::too_many_lines
)]
use super::{Ntt, Vector};
use crate::Field;

pub struct Small {
    len: usize,
    ntt: fn(Vector),
}

impl Small {
    fn new(len: usize) -> Self {
        let ntt = match len {
            2 => ntt_2,
            _ => panic!("Unsupported size"),
        };
        Self { len, ntt }
    }
}

impl Ntt for Small {
    fn len(&self) -> usize {
        self.len
    }

    fn ntt(&self, values: Vector) {
        self.ntt(values);
    }
}

/// Apply a small NTT to `values`, or return `false` if the size is not
/// supported.
pub fn ntt(values: Vector) -> bool {
    match values.len() {
        ..=1 => return true,
        2 => ntt_2(values),
        _ => return false,
    }
    true
}

/// Size 2 NTT.
///
/// # Panics
///
/// Panics if `values.len() != 2`.
pub fn ntt_2(mut values: Vector) {
    assert_eq!(values.len(), 2);
    let a0 = values[0];
    let a1 = values[1];
    let (a0, a1) = (a0 + a1, a0 - a1);
    values[0] = a0;
    values[1] = a1;
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_ntt_fn, *};

    #[test]
    fn test_small_ntt() {
        for size in [0, 1, 2] {
            test_ntt_fn(|values| assert!(ntt(values)), size);
        }
    }

    #[test]
    fn test_ntt_2() {
        test_ntt_fn(ntt_2, 2);
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::{super::bench::bench_ntt, *};
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        bench_ntt(criterion, "small", ntt_2, 2);
    }
}
