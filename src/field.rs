use crate::algo;
use core::{iter, ops};
use std::fmt;

/// An element in the Goldilocks field.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Field(u64);

impl Field {
    pub const MODULUS: u64 = 0xffff_ffff_0000_0001;
    pub const BITS: u32 = 64;
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(Self::MODULUS - 1);
}

impl Field {
    /// Inverse of the field element, or zero.
    #[inline(always)]
    #[must_use]
    pub fn inv(self) -> Self {
        Self(algo::inv(self.0))
    }

    #[inline(always)]
    #[must_use]
    pub fn pow(self, exp: u64) -> Self {
        Self(algo::pow(self.0, exp))
    }

    #[inline(always)]
    #[must_use]
    pub fn omega(order: u64) -> Option<Self> {
        if (Self::MODULUS - 1) % order != 0 {
            return None;
        }
        Some(Self(algo::omega(order)))
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <u64 as fmt::Debug>::fmt(&self.0, f)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <u64 as fmt::Display>::fmt(&self.0, f)
    }
}

impl From<u64> for Field {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self(algo::reduce_64(value))
    }
}

impl From<u128> for Field {
    #[inline(always)]
    fn from(value: u128) -> Self {
        Self(algo::reduce_128(value))
    }
}

impl From<Field> for u64 {
    #[inline(always)]
    fn from(value: Field) -> Self {
        value.0
    }
}

impl ops::Add for Field {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(algo::add(self.0, rhs.0))
    }
}

impl ops::AddAssign for Field {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = algo::add(self.0, rhs.0);
    }
}

impl iter::Sum for Field {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.fold(0, |a, n| algo::add(a, n.0)))
    }
}

impl ops::Neg for Field {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Self(Self::MODULUS - self.0)
    }
}

impl ops::Sub for Field {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(algo::add(self.0, Self::MODULUS - rhs.0))
    }
}

impl ops::SubAssign for Field {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = algo::add(self.0, Self::MODULUS - rhs.0);
    }
}

impl ops::Mul for Field {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self(algo::mul(self.0, rhs.0))
    }
}

impl ops::MulAssign for Field {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = algo::mul(self.0, rhs.0);
    }
}

impl iter::Product for Field {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.fold(1, |a, n| algo::mul(a, n.0)))
    }
}

impl ops::Div for Field {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self(algo::mul(self.0, algo::inv(rhs.0)))
    }
}

impl ops::DivAssign for Field {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.0 = algo::mul(self.0, algo::inv(rhs.0));
    }
}

impl ops::Shl<u64> for Field {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u64) -> Self {
        Self(algo::shift(self.0, rhs))
    }
}

impl ops::ShlAssign<u64> for Field {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u64) {
        self.0 = algo::shift(self.0, rhs);
    }
}

impl ops::Shr<u64> for Field {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u64) -> Self {
        Self(algo::shift(self.0, 191 * (rhs % 192)))
    }
}

impl ops::ShrAssign<u64> for Field {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u64) {
        self.0 = algo::shift(self.0, 191 * (rhs % 192));
    }
}
