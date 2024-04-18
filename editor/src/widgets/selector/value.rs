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

        let mut value_a = None;
        let mut value_b = None;
        let mut condition = None;
        let mut constant = None;
        let mut attribute = None;

        let val = match value.unwrap_or(&Value::ZERO) {
            Value::Const(val) => {
                constant = Some(*val);
                ValueType::Const
            }
            Value::Attribute(attr) => {
                attribute = Some(attr.clone());
                ValueType::Attribute
            }
            Value::Min(a, b) => {
                value_a = Some(*a.clone());
                value_b = Some(*b.clone());
                ValueType::Min
            }
            Value::Max(a, b) => {
                value_a = Some(*a.clone());
                value_b = Some(*b.clone());
                ValueType::Max
            }
            Value::Floor(val) => {
                value_a = Some(*val.clone());
                ValueType::Floor
            }
            Value::Ceil(val) => {
                value_a = Some(*val.clone());
                ValueType::Ceil
            }
            Value::Round(val) => {
                value_a = Some(*val.clone());
                ValueType::Round
            }
            Value::Abs(val) => {
                value_a = Some(*val.clone());
                ValueType::Abs
            }
            Value::Add(a, b) => {
                value_a = Some(*a.clone());
                value_b = Some(*b.clone());
                ValueType::Add
            }
            Value::Sub(a, b) => {
                value_a = Some(*a.clone());
                value_b = Some(*b.clone());
                ValueType::Sub
            }
            Value::Mul(a, b) => {
                value_a = Some(*a.clone());
                value_b = Some(*b.clone());
                ValueType::Mul
            }
            Value::Div(a, b) => {
                value_a = Some(*a.clone());
                value_b = Some(*b.clone());
                ValueType::Div
            }
            Value::Rem(a, b) => {
                value_a = Some(*a.clone());
                value_b = Some(*b.clone());
                ValueType::Rem
            }
            Value::If {
                condition: cond,
                if_true,
                if_false,
            } => {
                condition = Some(*cond.clone());
                value_a = Some(*if_true.clone());
                value_b = Some(*if_false.clone());
                ValueType::If
            }
            Value::Dice { count, size } => {
                value_a = Some(*count.clone());
                value_b = Some(*size.clone());
                ValueType::Dice
            }
        };

        let constant_string = constant.map_or_else(String::new, |d| d.to_string());
        let selector = None;

        Self {
            depth,
            selector,
            on_submit,
            on_cancel,
            val,
            constant,
            constant_string,
            value_a,
            value_b,
            condition,
            attribute,
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
        self.selector
            .as_ref()
            .map_or(Ok(()), |selector| write!(f, "{selector}"))
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
