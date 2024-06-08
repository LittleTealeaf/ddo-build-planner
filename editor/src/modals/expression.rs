use core::{convert::Into, str::FromStr};
use std::collections::HashMap;

use builder::{
    attribute::Attribute,
    bonus::{Condition, Value},
};
use condition::ConditionType;
use iced::{Application, Command};
use rust_decimal::Decimal;
use ui::HandleMessage;
use utils::from_into::FromInto;
use value::ValueType;

use crate::{App, Message};

mod condition;
mod value;

#[derive(Debug, Clone)]
pub struct ModalExpression {
    title: Option<String>,
    cached_value: Option<Value>,
    cached_condition: Option<Condition>,
    selectors: HashMap<usize, InternalSelector>,
    counter: usize,
    base: usize,
    on_submit: Option<Message>,
    on_cancel: Option<Message>,
}

impl ModalExpression {
    pub fn value<V>(value: V) -> Self
    where
        V: Into<Option<Value>>,
    {
        let value = Option::<Value>::from_into(value);
        let mut selector = Self::new();
        selector.cached_value.clone_from(&value);
        selector.base = selector.add_selector_value(value);
        selector
    }

    pub fn condition<C>(condition: C) -> Self
    where
        C: Into<Option<Condition>>,
    {
        let condition = Option::<Condition>::from_into(condition);
        let mut selector = Self::new();
        selector.cached_condition.clone_from(&condition);
        selector.base = selector.add_selector_condition(condition);
        selector
    }

    pub fn on_cancel<M>(self, message: M) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_cancel: Some(message.into()),
            ..self
        }
    }

    pub fn on_cancel_maybe<M>(self, message: Option<M>) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_cancel: message.map(Into::into),
            ..self
        }
    }

    pub fn on_submit<M>(self, message: M) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_submit: Some(message.into()),
            ..self
        }
    }

    pub fn on_submit_maybe<M>(self, message: Option<M>) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_submit: message.map(Into::into),
            ..self
        }
    }

    pub fn title<S>(self, title: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    pub fn title_maybe<S>(self, title: Option<S>) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: title.map(Into::into),
            ..self
        }
    }

    pub fn get_condition(&self) -> Option<Condition> {
        self.cached_condition.clone()
    }

    pub fn get_value(&self) -> Option<Value> {
        self.cached_value.clone()
    }
}

impl ModalExpression {
    fn new() -> Self {
        Self {
            base: 0,
            cached_value: None,
            cached_condition: None,
            title: None,
            selectors: HashMap::new(),
            counter: 0,
            on_submit: None,
            on_cancel: None,
        }
    }

    fn update_cached(&mut self) {
        self.cached_condition = self.get_internal_condition(self.base);
        self.cached_value = self.get_internal_value(self.base);
    }

    fn add_selector(&mut self, selector: InternalSelector) -> usize {
        let index = self.counter;
        self.selectors.insert(index, selector);
        self.counter += 1;
        index
    }

    fn add_selector_value<V>(&mut self, value: V) -> usize
    where
        V: Into<Option<Value>>,
    {
        let child = InternalSelector::value(self, value.into());
        self.add_selector(child)
    }

    fn add_selector_condition<C>(&mut self, condition: C) -> usize
    where
        C: Into<Option<Condition>>,
    {
        let child = InternalSelector::condition(self, condition.into());
        self.add_selector(child)
    }

    fn handle_change_type(&mut self, id: usize, selector_type: ModalExpressionType) {
        let (val_a, val_b, cond_a, cond_b) = match selector_type {
            ModalExpressionType::Value(v) => match v {
                ValueType::Add
                | ValueType::Sub
                | ValueType::Mul
                | ValueType::Div
                | ValueType::Rem
                | ValueType::Dice
                | ValueType::Max
                | ValueType::Min => (true, true, false, false),
                ValueType::Ceil | ValueType::Round | ValueType::Abs | ValueType::Floor => {
                    (true, false, false, false)
                }
                ValueType::If => (true, true, true, false),
                _ => (false, false, false, false),
            },
            ModalExpressionType::Condition(c) => match c {
                ConditionType::Not => (false, false, true, false),
                ConditionType::GreaterThan | ConditionType::LessThan | ConditionType::EqualTo => {
                    (true, true, false, false)
                }
                ConditionType::True | ConditionType::False => (false, false, false, false),
                ConditionType::And | ConditionType::Or | ConditionType::Xor => {
                    (false, false, true, true)
                }
            },
        };

        let Some(mut selector) = self.selectors.remove(&id) else {
            return;
        };

        selector.value_a = match (val_a, selector.value_a) {
            (true, None) => Some(self.add_selector_value(Value::ZERO)),
            (false, Some(index)) => {
                self.selectors.remove(&index);
                None
            }
            (_, val) => val,
        };

        selector.value_b = match (val_b, selector.value_b) {
            (true, None) => Some(self.add_selector_value(Value::ZERO)),
            (false, Some(index)) => {
                self.selectors.remove(&index);
                None
            }
            (_, val) => val,
        };

        selector.condition_a = match (cond_a, selector.condition_a) {
            (true, None) => Some(self.add_selector_condition(Condition::TRUE)),
            (false, Some(index)) => {
                self.selectors.remove(&index);
                None
            }
            (_, val) => val,
        };

        selector.condition_b = match (cond_b, selector.condition_b) {
            (true, None) => Some(self.add_selector_condition(Condition::TRUE)),
            (false, Some(index)) => {
                self.selectors.remove(&index);
                None
            }
            (_, val) => val,
        };

        self.selectors.insert(id, selector);
    }
}

#[derive(Debug, Clone)]
pub enum ModalExpressionMessage {
    Message(usize, ModalExpressionInternalMessage),
    Submit,
    Cancel,
}

impl From<ModalExpressionMessage> for Message {
    fn from(value: ModalExpressionMessage) -> Self {
        Self::ExpressionSelector(value)
    }
}

#[derive(Debug, Clone)]
struct InternalSelector {
    value_a: Option<usize>,
    value_b: Option<usize>,
    condition_a: Option<usize>,
    condition_b: Option<usize>,
    attribute: Option<Attribute>,
    constant: Option<Decimal>,
    constant_str: String,
    selector_type: ModalExpressionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalExpressionType {
    Value(ValueType),
    Condition(ConditionType),
}

#[derive(Debug, Clone)]
pub enum ModalExpressionInternalMessage {
    SetType(ModalExpressionType),
    SelectAttribute,
    OnAttributeSelected,
    ConstInput(String),
}

impl From<(usize, ModalExpressionInternalMessage)> for Message {
    fn from((id, message): (usize, ModalExpressionInternalMessage)) -> Self {
        ModalExpressionMessage::Message(id, message).into()
    }
}

impl HandleMessage<ModalExpressionMessage> for App {
    fn handle_message(
        &mut self,
        message: ModalExpressionMessage,
    ) -> Command<<Self as Application>::Message> {
        let Some(modal) = &mut self.expression_selector else {
            return Command::none();
        };

        match message {
            ModalExpressionMessage::Submit => {
                let command = modal
                    .on_submit
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.expression_selector = None;
                command
            }
            ModalExpressionMessage::Cancel => {
                let command = modal
                    .on_cancel
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.expression_selector = None;
                command
            }
            ModalExpressionMessage::Message(id, message) => match message {
                ModalExpressionInternalMessage::SetType(selector_type) => {
                    if let Some(selector) = modal.selectors.get(&id) {
                        if selector.selector_type != selector_type {
                            modal.handle_change_type(id, selector_type);
                        }
                    }

                    Command::none()
                }
                ModalExpressionInternalMessage::SelectAttribute => {
                    self.attribute_selector = Some(
                        self.select_attribute()
                            .title("Select Attribute")
                            .on_submit((id, ModalExpressionInternalMessage::OnAttributeSelected)),
                    );
                    Command::none()
                }
                ModalExpressionInternalMessage::OnAttributeSelected => {
                    if let (Some(selector), Some(attribute_selector)) =
                        (modal.selectors.get_mut(&id), &self.attribute_selector)
                    {
                        selector.attribute = attribute_selector.get_attribute();
                        modal.update_cached();
                    }

                    Command::none()
                }

                ModalExpressionInternalMessage::ConstInput(input) => {
                    if let Some(selector) = modal.selectors.get_mut(&id) {
                        selector.constant_str = input;
                        selector.constant = Decimal::from_str(&selector.constant_str).ok();
                        modal.update_cached();
                    }

                    Command::none()
                }
            },
        }
    }
}
