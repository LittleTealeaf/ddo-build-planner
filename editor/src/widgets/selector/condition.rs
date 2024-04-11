use core::fmt::{Display, Formatter, Result};

use builder::bonus::{Condition, Value};
use iced::{
    alignment::{Horizontal, Vertical},
    theme,
    widget::{button, column, horizontal_space, row, text, vertical_space, Column},
    Application, Command, Element, Renderer,
};
use ui::{HandleMessage, HandleView};

use crate::{App, Message};

use super::{
    value::ValueSelector, SelectorInternalMessage, SelectorMessage, SelectorWidgetMessage,
};

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
    /// Do we need this or can we remove this?
    dropdown: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

impl ConditionType {
    const TYPES: [Self; 9] = [
        Self::Not,
        Self::GreaterThan,
        Self::LessThan,
        Self::EqualTo,
        Self::True,
        Self::False,
        Self::And,
        Self::Or,
        Self::Xor,
    ];
}

impl ConditionType {
    const fn show_condition_a(self) -> bool {
        matches!(self, Self::Not | Self::And | Self::Or | Self::Xor)
    }

    const fn show_condition_b(self) -> bool {
        matches!(self, Self::And | Self::Or | Self::Xor)
    }

    const fn show_value_a(self) -> bool {
        matches!(self, Self::GreaterThan | Self::LessThan | Self::EqualTo)
    }

    const fn show_value_b(self) -> bool {
        matches!(self, Self::GreaterThan | Self::LessThan | Self::EqualTo)
    }
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
            dropdown: false,
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
    SetDropdown(bool),
}

impl ConditionSelectorMessage {
    const fn into_widget_message(self, depth: usize) -> SelectorWidgetMessage {
        SelectorWidgetMessage::Selector(depth, SelectorMessage::Condition(self))
    }

    const fn into_message(self, depth: usize) -> Message {
        Message::Selector(self.into_widget_message(depth))
    }
}

impl<'a> HandleMessage<SelectorInternalMessage<'a>, App> for ConditionSelector {
    fn handle_message(
        &mut self,
        message: SelectorInternalMessage<'a>,
    ) -> Command<<App as Application>::Message> {
        if message.depth == self.depth {
            match message.content {
                SelectorMessage::Condition(m) => match m {
                    ConditionSelectorMessage::SetDropdown(dropdown) => {
                        self.dropdown = dropdown;
                        Command::none()
                    }
                    ConditionSelectorMessage::SetType(cond) => {
                        self.cond = cond;
                        self.dropdown = false;
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
                        self.selector =
                            Some(ConditionSubSelector::ValueA(Box::new(ValueSelector::new(
                                self.depth + 1,
                                self.value_a.as_ref(),
                                ConditionSelectorMessage::SubmitSubSelector
                                    .into_widget_message(self.depth),
                                ConditionSelectorMessage::CancelSubSelector
                                    .into_widget_message(self.depth),
                            ))));
                        Command::none()
                    }
                    ConditionSelectorMessage::EditValueB => {
                        self.selector =
                            Some(ConditionSubSelector::ValueB(Box::new(ValueSelector::new(
                                self.depth + 1,
                                self.value_b.as_ref(),
                                ConditionSelectorMessage::SubmitSubSelector
                                    .into_widget_message(self.depth),
                                ConditionSelectorMessage::CancelSubSelector
                                    .into_widget_message(self.depth),
                            ))));
                        Command::none()
                    }
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
                        selector.handle_message(message)
                    }
                    ConditionSubSelector::ValueA(selector)
                    | ConditionSubSelector::ValueB(selector) => selector.handle_message(message),
                })
        }
    }
}

impl HandleView<App> for ConditionSelector {
    fn handle_view<'a>(
        &'a self,
        app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        self.selector.as_ref().map_or_else(
            || {
                column!(
                    row!(
                        column(ConditionType::TYPES.map(|condition| {
                            button(
                                text(format!("{condition}"))
                                    .vertical_alignment(Vertical::Center)
                                    .horizontal_alignment(Horizontal::Center),
                            )
                            .on_press(
                                ConditionSelectorMessage::SetType(condition)
                                    .into_message(self.depth),
                            )
                            .style(if condition == self.cond {
                                theme::Button::Primary
                            } else {
                                theme::Button::Text
                            })
                            .into()
                        })),
                        Column::new()
                            .push_maybe(self.cond.show_value_a().then(|| {
                                row!(
                                    button(text("Value A")).on_press(
                                        ConditionSelectorMessage::EditValueA
                                            .into_message(self.depth)
                                    ),
                                    self.value_a
                                        .as_ref()
                                        .map_or_else(|| text("None Selected"), text)
                                )
                            }))
                            .push_maybe(self.cond.show_value_b().then(|| {
                                row!(
                                    button(text("Value B")).on_press(
                                        ConditionSelectorMessage::EditValueB
                                            .into_message(self.depth)
                                    ),
                                    self.value_b
                                        .as_ref()
                                        .map_or_else(|| text("None Selected"), text)
                                )
                            }))
                            .push_maybe(self.cond.show_condition_a().then(|| {
                                row!(
                                    button(text("Condition A")).on_press(
                                        ConditionSelectorMessage::EditConditionA
                                            .into_message(self.depth)
                                    ),
                                    self.condition_a
                                        .as_ref()
                                        .map_or_else(|| text("None Selected"), text)
                                )
                            }))
                            .push_maybe(self.cond.show_condition_b().then(|| {
                                row!(
                                    button(text("Condition B")).on_press(
                                        ConditionSelectorMessage::EditConditionB
                                            .into_message(self.depth)
                                    ),
                                    self.condition_b
                                        .as_ref()
                                        .map_or_else(|| text("None Selected"), text)
                                )
                            }))
                    ),
                    vertical_space(),
                    row!(
                        horizontal_space(),
                        button(text("Cancel"))
                            .style(theme::Button::Secondary)
                            .on_press(Message::Selector(self.on_cancel.clone())),
                        button(text("Submit"))
                            .style(theme::Button::Primary)
                            .on_press(Message::Selector(self.on_submit.clone())),
                    )
                )
                .into()
            },
            |selector| match selector {
                ConditionSubSelector::ConditionA(selector)
                | ConditionSubSelector::ConditionB(selector) => selector.handle_view(app),
                ConditionSubSelector::ValueA(selector) | ConditionSubSelector::ValueB(selector) => {
                    selector.handle_view(app)
                }
            },
        )
    }
}
