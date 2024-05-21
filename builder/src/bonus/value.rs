use core::fmt;
use core::{
    fmt::Display,
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use std::collections::HashSet;

use itertools::Itertools;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, AttributeDependencies, ToAttribute};

use super::{Condition, Depth, HasDice};

/// Represents a value of a [`Bonus`]
///
/// [`Bonus`]: crate::bonus::Bonus
#[derive(Clone, PartialEq, Debug, Hash, Eq, Serialize, Deserialize)]
pub enum Value {
    /// Hard codes a specific [`Decimal`] value.
    #[serde(rename = "v", alias = "Const")]
    Const(Decimal),
    /// Copy the total value of some [`Attribute`].
    #[serde(rename = "at", alias = "Attribute")]
    Attribute(Attribute),
    /// Returns the minimum value of the two
    #[serde(rename = "mi", alias = "Min")]
    Min(Box<Value>, Box<Value>),
    /// Returns the maximum value of the two
    #[serde(rename = "ma", alias = "Max")]
    Max(Box<Value>, Box<Value>),
    /// Floors the inner value to a whole number
    #[serde(rename = "fl", alias = "Floor")]
    Floor(Box<Value>),
    /// Ceils the inner value to a whole number
    #[serde(rename = "ce", alias = "Ceil")]
    Ceil(Box<Value>),
    /// Rounds the value to the closest whole number
    #[serde(rename = "ro", alias = "Round")]
    Round(Box<Value>),
    /// Makes the value positive if it is negative
    #[serde(rename = "ab", alias = "Abs")]
    Abs(Box<Value>),
    /// Adds the first value to the second value
    #[serde(rename = "a", alias = "Add")]
    Add(Box<Value>, Box<Value>),
    /// Subtracts the second value from the first value
    #[serde(rename = "s", alias = "Sub")]
    Sub(Box<Value>, Box<Value>),
    /// Multiplies the two values
    #[serde(rename = "m", alias = "Mul")]
    Mul(Box<Value>, Box<Value>),
    /// Divides the first value by the second value
    #[serde(rename = "d", alias = "Div")]
    Div(Box<Value>, Box<Value>),
    /// Returns the remainder from dividing the first value by the second value
    #[serde(rename = "r", alias = "Rem")]
    Rem(Box<Value>, Box<Value>),
    /// Returns `if_true` if `condition` is true, otherwise returns `if_false`
    #[serde(rename = "i", alias = "If")]
    If {
        /// The condition needed to be checked
        #[serde(rename = "c", alias = "condition")]
        condition: Box<Condition>,
        /// The value to return if the condition returns true
        #[serde(rename = "t", alias = "if_true")]
        if_true: Box<Value>,
        /// The value to return if the condition returns false
        #[serde(rename = "f", alias = "if_false")]
        if_false: Box<Value>,
    },
    /// Represents a die roll. Attributes will be calculated based on the mean roll of the dice
    #[serde(rename = "di", alias = "Dice")]
    Dice {
        /// The number of dice to roll
        #[serde(rename = "c", alias = "count")]
        count: Box<Value>,
        /// The dice size
        #[serde(rename = "s", alias = "size")]
        size: Box<Value>,
    },
}

/// Constants
impl Value {
    /// A constant representing 0
    pub const ZERO: Self = Self::Const(Decimal::ZERO);

    /// A constant representing 1
    pub const ONE: Self = Self::Const(Decimal::ONE);

    /// A constant representing -1
    pub const NEGATIVE_ONE: Self = Self::Const(Decimal::NEGATIVE_ONE);

    /// A constant representing 2
    pub const TWO: Self = Self::Const(Decimal::TWO);

    /// A constant representing 10
    pub const TEN: Self = Self::Const(Decimal::TEN);

    /// A constant representing 100
    pub const ONE_HUNDRED: Self = Self::Const(Decimal::ONE_HUNDRED);

    /// A constant representing the largest value that can be represented
    pub const MAX: Self = Self::Const(Decimal::MAX);

    /// A constant representing the smallest value that can be represented
    pub const MIN: Self = Self::Const(Decimal::MIN);
}

/// Operations to simplify writing formulas
impl Value {
    #[must_use]
    /// Shortcut for [`Value::Dice`]
    ///
    /// Represents a dice roll that can either go from 1 to `size`, and is rolled `count` number of
    /// times
    pub fn dice<C, I>(count: C, size: I) -> Self
    where
        C: Into<Self>,
        I: Into<Self>,
    {
        Self::Dice {
            count: Box::new(count.into()),
            size: Box::new(size.into()),
        }
    }

    /// Shortcut for [`Value::If`]
    #[must_use]
    pub fn condition<C, T, F>(condition: C, if_true: T, if_false: F) -> Self
    where
        C: Into<Condition>,
        T: Into<Self>,
        F: Into<Self>,
    {
        Self::If {
            condition: Box::new(condition.into()),
            if_true: Box::new(if_true.into()),
            if_false: Box::new(if_false.into()),
        }
    }

    /// Calculates the mean of some list or set
    ///
    /// # Panics
    /// Panics if there are 0 items in the iterator
    pub fn mean<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let mut count = 0;

        let value = Self::iter_sum(iter.into_iter().map(|c| {
            count += 1;
            c
        }));

        value / count.to_value()
    }

    /// Returns the minimum of all of the values
    ///
    /// # Panics
    /// Panics if an iterator with no items is passed in
    pub fn iter_min<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter()
            .tree_reduce(Self::min)
            .expect("Expected at least one value")
    }

    /// Returns the maximum of all of the values
    ///
    /// # Panics
    /// Panics if an iterator with no items is passed in
    pub fn iter_max<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter()
            .tree_reduce(Self::max)
            .expect("Expected at least one value")
    }

    /// Returns the sum of the values within the iterator
    ///
    /// # Panics
    /// Panics if an iterator with no items is passed in
    pub fn iter_sum<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter()
            .tree_reduce(Self::add)
            .expect("Expected at least one value")
    }

    /// Returns the sum of the values within the iterator
    ///
    /// # Panics
    /// Panics if an iterator with no items is passed in
    pub fn iter_product<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter()
            .tree_reduce(Self::mul)
            .expect("Expected at least one value")
    }

    /// Shortcut for [`Condition::Floor`]
    ///
    /// [`Condition::Floor`]: Self#variant.Floor
    #[must_use]
    pub fn floor(self) -> Self {
        Self::Floor(Box::new(self))
    }

    /// Shortcut for [`Condition::Ceil`]
    ///
    /// [`Condition::Ceil`]: Self#variant.Ceil
    #[must_use]
    pub fn ceil(self) -> Self {
        Self::Ceil(Box::new(self))
    }

    /// Shortcut for [`Condition::Round`]
    ///
    /// [`Condition::Round`]: Self#variant.Round
    #[must_use]
    pub fn round(self) -> Self {
        Self::Round(Box::new(self))
    }

    /// Shortcut for [`Condition::Abs`]
    ///
    /// [`Condition::Abs`]: Self#variant.Abs
    #[must_use]
    pub fn abs(self) -> Self {
        Self::Abs(Box::new(self))
    }

    /// Returns the reciprocal
    ///
    /// The reciprocal of value `x` is equivalent to `1 / x`
    #[must_use]
    pub fn recip(self) -> Self {
        Self::ONE / self
    }

    /// Shortcut for [`Condition::Max`]
    ///
    /// [`Condition::Max`]: Self#variant.Min
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self::Max(Box::new(self), Box::new(other))
    }

    /// Shortcut for [`Condition::Min`]
    ///
    /// [`Condition::Min`]: Self#variant.Min
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self::Min(Box::new(self), Box::new(other))
    }
}

/// Implements a shortcut to using [`Value::from`]
pub trait ToValue {
    /// Converts this into a value
    fn to_value(self) -> Value;
}

impl<T> ToValue for T
where
    Value: From<T>,
{
    fn to_value(self) -> Value {
        Value::from(self)
    }
}

impl Depth for Value {
    fn get_depth(&self) -> usize {
        1 + match self {
            Self::Const(_) | Self::Attribute(_) => 0,
            Self::Min(a, b)
            | Self::Max(a, b)
            | Self::Add(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::Rem(a, b) => a.get_depth().max(b.get_depth()),
            Self::Round(a) | Self::Abs(a) | Self::Floor(a) | Self::Ceil(a) => a.get_depth(),
            Self::If {
                condition,
                if_true,
                if_false,
            } => condition
                .get_depth()
                .max(if_true.get_depth())
                .max(if_false.get_depth()),
            Self::Dice { count, size } => count.get_depth().max(size.get_depth()),
        }
    }
}

impl HasDice for Value {
    fn has_dice(&self) -> bool {
        match self {
            Self::Const(_) | Self::Attribute(_) => false,
            Self::Min(a, b)
            | Self::Max(a, b)
            | Self::Add(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::Rem(a, b) => a.has_dice() || b.has_dice(),
            Self::Floor(val) | Self::Ceil(val) | Self::Round(val) | Self::Abs(val) => {
                val.has_dice()
            }
            Self::If {
                condition,
                if_true,
                if_false,
            } => condition.has_dice() || if_true.has_dice() || if_false.has_dice(),
            Self::Dice { .. } => true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            Self::Round(val) => write!(f, "Round({val})"),
            Self::If {
                condition,
                if_true,
                if_false,
            } => {
                write!(f, "If ({condition}) then {if_true} else {if_false}")
            }
            Self::Dice { count, size } => write!(f, "({count})d({size})"),
        }
    }
}

impl AttributeDependencies for Value {
    fn has_attr_dependency(&self, attribute: &Attribute) -> bool {
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
            Self::Round(val) | Self::Abs(val) | Self::Ceil(val) | Self::Floor(val) => {
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
            Self::Dice { count, size } => {
                count.has_attr_dependency(attribute) || size.has_attr_dependency(attribute)
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
                set.insert(attr.clone());
            }
            Self::Round(val) | Self::Abs(val) | Self::Ceil(val) | Self::Floor(val) => {
                val.include_attr_dependency(set);
            }
            Self::If {
                condition,
                if_true,
                if_false,
            } => {
                condition.include_attr_dependency(set);
                if_true.include_attr_dependency(set);
                if_false.include_attr_dependency(set);
            }
            Self::Dice { count, size } => {
                count.include_attr_dependency(set);
                size.include_attr_dependency(set);
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
        Self::Attribute(value.to_attribute())
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
        Self::Add(Box::new(self), Box::new(rhs))
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Mul(Box::new(self), Box::new(rhs))
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Div(Box::new(self), Box::new(rhs))
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Rem(Box::new(self), Box::new(rhs))
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Mul(Box::new(self), Box::new(Self::NEGATIVE_ONE))
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

/// Expands to a static representation of a constant value
#[macro_export]
macro_rules! val {
    ($value:literal) => {
        $crate::bonus::Value::Const(rust_decimal_macros::dec!($value))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    mod val_macro {
        use rust_decimal_macros::dec;

        use super::*;

        #[test]
        fn returns_constant() {
            let value = val!(5);
            assert_eq!(value, Value::Const(dec!(5)));
        }

        #[test]
        fn negative_values() {
            let value = val!(-5);
            assert_eq!(value, Value::Const(dec!(-5)));
        }
    }

    mod consts {
        use super::*;

        #[test]
        fn zero() {
            assert_eq!(Value::ZERO, Value::from(0));
        }

        #[test]
        fn one() {
            assert_eq!(Value::ONE, Value::from(1));
        }

        #[test]
        fn negative_one() {
            assert_eq!(Value::NEGATIVE_ONE, Value::from(-1));
        }

        #[test]
        fn one_hundred() {
            assert_eq!(Value::ONE_HUNDRED, Value::from(100));
        }

        #[test]
        fn two() {
            assert_eq!(Value::TWO, Value::from(2));
        }

        #[test]
        fn max() {
            assert_eq!(Value::MAX, Value::from(Decimal::MAX));
        }

        #[test]
        fn min() {
            assert_eq!(Value::MIN, Value::from(Decimal::MIN));
        }
    }
}
