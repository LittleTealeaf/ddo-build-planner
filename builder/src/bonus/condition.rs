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
    pub fn has(attribute: Attribute) -> Self {
        Self::GreaterThan(attribute.into(), 0f32.into())
    }

    /// Requires that the character does not have some attribute
    pub fn not_have(attribute: Attribute) -> Self {
        Self::EqualTo(attribute.into(), 0f32.into())
    }
}

impl AttributeDependencies for Condition {
    fn has_attr_dependency(&self, attribute: Attribute) -> bool {
        match self {
            Condition::Not(cond) => cond.has_attr_dependency(attribute),
            Condition::GreaterThan(a, b)
            | Condition::LessThan(a, b)
            | Condition::EqualTo(a, b)
            | Condition::NotEqualTo(a, b) => {
                a.has_attr_dependency(attribute) || b.has_attr_dependency(attribute)
            }
            Condition::Any(conds)
            | Condition::All(conds)
            | Condition::NotAny(conds)
            | Condition::NotAll(conds) => conds
                .into_iter()
                .any(|cond| cond.has_attr_dependency(attribute)),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Not(condition) => write!(f, "Not {}", *condition),
            Condition::GreaterThan(a, b) => write!(f, "{} is greater than {}", a, b),
            Condition::LessThan(a, b) => write!(f, "{} is less than {}", a, b),
            Condition::EqualTo(a, b) => write!(f, "{} is equal to {}", a, b),
            Condition::NotEqualTo(a, b) => write!(f, "{} is not equal to {}", a, b),
            Condition::Any(conditions) => write!(f, "Any of {:?}", conditions),
            Condition::All(conditions) => write!(f, "All of {:?}", conditions),
            Condition::NotAll(conditions) => write!(f, "Not all of {:?}", conditions),
            Condition::NotAny(conditions) => write!(f, "Not any of {:?}", conditions),
        }
    }
}
