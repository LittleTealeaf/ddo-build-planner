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

/// Constants
impl Condition {
    /// Constant TRUE value
    pub const TRUE: Self = Self::Constant(true);
    /// Constant FALSE value
    pub const FALSE: Self = Self::Constant(false);
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

/// [`Condition`] shortcuts
impl Value {
    /// Returns a condition that requires that this value is greater than the other value
    #[must_use]
    pub const fn greater_than(self, other: Self) -> Condition {
        Condition::GreaterThan(self, other)
    }

    /// Returns a condition that requires that this value is less than the other value
    #[must_use]
    pub const fn less_than(self, other: Self) -> Condition {
        Condition::LessThan(self, other)
    }

    /// Returns a condition that this value is equal to the other valuew
    #[must_use]
    pub const fn equal_to(self, other: Self) -> Condition {
        Condition::EqualTo(self, other)
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

#[cfg(test)]
mod tests {
    use super::*;

    mod from {
        use super::*;

        #[test]
        fn bool() {
            assert_eq!(Condition::TRUE, Condition::from(true));
            assert_eq!(Condition::FALSE, Condition::from(false));
        }
    }

    mod chain {
        use super::*;

        #[test]
        fn and() {
            let found = Condition::FALSE.and(Condition::TRUE);
            let expected = Condition::And(Condition::FALSE.into(), Condition::TRUE.into());
            assert_eq!(found, expected);
        }

        #[test]
        fn or() {
            let found = Condition::FALSE.or(Condition::TRUE);
            let expected = Condition::Or(Condition::FALSE.into(), Condition::TRUE.into());
            assert_eq!(found, expected);
        }

        #[test]
        fn xor() {
            let found = Condition::FALSE.xor(Condition::TRUE);
            let expected = Condition::Xor(Condition::FALSE.into(), Condition::TRUE.into());
            assert_eq!(found, expected);
        }

        #[test]
        fn nand() {
            let found = Condition::FALSE.nand(Condition::TRUE);
            let expected = Condition::Not(Box::new(Condition::And(
                Box::new(Condition::FALSE),
                Box::new(Condition::TRUE),
            )));
            assert_eq!(found, expected);
        }

        #[test]
        fn nor() {
            let found = Condition::FALSE.nor(Condition::TRUE);
            let expected = Condition::Not(Box::new(Condition::Or(
                Box::new(Condition::FALSE),
                Box::new(Condition::TRUE),
            )));
            assert_eq!(found, expected);
        }

        #[test]
        fn xnor() {
            let found = Condition::FALSE.xnor(Condition::TRUE);
            let expected = Condition::Not(Box::new(Condition::Xor(
                Box::new(Condition::FALSE),
                Box::new(Condition::TRUE),
            )));
            assert_eq!(found, expected);
        }
    }

    mod values {
        use super::*;

        #[test]
        fn greater_than() {
            let found = Value::from(0).greater_than(Value::from(1));
            let expected = Condition::GreaterThan(Value::from(0), Value::from(1));
            assert_eq!(found, expected);
        }

        #[test]
        fn less_than() {
            let found = Value::from(0).less_than(Value::from(1));
            let expected = Condition::LessThan(Value::from(0), Value::from(1));
            assert_eq!(found, expected);
        }

        #[test]
        fn equal_to() {
            let found = Value::from(0).equal_to(Value::from(1));
            let expected = Condition::EqualTo(Value::from(0), Value::from(1));
            assert_eq!(found, expected);
        }
    }

    mod ops {
        use super::*;

        #[test]
        fn not() {
            let condition = Condition::from(false).not();
            assert!(matches!(condition, Condition::Not(_)));
        }

        #[test]
        fn bitand() {
            let found = Condition::FALSE & Condition::TRUE;
            let expected = Condition::And(Condition::FALSE.into(), Condition::TRUE.into());
            assert_eq!(found, expected);
        }

        #[test]
        fn bitor() {
            let found = Condition::FALSE | Condition::TRUE;
            let expected = Condition::Or(Condition::FALSE.into(), Condition::TRUE.into());
            assert_eq!(found, expected);
        }

        #[test]
        fn bitxor() {
            let found = Condition::FALSE ^ Condition::TRUE;
            let expected = Condition::Xor(Condition::FALSE.into(), Condition::TRUE.into());
            assert_eq!(found, expected);
        }
    }

    mod iterators {
        use super::*;

        #[test]
        fn cond_any_returns_none_when_empty() {
            assert!([].cond_any().is_none());
        }

        #[test]
        fn cond_all_returns_none_when_empty() {
            assert!([].cond_all().is_none());
        }

        #[test]
        fn cond_none_returns_none_when_empty() {
            assert!([].cond_none().is_none());
        }

        #[test]
        fn cond_not_all_returns_none_when_empty() {
            assert!([].cond_not_all().is_none());
        }

        #[test]
        fn cond_any() {
            fn test_condition(condition: Condition) {
                match condition {
                    Condition::Constant(_) => {}
                    Condition::Or(a, b) => {
                        test_condition(*a);
                        test_condition(*b);
                    }
                    cond => panic!("Found illegal condition: {cond}"),
                }
            }

            let condition = [Condition::FALSE; 100]
                .cond_any()
                .expect("Expected Some(condition)");

            test_condition(condition);
        }

        #[test]
        fn cond_all() {
            fn test_condition(condition: Condition) {
                match condition {
                    Condition::Constant(_) => {}
                    Condition::And(a, b) => {
                        test_condition(*a);
                        test_condition(*b);
                    }
                    cond => panic!("Found illegal condition: {cond}"),
                }
            }

            let condition = [Condition::FALSE; 100]
                .cond_all()
                .expect("Expected Some(condition)");

            test_condition(condition);
        }

        #[test]
        fn cond_none() {
            fn test_condition(condition: Condition) {
                match condition {
                    Condition::Constant(_) => {}
                    Condition::Or(a, b) => {
                        test_condition(*a);
                        test_condition(*b);
                    }
                    cond => panic!("Found illegal condition: {cond}"),
                }
            }

            let condition = [Condition::FALSE; 100]
                .cond_none()
                .expect("Expected Some(condition)");

            match condition {
                Condition::Not(condition) => test_condition(*condition),
                condition => panic!("Condition does not start with Not: {condition}"),
            }
        }

        #[test]
        fn cond_not_all() {
            fn test_condition(condition: Condition) {
                match condition {
                    Condition::Constant(_) => {}
                    Condition::And(a, b) => {
                        test_condition(*a);
                        test_condition(*b);
                    }
                    cond => panic!("Found illegal condition: {cond}"),
                }
            }

            let condition = [Condition::FALSE; 100]
                .cond_not_all()
                .expect("Expected Some(condition)");

            match condition {
                Condition::Not(condition) => test_condition(*condition),
                condition => panic!("Condition does not start with Not: {condition}"),
            }
        }
    }
}
