use std::{
    collections::HashSet,
    fmt::Display,
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use itertools::Itertools;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, AttributeDependencies, ToAttribute};

use super::{Condition, Depth};

/// Represents a value of a [`Bonus`]
///
/// [`Bonus`]: crate::bonus::Bonus
#[derive(Clone, PartialEq, Debug, Hash, Eq, Serialize, Deserialize)]
pub enum Value {
    /// Hard codes a specific [`Decimal`] value.
    Const(Decimal),
    /// Copy the total value of some [`Attribute`].
    Attribute(Attribute),
    /// Returns the minimum value of the two
    Min(Box<Value>, Box<Value>),
    /// Returns the maximum value of the two
    Max(Box<Value>, Box<Value>),
    /// Floors the inner value to a whole number
    Floor(Box<Value>),
    /// Ceils the inner value to a whole number
    Ceil(Box<Value>),
    /// Makes the value positive if it is negative
    Abs(Box<Value>),
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
    /// Shortcut for [`Condition::If`]
    ///
    /// [`Condition::If`]: Self#variant.If
    #[must_use]
    pub fn condition(
        condition: impl Into<Condition>,
        if_true: impl Into<Self>,
        if_false: impl Into<Self>,
    ) -> Self {
        Self::If {
            condition: Box::new(condition.into()),
            if_true: Box::new(if_true.into()),
            if_false: Box::new(if_false.into()),
        }
    }

    /// Allows the simplification of using impl
    #[must_use]
    pub fn attribute(attribute: impl Into<Attribute>) -> Self {
        Self::Attribute(attribute.into())
    }

    /// Calculates the mean of some list or set
    ///
    /// # Panics
    /// Panics if there are 0 items in the iterator
    pub fn mean(iter: impl IntoIterator<Item = Self>) -> Self {
        let (sum, count) = iter
            .into_iter()
            .map(|a| (a, 1))
            .tree_fold1(|(v1, c1), (v2, c2)| (v1 + v2, c1 + c2))
            .expect("Expected at least one value");

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

    /// Shortcut for [`Condition::Floor`]
    ///
    /// [`Condition::Floor`]: Self#variant.Floor
    #[must_use]
    pub fn floor(self) -> Self {
        Self::Floor(self.into())
    }

    /// Shortcut for [`Condition::Ceil`]
    ///
    /// [`Condition::Ceil`]: Self#variant.Ceil
    #[must_use]
    pub fn ceil(self) -> Self {
        Self::Ceil(self.into())
    }

    /// Shortcut for [`Condition::Abs`]
    ///
    /// [`Condition::Abs`]: Self#variant.Abs
    #[must_use]
    pub fn abs(self) -> Self {
        Self::Abs(self.into())
    }

    /// Returns the reciprocol
    ///
    /// The reciprocol of value `x` is equivilant to `1 / x`
    #[must_use]
    pub fn recip(self) -> Self {
        Self::Const(Decimal::ONE) / self
    }

    /// Shortcut for [`Condition::Max`]
    ///
    /// [`Condition::Max`]: Self#variant.Min
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self::Max(self.into(), other.into())
    }

    /// Shortcut for [`Condition::Min`]
    ///
    /// [`Condition::Min`]: Self#variant.Min
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
            Self::Abs(a) | Self::Floor(a) | Self::Ceil(a) => a.get_depth(),
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
            Self::Ceil(val) => write!(f, "Ceil({val})"),
            Self::Abs(val) => write!(f, "|{val}|"),
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
            Self::Abs(val) | Self::Ceil(val) | Self::Floor(val) => {
                val.has_attr_dependency(attribute)
            }
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
            Self::Abs(val) | Self::Ceil(val) | Self::Floor(val) => val.include_attr_dependency(set),
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

macro_rules! try_from_primative {
    ($($type:ty), +) => {
        $(
            impl TryFrom<$type> for Value {
                type Error = rust_decimal::Error;
                fn try_from(value: $type) -> Result<Self, Self::Error> {
                    Ok(Self::Const(Decimal::try_from(value)?))
                }
            }
        )+
    }
}

try_from_primative!(f32, f64);

impl From<Decimal> for Value {
    fn from(value: Decimal) -> Self {
        Self::Const(value)
    }
}

impl<T> From<T> for Value
where
    T: ToAttribute,
{
    fn from(value: T) -> Self {
        value.to_attribute().into()
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

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//
//     mod shortcuts {
//         use super::*;
//
//         #[test]
//         fn and() {
//
//         }
//     }
//
//     mod ops {
//         use super::*;
//
//         #[test]
//         fn add() {
//             let value = Value::from(1) + Value::from(2);
//             let expected = Value::from(1).add(${1:rhs})$0
//         }
//
//         #[test]
//         fn sub() {
//             let value = Value::from(1) - Value::from(1);
//             assert!(matches!(value, Value::Sub(_, _)));
//         }
//
//         #[test]
//         fn mul() {
//             let value = Value::from(1) * Value::from(1);
//             assert!(matches!(value, Value::Mul(_, _)));
//         }
//
//         #[test]
//         fn div() {
//             let value = Value::from(1) / Value::from(1);
//             assert!(matches!(value, Value::Div(_, _)));
//         }
//
//         #[test]
//         fn rem() {
//             let value = Value::from(1) % Value::from(1);
//             assert!(matches!(value, Value::Rem(_, _)));
//         }
//     }
// }
