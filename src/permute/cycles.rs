use super::Permute;
use rayon::prelude::*;
use std::{
    fmt,
    sync::{atomic::{AtomicPtr, Ordering}, Arc}, collections::BTreeMap,
};

pub struct Cycles<I: Index> {
    size:     usize,
    cycles:   Vec<(usize, Vec<I>)>,
    parallel: bool,
}

pub trait Index: 'static + Copy + Send + Sync {
    fn max() -> usize;
    fn from(n: usize) -> Self;
    fn to(self) -> usize;
}

pub fn from_fn<T: 'static + Copy + Send + Sync>(size: usize, permutation: impl Fn(usize) -> usize) -> Arc<dyn Permute<T>> {
    if size <= u16::MAX as usize {
        Arc::new(Cycles::<u16>::from_fn(size, permutation)) as Arc<dyn Permute<T>>
    } else if size <= u32::MAX as usize {
        Arc::new(Cycles::<u32>::from_fn(size, permutation)) as Arc<dyn Permute<T>>
    } else {
        Arc::new(Cycles::<usize>::from_fn(size, permutation)) as Arc<dyn Permute<T>>
    }
}

impl<I: Index> Cycles<I> {
    pub fn from_fn(size: usize, permutation: impl Fn(usize) -> usize) -> Self {
        assert!(size <= I::max(), "Invalid index type for size.");
        let mut cycles = BTreeMap::<usize, Vec<I>>::new();
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
                cycle.push(I::from(j));
                done[j] = true;
                j = permutation(j);
                if j == i {
                    break;
                }
            }

            // Add non-trivial cycles to the collection.
            if cycle.len() > 1 {
                cycles.entry(cycle.len()).or_default().extend(cycle.drain(..).rev());
            } else {
                cycle.clear();
            }
        }
        let cycles = cycles.into_iter().collect();
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

        // Cycles
        for (length, cycles) in self.cycles.iter() {
            debug_assert!(*length >= 2);
            debug_assert_eq!(cycles.len() % *length, 0);
            for cycle in cycles.chunks_exact(*length) {
                unsafe {
                    // SAFETY: We know cycles has length `length` >= 2 and contains
                    // only valid non-overlapping indices into `values`.
                    let mut dst = cycle.get_unchecked(0).to();
                    let temp = *values.get_unchecked(dst);
                    for src in cycle.iter().skip(1).map(|i| i.to()) {
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

        self.cycles
            .par_iter()
            .for_each(|(length, cycles)| {
                debug_assert!(*length >= 2);
                debug_assert_eq!(cycles.len() % *length, 0);
                cycles.par_chunks_exact(*length).for_each(|cycle| {
                    let values = values.load(Ordering::Relaxed);
                    unsafe {
                        // SAFETY: We know cycles has length `length` >= 2 and contains
                        // only valid non-overlapping indices into `values`.
                        let mut dst = cycle.get_unchecked(0).to();
                        let temp = *values.add(dst);
                        for src in cycle.iter().skip(1).map(|i| i.to()) {
                            *values.add(dst) = *values.add(src);
                            dst = src;
                        }
                        *values.add(dst) = temp;
                    }
                });
            });
    }
}

impl<I: Index, T: 'static + Copy + Send + Sync> Permute<T> for Cycles<I> {
    fn len(&self) -> usize {
        self.size
    }

    fn permute(&self, values: &mut [T]) {
        self.apply(values)
    }
}

impl<I: Index> fmt::Debug for Cycles<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (length, cycles) in self.cycles.iter() {
            for (j, cycle) in cycles.chunks_exact(*length).enumerate() {
                write!(f, "(")?;
                for (k, &x) in cycle.iter().enumerate() {
                    if k > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", x.to())?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

impl Index for u16 {
    fn max() -> usize {
        u16::MAX as usize
    }

    fn from(n: usize) -> Self {
        n as u16
    }

    fn to(self) -> usize {
        self as usize
    }
}

impl Index for u32 {
    fn max() -> usize {
        u32::MAX as usize
    }

    fn from(n: usize) -> Self {
        n as u32
    }

    fn to(self) -> usize {
        self as usize
    }
}

impl Index for usize {
    fn max() -> usize {
        usize::MAX
    }

    fn from(n: usize) -> Self {
        n
    }

    fn to(self) -> usize {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::permute;

    use super::*;

    fn test_transpose(a: usize, b: usize) {
        let size = a * b;
        let permutation = permute::permutation::transpose(a, b);
        let cycles = Cycles::<usize>::from_fn(size, permutation);
        let mut values = (0..size).collect::<Vec<_>>();
        let mut expected = values.clone();
        cycles.permute(&mut values);
        permute::transpose_copy(&mut expected, (a, b));
        assert_eq!(values, expected);
    }

    #[test]
    fn test_transpose_small() {
        for a in 0..=4 {
            for b in 0..=4 {
                test_transpose(a, b);
            }
        }
    }

    #[test]
    fn test_transpose_16x32() {
        test_transpose(16, 32);
    }

    #[test]
    fn test_transpose_120x128() {
        test_transpose(120, 128);
    }
}
