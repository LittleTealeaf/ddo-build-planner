use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, AttributeDependencies};

use super::Value;

/// Describes an attribute-based condition that must be met for a bonus to be included.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Condition {
    /// Requires that a condition is not true
    Not(Box<Condition>),
    /// Requires that one value is greater than the next value
    GreaterThan(Value, Value),
    /// Requires that one value is less than the next value
    LessThan(Value, Value),
    /// Requires that one value is equal to another value
    EqualTo(Value, Value),
    /// Requires that one value is not equal to another value
    NotEqualTo(Value, Value),
    /// Requires that some of the conditions are true
    Any(Vec<Condition>),
    /// Requires that all of the conditions are true
    All(Vec<Condition>),
    /// Requires that all of the conditions are false
    NotAny(Vec<Condition>),
    /// Requires that some of the conditions are false
    NotAll(Vec<Condition>),
}

/// Generator functions to abstract away stndard conditions
impl Condition {
    /// Requires that the character has some attribute
    #[must_use]
    pub fn has(attribute: Attribute) -> Self {
        Self::GreaterThan(attribute.into(), 0f32.into())
    }

    /// Requires that the character does not have some attribute
    #[must_use]
    pub fn not_have(attribute: Attribute) -> Self {
        Self::EqualTo(attribute.into(), 0f32.into())
    }
}

impl AttributeDependencies for Condition {
    fn has_attr_dependency(&self, attribute: Attribute) -> bool {
        match self {
            Self::Not(cond) => cond.has_attr_dependency(attribute),
            Self::GreaterThan(a, b)
            | Self::LessThan(a, b)
            | Self::EqualTo(a, b)
            | Self::NotEqualTo(a, b) => {
                a.has_attr_dependency(attribute) || b.has_attr_dependency(attribute)
            }
            Self::Any(conds) | Self::All(conds) | Self::NotAny(conds) | Self::NotAll(conds) => {
                conds.iter().any(|cond| cond.has_attr_dependency(attribute))
            }
        }
    }

    fn include_attr_dependency(&self, set: &mut im::OrdSet<Attribute>) {
        match self {
            Self::Not(cond) => cond.include_attr_dependency(set),
            Self::GreaterThan(a, b)
            | Self::LessThan(a, b)
            | Self::EqualTo(a, b)
            | Self::NotEqualTo(a, b) => {
                a.include_attr_dependency(set);
                b.include_attr_dependency(set);
            }
            Self::Any(conds) | Self::All(conds) | Self::NotAny(conds) | Self::NotAll(conds) => {
                for cond in conds {
                    cond.include_attr_dependency(set);
                }
            }
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Not(condition) => write!(f, "Not {}", *condition),
            Self::GreaterThan(a, b) => write!(f, "{a} is greater than {b}"),
            Self::LessThan(a, b) => write!(f, "{a} is less than {b}"),
            Self::EqualTo(a, b) => write!(f, "{a} is equal to {b}"),
            Self::NotEqualTo(a, b) => write!(f, "{a} is not equal to {b}"),
            Self::Any(conditions) => write!(f, "Any of {conditions:?}"),
            Self::All(conditions) => write!(f, "All of {conditions:?}"),
            Self::NotAll(conditions) => write!(f, "Not all of {conditions:?}"),
            Self::NotAny(conditions) => write!(f, "Not any of {conditions:?}"),
        }
    }
}
