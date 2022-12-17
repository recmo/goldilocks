use crate::algo;
use core::{iter, ops};

/// An element in the Goldilocks field.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field(u64);

impl Field {
    const MODULUS: u64 = 0xffff_ffff_0000_0001;
    const MAX: Field = Self(Self::MODULUS - 1);
}

impl From<u64> for Field {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self(algo::reduce_1(value))
    }
}

impl From<Field> for u64 {
    #[inline(always)]
    fn from(value: Field) -> Self {
        value.0
    }
}

impl ops::Add<Field> for Field {
    type Output = Field;

    #[inline(always)]
    fn add(self, rhs: Field) -> Self {
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
    type Output = Field;

    #[inline(always)]
    fn neg(self) -> Self {
        Self(Self::MODULUS - self.0)
    }
}

impl ops::Sub for Field {
    type Output = Field;

    #[inline(always)]
    fn sub(self, rhs: Field) -> Self {
        Self(algo::add(self.0, Self::MODULUS - rhs.0))
    }
}

impl ops::SubAssign for Field {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = algo::add(self.0, Self::MODULUS - rhs.0);
    }
}

impl ops::Mul<Field> for Field {
    type Output = Field;

    #[inline(always)]
    fn mul(self, rhs: Field) -> Self {
        Self(algo::mul(self.0, rhs.0))
    }
}

impl ops::MulAssign<Field> for Field {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Field) {
        self.0 = algo::mul(self.0, rhs.0);
    }
}

impl iter::Product for Field {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.fold(1, |a, n| algo::mul(a, n.0)))
    }
}
