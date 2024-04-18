use core::fmt::{Display, Formatter, Result};

use builder::bonus::{Condition, Value};

use self::types::ConditionType;

use super::{value::ValueSelector, SelectorWidgetMessage};

pub mod message;
pub mod types;
pub mod ui;

#[derive(Debug, Clone)]
pub struct ConditionSelector {
    depth: usize,
    selector: Option<ConditionSubSelector>,
    on_submit: SelectorWidgetMessage,
    on_cancel: SelectorWidgetMessage,
    cond: ConditionType,
    condition_a: Option<Condition>,
    condition_b: Option<Condition>,
    value_a: Option<Value>,
    value_b: Option<Value>,
}

impl Display for ConditionSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.cond)?;
        self.selector
            .as_ref()
            .map_or(Ok(()), |selector| write!(f, "{selector}"))
    }
}

impl ConditionSelector {
    pub fn new(
        depth: usize,
        value: Option<&Condition>,
        on_submit: SelectorWidgetMessage,
        on_cancel: SelectorWidgetMessage,
    ) -> Self {
        let (cond, value_a, value_b, condition_a, condition_b) = match value {
            Some(Condition::Not(value)) => {
                (ConditionType::Not, None, None, Some(*value.clone()), None)
            }
            Some(Condition::GreaterThan(a, b)) => (
                ConditionType::GreaterThan,
                Some(a.clone()),
                Some(b.clone()),
                None,
                None,
            ),
            Some(Condition::LessThan(a, b)) => (
                ConditionType::LessThan,
                Some(a.clone()),
                Some(b.clone()),
                None,
                None,
            ),
            Some(Condition::EqualTo(a, b)) => (
                ConditionType::EqualTo,
                Some(a.clone()),
                Some(b.clone()),
                None,
                None,
            ),
            Some(&Condition::FALSE) => (ConditionType::False, None, None, None, None),
            Some(Condition::And(a, b)) => (
                ConditionType::And,
                None,
                None,
                Some(*a.clone()),
                Some(*b.clone()),
            ),
            Some(Condition::Or(a, b)) => (
                ConditionType::Or,
                None,
                None,
                Some(*a.clone()),
                Some(*b.clone()),
            ),
            Some(Condition::Xor(a, b)) => (
                ConditionType::Xor,
                None,
                None,
                Some(*a.clone()),
                Some(*b.clone()),
            ),
            _ => (ConditionType::True, None, None, None, None),
        };

        Self {
            depth,
            on_submit,
            on_cancel,
            selector: None,
            value_a,
            value_b,
            condition_a,
            condition_b,
            cond,
        }
    }

    pub fn get_condition(&self) -> Option<Condition> {
        Some(match self.cond {
            ConditionType::Not => !(self.condition_a.clone()?),
            ConditionType::GreaterThan => self.value_a.clone()?.greater_than(self.value_b.clone()?),
            ConditionType::LessThan => self.value_a.clone()?.less_than(self.value_b.clone()?),
            ConditionType::EqualTo => self.value_a.clone()?.equal_to(self.value_b.clone()?),
            ConditionType::True => Condition::TRUE,
            ConditionType::False => Condition::FALSE,
            ConditionType::And => self.condition_a.clone()? & self.condition_b.clone()?,
            ConditionType::Or => self.condition_a.clone()? | self.condition_b.clone()?,
            ConditionType::Xor => self.condition_a.clone()? ^ self.condition_b.clone()?,
        })
    }
}


#[derive(Debug, Clone)]
pub enum ConditionSubSelector {
    ConditionA(Box<ConditionSelector>),
    ConditionB(Box<ConditionSelector>),
    ValueA(Box<ValueSelector>),
    ValueB(Box<ValueSelector>),
}

impl Display for ConditionSubSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::ConditionA(selector) => write!(f, "> Condition A {selector}"),
            Self::ConditionB(selector) => write!(f, "> Condition B {selector}"),
            Self::ValueA(selector) => write!(f, "> Value A {selector}"),
            Self::ValueB(selector) => write!(f, "> Value B {selector}"),
        }
    }
}
