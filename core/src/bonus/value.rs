use core::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use rust_decimal::Decimal;

use itertools::Itertools;

use crate::{
    attribute::Attribute,
    bonus::{traits::ContainsAttribute, BonusCondition},
};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, derive_more::Display,
)]
pub enum BonusValue {
    Const(Decimal),
    Attribute(Attribute),
    #[display("Context[{_0}]({_1})")]
    Snapshot(u32, Box<Self>),
    #[display("Min({_0}, {_1})")]
    Min(Box<Self>, Box<Self>),
    #[display("Max({_0}, {_1})")]
    Max(Box<Self>, Box<Self>),
    #[display("Floor({_0})")]
    Floor(Box<Self>),
    #[display("Ceil({_0})")]
    Ceil(Box<Self>),
    #[display("Round({_0})")]
    Round(Box<Self>),
    #[display("Abs({_0})")]
    Abs(Box<Self>),
    #[display("({_0} + {_1})")]
    Add(Box<Self>, Box<Self>),
    #[display("({_0} - {_1})")]
    Sub(Box<Self>, Box<Self>),
    #[display("({_0} * {_1})")]
    Mul(Box<Self>, Box<Self>),
    #[display("({_0} / {_1})")]
    Div(Box<Self>, Box<Self>),
    #[display("({_0} % {_1})")]
    Rem(Box<Self>, Box<Self>),
    #[display("If({condition}, {if_true}, {if_false})")]
    If {
        condition: Box<BonusCondition>,
        if_true: Box<Self>,
        if_false: Box<Self>,
    },
    #[display("Dice({count} d {size})")]
    Dice {
        count: Box<Self>,
        size: Box<Self>,
    },
}

#[macro_export]
macro_rules! val {
    ($value:literal) => {
        $crate::bonus::BonusValue::Const(rust_decimal_macros::dec!($value))
    };
}

impl BonusValue {
    pub const ZERO: Self = Self::Const(Decimal::ZERO);
    pub const ONE: Self = Self::Const(Decimal::ONE);
    pub const NEGATIVE_ONE: Self = Self::Const(Decimal::NEGATIVE_ONE);
    pub const TWO: Self = Self::Const(Decimal::TWO);
    pub const ONE_HUNDRED: Self = Self::Const(Decimal::ONE_HUNDRED);
    pub const MAX: Self = Self::Const(Decimal::MAX);
    pub const MIN: Self = Self::Const(Decimal::MIN);
}

impl BonusValue {
    pub fn context<V>(config: u32, value: V) -> Self
    where
        V: Into<Self>,
    {
        Self::Snapshot(config, Box::new(value.into()))
    }

    #[must_use]
    pub fn dice<C, S>(count: C, size: S) -> Self
    where
        C: Into<Self>,
        S: Into<Self>,
    {
        Self::Dice {
            count: Box::new(count.into()),
            size: Box::new(size.into()),
        }
    }

    pub fn condition<C, T, F>(condition: C, if_true: T, if_false: F) -> Self
    where
        C: Into<BonusCondition>,
        T: Into<Self>,
        F: Into<Self>,
    {
        match condition.into() {
            BonusCondition::Constant(true) => if_true.into(),
            BonusCondition::Constant(false) => if_false.into(),
            condition => Self::If {
                condition: Box::new(condition),
                if_true: Box::new(if_true.into()),
                if_false: Box::new(if_false.into()),
            },
        }
    }

    pub fn gated<C, T>(condition: C, if_true: T) -> Self
    where
        C: Into<BonusCondition>,
        T: Into<Self>,
    {
        Self::condition(condition, if_true, Self::ZERO)
    }
}

impl BonusValue {
    #[must_use]
    pub fn floor(self) -> Self {
        Self::Floor(Box::new(self))
    }

    #[must_use]
    pub fn ceil(self) -> Self {
        Self::Ceil(Box::new(self))
    }

    #[must_use]
    pub fn round(self) -> Self {
        Self::Round(Box::new(self))
    }

    #[must_use]
    pub fn abs(self) -> Self {
        Self::Abs(Box::new(self))
    }

    #[must_use]
    pub fn recip(self) -> Self {
        Self::ONE / self
    }

    #[must_use]
    pub fn max(self, other: Self) -> Self {
        match (self, other) {
            (Self::Const(left), Self::Const(right)) => Self::Const(left.max(right)),
            (left, right) => Self::Max(Box::new(left), Box::new(right)),
        }
    }

    #[must_use]
    pub fn min(self, other: Self) -> Self {
        match (self, other) {
            (Self::Const(left), Self::Const(right)) => Self::Const(left.min(right)),
            (left, right) => Self::Min(Box::new(left), Box::new(right)),
        }
    }

    #[must_use]
    pub fn as_multiplier(self) -> Self {
        Self::ONE + (self / Self::ONE_HUNDRED)
    }
}

impl BonusValue {
    pub fn iter_sum<I>(iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter().tree_reduce(Self::add)
    }

    pub fn iter_product<I>(iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter().tree_reduce(Self::mul)
    }

    pub fn iter_min<I>(iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter().tree_reduce(Self::min)
    }

    pub fn iter_max<I>(iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        iter.into_iter().tree_reduce(Self::max)
    }

    pub fn mean<I>(iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        let mut count = 0;
        let counted = iter.into_iter().inspect(|_| count += 1);
        let sum = Self::iter_sum(counted)?;
        Some(sum / Self::from(count))
    }
}

impl From<Attribute> for BonusValue {
    fn from(value: Attribute) -> Self {
        Self::Attribute(value)
    }
}

impl From<Decimal> for BonusValue {
    fn from(value: Decimal) -> Self {
        Self::Const(value)
    }
}

macro_rules! from_primitive {
    ($($type:ty), +) => {
        $(
            impl From<$type> for BonusValue {
                fn from(value: $type) -> Self {
                    Self::Const(Decimal::from(value))
                }
            }
        )+
    };
}

from_primitive!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, u128, i128);

macro_rules! try_from_primitive {
    ($($type:ty), +) => {
        $(
            impl TryFrom<$type> for BonusValue {
                type Error = rust_decimal::Error;
                fn try_from(value: $type) -> Result<Self, Self::Error> {
                    Ok(Self::Const(Decimal::try_from(value)?))
                }
            }
        )+
    }
}

try_from_primitive!(f32, f64);

impl Add for BonusValue {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Const(left), Self::Const(right)) => Self::Const(left + right),
            (left, right) => Self::Add(Box::new(left), Box::new(right)),
        }
    }
}
impl Sub for BonusValue {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Const(left), Self::Const(right)) => Self::Const(left - right),
            (left, right) => Self::Sub(Box::new(left), Box::new(right)),
        }
    }
}

impl Mul for BonusValue {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Const(left), Self::Const(right)) => Self::Const(left * right),
            (left, right) => Self::Mul(Box::new(left), Box::new(right)),
        }
    }
}

impl Div for BonusValue {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Const(left), Self::Const(right)) => Self::Const(left / right),
            (left, right) => Self::Div(Box::new(left), Box::new(right)),
        }
    }
}

impl Rem for BonusValue {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Const(left), Self::Const(right)) => Self::Const(left % right),
            (left, right) => Self::Rem(Box::new(left), Box::new(right)),
        }
    }
}

impl Neg for BonusValue {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Const(cst) => Self::Const(cst.neg()),
            val => Self::Mul(Box::new(val), Box::new(Self::NEGATIVE_ONE)),
        }
    }
}

impl AddAssign for BonusValue {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone().add(rhs);
    }
}

impl SubAssign for BonusValue {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone().sub(rhs);
    }
}

impl MulAssign for BonusValue {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone().mul(rhs);
    }
}

impl DivAssign for BonusValue {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone().div(rhs);
    }
}

impl RemAssign for BonusValue {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.clone().rem(rhs);
    }
}

impl Sum for BonusValue {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::iter_sum(iter).unwrap_or(Self::ZERO)
    }
}

impl Product for BonusValue {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::iter_product(iter).unwrap_or(Self::ONE)
    }
}

impl ContainsAttribute for BonusValue {
    fn any_attribute<'a, F>(&'a self, fun: &F) -> bool
    where
        F: Fn(&'a Attribute) -> bool,
    {
        match self {
            Self::Const(_) => false,
            Self::Attribute(attr) => fun(attr),
            Self::Snapshot(_, bonus_value)
            | Self::Floor(bonus_value)
            | Self::Ceil(bonus_value)
            | Self::Round(bonus_value)
            | Self::Abs(bonus_value) => bonus_value.any_attribute(fun),
            Self::Min(bonus_value, bonus_value1)
            | Self::Max(bonus_value, bonus_value1)
            | Self::Add(bonus_value, bonus_value1)
            | Self::Sub(bonus_value, bonus_value1)
            | Self::Mul(bonus_value, bonus_value1)
            | Self::Div(bonus_value, bonus_value1)
            | Self::Rem(bonus_value, bonus_value1) => {
                bonus_value.any_attribute(fun) || bonus_value1.any_attribute(fun)
            }
            Self::If {
                condition,
                if_true,
                if_false,
            } => {
                condition.any_attribute(fun)
                    || if_true.any_attribute(fun)
                    || if_false.any_attribute(fun)
            }
            Self::Dice { count, size } => count.any_attribute(fun) || size.any_attribute(fun),
        }
    }
}
