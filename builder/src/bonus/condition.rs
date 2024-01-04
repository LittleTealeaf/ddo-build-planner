use std::{
    collections::HashSet,
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor, Not},
};

use itertools::Itertools;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, AttributeDependencies};

use super::{Depth, Value};

/// Describes an attribute-based condition that must be met for a bonus to be included.
#[derive(Hash, Clone, Eq, Debug, Serialize, Deserialize, PartialEq)]
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
    #[must_use]
    pub fn has(attribute: impl Into<Attribute>) -> Self {
        Self::GreaterThan(
            Value::Attribute(attribute.into()),
            Value::Const(Decimal::ZERO),
        )
    }
}

/// Chain Operations
impl Condition {
    /// Logical AND
    ///
    /// Returns true if both values are true
    #[must_use]
    pub fn and(self, other: Self) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }

    /// Logical OR
    ///
    /// Returns true if one value is true
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }

    /// Logical XOR
    #[must_use]
    pub fn xor(self, other: Self) -> Self {
        Self::Xor(Box::new(self), Box::new(other))
    }

    /// Logical NAND
    ///
    /// Returns false if both outputs are true, otherwise returns true
    #[must_use]
    pub fn nand(self, other: Self) -> Self {
        Self::Not(Box::new(Self::And(Box::new(self), Box::new(other))))
    }

    /// Logical NOR
    ///
    /// Returns true if both outputs are false
    #[must_use]
    pub fn nor(self, other: Self) -> Self {
        Self::Not(Box::new(Self::Or(Box::new(self), Box::new(other))))
    }

    /// Logical XNOR
    ///
    /// Returns true if the values are either both true or both false
    #[must_use]
    pub fn xnor(self, other: Self) -> Self {
        Self::Not(Box::new(Self::Xor(Box::new(self), Box::new(other))))
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

    fn include_attr_dependency(&self, set: &mut HashSet<Attribute>) {
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

impl Depth for Condition {
    fn get_depth(&self) -> usize {
        match self {
            Self::Constant(_) => 1,
            Self::Not(a) => a.get_depth(),
            Self::GreaterThan(a, b) | Self::LessThan(a, b) | Self::EqualTo(a, b) => {
                a.get_depth().max(b.get_depth())
            }
            Self::And(a, b) | Self::Or(a, b) | Self::Xor(a, b) => a.get_depth().max(b.get_depth()),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Not(condition) => write!(f, "Not ({})", *condition),
            Self::GreaterThan(a, b) => write!(f, "{a} > {b}"),
            Self::LessThan(a, b) => write!(f, "{a} < {b}"),
            Self::EqualTo(a, b) => write!(f, "{a} == {b}"),
            Self::Constant(value) => write!(f, "{value}"),
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
        Self::And(Box::new(self), Box::new(rhs))
    }
}

impl BitOr for Condition {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Or(Box::new(self), Box::new(rhs))
    }
}

impl BitXor for Condition {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Xor(Box::new(self), Box::new(rhs))
    }
}

/// Condition folding functions that consumes a set of conditions into a single condition
pub trait ConditionFold {
    /// Returns a condition that returns `true` if any of the conditions are `true`. Returns [`None`] if there are no items in the iterator
    fn cond_any(self) -> Option<Condition>;
    /// Returns a condition that returns `true` if all of the conditions are `true`. Returns [`None`] if there are no items in the iterator
    fn cond_all(self) -> Option<Condition>;
    /// Returns a condition that returns `true` if none of the conditions are `true`. Returns [`None`] if there are no items in the iterator
    fn cond_none(self) -> Option<Condition>;
    /// Returns a condition that returns `true` if not all of the conditions are `true`. Returns [`None`] if there are no items in the iterator
    fn cond_not_all(self) -> Option<Condition>;
}

impl<I> ConditionFold for I
where
    I: IntoIterator<Item = Condition>,
{
    fn cond_any(self) -> Option<Condition> {
        self.into_iter().tree_fold1(|a, b| a | b)
    }

    fn cond_all(self) -> Option<Condition> {
        self.into_iter().tree_fold1(|a, b| a & b)
    }

    fn cond_none(self) -> Option<Condition> {
        self.cond_any().map(Condition::not)
    }

    fn cond_not_all(self) -> Option<Condition> {
        self.cond_all().map(Condition::not)
    }
}
