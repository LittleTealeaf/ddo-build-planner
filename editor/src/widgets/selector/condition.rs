use core::fmt::{Display, Formatter, Result};

use builder::{
    attribute::Attribute,
    bonus::{Condition, Value},
};
use iced::{Application, Command};
use ui::HandleMessage;

use crate::Editor;

use super::{value::ValueSelector, SelectorMessage, SelectorWidgetMessage};

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

#[derive(Debug, Clone)]
pub enum ConditionType {
    Not,
    GreaterThan,
    LessThan,
    EqualTo,
    True,
    False,
    And,
    Or,
    Xor,
}

impl Display for ConditionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Not => write!(f, "Not"),
            Self::GreaterThan => write!(f, "Greater Than"),
            Self::LessThan => write!(f, "Less Than"),
            Self::EqualTo => write!(f, "Equal To"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Xor => write!(f, "Xor"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConditionSubSelector {
    ConditionA(Box<ConditionSelector>),
    ConditionB(Box<ConditionSelector>),
    ValueA(Box<ValueSelector>),
    ValueB(Box<ValueSelector>),
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
pub enum ConditionSelectorMessage {
    SetType(ConditionType),
    SubmitSubSelector,
    CancelSubSelector,
    EditValueA,
    EditValueB,
    EditConditionA,
    EditConditionB,
}

impl ConditionSelectorMessage {
    const fn into_widget_message(self, depth: usize) -> SelectorWidgetMessage {
        SelectorWidgetMessage::Selector(depth, SelectorMessage::Condition(self))
    }
}

impl HandleMessage<(usize, SelectorMessage, &[Attribute]), Editor> for ConditionSelector {
    fn handle_message(
        &mut self,
        (depth, message, attributes): (usize, SelectorMessage, &[Attribute]),
    ) -> Command<<Editor as Application>::Message> {
        if depth == self.depth {
            match message {
                SelectorMessage::Condition(message) => match message {
                    ConditionSelectorMessage::SetType(cond) => {
                        self.cond = cond;
                        Command::none()
                    }
                    ConditionSelectorMessage::SubmitSubSelector => {
                        if let Some(selector) = &self.selector {
                            match selector {
                                ConditionSubSelector::ConditionA(selector) => {
                                    self.condition_a = selector.get_condition();
                                }
                                ConditionSubSelector::ConditionB(selector) => {
                                    self.condition_b = selector.get_condition();
                                }
                                ConditionSubSelector::ValueA(_) => todo!(),
                                ConditionSubSelector::ValueB(_) => todo!(),
                            }
                        }
                        self.selector = None;
                        Command::none()
                    }
                    ConditionSelectorMessage::EditValueA => todo!("Value Selector"),
                    ConditionSelectorMessage::EditValueB => todo!("Value Selector"),
                    ConditionSelectorMessage::EditConditionA => {
                        self.selector =
                            Some(ConditionSubSelector::ConditionA(Box::new(Self::new(
                                self.depth + 1,
                                self.condition_a.as_ref(),
                                ConditionSelectorMessage::SubmitSubSelector
                                    .into_widget_message(self.depth),
                                ConditionSelectorMessage::CancelSubSelector
                                    .into_widget_message(self.depth),
                            ))));
                        Command::none()
                    }
                    ConditionSelectorMessage::EditConditionB => {
                        self.selector =
                            Some(ConditionSubSelector::ConditionB(Box::new(Self::new(
                                self.depth + 1,
                                self.condition_b.as_ref(),
                                ConditionSelectorMessage::SubmitSubSelector
                                    .into_widget_message(self.depth),
                                ConditionSelectorMessage::CancelSubSelector
                                    .into_widget_message(self.depth),
                            ))));
                        Command::none()
                    }
                    ConditionSelectorMessage::CancelSubSelector => {
                        self.selector = None;
                        Command::none()
                    }
                },
                _ => Command::none(),
            }
        } else {
            self.selector
                .as_mut()
                .map_or_else(Command::none, |selector| match selector {
                    ConditionSubSelector::ConditionA(selector)
                    | ConditionSubSelector::ConditionB(selector) => {
                        selector.handle_message((depth, message, attributes))
                    }
                    ConditionSubSelector::ValueA(_) => todo!("Value Selector Handle Message"),
                    ConditionSubSelector::ValueB(_) => todo!("Value Selector Handle Message"),
                })
        }
    }
}
