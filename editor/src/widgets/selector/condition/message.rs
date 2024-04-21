use iced::{Application, Command};
use ui::HandleMessage;

use crate::{
    widgets::selector::{
        value::ValueSelector, IntoSelectorMessage, SelectorInternalMessage, SelectorMessage,
        SelectorWidgetMessage,
    },
    App,
};

use super::{ConditionSelector, ConditionSubSelector, ConditionType};

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

impl IntoSelectorMessage for ConditionSelectorMessage {
    fn into_selector_message(self, depth: usize) -> SelectorWidgetMessage {
        SelectorWidgetMessage::Selector(depth, SelectorMessage::Condition(self))
    }
}

impl<'a> HandleMessage<SelectorInternalMessage<'a>, App> for ConditionSelector {
    fn handle_message(
        &mut self,
        message: SelectorInternalMessage<'a>,
    ) -> Command<<App as Application>::Message> {
        if message.depth != self.depth {
            let Some(selector) = &mut self.selector else {
                return Command::none();
            };

            return match selector {
                ConditionSubSelector::ConditionA(selector)
                | ConditionSubSelector::ConditionB(selector) => selector.handle_message(message),
                ConditionSubSelector::ValueA(selector) | ConditionSubSelector::ValueB(selector) => {
                    selector.handle_message(message)
                }
            };
        }

        let SelectorMessage::Condition(condition_message) = message.content else {
            return Command::none();
        };

        match condition_message {
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
                        ConditionSubSelector::ValueA(selector) => {
                            self.value_a = selector.get_value();
                        }
                        ConditionSubSelector::ValueB(selector) => {
                            self.value_b = selector.get_value();
                        }
                    }
                    self.selector = None;
                }
                Command::none()
            }
            ConditionSelectorMessage::EditValueA => {
                self.selector = Some(ConditionSubSelector::ValueA(Box::new(ValueSelector::new(
                    self.depth + 1,
                    self.value_a.as_ref(),
                    ConditionSelectorMessage::SubmitSubSelector.into_selector_message(self.depth),
                    ConditionSelectorMessage::CancelSubSelector.into_selector_message(self.depth),
                ))));
                Command::none()
            }
            ConditionSelectorMessage::EditValueB => {
                self.selector = Some(ConditionSubSelector::ValueB(Box::new(ValueSelector::new(
                    self.depth + 1,
                    self.value_b.as_ref(),
                    ConditionSelectorMessage::SubmitSubSelector.into_selector_message(self.depth),
                    ConditionSelectorMessage::CancelSubSelector.into_selector_message(self.depth),
                ))));
                Command::none()
            }
            ConditionSelectorMessage::EditConditionA => {
                self.selector = Some(ConditionSubSelector::ConditionA(Box::new(Self::new(
                    self.depth + 1,
                    self.condition_a.as_ref(),
                    ConditionSelectorMessage::SubmitSubSelector.into_selector_message(self.depth),
                    ConditionSelectorMessage::CancelSubSelector.into_selector_message(self.depth),
                ))));
                Command::none()
            }
            ConditionSelectorMessage::EditConditionB => {
                self.selector = Some(ConditionSubSelector::ConditionB(Box::new(Self::new(
                    self.depth + 1,
                    self.condition_b.as_ref(),
                    ConditionSelectorMessage::SubmitSubSelector.into_selector_message(self.depth),
                    ConditionSelectorMessage::CancelSubSelector.into_selector_message(self.depth),
                ))));
                Command::none()
            }
            ConditionSelectorMessage::CancelSubSelector => {
                self.selector = None;
                Command::none()
            }
        }
    }
}
