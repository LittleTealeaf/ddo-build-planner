mod simplify;

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use itertools::Itertools;

use crate::{
    attribute::Attribute,
    bonus::{traits::ContainsAttribute, BonusValue},
};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, derive_more::Display,
)]
pub enum BonusCondition {
    #[display("!({_0})")]
    Not(Box<Self>),
    #[display("{_0} > {_1}")]
    GreaterThan(BonusValue, BonusValue),
    #[display("{_0} >= {_1}")]
    GreaterEqualTo(BonusValue, BonusValue),
    #[display("{_0} < {_1}")]
    LessThan(BonusValue, BonusValue),
    #[display("{_0} <= {_1}")]
    LessEqualTo(BonusValue, BonusValue),
    #[display("{_0} = {_1}")]
    EqualTo(BonusValue, BonusValue),
    #[display("{_0}")]
    Constant(bool),
    #[display("({_0} & {_1})")]
    And(Box<Self>, Box<Self>),
    #[display("({_0} | {_1})")]
    Or(Box<Self>, Box<Self>),
    #[display("({_0} ^ {_1})")]
    Xor(Box<Self>, Box<Self>),
}

impl BonusCondition {
    pub const TRUE: Self = Self::Constant(true);
    pub const FALSE: Self = Self::Constant(false);

    /// Returns `true` if the condition is an explicit Constant `true`
    #[must_use]
    pub const fn is_true(&self) -> bool {
        matches!(&self, Self::Constant(true))
    }

    /// Returns `true` if the condition is an explicit Constant `false`
    #[must_use]
    pub const fn is_false(&self) -> bool {
        matches!(&self, Self::Constant(true))
    }
}

impl Default for BonusCondition {
    fn default() -> Self {
        Self::TRUE
    }
}

impl BonusCondition {
    /// Logical AND
    ///
    /// Returns true if both values are true
    #[must_use]
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Constant(left), Self::Constant(right)) => Self::Constant(left && right),
            (Self::Constant(true), other) | (other, Self::Constant(true)) => other,
            (Self::Constant(false), _) | (_, Self::Constant(false)) => Self::FALSE,
            (left, right) => Self::And(Box::new(left), Box::new(right)),
        }
    }

    /// Logical OR
    ///
    /// Returns true if one value is true
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::Constant(true), _) | (_, Self::Constant(true)) => Self::TRUE,
            (Self::Constant(false), other) | (other, Self::Constant(false)) => other,
            (left, right) => Self::Or(Box::new(left), Box::new(right)),
        }
    }

    /// Logical XOR
    #[must_use]
    pub fn xor(self, other: Self) -> Self {
        match (self, other) {
            (Self::Constant(left), Self::Constant(right)) => Self::Constant(left ^ right),

            // If one is false, XOR just returns the other value (false ^ X = X)
            (Self::Constant(false), other) | (other, Self::Constant(false)) => other,

            // If one is true, XOR returns the inverse of the other value (true ^ X = !X)
            // We already know it's not a constant, so avoiding .not() additional calc
            (Self::Constant(true), other) | (other, Self::Constant(true)) => {
                Self::Not(Box::new(other))
            }

            // Otherwise, box them into a new Xor node
            (left, right) => Self::Xor(Box::new(left), Box::new(right)),
        }
    }

    /// Logical NAND
    ///
    /// Returns false if both outputs are true, otherwise returns true
    #[must_use]
    pub fn nand(self, other: Self) -> Self {
        self.and(other).not()
    }

    /// Logical NOR
    ///
    /// Returns true if both outputs are false
    #[must_use]
    pub fn nor(self, other: Self) -> Self {
        self.or(other).not()
    }

    /// Logical XNOR
    ///
    /// Returns true if the values are either both true or both false
    #[must_use]
    pub fn xnor(self, other: Self) -> Self {
        self.xor(other).not()
    }
}

impl BonusCondition {
    pub fn any<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter()
            .tree_reduce(|a, b| a | b)
            .unwrap_or(Self::FALSE)
    }

    pub fn all<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter()
            .tree_reduce(|a, b| a & b)
            .unwrap_or(Self::TRUE)
    }

    pub fn none<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self::any(iter).not()
    }

    pub fn not_all<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self::all(iter).not()
    }
}

/// [`Condition`] shortcuts
impl BonusValue {
    /// Returns a condition that requires that this value is greater than the other value
    #[must_use]
    pub const fn greater_than(self, other: Self) -> BonusCondition {
        BonusCondition::GreaterThan(self, other)
    }

    /// Returns a condition that requires that this value is less than the other value
    #[must_use]
    pub const fn less_than(self, other: Self) -> BonusCondition {
        BonusCondition::LessThan(self, other)
    }

    /// Returns a condition that this value is equal to the other value
    #[must_use]
    pub const fn equal_to(self, other: Self) -> BonusCondition {
        BonusCondition::EqualTo(self, other)
    }

    /// Returns a condition that this value is greater than or equal to the other value
    #[must_use]
    pub const fn greater_or_equal_to(self, other: Self) -> BonusCondition {
        BonusCondition::GreaterEqualTo(self, other)
    }

    /// Returns a condition that this value is equal to or less than the other value
    #[must_use]
    pub const fn less_or_equal_to(self, other: Self) -> BonusCondition {
        BonusCondition::LessEqualTo(self, other)
    }
}

impl Not for BonusCondition {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Constant(val) => Self::Constant(!val),
            cond => Self::Not(Box::new(cond)),
        }
    }
}

impl BitAnd for BonusCondition {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitAndAssign for BonusCondition {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.clone().bitand(rhs);
    }
}

impl BitOr for BonusCondition {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitOrAssign for BonusCondition {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.clone().bitor(rhs);
    }
}

impl BitXor for BonusCondition {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl BitXorAssign for BonusCondition {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.clone().bitxor(rhs);
    }
}

impl ContainsAttribute for BonusCondition {
    fn any_attribute<'a, F>(&'a self, fun: &F) -> bool
    where
        F: Fn(&'a Attribute) -> bool,
    {
        match self {
            Self::Not(bonus_condition) => bonus_condition.any_attribute(fun),
            Self::GreaterThan(val, val1)
            | Self::GreaterEqualTo(val, val1)
            | Self::LessThan(val, val1)
            | Self::LessEqualTo(val, val1)
            | Self::EqualTo(val, val1) => val1.any_attribute(fun) || val.any_attribute(fun),
            Self::Constant(_) => false,
            Self::And(bonus_condition, bonus_condition1)
            | Self::Or(bonus_condition, bonus_condition1)
            | Self::Xor(bonus_condition, bonus_condition1) => {
                bonus_condition.any_attribute(fun) || bonus_condition1.any_attribute(fun)
            }
        }
    }
}
