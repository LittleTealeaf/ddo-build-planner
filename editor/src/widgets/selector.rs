use builder::attribute::Attribute;
use iced::{Application, Command};
use itertools::Itertools;
use ui::HandleMessage;

use crate::{Editor, Message};

use self::{
    attribute::{AttributeSelector, AttributeSelectorMessage},
    condition::ConditionSelector,
    value::ValueSelector,
};

mod attribute;
mod condition;
mod value;

// Three main modes:
// - Attribute
// - Value
// - Condition

#[derive(Debug, Clone)]
pub struct SelectorWidget<'a> {
    selector: Option<Selector<'a>>,
    attributes: Vec<Attribute>,
    on_submit: Option<Message>,
    on_cancel: Option<Message>,
}

#[derive(Debug, Clone)]
pub enum Selector<'a> {
    Attribute(AttributeSelector<'a>),
    Value(ValueSelector),
    Condition(ConditionSelector),
}

#[derive(Debug, Clone)]
pub enum SelectorWidgetMessage {
    Selector(usize, SelectorMessage),
    Submit,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum SelectorMessage {
    Attribute(AttributeSelectorMessage),
}

impl<'a> SelectorWidget<'a> {
    pub fn new<I>(attributes: I) -> Self
    where
        I: IntoIterator<Item = Attribute>,
    {
        Self {
            selector: None,
            attributes: attributes.into_iter().collect(),
            on_submit: None,
            on_cancel: None,
        }
    }

    pub fn set_on_submit(&mut self, on_submit: Option<Message>) {
        self.on_submit = on_submit;
    }

    pub fn set_on_cancel(&mut self, on_cancel: Option<Message>) {
        self.on_cancel = on_cancel;
    }

    pub fn select_attribute(&'a mut self, selected: Option<&Attribute>) {
        self.selector = Some(Selector::Attribute(AttributeSelector::new(
            0,
            &self.attributes,
            selected
                .and_then(|attribute| self.attributes.iter().find_position(|a| a.eq(&attribute)))
                .map(|(index, _)| index),
            SelectorWidgetMessage::Submit,
            SelectorWidgetMessage::Cancel,
        )));
    }

    pub fn get_attribute(&self) -> Option<&'a Attribute> {
        if let Some(Selector::Attribute(selector)) = &self.selector {
            selector.get_selected()
        } else {
            None
        }
    }
}

impl<'a> HandleMessage<SelectorWidgetMessage, Editor> for SelectorWidget<'a> {
    fn handle_message(
        &mut self,
        message: SelectorWidgetMessage,
    ) -> Command<<Editor as Application>::Message> {
        match message {
            SelectorWidgetMessage::Selector(depth, message) => self
                .selector
                .as_mut()
                .map_or_else(Command::none, |i| i.handle_message((depth, message))),
            SelectorWidgetMessage::Submit => todo!(),
            SelectorWidgetMessage::Cancel => todo!(),
        }
    }
}

impl<'a> HandleMessage<(usize, SelectorMessage), Editor> for Selector<'a> {
    fn handle_message(
        &mut self,
        message: (usize, SelectorMessage),
    ) -> Command<<Editor as Application>::Message> {
        match self {
            Selector::Attribute(selector) => selector.handle_message(message),
            Selector::Value(_) => todo!(),
            Selector::Condition(_) => todo!(),
        }
    }
}
