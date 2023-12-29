use std::{
    collections::HashSet,
    fmt::Display,
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use itertools::Itertools;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, AttributeDependencies};

use super::{Condition, Depth};

/// Represents a value of a [`Bonus`]
///
/// [`Bonus`]: crate::bonus::Bonus
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Value {
    /// Just a simple [`f32`] value.
    Const(Decimal),
    /// Copy the total value of some [`Attribute`].
    Attribute(Attribute),
    /// Returns the minimum value of the two
    Min(Box<Value>, Box<Value>),
    /// Returns the maximum value of the two
    Max(Box<Value>, Box<Value>),
    /// Floors the inner value to a whole number
    Floor(Box<Value>),
    /// Adds the first value to the second value
    Add(Box<Value>, Box<Value>),
    /// Subtracts the second value from the first value
    Sub(Box<Value>, Box<Value>),
    /// Multiplies the two values
    Mul(Box<Value>, Box<Value>),
    /// Divides the first value by the second value
    Div(Box<Value>, Box<Value>),
    /// Returns the remainder from dividing the first value by the second value
    Rem(Box<Value>, Box<Value>),
    /// Returns `if_true` if `condition` is true, otherwise returns `if_false`
    If {
        /// The condition needed to be checked
        condition: Box<Condition>,
        /// The value to return if the condition returns true
        if_true: Box<Value>,
        /// The value to return if the condition returns false
        if_false: Box<Value>,
    },
}

/// Operations to simplify writing formulas
impl Value {
    /// Calculates the mean of some list or set
    ///
    /// # Panics
    /// Panics if there are 0 items in the iterator
    pub fn mean(iter: impl IntoIterator<Item = Self>) -> Self {
        let (sum, count) = iter
            .into_iter()
            .map(|a| (a, 1))
            .tree_fold1(|(v1, c1), (v2, c2)| (v1 + v2, c1 + c2))
            .unwrap();

        sum / Self::from(count)
    }

    /// Returns the minimum of all of the values
    ///
    /// # Panics
    /// Panics if an iterator with no items is passed in
    pub fn iter_min(iter: impl IntoIterator<Item = Self>) -> Self {
        iter.into_iter()
            .tree_fold1(Self::min)
            .expect("Expected at least one value")
    }

    /// Returns the maximum of all of the values
    ///
    /// # Panics
    /// Panics if an iterator with no items is passed in
    pub fn iter_max(iter: impl IntoIterator<Item = Self>) -> Self {
        iter.into_iter()
            .tree_fold1(Self::max)
            .expect("Expected at least one value")
    }

    /// Returns the sum of the values within the iterator
    ///
    /// # Panics
    /// Panics if an iterator with no items is passed in
    pub fn iter_sum(iter: impl IntoIterator<Item = Self>) -> Self {
        iter.into_iter()
            .tree_fold1(Self::add)
            .expect("Expected at least one value")
    }

    /// Returns the sum of the values within the iterator
    ///
    /// # Panics
    /// Panics if an iterator with no items is passed in
    pub fn iter_product(iter: impl IntoIterator<Item = Self>) -> Self {
        iter.into_iter()
            .tree_fold1(Self::mul)
            .expect("Expected at least one value")
    }

    /// Floors the value
    #[must_use]
    pub fn floor(self) -> Self {
        Self::Floor(self.into())
    }

    /// Cielings the value
    #[must_use]
    pub fn ciel(self) -> Self {
        (self + Self::from(1)).floor()
    }

    /// Finds the reciprocol of the value.
    ///
    /// The reciprocol of value `x` is equivilant to `1 / x`
    #[must_use]
    pub fn recip(self) -> Self {
        Self::Const(1.into()) / self
    }

    /// Returns the maximum of this or another value
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self::Max(self.into(), other.into())
    }

    /// Returns the minimum of this or another value
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self::Min(self.into(), other.into())
    }
}

impl Depth for Value {
    fn get_depth(&self) -> usize {
        match self {
            Self::Const(_) | Self::Attribute(_) => 1,
            Self::Min(a, b)
            | Self::Max(a, b)
            | Self::Add(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::Rem(a, b) => a.get_depth().max(b.get_depth()),
            Self::Floor(a) => a.get_depth(),
            Self::If {
                condition,
                if_true,
                if_false,
            } => condition
                .get_depth()
                .max(if_true.get_depth())
                .max(if_false.get_depth()),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(a, b) => write!(f, "({a} + {b})"),
            Self::Sub(a, b) => write!(f, "({a} - {b})"),
            Self::Mul(a, b) => write!(f, "({a} * {b})"),
            Self::Div(a, b) => write!(f, "({a} / {b})"),
            Self::Rem(a, b) => write!(f, "({a} % {b})"),
            Self::Const(value) => value.fmt(f),
            Self::Attribute(attr) => attr.fmt(f),
            Self::Min(a, b) => write!(f, "Min({a}, {b})"),
            Self::Max(a, b) => write!(f, "Max({a}, {b})"),
            Self::Floor(val) => write!(f, "Floor({val})"),
            Self::If {
                condition,
                if_true,
                if_false,
            } => {
                write!(f, "If ({condition}) then {if_true} else {if_false}")
            }
        }
    }
}

impl AttributeDependencies for Value {
    fn has_attr_dependency(&self, attribute: Attribute) -> bool {
        match self {
            Self::Add(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::Rem(a, b)
            | Self::Max(a, b)
            | Self::Min(a, b) => {
                a.has_attr_dependency(attribute) || b.has_attr_dependency(attribute)
            }
            Self::Const(_) => false,
            Self::Attribute(attr) => attribute.eq(attr),
            Self::Floor(val) => val.has_attr_dependency(attribute),
            Self::If {
                condition,
                if_true,
                if_false,
            } => {
                condition.has_attr_dependency(attribute)
                    || if_true.has_attr_dependency(attribute)
                    || if_false.has_attr_dependency(attribute)
            }
        }
    }

    fn include_attr_dependency(&self, set: &mut HashSet<Attribute>) {
        match self {
            Self::Const(_) => {}
            Self::Add(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::Rem(a, b)
            | Self::Min(a, b)
            | Self::Max(a, b) => {
                a.include_attr_dependency(set);
                b.include_attr_dependency(set);
            }
            Self::Attribute(attr) => {
                set.insert(*attr);
            }
            Self::Floor(val) => val.include_attr_dependency(set),
            Self::If {
                condition,
                if_true,
                if_false,
            } => {
                condition.include_attr_dependency(set);
                if_true.include_attr_dependency(set);
                if_false.include_attr_dependency(set);
            }
        }
    }
}

macro_rules! from_primative {
    ($($type:ty), +) => {
        $(
            impl From<$type> for Value {
                fn from(value: $type) -> Self {
                    Self::Const(Decimal::from(value))
                }
            }
        )+
    };
}

from_primative!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, u128, i128);

impl From<Decimal> for Value {
    fn from(value: Decimal) -> Self {
        Self::Const(value)
    }
}

impl From<Attribute> for Value {
    fn from(value: Attribute) -> Self {
        Self::Attribute(value)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Add(self.into(), rhs.into())
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sub(self.into(), rhs.into())
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Mul(self.into(), rhs.into())
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Div(self.into(), rhs.into())
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Rem(self.into(), rhs.into())
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Mul(self.into(), Self::Const(Decimal::NEGATIVE_ONE).into())
    }
}

impl Sum for Value {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::iter_sum(iter)
    }
}

impl Product for Value {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::iter_product(iter)
    }
}
