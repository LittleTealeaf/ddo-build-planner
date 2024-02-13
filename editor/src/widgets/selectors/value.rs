use core::fmt;
use std::fmt::Display;

use builder::{
    attribute::Attribute,
    bonus::{Condition, Value},
};
use iced::{
    theme,
    widget::{button, column, horizontal_space, pick_list, row, text, Column},
    Command, Element, Length, Renderer,
};
use iced_aw::{card, modal};
use rust_decimal::Decimal;

use crate::{AppMessage, AppTheme, Message};

use super::{
    attribute::{AttributeSelector, MAttributeSelector},
    condition::{ConditionSelector, MConditionSelector},
};

#[derive(Clone, Debug, Default)]
pub struct ValueSelector {
    child: Option<ChildSelector>,
    value_type: ValueType,
    constant: Option<Decimal>,
    attributes: Vec<Attribute>,
    attribute: Option<Attribute>,
    value_a: Option<Value>,
    value_b: Option<Value>,
    condition: Option<Condition>,
}

#[derive(Debug, Clone)]
enum ChildSelector {
    Attribute(Box<AttributeSelector>),
    ValueA(Box<ValueSelector>),
    ValueB(Box<ValueSelector>),
    Condition(Box<ConditionSelector>),
}

#[derive(Clone, Debug, Copy, Default, Eq, PartialEq)]
pub enum ValueType {
    #[default]
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

impl ValueType {
    const ALL: [Self; 15] = [
        Self::Const,
        Self::Attribute,
        Self::Min,
        Self::Max,
        Self::Floor,
        Self::Ceil,
        Self::Round,
        Self::Abs,
        Self::Add,
        Self::Sub,
        Self::Mul,
        Self::Div,
        Self::Rem,
        Self::If,
        Self::Dice,
    ];
}

impl Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const => write!(f, "Constant"),
            Self::Attribute => write!(f, "Attribute"),
            Self::Min => write!(f, "Minimum"),
            Self::Max => write!(f, "Maximum"),
            Self::Floor => write!(f, "Floor"),
            Self::Ceil => write!(f, "Ceil"),
            Self::Round => write!(f, "Round"),
            Self::Abs => write!(f, "Absolute"),
            Self::Add => write!(f, "Add"),
            Self::Sub => write!(f, "Subtraction"),
            Self::Mul => write!(f, "Multiplication"),
            Self::Div => write!(f, "Division"),
            Self::Rem => write!(f, "Remainder"),
            Self::If => write!(f, "Conditional"),
            Self::Dice => write!(f, "Dice"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MValueSelector {
    SelectValue(Box<MValueSelector>),
    SelectCondition(Box<MConditionSelector>),
    SelectAttribute(Box<MAttributeSelector>),
    SubmitSelection,
    CancelSelection,
    SetType(ValueType),
    OpenSelectCondition,
    OpenSelectValueA,
    OpenSelectValueB,
    OpenSelectAttribute,
}

impl ValueSelector {
    pub fn new<I>(attributes: I) -> Self
    where
        I: IntoIterator<Item = Attribute>,
    {
        Self {
            attributes: attributes.into_iter().collect(),
            ..Default::default()
        }
    }

    pub fn set_value_maybe(self, value: Option<&Value>) -> Self {
        if let Some(value) = value {
            self.set_value(value)
        } else {
            self
        }
    }

    pub fn set_value(self, value: &Value) -> Self {
        match value {
            Value::Const(val) => Self {
                value_type: ValueType::Const,
                constant: Some(*val),
                ..self
            },
            Value::Attribute(attribute) => Self {
                value_type: ValueType::Attribute,
                attribute: Some(attribute.clone()),
                ..self
            },
            Value::Min(a, b) => Self {
                value_type: ValueType::Min,
                value_a: Some(*a.clone()),
                value_b: Some(*b.clone()),
                ..self
            },
            Value::Max(a, b) => Self {
                value_type: ValueType::Max,
                value_a: Some(*a.clone()),
                value_b: Some(*b.clone()),
                ..self
            },
            Value::Floor(val) => Self {
                value_type: ValueType::Floor,
                value_a: Some(*val.clone()),
                ..self
            },
            Value::Ceil(val) => Self {
                value_type: ValueType::Ceil,
                value_a: Some(*val.clone()),
                ..self
            },
            Value::Round(val) => Self {
                value_type: ValueType::Round,
                value_a: Some(*val.clone()),
                ..self
            },
            Value::Abs(val) => Self {
                value_type: ValueType::Abs,
                value_a: Some(*val.clone()),
                ..self
            },
            Value::Add(a, b) => Self {
                value_type: ValueType::Add,
                value_a: Some(*a.clone()),
                value_b: Some(*b.clone()),
                ..self
            },
            Value::Sub(a, b) => Self {
                value_type: ValueType::Sub,
                value_a: Some(*a.clone()),
                value_b: Some(*b.clone()),
                ..self
            },
            Value::Mul(a, b) => Self {
                value_type: ValueType::Mul,
                value_a: Some(*a.clone()),
                value_b: Some(*b.clone()),
                ..self
            },
            Value::Div(a, b) => Self {
                value_type: ValueType::Div,
                value_a: Some(*a.clone()),
                value_b: Some(*b.clone()),
                ..self
            },
            Value::Rem(a, b) => Self {
                value_type: ValueType::Rem,
                value_a: Some(*a.clone()),
                value_b: Some(*b.clone()),
                ..self
            },
            Value::If {
                condition,
                if_true,
                if_false,
            } => Self {
                value_type: ValueType::If,
                condition: Some(*condition.clone()),
                value_a: Some(*if_true.clone()),
                value_b: Some(*if_false.clone()),
                ..self
            },
            Value::Dice { count, size } => Self {
                value_type: ValueType::Dice,
                value_a: Some(*count.clone()),
                value_b: Some(*size.clone()),
                ..self
            },
        }
    }

    pub fn value(&self) -> Option<Value> {
        Some(match self.value_type {
            ValueType::Const => Value::Const(self.constant?),
            ValueType::Attribute => Value::Attribute(self.attribute.clone()?),
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

    pub fn message(&mut self, message: MValueSelector) -> Command<Message> {
        match message {
            MValueSelector::SelectValue(m) => {
                if let Some(ChildSelector::ValueA(sel) | ChildSelector::ValueB(sel)) =
                    &mut self.child
                {
                    sel.message(*m)
                } else {
                    Command::none()
                }
            }
            MValueSelector::SelectCondition(_) => todo!(),
            MValueSelector::SelectAttribute(_) => todo!(),
            MValueSelector::SubmitSelection => todo!(),
            MValueSelector::CancelSelection => {
                self.child = None;
                Command::none()
            }
            MValueSelector::SetType(value_type) => {
                self.value_type = value_type;
                Command::none()
            }
            MValueSelector::OpenSelectCondition => {
                todo!()
            }
            MValueSelector::OpenSelectValueA => {
                self.child = Some(ChildSelector::ValueA(Box::new(
                    Self::new(self.attributes.clone()).set_value_maybe(self.value_a.as_ref()),
                )));
                Command::none()
            }
            MValueSelector::OpenSelectValueB => {
                self.child = Some(ChildSelector::ValueB(Box::new(
                    Self::new(self.attributes.clone()).set_value_maybe(self.value_b.as_ref()),
                )));
                Command::none()
            }
            MValueSelector::OpenSelectAttribute => {
                self.child = Some(ChildSelector::Attribute(Box::new(
                    AttributeSelector::new(self.attributes.clone())
                        .select_maybe(self.attribute.as_ref()),
                )));
                Command::none()
            }
        }
    }

    pub fn view<'a, F>(
        &'a self,
        on_submit: Message,
        on_cancel: Message,
        convert: F,
    ) -> Element<'_, AppMessage, Renderer<AppTheme>>
    where
        F: Fn(MValueSelector) -> AppMessage + 'a + Clone,
    {
        modal(
            card(
                text("Edit Value"),
                column(vec![
                    pick_list(&ValueType::ALL[..], Some(self.value_type), {
                        let convert = convert.clone();
                        move |selected| convert(MValueSelector::SetType(selected))
                    })
                    .into(),
                    match self.value_type {
                        ValueType::Attribute => row(vec![
                            text(
                                self.attribute
                                    .as_ref()
                                    .map_or("None".to_owned(), |attr| format!("{attr}")),
                            )
                            .into(),
                            button(text("Set Attribute"))
                                .on_press(convert(MValueSelector::OpenSelectAttribute))
                                .into(),
                        ])
                        .into(),
                        _ => text("Hello world").into(),
                    },
                ]),
            )
            .foot(row!(
                horizontal_space(Length::Fill),
                button(text("Cancel"))
                    .style(theme::Button::Secondary)
                    .on_press(on_cancel),
                horizontal_space(10),
                button(text("Submit"))
                    .style(theme::Button::Primary)
                    .on_press_maybe(self.value().map(|_| on_submit))
            )),
            self.child.as_ref().map(|selector| match selector {
                ChildSelector::Attribute(sel) => {
                    let convert = convert.clone();
                    sel.view(
                        convert(MValueSelector::SubmitSelection),
                        convert(MValueSelector::CancelSelection),
                        move |message| convert(MValueSelector::SelectAttribute(Box::new(message))),
                    )
                }
                ChildSelector::ValueA(sel) | ChildSelector::ValueB(sel) => {
                    let convert = convert.clone();
                    sel.view(
                        convert(MValueSelector::SubmitSelection),
                        convert(MValueSelector::CancelSelection),
                        move |message| convert(MValueSelector::SelectValue(Box::new(message))),
                    )
                }
                ChildSelector::Condition(sel) => todo!(),
            }),
        )
        .into()
    }
}
