use builder::attribute::Attribute;
use iced::{Application, Command, Element, Renderer};
use itertools::Itertools;
use ui::{HandleMessage, HandleView};

use crate::{Editor, Message};

use self::{
    attribute::{AttributeSelector, AttributeSelectorMessage},
    condition::{ConditionSelector, ConditionSelectorMessage},
    value::{ValueSelector, ValueSelectorMessage},
};

mod attribute;
mod condition;
mod value;

// Three main modes:
// - Attribute
// - Value
// - Condition

#[derive(Debug, Clone)]
pub struct SelectorWidget {
    selector: Option<Selector>,
    attributes: Vec<Attribute>,
    on_submit: Option<Message>,
    on_cancel: Option<Message>,
}

#[derive(Debug, Clone)]
pub enum Selector {
    Attribute(AttributeSelector),
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
    Value(ValueSelectorMessage),
    Condition(ConditionSelectorMessage),
}

impl SelectorWidget {
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

    pub fn select_attribute(&mut self, selected: Option<&Attribute>) {
        self.selector = Some(Selector::Attribute(AttributeSelector::new(
            0,
            selected
                .and_then(|attribute| self.attributes.iter().find_position(|a| a.eq(&attribute)))
                .map(|(index, _)| index),
            SelectorWidgetMessage::Submit,
            SelectorWidgetMessage::Cancel,
        )));
    }

    pub fn get_attribute(&self) -> Option<&'_ Attribute> {
        if let Some(Selector::Attribute(selector)) = &self.selector {
            selector.get_selected(&self.attributes)
        } else {
            None
        }
    }
}

impl HandleView<Editor> for SelectorWidget {
    fn handle_view<'a>(
        &'a self,
        app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, <Editor as Application>::Theme, Renderer>
    {
        match &self.selector {
            Some(Selector::Attribute(selector)) => selector.handle_view(app),
            _ => todo!(),
        }
    }
}

impl HandleMessage<SelectorWidgetMessage, Editor> for SelectorWidget {
    fn handle_message(
        &mut self,
        message: SelectorWidgetMessage,
    ) -> Command<<Editor as Application>::Message> {
        match message {
            SelectorWidgetMessage::Selector(depth, message) => {
                self.selector.as_mut().map_or_else(Command::none, |i| {
                    i.handle_message((depth, message, &self.attributes))
                })
            }
            SelectorWidgetMessage::Submit => todo!(),
            SelectorWidgetMessage::Cancel => todo!(),
        }
    }
}

impl HandleMessage<(usize, SelectorMessage, &[Attribute]), Editor> for Selector {
    fn handle_message(
        &mut self,
        (depth, message, attributes): (usize, SelectorMessage, &[Attribute]),
    ) -> Command<<Editor as Application>::Message> {
        match self {
            Self::Attribute(selector) => selector.handle_message((depth, message)),
            Self::Value(selector) => selector.handle_message((depth, message, attributes)),
            Self::Condition(selector) => selector.handle_message((depth, message, attributes)),
        }
    }
}
