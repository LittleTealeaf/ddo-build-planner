use builder::{
    attribute::Attribute,
    bonus::{Condition, ToValue, Value},
};
use core::fmt::{Display, Formatter, Result};
use core::str::FromStr;
use iced::{Application, Command, Element, Renderer};
use itertools::Itertools;
use rust_decimal::Decimal;
use ui::{HandleMessage, HandleView};

use crate::App;

use super::{
    attribute::AttributeSelector, condition::ConditionSelector, SelectorInternalMessage,
    SelectorMessage, SelectorWidgetMessage,
};

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

#[derive(Debug, Clone)]
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

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Const => write!(f, "Constant"),
            Self::Attribute => write!(f, "Attribute"),
            Self::Min => write!(f, "Min"),
            Self::Max => write!(f, "Max"),
            Self::Floor => write!(f, "Floor"),
            Self::Ceil => write!(f, "Ceil"),
            Self::Round => write!(f, "Round"),
            Self::Abs => write!(f, "Absolute"),
            Self::Add => write!(f, "Add"),
            Self::Sub => write!(f, "Subtract"),
            Self::Mul => write!(f, "Multiply"),
            Self::Rem => write!(f, "Remainter"),
            Self::If => write!(f, "If"),
            Self::Dice => write!(f, "Dice"),
            Self::Div => write!(f, "Divide"),
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

#[derive(Debug, Clone)]
pub enum ValueSelectorMessage {
    SetType(ValueType),
    SubmitSubSelector,
    CancelSubSelector,
    EditValueA,
    EditValueB,
    EditCondition,
    EditAttribute,
    UpdateDecimalString(String),
}

impl ValueSelectorMessage {
    const fn into_widget_message(self, depth: usize) -> SelectorWidgetMessage {
        SelectorWidgetMessage::Selector(depth, SelectorMessage::Value(self))
    }
}

impl<'a> HandleMessage<SelectorInternalMessage<'a>, App> for ValueSelector {
    fn handle_message(
        &mut self,
        message: SelectorInternalMessage<'a>,
    ) -> Command<<App as Application>::Message> {
        if message.depth == self.depth {
            match message.content {
                SelectorMessage::Value(m) => match m {
                    ValueSelectorMessage::SetType(val) => {
                        self.val = val;
                        Command::none()
                    }
                    ValueSelectorMessage::SubmitSubSelector => {
                        if let Some(selector) = &self.selector {
                            match selector {
                                ValueSubSelector::ValueA(selector) => {
                                    self.value_a = selector.get_value();
                                }
                                ValueSubSelector::ValueB(selector) => {
                                    self.value_b = selector.get_value();
                                }
                                ValueSubSelector::Condition(selector) => {
                                    self.condition = selector.get_condition();
                                }
                                ValueSubSelector::Attribute(selector) => {
                                    self.attribute =
                                        selector.get_attribute(message.attributes).cloned();
                                }
                            }
                            self.selector = None;
                        }
                        Command::none()
                    }
                    ValueSelectorMessage::CancelSubSelector => {
                        self.selector = None;
                        Command::none()
                    }
                    ValueSelectorMessage::EditValueA => {
                        self.selector = Some(ValueSubSelector::ValueA(Box::new(Self::new(
                            self.depth + 1,
                            self.value_a.as_ref(),
                            ValueSelectorMessage::SubmitSubSelector.into_widget_message(self.depth),
                            ValueSelectorMessage::CancelSubSelector.into_widget_message(self.depth),
                        ))));
                        Command::none()
                    }
                    ValueSelectorMessage::EditValueB => {
                        self.selector = Some(ValueSubSelector::ValueB(Box::new(Self::new(
                            self.depth + 1,
                            self.value_a.as_ref(),
                            ValueSelectorMessage::SubmitSubSelector.into_widget_message(self.depth),
                            ValueSelectorMessage::CancelSubSelector.into_widget_message(self.depth),
                        ))));
                        Command::none()
                    }
                    ValueSelectorMessage::EditCondition => {
                        self.selector = Some(ValueSubSelector::Condition(Box::new(
                            ConditionSelector::new(
                                self.depth + 1,
                                self.condition.as_ref(),
                                ValueSelectorMessage::SubmitSubSelector
                                    .into_widget_message(self.depth),
                                ValueSelectorMessage::CancelSubSelector
                                    .into_widget_message(self.depth),
                            ),
                        )));
                        Command::none()
                    }
                    ValueSelectorMessage::EditAttribute => {
                        self.selector = Some(ValueSubSelector::Attribute(Box::new(
                            AttributeSelector::new(
                                self.depth + 1,
                                self.attribute.as_ref().and_then(|a| {
                                    message
                                        .attributes
                                        .iter()
                                        .find_position(|b| a.eq(b))
                                        .map(|(i, _)| i)
                                }),
                                ValueSelectorMessage::SubmitSubSelector
                                    .into_widget_message(self.depth),
                                ValueSelectorMessage::CancelSubSelector
                                    .into_widget_message(self.depth),
                            ),
                        )));
                        Command::none()
                    }
                    ValueSelectorMessage::UpdateDecimalString(string) => {
                        self.constant_string = string;
                        self.constant = Decimal::from_str(&self.constant_string).ok();
                        Command::none()
                    }
                },
                _ => panic!("Invalid Value Type"),
            }
        } else {
            self.selector
                .as_mut()
                .map_or_else(Command::none, |selector| match selector {
                    ValueSubSelector::ValueA(selector) | ValueSubSelector::ValueB(selector) => {
                        selector.handle_message(message)
                    }
                    ValueSubSelector::Condition(selector) => selector.handle_message(message),
                    ValueSubSelector::Attribute(selector) => selector.handle_message(message),
                })
        }
    }
}

impl HandleView<App> for ValueSelector {
    fn handle_view<'a>(
        &'a self,
        app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        self.selector.as_ref().map_or_else(
            || todo!(),
            |selector| match selector {
                ValueSubSelector::Attribute(selector) => selector.handle_view(app),
                ValueSubSelector::Condition(selector) => selector.handle_view(app),
                ValueSubSelector::ValueA(selector) | ValueSubSelector::ValueB(selector) => {
                    selector.handle_view(app)
                }
            },
        )
    }
}
