use std::fmt::Display;

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
    /// Sums each of the values
    Sum(Vec<Value>),
    /// Multiplies each of the values
    Product(Vec<Value>),
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
    pub fn mean(values: Vec<Value>) -> Self {
        Self::Product(vec![
            Self::Value((values.len() as f32).recip()),
            Self::Sum(values),
        ])
    }

    /// Makes the given value negative
    pub fn negative(value: Value) -> Self {
        Self::Product(vec![value, Self::Value(-1f32)])
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => value.fmt(f),
            Self::Attribute(attr) => attr.fmt(f),
            Self::Sum(vals) => {
                write!(f, "(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;
                }

                for val in iter {
                    write!(f, " + {val}")?;
                }

                write!(f, ")")
            }
            Self::Product(vals) => {
                write!(f, "(")?;

                let mut iter = vals.iter();

                if let Some(val) = iter.next() {
                    val.fmt(f)?;

                    for val in iter {
                        write!(f, " * {val}")?;
                    }
                }

                write!(f, ")")
            }
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
            Self::Value(_) => false,
            Self::Attribute(attr) => attribute.eq(attr),
            Self::Min(vals) | Self::Max(vals) | Self::Product(vals) | Self::Sum(vals) => {
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
            Self::Attribute(attr) => {
                set.insert(*attr);
            }
            Self::Min(vals) | Self::Max(vals) | Self::Product(vals) | Self::Sum(vals) => {
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
