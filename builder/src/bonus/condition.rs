use std::fmt::Display;

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
    /// Requires that some of the conditions are true
    Any(Vec<Condition>),
    /// Requires that all of the conditions are true
    All(Vec<Condition>),
    /// Always True
    True,
    /// Always False
    False,
}

impl From<bool> for Condition {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

/// Generator functions to abstract away stndard conditions
impl Condition {
    /// Requires that the character has some attribute
    pub fn has(attribute: Attribute) -> Self {
        Self::GreaterThan(attribute.into(), 0f32.into())
    }

    /// Requires that the character does not have some attribute
    pub fn not_have(attribute: Attribute) -> Self {
        Self::EqualTo(attribute.into(), 0f32.into())
    }

    /// Requires that none of the conditions are true
    pub fn none(conditions: Vec<Condition>) -> Self {
        Self::Not(Box::new(Self::Any(conditions)))
    }

    /// Requires that at least one of the conditions is false
    pub fn not_all(conditions: Vec<Condition>) -> Self {
        Self::Not(Box::new(Self::All(conditions)))
    }

    /// Requires that one value is not equal to the other value
    pub fn not_eq(a: Value, b: Value) -> Self {
        Self::Not(Box::new(Self::EqualTo(a, b)))
    }
}

impl AttributeDependencies for Condition {
    fn has_attr_dependency(&self, attribute: Attribute) -> bool {
        match self {
            Self::Not(cond) => cond.has_attr_dependency(attribute),
            Self::GreaterThan(a, b) | Self::LessThan(a, b) | Self::EqualTo(a, b) => {
                a.has_attr_dependency(attribute) || b.has_attr_dependency(attribute)
            }
            Self::Any(conds) | Self::All(conds) => {
                conds.iter().any(|cond| cond.has_attr_dependency(attribute))
            }
            _ => false,
        }
    }

    fn include_attr_dependency(&self, set: &mut im::OrdSet<Attribute>) {
        match self {
            Self::Not(cond) => cond.include_attr_dependency(set),
            Self::GreaterThan(a, b) | Self::LessThan(a, b) | Self::EqualTo(a, b) => {
                a.include_attr_dependency(set);
                b.include_attr_dependency(set);
            }
            Self::Any(conds) | Self::All(conds) => {
                for cond in conds {
                    cond.include_attr_dependency(set);
                }
            }
            _ => {}
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
            Self::Any(conditions) => write!(f, "Any of {conditions:?}"),
            Self::All(conditions) => write!(f, "All of {conditions:?}"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
        }
    }
}
