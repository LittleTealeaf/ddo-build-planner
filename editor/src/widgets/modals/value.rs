use builder::{
    attribute::Attribute,
    bonus::{Condition, Value},
};
use iced::Command;
use rust_decimal::Decimal;

use crate::Message;

use super::{
    attribute::{AttributeSelector, MAttributeSelector},
    condition::{ConditionSelector, MConditionSelector},
};

#[derive(Clone, Debug)]
pub struct ValueSelector {
    selected_type: ValueType,
    constant: Option<Decimal>,
    attribute: Option<Attribute>,
    val_a: Option<Value>,
    val_b: Option<Value>,
    condition: Option<Condition>,
    selector: EditSelector,
}

#[derive(Clone, Debug, Copy)]
pub enum ValueType {
    Const,
    Attribute,
    Min,
    Max,
    Floor,
    Ceil,
    Round,
    Abs,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    If,
    Dice,
}

impl ValueSelector {
    pub fn value(&self) -> Option<Value> {
        Some(match self.selected_type {
            ValueType::Const => Value::Const(self.constant?),
            ValueType::Attribute => Value::Attribute(self.attribute.clone()?),
            ValueType::Min => self.val_a.clone()?.min(self.val_b.clone()?),
            ValueType::Max => self.val_a.clone()?.max(self.val_b.clone()?),
            ValueType::Floor => self.val_a.clone()?.floor(),
            ValueType::Ceil => self.val_a.clone()?.ceil(),
            ValueType::Round => self.val_a.clone()?.round(),
            ValueType::Abs => self.val_a.clone()?.abs(),
            ValueType::Add => self.val_a.clone()? + self.val_b.clone()?,
            ValueType::Sub => self.val_a.clone()? - self.val_b.clone()?,
            ValueType::Mul => self.val_a.clone()? * self.val_b.clone()?,
            ValueType::Div => self.val_a.clone()? / self.val_b.clone()?,
            ValueType::Rem => self.val_a.clone()? % self.val_b.clone()?,
            ValueType::If => Value::condition(
                self.condition.clone()?,
                self.val_a.clone()?,
                self.val_b.clone()?,
            ),
            ValueType::Dice => Value::dice(self.val_a.clone()?, self.val_b.clone()?),
        })
    }
}

#[derive(Clone, Debug)]
enum EditSelector {
    None,
    Attr(Box<AttributeSelector>),
    ValA(Box<ValueSelector>),
    ValB(Box<ValueSelector>),
    Condition(Box<ConditionSelector>),
}

#[derive(Clone, Debug)]
pub enum MValueSelector {
    SelectValue(Box<MValueSelector>),
    SelectCondition(Box<MConditionSelector>),
    SelectAttribute(Box<MAttributeSelector>),
    SubmitSelector,
}

impl ValueSelector {
    pub fn message(&mut self, message: MValueSelector) -> Command<Message> {
        match message {
            MValueSelector::SelectValue(m) => {
                if let EditSelector::ValA(sel) | EditSelector::ValB(sel) = &mut self.selector {
                    sel.message(*m)
                } else {
                    Command::none()
                }
            }
            MValueSelector::SelectCondition(_) => todo!(),
            MValueSelector::SelectAttribute(m) => {
                if let EditSelector::Attr(sel) = &mut self.selector {
                    sel.message(*m)
                } else {
                    Command::none()
                }
            }
            MValueSelector::SubmitSelector => match &self.selector {
                EditSelector::Attr(_) => todo!(),
                EditSelector::ValA(sel) => {
                    if let Some(val) = sel.value() {
                        self.val_a = Some(val);
                    }
                    Command::none()
                }
                EditSelector::ValB(sel) => {
                    if let Some(val) = sel.value() {
                        self.val_b = Some(val);
                    }
                    Command::none()
                }
                _ => Command::none(),
            },
        }
    }
}
