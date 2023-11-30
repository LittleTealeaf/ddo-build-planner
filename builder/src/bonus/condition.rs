use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor, Not},
};

use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, AttributeDependencies};

use super::Value;

/// Describes an attribute-based condition that must be met for a bonus to be included.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Condition {
    /// Requires that a condition is not true
    Not(Box<Condition>),
    /// Requires that one value is greater than the next value
    GreaterThan(Value, Value),
    /// Requires that one value is less than the next value
    LessThan(Value, Value),
    /// Requires that one value is equal to another value
    EqualTo(Value, Value),
    /// Insert a constant value
    Constant(bool),
    /// Requires that both conditions are satisfied
    And(Box<Self>, Box<Self>),
    /// Requires that at least one condition is satisfied
    Or(Box<Self>, Box<Self>),
    /// Exclusive Or gate, requires that only one is true
    Xor(Box<Self>, Box<Self>),
}

impl From<bool> for Condition {
    fn from(value: bool) -> Self {
        Self::Constant(value)
    }
}

/// Additional constructors for more complicated conditions
impl Condition {
    /// Requires that the character has some attribute
    pub fn has(attribute: Attribute) -> Self {
        Self::GreaterThan(attribute.into(), 0f32.into())
    }

    /// Requires that all of the provided conditions are true
    ///
    /// Returns [`None`] if the iterator has no values
    pub fn all<I>(conditions: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        conditions.into_iter().reduce(Self::and)
    }

    /// Requires that any of the provided conditions are true
    ///
    /// Returns [`None`] if the iterator has no values
    pub fn any<I>(conditions: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        conditions.into_iter().reduce(Self::or)
    }

    /// Requires that none of the conditions are true
    ///
    /// Returns [`None`] if the iterator has no values
    pub fn none<I>(conditions: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        Some(!Self::any(conditions)?)
    }

    /// Requires that at least one of the conditions is false
    ///
    /// Returns [`None`] if the iterator has no values
    pub fn not_all<I>(conditions: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        Some(!Self::all(conditions)?)
    }
}

/// Chain Operations
impl Condition {
    /// Logical AND
    ///
    /// Returns true if both values are true
    #[must_use]
    pub fn and(self, other: Self) -> Self {
        Self::And(self.into(), other.into())
    }

    /// Logical OR
    ///
    /// Returns true if one value is true
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        Self::Or(self.into(), other.into())
    }

    /// Logical XOR
    #[must_use]
    pub fn xor(self, other: Self) -> Self {
        Self::Xor(self.into(), other.into())
    }

    /// Logical NAND
    ///
    /// Returns false if both outputs are true, otherwise returns true
    #[must_use]
    pub fn nand(self, other: Self) -> Self {
        !self.and(other)
    }

    /// Logical NOR
    ///
    /// Returns true if both outputs are false
    #[must_use]
    pub fn nor(self, other: Self) -> Self {
        !self.or(other)
    }

    /// Logical XNOR
    ///
    /// Returns true if the values are either both true or both false
    #[must_use]
    pub fn xnor(self, other: Self) -> Self {
        !self.xor(other)
    }
}

impl AttributeDependencies for Condition {
    fn has_attr_dependency(&self, attribute: Attribute) -> bool {
        match self {
            Self::Not(cond) => cond.has_attr_dependency(attribute),
            Self::And(a, b) | Self::Or(a, b) | Self::Xor(a, b) => {
                a.has_attr_dependency(attribute) || b.has_attr_dependency(attribute)
            }
            Self::GreaterThan(a, b) | Self::LessThan(a, b) | Self::EqualTo(a, b) => {
                a.has_attr_dependency(attribute) || b.has_attr_dependency(attribute)
            }
            Self::Constant(_) => false,
        }
    }

    fn include_attr_dependency(&self, set: &mut im::OrdSet<Attribute>) {
        match self {
            Self::Not(cond) => cond.include_attr_dependency(set),

            Self::And(a, b) | Self::Or(a, b) | Self::Xor(a, b) => {
                a.include_attr_dependency(set);
                b.include_attr_dependency(set);
            }
            Self::GreaterThan(a, b) | Self::LessThan(a, b) | Self::EqualTo(a, b) => {
                a.include_attr_dependency(set);
                b.include_attr_dependency(set);
            }
            Self::Constant(_) => {}
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Not(condition) => write!(f, "Not ({})", *condition),
            Self::GreaterThan(a, b) => write!(f, "{a} is greater than {b}"),
            Self::LessThan(a, b) => write!(f, "{a} is less than {b}"),
            Self::EqualTo(a, b) => write!(f, "{a} is equal to {b}"),
            Self::Constant(true) => write!(f, "True"),
            Self::Constant(false) => write!(f, "False"),
            Self::And(a, b) => write!(f, "({a}) AND ({b})"),
            Self::Or(a, b) => write!(f, "({a}) OR ({b})"),
            Self::Xor(a, b) => write!(f, "({a}) == ({b})"),
        }
    }
}

impl Not for Condition {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Not(self.into())
    }
}

impl BitAnd for Condition {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::And(self.into(), rhs.into())
    }
}

impl BitOr for Condition {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Or(self.into(), rhs.into())
    }
}

impl BitXor for Condition {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Xor(self.into(), rhs.into())
    }
}
