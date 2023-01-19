pub mod algo;
mod rand;

use core::{iter, ops};
use std::{fmt, ops::Neg};

/// An element in the Goldilocks field.
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
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
    #[must_use]
    pub const fn new(value: u64) -> Self {
        assert!(value < Self::MODULUS);
        let value = (((value as u128) * (algo::MONT_R1 as u128)) % (Self::MODULUS as u128)) as u64;
        Self(value)
    }

    /// Inverse of the field element, or zero.
    #[inline(always)]
    #[must_use]
    pub fn inv(self) -> Self {
        Self(algo::mont_mul(algo::inv(self.0), algo::MONT_R3))
    }

    #[inline(always)]
    #[must_use]
    pub fn pow(self, exp: u64) -> Self {
        Self(algo::mont_pow(self.0, exp))
    }

    #[inline(always)]
    #[must_use]
    pub fn root(order: u64) -> Option<Self> {
        if algo::ORDER % order != 0 {
            return None;
        }
        let exponent = algo::ORDER / order;
        Some(Self(algo::mont_pow(algo::GENERATOR_R, exponent)))
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = u64::from(self);
        // let n = self.0;
        <u64 as fmt::Debug>::fmt(&n, f)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = u64::from(self);
        <u64 as fmt::Display>::fmt(&n, f)
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = u64::from(self);
        let b = u64::from(other);
        a.partial_cmp(&b)
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = u64::from(self);
        let b = u64::from(other);
        a.cmp(&b)
    }
}

impl From<i32> for Field {
    #[inline(always)]
    fn from(value: i32) -> Self {
        #[allow(clippy::cast_sign_loss)]
        if value >= 0 {
            debug_assert!((value as u64) < Self::MODULUS);
            Self(algo::mont_mul(value as u64, algo::MONT_R2))
        } else {
            Self(algo::mont_mul(-value as u64, algo::MONT_R2)).neg()
        }
    }
}

impl From<u64> for Field {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self(algo::mont_mul(algo::reduce_64(value), algo::MONT_R2))
    }
}

impl From<u128> for Field {
    #[inline(always)]
    fn from(value: u128) -> Self {
        Self(algo::mont_mul(algo::reduce_128(value), algo::MONT_R2))
    }
}

impl From<Field> for u64 {
    #[inline(always)]
    fn from(value: Field) -> Self {
        algo::mont_mul(value.0, 1)
    }
}

impl From<&Field> for u64 {
    #[inline(always)]
    fn from(value: &Field) -> Self {
        algo::mont_mul(value.0, 1)
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
        Self(algo::sub(0, self.0))
    }
}

impl ops::Sub for Field {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(algo::sub(self.0, rhs.0))
    }
}

impl ops::SubAssign for Field {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = algo::sub(self.0, rhs.0);
    }
}

impl ops::Mul for Field {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Self(algo::mont_mul(self.0, rhs.0))
    }
}

impl ops::MulAssign for Field {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = algo::mont_mul(self.0, rhs.0);
    }
}

impl iter::Product for Field {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.fold(1, |a, n| algo::mont_mul(a, n.0)))
    }
}

impl ops::Div for Field {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Self(algo::mont_mul(self.0, rhs.inv().0))
    }
}

impl ops::DivAssign for Field {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.0 = algo::mont_mul(self.0, rhs.inv().0);
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{
        arbitrary::Arbitrary,
        num::u64,
        prop_assume, proptest,
        strategy::{BoxedStrategy, Strategy},
    };

    impl Arbitrary for Field {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> BoxedStrategy<Self> {
            (0..algo::MODULUS).prop_map(Self).boxed()
        }
    }

    #[test]
    fn test_construct() {
        proptest!(|(n: u64)| {
            prop_assume!(n < algo::MODULUS);
            let a = Field::new(n);
            let b = Field::from(n);
            assert_eq!(a, b);
            assert_eq!(u64::from(a), n);
        });
    }

    #[test]
    fn test_add_identity() {
        proptest!(|(a: Field)| {
            let b = a + Field::new(0);
            assert_eq!(a, b);
        });
    }

    #[test]
    fn test_add_commutative() {
        proptest!(|(a: Field, b: Field)| {
            let ab = a + b;
            let ba = b + a;
            assert_eq!(ab, ba);
        });
    }

    #[test]
    fn test_add_associative() {
        proptest!(|(a: Field, b: Field, c: Field)| {
            let a_bc = a + (b + c);
            let ab_c = (a + b) + c;
            assert_eq!(a_bc, ab_c);
        });
    }

    #[test]
    fn test_sub() {
        proptest!(|(a: Field, b: Field)| {
            let ab = a + b;
            let ab_b = ab - b;
            assert_eq!(a, ab_b);
        });
    }

    #[test]
    fn test_mul_identity() {
        proptest!(|(a: Field)| {
            let b = a * Field::new(1);
            assert_eq!(a, b);
        });
    }

    #[test]
    fn test_mul_commutative() {
        proptest!(|(a: Field, b: Field)| {
            let ab = a * b;
            let ba = b * a;
            assert_eq!(ab, ba);
        });
    }

    #[test]
    fn test_mul_associative() {
        proptest!(|(a: Field, b: Field, c: Field)| {
            let a_bc = a * (b * c);
            let ab_c = (a * b) * c;
            assert_eq!(a_bc, ab_c);
        });
    }

    #[test]
    fn test_distributive() {
        proptest!(|(a: Field, b: Field, c: Field)| {
            let a_bc = a * (b + c);
            let ab_ac = (a * b) + (a * c);
            assert_eq!(a_bc, ab_ac);
        });
    }

    #[test]
    fn test_root() {
        let omega_5 = Field::root(5).unwrap();
        assert_eq!(omega_5.pow(5), Field::new(1));
        let omega_4 = Field::root(4).unwrap();
        assert_eq!(omega_4.pow(4), Field::new(1));
        assert_eq!(omega_4.pow(2), -Field::new(1));
    }
}

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench {
    use super::*;
    use criterion::Criterion;

    pub fn group(criterion: &mut Criterion) {
        algo::bench::group(criterion);
    }
}
