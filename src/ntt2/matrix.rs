use super::Vector;
use crate::Field;
use core::{
    marker::PhantomData,
    mem,
    ops::{Index, IndexMut},
};

/// Similar to `ArrayViewMut2` from the `ndarray` crate.
///
/// This is nearly identical to an `ArrayViewMut2` from the `ndarray` crate, but
/// also supports negative strides.
pub struct Matrix<'a> {
    pub(crate) life:   PhantomData<&'a mut Field>,
    pub(crate) data:   *mut Field,
    pub(crate) len:    [usize; 2],
    pub(crate) stride: [isize; 2],
}

impl<'a> Matrix<'a> {
    pub fn reborrow<'b>(&'b mut self) -> Matrix<'b> {
        Matrix {
            life:   PhantomData,
            data:   self.data,
            len:    self.len,
            stride: self.stride,
        }
    }

    pub fn column<'b>(&'b mut self, index: usize) -> Vector<'b> {
        assert!(index < self.len[1], "Column index out of range");
        Vector {
            life:   PhantomData,
            data:   unsafe {
                // SAFETY: This is a valid index.
                self.data.offset((index as isize) * self.stride[1])
            },
            len:    self.len[0],
            stride: self.stride[0],
        }
    }

    pub fn row<'b>(&'b mut self, index: usize) -> Vector<'b> {
        assert!(index < self.len[0], "Row index out of range");
        Vector {
            life:   PhantomData,
            data:   unsafe {
                // SAFETY: This is a valid index.
                self.data.offset((index as isize) * self.stride[0])
            },
            len:    self.len[1],
            stride: self.stride[1],
        }
    }

    /// Transposes the matrix in place.
    ///
    /// **Note**. This transposes the underlying data, not just the view.
    pub fn transpose(&mut self) {
        assert_eq!(self.len[0], self.len[1], "Matrix must be square");
        for i in 0..self.len[0] {
            for j in i + 1..self.len[1] {
                let a = self[[i, j]];
                let b = self[[j, i]];
                self[[i, j]] = b;
                self[[j, i]] = a;
            }
        }
    }
}

impl Index<[usize; 2]> for Matrix<'_> {
    type Output = Field;

    #[inline(always)]
    #[must_use]
    fn index(&self, [i, j]: [usize; 2]) -> &Self::Output {
        assert!(i < self.len[0], "Row index out of range");
        assert!(j < self.len[1], "Column index out of range");
        unsafe {
            // SAFETY: This is a valid index.
            &*self
                .data
                .offset((i as isize) * self.stride[0] + (j as isize) * self.stride[1])
        }
    }
}

impl IndexMut<[usize; 2]> for Matrix<'_> {
    #[inline(always)]
    fn index_mut(&mut self, [i, j]: [usize; 2]) -> &mut Self::Output {
        assert!(i < self.len[0], "Row index out of range");
        assert!(j < self.len[1], "Column index out of range");
        unsafe {
            // SAFETY: This is a valid index.
            &mut *self
                .data
                .offset((i as isize) * self.stride[0] + (j as isize) * self.stride[1])
        }
    }
}
