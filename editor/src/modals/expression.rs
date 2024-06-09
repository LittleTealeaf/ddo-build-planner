use core::{convert::Into, str::FromStr};
use std::collections::HashMap;

use builder::{
    attribute::Attribute,
    bonus::{Condition, Value},
};
use condition::ConditionType;
use iced::{
    theme,
    widget::{
        button, column, container, horizontal_space, pick_list, row, scrollable, text, text_input,
    },
    Application, Command, Element, Length, Renderer,
};
use rust_decimal::Decimal;
use ui::{HandleMessage, HandleView};
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
        selector.update_cached();
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
        selector.update_cached();
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

        selector.selector_type = selector_type;

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
        let Some(modal) = &mut self.modal_expression else {
            return Command::none();
        };

        match message {
            ModalExpressionMessage::Submit => {
                let command = modal
                    .on_submit
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.modal_expression = None;
                command
            }
            ModalExpressionMessage::Cancel => {
                let command = modal
                    .on_cancel
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.modal_expression = None;
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
                    let attribute = modal
                        .selectors
                        .get(&id)
                        .and_then(|selector| selector.attribute.as_ref())
                        .cloned();

                    self.modal_attribute = Some(
                        self.select_attribute()
                            .title("Select Attribute")
                            .on_submit((id, ModalExpressionInternalMessage::OnAttributeSelected))
                            .select_maybe(attribute),
                    );
                    Command::none()
                }
                ModalExpressionInternalMessage::OnAttributeSelected => {
                    if let (Some(selector), Some(attribute_selector)) =
                        (modal.selectors.get_mut(&id), &self.modal_attribute)
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

impl ModalExpression {
    fn handle_internal_view(
        &self,
        id: usize,
    ) -> Option<Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer>>
    {
        let selector = self.selectors.get(&id)?;

        let cond_a = selector
            .condition_a
            .and_then(|id| self.handle_internal_view(id));
        let cond_b = selector
            .condition_b
            .and_then(|id| self.handle_internal_view(id));
        let val_a = selector
            .value_a
            .and_then(|id| self.handle_internal_view(id));
        let val_b = selector
            .value_b
            .and_then(|id| self.handle_internal_view(id));

        Some(
            match selector.selector_type {
                ModalExpressionType::Value(value_type) => container(column([
                    pick_list(ValueType::TYPES, Some(value_type), move |value_type| {
                        (
                            id,
                            ModalExpressionInternalMessage::SetType(ModalExpressionType::Value(
                                value_type,
                            )),
                        )
                            .into()
                    })
                    .into(),
                    row([
                        text("   ").into(),
                        match value_type {
                            ValueType::Const => container(
                                text_input("Constant", selector.constant_str.as_str()).on_input(
                                    move |string| {
                                        (id, ModalExpressionInternalMessage::ConstInput(string))
                                            .into()
                                    },
                                ),
                            ),
                            ValueType::Attribute => container(
                                button(selector.attribute.as_ref().map_or_else(
                                    || text("None"),
                                    |attribute| text(attribute.to_string()),
                                ))
                                .on_press(
                                    (id, ModalExpressionInternalMessage::SelectAttribute).into(),
                                ),
                            ),
                            ValueType::Min => {
                                container(column([text("Min").into(), val_a?, val_b?]))
                            }
                            ValueType::Max => {
                                container(column([text("Max").into(), val_a?, val_b?]))
                            }
                            ValueType::Floor => container(column([text("Floor").into(), val_a?])),
                            ValueType::Ceil => container(column([text("Ceil").into(), val_a?])),
                            ValueType::Round => container(column([text("Round").into(), val_a?])),
                            ValueType::Abs => container(column([text("Abs").into(), val_a?])),
                            ValueType::Add => {
                                container(column([text("Add").into(), val_a?, val_b?]))
                            }
                            ValueType::Sub => {
                                container(column([text("Subtract").into(), val_a?, val_b?]))
                            }
                            ValueType::Mul => {
                                container(column([text("Multiply").into(), val_a?, val_b?]))
                            }
                            ValueType::Div => {
                                container(column([text("Divide").into(), val_a?, val_b?]))
                            }
                            ValueType::Rem => {
                                container(column([text("Remainder").into(), val_a?, val_b?]))
                            }
                            ValueType::If => container(column([
                                text("If").into(),
                                cond_a?,
                                text("Then").into(),
                                val_a?,
                                text("Else").into(),
                                val_b?,
                            ])),
                            ValueType::Dice => container(column([
                                text("Count").into(),
                                val_a?,
                                text("Size").into(),
                                val_b?,
                            ])),
                        }
                        .into(),
                    ])
                    .into(),
                ])),
                ModalExpressionType::Condition(condition_type) => container(column!(
                    pick_list(
                        ConditionType::TYPES,
                        Some(condition_type),
                        move |condition_type| {
                            (
                                id,
                                ModalExpressionInternalMessage::SetType(
                                    ModalExpressionType::Condition(condition_type),
                                ),
                            )
                                .into()
                        }
                    ),
                    row!(
                        text("   "),
                        match condition_type {
                            ConditionType::Not => container(column!(text("Not"), cond_a?)),
                            ConditionType::GreaterThan =>
                                container(column!(val_a?, text("Greater Than"), val_b?)),
                            ConditionType::LessThan =>
                                container(column!(val_a?, text("Less Than"), val_b?)),
                            ConditionType::EqualTo =>
                                container(column!(val_a?, text("Equal To"), val_b?)),
                            ConditionType::True => container(text("True")),
                            ConditionType::False => container(text("False")),
                            ConditionType::And => container(column!(cond_a?, text("And"), cond_b?)),
                            ConditionType::Or => container(column!(cond_a?, text("Or"), cond_b?)),
                            ConditionType::Xor =>
                                container(column!(cond_a?, text("Exclusive Or"), cond_b?)),
                        }
                    )
                )),
            }
            .into(),
        )
    }
}

impl HandleView<App> for ModalExpression {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        column!(
            row!(
                text(
                    self.title
                        .as_ref()
                        .unwrap_or(&String::from("Configure Expression"))
                ),
                horizontal_space().width(Length::Fill),
                button("Cancel")
                    .on_press(ModalExpressionMessage::Cancel.into())
                    .style(theme::Button::Secondary),
                button("Submit")
                    .on_press_maybe(
                        (self.cached_value.is_some() || self.cached_condition.is_some())
                            .then_some(ModalExpressionMessage::Submit.into())
                    )
                    .style(theme::Button::Primary)
            ),
            scrollable(
                self.handle_internal_view(self.base)
                    .unwrap_or_else(|| text("No Base Expression Set").into())
            )
            .width(Length::Fill),
        )
        .into()
    }
}
