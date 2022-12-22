mod bit_reverse;
mod prefetch;
mod radix_sqrt;
mod recursive;
mod small;
mod transpose;

use crate::Field;
#[cfg(feature = "rayon")]
use rayon::{current_num_threads, prelude::*};
#[cfg(feature = "rayon")]
use std::cmp::max;
use tracing::{instrument, trace};

// Re-exports
// TODO: Only re-export for bench
pub use self::transpose::transpose_square_stretch;
pub use bit_reverse::{permute, permute_index};
#[cfg(feature = "memadvise")]
pub use memadvise::Advice;
#[cfg(feature = "memadvise")]
pub use prefetch::MemoryAdvise;
pub use prefetch::{Prefetch, PrefetchIndex};
pub use radix_sqrt::radix_sqrt;
pub use recursive::fft_vec_recursive;

/// Relevant papers:
/// * D. H. Bailey (1990). FFTs in external or hierarchical memory. <https://www.davidhbailey.com/dhbpapers/fftq.pdf>
/// * W. M. Gentleman & G. Sande (1966). Fast Fourier Transforms: for fun and
///   profit. <https://doi.org/10.1145/1464291.1464352> <http://cis.rit.edu/class/simg716/FFT_Fun_Profit.pdf>
/// * M. Frigo, C.E. Leiserson, H. Prokop & S. Ramachandran (1999).
///   Cache-oblivious algorithms. <http://supertech.csail.mit.edu/papers/FrigoLePr99.pdf>
/// * S. Johnson, M. Frigo (2005). The Design and Implementation of FFTW3. <http://www.fftw.org/fftw-paper-ieee.pdf>
/// * S. Johnson, M. Frigo (2012). Implementing FFTs in Practice. <https://cnx.org/contents/ulXtQbN7@15/Implementing-FFTs-in-Practice>
///   <https://www.csd.uwo.ca/~moreno/CS433-CS9624/Resources/Implementing_FFTs_in_Practice.pdf>
///
/// <https://doi.org/10.1007/978-981-13-9965-7_6>
/// <https://eprint.iacr.org/2016/504.pdf>

// OPT: Implement parallel strategies: https://inf.ethz.ch/personal/markusp/teaching/263-2300-ETH-spring12/slides/class19.pdf

// TODO: Implement "A modified split-radix FFT with fewer arithmetic operations"
// See http://www.fftw.org/newsplit.pdf

// TODO: Winograd FFT? https://pdfs.semanticscholar.org/cdfc/fed48f6f7e26a2986df8890f3f67087336d5.pdf

/// <https://www.csd.uwo.ca/~moreno/CS433-CS9624/Resources/Implementing_FFTs_in_Practice.pdf>

// https://ocw.mit.edu/courses/electrical-engineering-and-computer-science/6-973-communication-system-design-spring-2006/lecture-notes/lecture_8.pdf

pub trait Fft {
    fn fft(&mut self);
    fn ifft(&mut self);
    fn fft_cofactor(&mut self, cofactor: Field);
    fn ifft_cofactor(&mut self, cofactor: Field);
    fn fft_root(&mut self, root: Field);
    fn clone_shifted(&mut self, source: &[Field], cofactor: Field);
}

/// Blanket implementation of [`Fft`] for all slices of a [`FieldLike`]
impl Fft for [Field] {
    #[instrument(level = "trace", skip_all, fields(size = self.len()))]
    fn fft(&mut self) {
        let root = Field::root(self.len() as u64).expect("No root of unity for input length");
        self.fft_root(root);
    }

    fn ifft(&mut self) {
        let inverse_root = Field::root(self.len() as u64)
            .expect("No root of unity for input length")
            .pow((self.len() - 1) as u64);
        let inverse_length = Field::from(self.len() as u64).inv();
        self.fft_root(inverse_root);
        trace!("BEGIN Inverse shift");
        for e in self.iter_mut() {
            *e *= inverse_length;
        }
        trace!("END Inverse shift");
    }

    #[cfg(not(feature = "rayon"))]
    fn clone_shifted(&mut self, source: &[Field], cofactor: Field) {
        trace!("BEGIN Clone shifted");
        let mut c = Field::from(1);
        for (destination, source) in self.iter_mut().zip(source.iter()) {
            *destination = *source * c;
            c *= cofactor;
        }
        trace!("END Clone shifted");
    }

    #[cfg(feature = "rayon")]
    fn clone_shifted(&mut self, source: &[Field], cofactor: Field) {
        // TODO: Write benchmark and tune chunk size
        const MIN_CHUNK_SIZE: usize = 1024;
        trace!("BEGIN Clone shifted");
        let chunk_size = max(MIN_CHUNK_SIZE, source.len() / current_num_threads());
        let chunks = self
            .par_chunks_mut(chunk_size)
            .zip(source.par_chunks(chunk_size));
        chunks.enumerate().for_each(|(i, (destination, source))| {
            let mut c = cofactor.pow((i * chunk_size) as u64);
            for (destination, source) in destination.iter_mut().zip(source.iter()) {
                *destination = *source * c;
                c *= cofactor;
            }
        });
        trace!("END Clone shifted");
    }

    fn fft_cofactor(&mut self, cofactor: Field) {
        // TODO: This patterns happens often, abstract?
        trace!("BEGIN Cofactor shift");
        let mut c = Field::from(1);
        for element in self.iter_mut() {
            *element *= c;
            c *= cofactor;
        }
        trace!("END Cofactor shift");
        self.fft();
    }

    fn ifft_cofactor(&mut self, cofactor: Field) {
        self.ifft();
        let cofactor = cofactor.inv();
        trace!("BEGIN Cofactor shift");
        let mut c = Field::from(1);
        for element in self.iter_mut() {
            *element *= c;
            c *= cofactor;
        }
        trace!("END Cofactor shift");
    }

    fn fft_root(&mut self, root: Field) {
        const RADIX_SQRT_TRESHOLD: usize = 1 << 10;
        if self.len() >= RADIX_SQRT_TRESHOLD {
            radix_sqrt(self, root);
        } else {
            let twiddles = get_twiddles(root, self.len());
            trace!("Recursive FFT of size {}", self.len());
            fft_vec_recursive(self, &twiddles, 0, 1, 1);
        }
    }
}

// TODO: Memoize
#[instrument(skip(root))]
pub fn get_twiddles(root: Field, size: usize) -> Vec<Field> {
    debug_assert!(size.is_power_of_two());
    debug_assert_eq!(root.pow(size as u64), Field::from(1));
    trace!("Computing {} twiddles", size / 2);
    let mut twiddles = (0..size / 2)
        .map(|i| root.pow(i as u64))
        .collect::<Vec<_>>();
    permute(&mut twiddles);
    twiddles
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    pub(super) fn arb_elem() -> impl Strategy<Value = Field> {
        any::<u64>().prop_map(Field::from)
    }

    // Generate a power-of-two size
    pub(super) fn arb_vec_size(size: usize) -> impl Strategy<Value = Vec<Field>> {
        prop::collection::vec(arb_elem(), size)
    }

    // Generate a power-of-two size
    pub(super) fn arb_vec() -> impl Strategy<Value = Vec<Field>> {
        (0_usize..=9).prop_flat_map(|size| arb_vec_size(1_usize << size))
    }

    // O(n^2) reference implementation evaluating
    //     x_i' = Sum_j x_j * omega_n^(ij)
    // directly using Horner's method.
    // New lint in nightly
    #[allow(clippy::unknown_clippy_lints)]
    // False positive
    #[allow(clippy::same_item_push)]
    fn reference_fft(x: &[Field], inverse: bool) -> Vec<Field> {
        let mut root = Field::root(x.len() as u64).unwrap();
        if inverse {
            root = root.inv();
        }
        let mut result = Vec::with_capacity(x.len());
        let mut root_i = Field::from(1);
        for _ in 0..x.len() {
            let mut sum = Field::from(0);
            let mut root_ij = Field::from(1);
            for xj in x {
                sum += *xj * root_ij;
                root_ij *= root_i;
            }
            result.push(sum);
            root_i *= root;
        }
        if inverse {
            let inverse_length = Field::from(x.len() as u64).inv();
            for x in &mut result {
                *x *= inverse_length;
            }
        }
        result
    }

    #[allow(dead_code)]
    pub(super) fn ref_fft_inplace(values: &mut [Field]) {
        let result = reference_fft(values, false);
        values.clone_from_slice(&result);
    }

    pub(super) fn ref_fft_permuted(values: &mut [Field]) {
        let result = reference_fft(values, false);
        values.clone_from_slice(&result);
        permute(values);
    }

    proptest! {
        #[test]
        fn fft_ref_inv(orig in arb_vec()) {
            let f = reference_fft(&orig, false);
            let f2 = reference_fft(&f, true);
            prop_assert_eq!(f2, orig);
        }

        #[test]
        fn fft_ref(values in arb_vec()) {
            let mut expected = values.clone();
            ref_fft_permuted(&mut expected);
            let mut result = values;
            result.fft();
            prop_assert_eq!(result, expected);
        }
    }

    proptest!(
        #[test]
        fn ifft_is_inverse(v: Vec<Field>) {
            prop_assume!(!v.is_empty());

            let truncated = &v[0..(1 + v.len()).next_power_of_two() / 2].to_vec();
            let mut result = truncated.clone();
            result.fft();
            permute(&mut result);
            result.ifft();
            permute(&mut result);

            prop_assert_eq!(&result, truncated);
        }
    );
}

#[cfg(feature = "criterion")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        transpose::bench::group(criterion);
    }
}
