use super::{Ntt, Vector};
use crate::{divisors::split, permute::transpose, Field};
use rayon::prelude::*;

pub struct CooleyTukey<A: Ntt, B: Ntt> {
    inner_a:  A,
    inner_b:  B,
    len:      usize,
    split:    (usize, usize),
    root:     Field,
    twiddles: Vec<Field>,
    parallel: bool,
    ordered:  bool,
}

impl<A: Ntt, B: Ntt> CooleyTukey<A, B> {
    pub fn new(inner_a: A, inner_b: B) -> Self {
        let (a, b) = (inner_a.len(), inner_b.len());
        let len = a * b;
        let root = Field::root(len as u64)
            .expect("NTT length does not divide multiplicative group order.");
        let twiddles = (0..inner_a.len() * inner_b.len())
            .map(|i| root.pow(i as u64))
            .collect();
        Self {
            inner_a,
            inner_b,
            len,
            split: (a, b),
            root,
            twiddles,
            parallel: false,
            ordered: true,
        }
    }
}

impl<A: Ntt, B: Ntt> Ntt for CooleyTukey<A, B> {
    fn len(&self) -> usize {
        self.len
    }

    fn ntt(&self, values: Vector) {
        assert_eq!(values.len(), self.len);
        let (a, b) = self.split;

        // Re-intepret as a a x b matrix.
        let mut matrix = values.as_matrix(self.split);

        // Operate over columns applying twiddles
        for i in 0..b {
            let mut column = matrix.column(i);
            self.inner_a.ntt(column.reborrow());

            for j in 1..b {
                column[j] *= self.root.pow((i * j) as u64);
            }
        }

        // Operate over rows
        for i in 0..a {
            self.inner_b.ntt(matrix.row(i));
        }

        // Transpose
        if self.ordered {
            matrix.transpose();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{naive, tests::test_ntt_fn},
        *,
    };

    // #[test]

    // fn test_4() {
    //     test_ntt_fn(
    //         |values| {
    //             recurse_ndarray(values.into(), naive::ntt, (4, 4));
    //         },
    //         16,
    //     );
    // }
}
