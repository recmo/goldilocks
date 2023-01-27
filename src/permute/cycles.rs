use std::{fmt, sync::atomic::{AtomicPtr, Ordering}};
use rayon::prelude::*;
use super::Permute;

pub struct Cycles {
    size: usize,
    cycles: Vec<Vec<usize>>,
    parallel: bool,
}

impl Cycles {
    pub fn from_fn(size: usize, permutation: impl Fn(usize) -> usize) -> Self {
        let mut cycles = Vec::new();
        let mut done = vec![false; size];
        let mut cycle = Vec::new();

        for i in 0..size {
            // Skip if already part of a cycle.
            if done[i] {
                continue;
            }

            // Follow the cycle.
            let mut j = i;
            loop {
                assert!(j < size, "Not a permutation: out of bounds.");
                assert!(!done[j], "Not a permutation: not bijective.");
                cycle.push(j);
                done[j] = true;
                j = permutation(j);
                if j == i {
                    break;
                }
            }

            // Add the cycle to the list sorted by length.
            let length = cycle.len();
            if cycles.len() < length {
                cycles.resize(length, Vec::new());
            }
            cycles[cycle.len() - 1].extend(cycle.drain(..).rev());
        }
        Self {
            size,
            cycles,
            parallel: size > 1 << 21,
        }
    }

    fn apply<T: Copy + Send + Sync>(&self, values: &mut [T]) {
        if self.parallel {
            self.par_apply(values);
        } else {
            self.ser_apply(values);
        }
    }

    fn ser_apply<T: Copy + Send + Sync>(&self, values: &mut [T]) {
        assert_eq!(values.len(), self.size);

        // Skip cycles of length 1.
        let mut cycles = self.cycles.iter().skip(1);

        // Swaps (special case for cycles of length 2)
        for swap in cycles.next().iter().flat_map(|cycle| cycle.chunks_exact(2)) {
            unsafe {
                // SAFETY: We know cycles has length 2 and contains only valid
                // non-overlapping indices into `values`.
                let a = *swap.get_unchecked(0);
                let b = *swap.get_unchecked(1);
                // TODO: Replace with `swap_unchecked` when stabilized.
                let tmp = *values.get_unchecked(a);
                *values.get_unchecked_mut(a) = *values.get_unchecked(b);
                *values.get_unchecked_mut(b) = tmp;
            }
        }

        // Cycles
        for (i, cycles) in cycles.enumerate() {
            let length = i + 2;
            for cycle in cycles.chunks_exact(length) {

                unsafe {
                    // SAFETY: We know cycles has length `length` >= 3 and contains
                    // only valid non-overlapping indices into `values`.
                    let mut dst = *cycle.get_unchecked(0);
                    let temp = *values.get_unchecked(dst);
                    for &src in cycle.iter().skip(1) {
                        *values.get_unchecked_mut(dst) = *values.get_unchecked(src);
                        dst = src;
                    }
                    *values.get_unchecked_mut(dst) = temp;
                }
            }
        }
    }

    fn par_apply<T: Copy + Send + Sync>(&self, values: &mut [T]) {
        assert_eq!(values.len(), self.size);

        // SAFETY: All cycles are disjoint, so we can safely move the values in
        // parallel. For this we need to use a pointer we can share between threads.
        let values = AtomicPtr::new(values.as_mut_ptr());

        self.cycles.par_iter().enumerate().skip(1).for_each(|(i, cycles)| {
            let length = i + 1;
            if length == 2 {
                // Swaps (special case for cycles of length 2)
                cycles.par_chunks_exact(2).for_each(|swap| {
                    let values = values.load(Ordering::Relaxed);
                    unsafe {
                        // SAFETY: We know cycles has length 2 and contains only valid
                        // non-overlapping indices into `values`.
                        let a = *swap.get_unchecked(0);
                        let b = *swap.get_unchecked(1);
                        // TODO: Replace with `swap_unchecked` when stabilized.
                        let tmp = *values.add(a);
                        *values.add(a) = *values.add(b);
                        *values.add(b) = tmp;
                    }
                });
            } else {
                cycles.par_chunks_exact(length).for_each(|cycle| {
                    let values = values.load(Ordering::Relaxed);
                    unsafe {
                        // SAFETY: We know cycles has length `length` >= 3 and contains
                        // only valid non-overlapping indices into `values`.
                        let mut dst = *cycle.get_unchecked(0);
                        let temp = *values.add(dst);
                        for &src in cycle.iter().skip(1) {
                            *values.add(dst) = *values.add(src);
                            dst = src;
                        }
                        *values.add(dst) = temp;
                    }
                });
            }
        });
    }
}

impl<T: Copy + Send + Sync> Permute<T> for Cycles {
    fn len(&self) -> usize {
        self.cycles.iter().map(|cycle| cycle.len()).sum()
    }

    fn permute(&self, values: &mut [T]) {
        self.apply(values)
    }
}

impl fmt::Debug for Cycles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, cycle) in self.cycles.iter().enumerate() {
            let length = i + 1;
            for (j, cycle) in cycle.chunks_exact(length).enumerate() {
                write!(f, "(")?;
                for (k, &x) in cycle.iter().enumerate() {
                    if k > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", x)?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::permute;

    use super::*;

    #[test]
    fn test_transpose() {
        let (a, b) = (3, 4);
        let size = a * b;
        let permutation = permute::permutation::transpose(a, b);
        let cycles = Cycles::from_fn(size, permutation);
        let mut values = (0..size).collect::<Vec<_>>();
        let mut expected = values.clone();
        cycles.permute(&mut values);
        permute::transpose_copy(&mut expected, (a, b));
        assert_eq!(values, expected);
    }
}
