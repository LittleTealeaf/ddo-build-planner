use core::ops::Not;

use builder::{
    attribute::Attribute,
    bonus::{Condition, Value},
};
use iced::{
    theme,
    widget::{button, column, horizontal_space, pick_list, row, text},
    Command, Length,
};
use iced_aw::{card, modal};

use crate::Message;

use super::{
    value::{MValueSelector, ValueSelector},
    BonusSelectorTrait,
};

mod choices;
pub use choices::*;

#[derive(Clone, Debug, Default)]
pub struct ConditionSelector {
    selected: ConditionChoice,
    child: Option<ChildSelector>,
    value_a: Option<Value>,
    value_b: Option<Value>,
    condition_a: Option<Condition>,
    condition_b: Option<Condition>,
}

#[derive(Debug, Clone)]
enum ChildSelector {
    ValueA(Box<ValueSelector>),
    ValueB(Box<ValueSelector>),
    ConditionA(Box<ConditionSelector>),
    ConditionB(Box<ConditionSelector>),
}

#[derive(Clone, Debug)]
pub enum MConditionSelector {
    SelectValue(Box<MValueSelector>),
    SelectCondition(Box<MConditionSelector>),
    SubmitValue(Option<Value>),
    SubmitCondition(Option<Condition>),
    SetSelected(ConditionChoice),
    OpenSelectConditionA,
    OpenSelectConditionB,
    OpenSelectValueA,
    OpenSelectValueB,
}

impl BonusSelectorTrait for ConditionSelector {
    type Message = MConditionSelector;

    type Output = Condition;

    fn new() -> Self {
        Self::default()
    }

    fn set_value(self, value: &Self::Output, _: &[Attribute]) -> Self {
        match value {
            Condition::Not(condition) => Self {
                selected: ConditionChoice::Not,
                condition_a: Some(*condition.clone()),
                ..self
            },
            Condition::GreaterThan(a, b) => Self {
                selected: ConditionChoice::GreaterThan,
                value_a: Some(a.clone()),
                value_b: Some(b.clone()),
                ..self
            },
            Condition::LessThan(a, b) => Self {
                selected: ConditionChoice::LessThan,
                value_a: Some(a.clone()),
                value_b: Some(b.clone()),
                ..self
            },
            Condition::EqualTo(a, b) => Self {
                selected: ConditionChoice::EqualTo,
                value_a: Some(a.clone()),
                value_b: Some(b.clone()),
                ..self
            },
            Condition::Constant(val) => Self {
                selected: if *val {
                    ConditionChoice::True
                } else {
                    ConditionChoice::False
                },
                ..self
            },
            Condition::And(a, b) => Self {
                selected: ConditionChoice::And,
                condition_a: Some(*a.clone()),
                condition_b: Some(*b.clone()),
                ..self
            },
            Condition::Or(a, b) => Self {
                selected: ConditionChoice::Or,
                condition_a: Some(*a.clone()),
                condition_b: Some(*b.clone()),
                ..self
            },
            Condition::Xor(a, b) => Self {
                selected: ConditionChoice::Xor,
                condition_a: Some(*a.clone()),
                condition_b: Some(*b.clone()),
                ..self
            },
        }
    }

    fn get_value(&self, _: &[Attribute]) -> Option<Self::Output> {
        Some(match self.selected {
            ConditionChoice::Not => self.condition_a.clone()?.not(),
            ConditionChoice::GreaterThan => {
                self.value_a.clone()?.greater_than(self.value_b.clone()?)
            }
            ConditionChoice::LessThan => self.value_a.clone()?.less_than(self.value_b.clone()?),
            ConditionChoice::EqualTo => self.value_a.clone()?.equal_to(self.value_b.clone()?),
            ConditionChoice::True => Condition::TRUE,
            ConditionChoice::False => Condition::FALSE,
            ConditionChoice::And => self.condition_a.clone()? & self.condition_b.clone()?,
            ConditionChoice::Or => self.condition_a.clone()? | self.condition_b.clone()?,
            ConditionChoice::Xor => self.condition_a.clone()? ^ self.condition_b.clone()?,
        })
    }

    fn message(&mut self, message: Self::Message, attributes: &[Attribute]) -> Command<Message> {
        match message {
            MConditionSelector::SelectValue(m) => {
                if let Some(ChildSelector::ValueA(sel) | ChildSelector::ValueB(sel)) =
                    &mut self.child
                {
                    sel.message(*m, attributes)
                } else {
                    Command::none()
                }
            }
            MConditionSelector::SelectCondition(m) => {
                if let Some(ChildSelector::ConditionA(sel) | ChildSelector::ConditionB(sel)) =
                    &mut self.child
                {
                    sel.message(*m, attributes)
                } else {
                    Command::none()
                }
            }
            MConditionSelector::SubmitValue(value) => {
                if value.is_some() {
                    match self.child {
                        Some(ChildSelector::ValueA(_)) => self.value_a = value,
                        Some(ChildSelector::ValueB(_)) => self.value_b = value,
                        _ => {}
                    }
                }
                self.child = None;
                Command::none()
            }
            MConditionSelector::SubmitCondition(condition) => {
                if condition.is_some() {
                    match self.child {
                        Some(ChildSelector::ConditionA(_)) => self.condition_a = condition,
                        Some(ChildSelector::ConditionB(_)) => self.condition_b = condition,
                        _ => {}
                    }
                }
                self.child = None;
                Command::none()
            }
            MConditionSelector::SetSelected(selected) => {
                self.selected = selected;
                Command::none()
            }
            MConditionSelector::OpenSelectConditionA => {
                self.child = Some(ChildSelector::ConditionA(Box::new(
                    Self::new().set_value_maybe(self.condition_a.as_ref(), attributes),
                )));
                Command::none()
            }
            MConditionSelector::OpenSelectConditionB => {
                self.child = Some(ChildSelector::ConditionB(Box::new(
                    Self::new().set_value_maybe(self.condition_b.as_ref(), attributes),
                )));
                Command::none()
            }
            MConditionSelector::OpenSelectValueA => {
                self.child = Some(ChildSelector::ValueA(Box::new(
                    ValueSelector::new().set_value_maybe(self.value_a.as_ref(), attributes),
                )));
                Command::none()
            }
            MConditionSelector::OpenSelectValueB => {
                self.child = Some(ChildSelector::ValueB(Box::new(
                    ValueSelector::new().set_value_maybe(self.value_b.as_ref(), attributes),
                )));
                Command::none()
            }
        }
    }

    fn view<'a, FSubmit, FConvert>(
        &'a self,
        submit: FSubmit,
        convert: FConvert,
        attributes: &[Attribute],
    ) -> iced::Element<'_, Message, iced::Renderer<crate::AppTheme>>
    where
        FSubmit: Fn(Option<Self::Output>) -> Message + 'a + Clone,
        FConvert: Fn(Self::Message) -> Message + 'a + Clone,
    {
        modal(
            card(
                text("Edit Condition"),
                column(vec![
                    pick_list(&ConditionChoice::ALL[..], Some(self.selected), {
                        let convert = convert.clone();
                        move |selected| convert(MConditionSelector::SetSelected(selected))
                    })
                    .into(),
                    match self.selected {
                        ConditionChoice::Not => text("boo").into(),
                        _ => text("Hi world").into(),
                    },
                ]),
            )
            .foot(row!(
                horizontal_space(Length::Fill),
                button(text("Cancel"))
                    .style(theme::Button::Secondary)
                    .on_press(submit(None)),
                horizontal_space(10),
                button(text("Submit"))
                    .style(theme::Button::Primary)
                    .on_press_maybe(
                        self.get_value(attributes)
                            .map(|condition| submit(Some(condition)))
                    )
            )),
            self.child.as_ref().map(|selector| match selector {
                ChildSelector::ValueA(sel) | ChildSelector::ValueB(sel) => sel.view(
                    {
                        let convert = convert.clone();
                        move |value| convert(MConditionSelector::SubmitValue(value))
                    },
                    {
                        let convert = convert.clone();
                        move |message| convert(MConditionSelector::SelectValue(Box::new(message)))
                    },
                    attributes,
                ),
                ChildSelector::ConditionA(sel) | ChildSelector::ConditionB(sel) => sel.view(
                    {
                        let convert = convert.clone();
                        move |condition| convert(MConditionSelector::SubmitCondition(condition))
                    },
                    {
                        let convert = convert.clone();
                        move |message| {
                            convert(MConditionSelector::SelectCondition(Box::new(message)))
                        }
                    },
                    attributes,
                ),
            }),
        )
        .into()
    }
}
