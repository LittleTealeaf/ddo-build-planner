use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, AttributeDependencies};

use super::Condition;

/// Represents a value of a [`Bonus`]
///
/// [`Bonus`]: crate::bonus::Bonus
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Value {
    /// Just a simple [`f32`] value.
    Value(f32),
    /// Copy the total value of some [`Attribute`].
    Attribute(Attribute),
    /// Returns the minimum value from the set
    Min(Vec<Value>),
    /// Returns the maximum value from the set
    Max(Vec<Value>),
    /// Floors the inner value to a whole number
    Floor(Box<Value>),
    /// Calculates the reciprocal of the number.
    ///
    /// For example, 5 would become 1/5
    Reciprocal(Box<Value>),
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

impl Value {
    /// Calculates the mean or average of the given values
    #[allow(clippy::cast_precision_loss)]
    pub fn mean(values: Vec<Self>) -> Self {
        let len = values.len();
        values.into_iter().sum::<Value>() / Self::Value(len as f32)
    }
}

/// Operations to simplify writing formulas
impl Value {
    /// Floors the value
    pub fn floor(self) -> Self {
        Self::Floor(self.into())
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
            Self::Value(value) => value.fmt(f),
            Self::Attribute(attr) => attr.fmt(f),
            Self::Min(vals) => {
                write!(f, "Min(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    for val in iter {
                        write!(f, ", {val}")?;
                    }
                }

                write!(f, ")")
            }
            Self::Max(vals) => {
                write!(f, "Max(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    for val in iter {
                        write!(f, ", {val}")?;
                    }
                }

                write!(f, ")")
            }
            Self::Floor(val) => write!(f, "Floor({val})"),
            Self::Reciprocal(val) => write!(f, "(1 / {val})"),
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
            | Self::Rem(a, b) => {
                a.has_attr_dependency(attribute) || b.has_attr_dependency(attribute)
            }
            Self::Value(_) => false,
            Self::Attribute(attr) => attribute.eq(attr),
            Self::Min(vals) | Self::Max(vals) => {
                vals.iter().any(|val| val.has_attr_dependency(attribute))
            }
            Self::Floor(val) | Self::Reciprocal(val) => val.has_attr_dependency(attribute),
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

    fn include_attr_dependency(&self, set: &mut im::OrdSet<Attribute>) {
        match self {
            Self::Value(_) => {}
            Self::Add(a, b)
            | Self::Sub(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::Rem(a, b) => {
                a.include_attr_dependency(set);
                b.include_attr_dependency(set);
            }
            Self::Attribute(attr) => {
                set.insert(*attr);
            }
            Self::Min(vals) | Self::Max(vals) => {
                for val in vals {
                    val.include_attr_dependency(set);
                }
            }
            Self::Reciprocal(val) | Self::Floor(val) => val.include_attr_dependency(set),
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

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Value(value)
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
        Self::Mul(self.into(), Self::Value(-1f32).into())
    }
}

impl Sum for Value {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut sum = iter.next().unwrap();

        for item in iter {
            sum = sum + item;
        }
        sum
    }
}
