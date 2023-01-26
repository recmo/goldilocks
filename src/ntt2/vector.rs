use super::Matrix;
use crate::Field;
use core::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

/// Similar to `&mut [Field]` but with a stride.
///
/// This is nearly identical to an `ArrayViewMut1` from the `ndarray` crate, but
/// with a copy-free `as_matrix` method. (Something `ndarray` does not support.
/// See [#390]).
///
/// [#390]: https://github.com/rust-ndarray/ndarray/issues/1172
pub struct Vector<'a> {
    pub(crate) life:   PhantomData<&'a mut Field>,
    pub(crate) data:   *mut Field,
    pub(crate) len:    usize,
    pub(crate) stride: isize,
}

impl<'a> Vector<'a> {
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<Field> {
        let mut vec = Vec::with_capacity(self.len);
        for i in 0..self.len {
            // OPT: avoid bounds check.
            vec.push(self[i]);
        }
        vec
    }

    #[must_use]
    pub fn reversed(self) -> Self {
        let last = self.len - 1;
        let data = unsafe {
            // SAFETY: This points to the last element of the vector.
            // With reversed stride, this is the first element.
            self.data.offset((last as isize) * self.stride)
        };
        Self {
            life: self.life,
            data,
            len: self.len,
            stride: -self.stride,
        }
    }

    #[must_use]
    pub fn as_matrix(self, (a, b): (usize, usize)) -> Matrix<'a> {
        assert_eq!(self.len, a * b);
        Matrix {
            life:   PhantomData,
            data:   self.data,
            len:    [a, b],
            stride: [self.stride * (b as isize), self.stride],
        }
    }

    #[must_use]
    pub fn reborrow<'b>(&'b mut self) -> Vector<'b> {
        Vector {
            life:   PhantomData,
            data:   self.data,
            len:    self.len,
            stride: self.stride,
        }
    }
}

impl<'a> From<&'a mut [Field]> for Vector<'a> {
    #[inline(always)]
    #[must_use]
    fn from(slice: &'a mut [Field]) -> Self {
        Self {
            life:   PhantomData,
            data:   slice.as_mut_ptr(),
            len:    slice.len(),
            stride: 1,
        }
    }
}

impl Index<usize> for Vector<'_> {
    type Output = Field;

    #[inline(always)]
    #[must_use]
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.len, "Index out of range");
        unsafe {
            // SAFETY: This is a valid index.
            &*self.data.offset((i as isize) * self.stride)
        }
    }
}

impl IndexMut<usize> for Vector<'_> {
    #[inline(always)]
    #[must_use]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        assert!(i < self.len, "Index out of range");
        unsafe {
            // SAFETY: This is a valid index.
            &mut *self.data.offset((i as isize) * self.stride)
        }
    }
}

impl<'a> Iterator for Vector<'a> {
    type Item = &'a mut Field;

    #[inline(always)]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            let value = unsafe {
                // SAFETY: `len` is non-zero, so this is a valid pointer.
                &mut *self.data
            };
            self.len -= 1;
            if self.len > 0 {
                self.data = unsafe {
                    // SAFETY: `len` is non-zero, so this is a valid pointer.
                    self.data.offset(self.stride)
                };
            }
            Some(value)
        }
    }

    #[inline(always)]
    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for Vector<'a> {
    #[inline(always)]
    #[must_use]
    fn len(&self) -> usize {
        self.len
    }
}
