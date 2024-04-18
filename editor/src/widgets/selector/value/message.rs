use core::str::FromStr;

use iced::{Application, Command};
use itertools::Itertools;
use rust_decimal::Decimal;
use ui::HandleMessage;

use crate::{
    widgets::selector::{
        attribute::AttributeSelector, condition::ConditionSelector, IntoSelectorMessage,
        SelectorInternalMessage, SelectorMessage, SelectorWidgetMessage,
    },
    App,
};

use super::{types::ValueType, ValueSelector, ValueSubSelector};

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

impl IntoSelectorMessage for ValueSelectorMessage {
    fn into_selector_message(self, depth: usize) -> SelectorWidgetMessage {
        SelectorWidgetMessage::Selector(depth, SelectorMessage::Value(self))
    }
}

impl<'a> HandleMessage<SelectorInternalMessage<'a>, App> for ValueSelector {
    fn handle_message(
        &mut self,
        message: SelectorInternalMessage<'a>,
    ) -> Command<<App as Application>::Message> {
        if message.depth != self.depth {
            let Some(selector) = &mut self.selector else {
                return Command::none();
            };

            return match selector {
                ValueSubSelector::ValueA(selector) | ValueSubSelector::ValueB(selector) => {
                    selector.handle_message(message)
                }
                ValueSubSelector::Condition(selector) => selector.handle_message(message),
                ValueSubSelector::Attribute(selector) => selector.handle_message(message),
            };
        }

        let SelectorMessage::Value(value_message) = message.content else {
            return Command::none();
        };

        match value_message {
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
                            self.attribute = selector.get_attribute(message.attributes).cloned();
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
                    ValueSelectorMessage::SubmitSubSelector.into_selector_message(self.depth),
                    ValueSelectorMessage::CancelSubSelector.into_selector_message(self.depth),
                ))));
                Command::none()
            }
            ValueSelectorMessage::EditValueB => {
                self.selector = Some(ValueSubSelector::ValueB(Box::new(Self::new(
                    self.depth + 1,
                    self.value_b.as_ref(),
                    ValueSelectorMessage::SubmitSubSelector.into_selector_message(self.depth),
                    ValueSelectorMessage::CancelSubSelector.into_selector_message(self.depth),
                ))));
                Command::none()
            }
            ValueSelectorMessage::EditCondition => {
                self.selector = Some(ValueSubSelector::Condition(Box::new(
                    ConditionSelector::new(
                        self.depth + 1,
                        self.condition.as_ref(),
                        ValueSelectorMessage::SubmitSubSelector.into_selector_message(self.depth),
                        ValueSelectorMessage::CancelSubSelector.into_selector_message(self.depth),
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
                        ValueSelectorMessage::SubmitSubSelector.into_selector_message(self.depth),
                        ValueSelectorMessage::CancelSubSelector.into_selector_message(self.depth),
                    ),
                )));
                Command::none()
            }
            ValueSelectorMessage::UpdateDecimalString(string) => {
                self.constant_string = string;
                self.constant = Decimal::from_str(&self.constant_string).ok();
                Command::none()
            }
        }
    }
}
