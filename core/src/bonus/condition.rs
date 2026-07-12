mod simplify;

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use itertools::Itertools;

use crate::bonus::BonusValue;

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
        Self::Not(Box::new(self.and(other)))
    }

    /// Logical NOR
    ///
    /// Returns true if both outputs are false
    #[must_use]
    pub fn nor(self, other: Self) -> Self {
        Self::Not(Box::new(self.or(other)))
    }

    /// Logical XNOR
    ///
    /// Returns true if the values are either both true or both false
    #[must_use]
    pub fn xnor(self, other: Self) -> Self {
        Self::Not(Box::new(self.xor(other)))
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
        Self::Not(Box::new(self))
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
