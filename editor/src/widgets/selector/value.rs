use builder::{
    attribute::Attribute,
    bonus::{Condition, ToValue, Value},
};
use core::fmt::{Display, Formatter, Result};
use rust_decimal::Decimal;

use self::types::ValueType;

use super::{attribute::AttributeSelector, condition::ConditionSelector, SelectorWidgetMessage};

pub mod message;
pub mod types;
pub mod ui;

#[derive(Debug, Clone)]
pub struct ValueSelector {
    depth: usize,
    selector: Option<ValueSubSelector>,
    on_submit: SelectorWidgetMessage,
    on_cancel: SelectorWidgetMessage,
    val: ValueType,
    constant: Option<Decimal>,
    constant_string: String,
    value_a: Option<Value>,
    value_b: Option<Value>,
    condition: Option<Condition>,
    attribute: Option<Attribute>,
}

impl ValueSelector {
    pub fn new<'v, V>(
        depth: usize,
        value: V,
        on_submit: SelectorWidgetMessage,
        on_cancel: SelectorWidgetMessage,
    ) -> Self
    where
        V: Into<Option<&'v Value>>,
    {
        let value: Option<&'v Value> = value.into();

        let (val, value_a, value_b, condition, attribute, constant) =
            match value.unwrap_or(&Value::ZERO) {
                Value::Const(decimal) => (ValueType::Const, None, None, None, None, Some(*decimal)),
                Value::Attribute(attribute) => (
                    ValueType::Attribute,
                    None,
                    None,
                    None,
                    Some(attribute.clone()),
                    None,
                ),
                Value::Min(a, b) => (
                    ValueType::Min,
                    Some(*a.clone()),
                    Some(*b.clone()),
                    None,
                    None,
                    None,
                ),
                Value::Max(a, b) => (
                    ValueType::Max,
                    Some(*a.clone()),
                    Some(*b.clone()),
                    None,
                    None,
                    None,
                ),
                Value::Floor(val) => (ValueType::Floor, Some(*val.clone()), None, None, None, None),
                Value::Ceil(val) => (ValueType::Ceil, Some(*val.clone()), None, None, None, None),
                Value::Round(val) => (ValueType::Round, Some(*val.clone()), None, None, None, None),
                Value::Abs(val) => (ValueType::Abs, Some(*val.clone()), None, None, None, None),
                Value::Add(a, b) => (
                    ValueType::Add,
                    Some(*a.clone()),
                    Some(*b.clone()),
                    None,
                    None,
                    None,
                ),
                Value::Sub(a, b) => (
                    ValueType::Sub,
                    Some(*a.clone()),
                    Some(*b.clone()),
                    None,
                    None,
                    None,
                ),
                Value::Mul(a, b) => (
                    ValueType::Mul,
                    Some(*a.clone()),
                    Some(*b.clone()),
                    None,
                    None,
                    None,
                ),
                Value::Div(a, b) => (
                    ValueType::Div,
                    Some(*a.clone()),
                    Some(*b.clone()),
                    None,
                    None,
                    None,
                ),
                Value::Rem(a, b) => (
                    ValueType::Rem,
                    Some(*a.clone()),
                    Some(*b.clone()),
                    None,
                    None,
                    None,
                ),
                Value::If {
                    condition,
                    if_true,
                    if_false,
                } => (
                    ValueType::If,
                    Some(*if_true.clone()),
                    Some(*if_false.clone()),
                    Some(*condition.clone()),
                    None,
                    None,
                ),
                Value::Dice { count, size } => (
                    ValueType::Dice,
                    Some(*count.clone()),
                    Some(*size.clone()),
                    None,
                    None,
                    None,
                ),
            };

        Self {
            val,
            value_a,
            value_b,
            condition,
            on_submit,
            on_cancel,
            constant,
            constant_string: constant.map_or_else(String::new, |d| d.to_string()),
            attribute,
            depth,
            selector: None,
        }
    }

    pub fn get_value(&self) -> Option<Value> {
        Some(match &self.val {
            ValueType::Const => Value::from(self.constant?),
            ValueType::Attribute => self.attribute.clone()?.to_value(),
            ValueType::Min => self.value_a.clone()?.min(self.value_b.clone()?),
            ValueType::Max => self.value_a.clone()?.max(self.value_b.clone()?),
            ValueType::Floor => self.value_a.clone()?.floor(),
            ValueType::Ceil => self.value_a.clone()?.ceil(),
            ValueType::Round => self.value_a.clone()?.round(),
            ValueType::Abs => self.value_a.clone()?.abs(),
            ValueType::Add => self.value_a.clone()? + self.value_b.clone()?,
            ValueType::Sub => self.value_a.clone()? - self.value_b.clone()?,
            ValueType::Mul => self.value_a.clone()? * self.value_b.clone()?,
            ValueType::Div => self.value_a.clone()? / self.value_b.clone()?,
            ValueType::Rem => self.value_a.clone()? % self.value_b.clone()?,
            ValueType::If => Value::condition(
                self.condition.clone()?,
                self.value_a.clone()?,
                self.value_b.clone()?,
            ),
            ValueType::Dice => Value::dice(self.value_a.clone()?, self.value_b.clone()?),
        })
    }
}

impl Display for ValueSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ", self.val)?;
        if let Some(selector) = &self.selector {
            write!(f, "{selector}")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueSubSelector {
    ValueA(Box<ValueSelector>),
    ValueB(Box<ValueSelector>),
    Condition(Box<ConditionSelector>),
    Attribute(Box<AttributeSelector>),
}

impl Display for ValueSubSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Self::ValueA(selector) => write!(f, "> Value A {selector}"),
            Self::ValueB(selector) => write!(f, "> Value B {selector}"),
            Self::Condition(selector) => write!(f, "> Condition {selector}"),
            Self::Attribute(_) => write!(f, "> Attribute"),
        }
    }
}
