use core::fmt::{self, Display};

use builder::{
    attribute::Attribute,
    bonus::{Condition, Value},
};
use iced::{
    widget::{button, column, text},
    Application, Command, Element, Renderer,
};
use itertools::Itertools;
use ui::{HandleMessage, HandleView};

use crate::{App, Message};

use self::{
    attribute::{AttributeSelector, AttributeSelectorMessage},
    condition::{ConditionSelector, ConditionSelectorMessage},
    value::{message::ValueSelectorMessage, ValueSelector},
};

mod attribute;
mod condition;
mod value;

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

impl Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attribute(_) => write!(f, "Attribute"),
            Self::Value(selector) => write!(f, "Value {selector}"),
            Self::Condition(selector) => write!(f, "Condition {selector}"),
        }
    }
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

    pub fn set_on_submit<M>(&mut self, on_submit: M)
    where
        M: Into<Message>,
    {
        self.on_submit = Some(on_submit.into());
    }

    pub fn with_on_submit<M>(mut self, on_submit: M) -> Self
    where
        M: Into<Message>,
    {
        self.set_on_submit(on_submit);
        self
    }

    pub fn set_on_cancel<M>(&mut self, on_cancel: M)
    where
        M: Into<Message>,
    {
        self.on_cancel = Some(on_cancel.into());
    }

    pub fn with_on_cancel<M>(mut self, on_cancel: M) -> Self
    where
        M: Into<Message>,
    {
        self.set_on_cancel(on_cancel);
        self
    }

    pub fn clear_on_submit(&mut self) {
        self.on_submit = None;
    }
    pub fn clear_on_cancel(&mut self) {
        self.on_cancel = None;
    }

    pub fn select_attribute<'a, A>(&mut self, selected: A)
    where
        A: Into<Option<&'a Attribute>>,
    {
        let selected: Option<&'a Attribute> = selected.into();

        self.selector = Some(Selector::Attribute(AttributeSelector::new(
            0,
            selected
                .and_then(|attribute| self.attributes.iter().find_position(|a| a.eq(&attribute)))
                .map(|(index, _)| index),
            SelectorWidgetMessage::Submit,
            SelectorWidgetMessage::Cancel,
        )));
    }

    pub fn with_select_attribute<'a, A>(mut self, selected: A) -> Self
    where
        A: Into<Option<&'a Attribute>>,
    {
        self.select_attribute(selected);
        self
    }

    pub fn get_attribute(&self) -> Option<&'_ Attribute> {
        let Some(Selector::Attribute(selector)) = &self.selector else {
            return None;
        };
        selector.get_attribute(&self.attributes)
    }

    pub fn select_condition<'a, C>(&mut self, selected: C)
    where
        C: Into<Option<&'a Condition>>,
    {
        let selected: Option<&'a Condition> = selected.into();
        self.selector = Some(Selector::Condition(ConditionSelector::new(
            0,
            selected,
            SelectorWidgetMessage::Submit,
            SelectorWidgetMessage::Cancel,
        )));
    }

    pub fn with_select_condition<'a, C>(mut self, selected: C) -> Self
    where
        C: Into<Option<&'a Condition>>,
    {
        self.select_condition(selected);
        self
    }

    pub fn get_condition(&self) -> Option<Condition> {
        let Some(Selector::Condition(selector)) = &self.selector else {
            return None;
        };
        selector.get_condition()
    }

    pub fn select_value<'a, V>(&mut self, selected: V)
    where
        V: Into<Option<&'a Value>>,
    {
        self.selector = Some(Selector::Value(ValueSelector::new(
            0,
            selected,
            SelectorWidgetMessage::Submit,
            SelectorWidgetMessage::Cancel,
        )));
    }

    pub fn with_select_value<'a, V>(mut self, selected: V) -> Self
    where
        V: Into<Option<&'a Value>>,
    {
        self.select_value(selected);
        self
    }

    pub fn get_value(&self) -> Option<Value> {
        let Some(Selector::Value(selector)) = &self.selector else {
            return None;
        };
        selector.get_value()
    }
}

impl HandleView<App> for SelectorWidget {
    fn handle_view<'a>(
        &'a self,
        app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        self.selector.as_ref().map_or_else(
            || {
                column!(
                    text("Error: Selector Not Speciied"),
                    button(text("Close")).on_press_maybe(self.on_cancel.clone())
                )
                .into()
            },
            |selector| {
                column!(
                    text(selector),
                    match selector {
                        Selector::Attribute(sel) => sel.handle_view(app),
                        Selector::Value(sel) => sel.handle_view(app),
                        Selector::Condition(sel) => sel.handle_view(app),
                    },
                )
                .into()
            },
        )
    }
}

impl HandleMessage<SelectorWidgetMessage> for App {
    fn handle_message(
        &mut self,
        message: SelectorWidgetMessage,
    ) -> Command<<Self as Application>::Message> {
        match message {
            SelectorWidgetMessage::Selector(depth, content) => self
                .selector
                .as_mut()
                .and_then(|widget| {
                    widget.selector.as_mut().map(|selector| {
                        selector.handle_message(SelectorInternalMessage {
                            depth,
                            content,
                            attributes: &widget.attributes,
                        })
                    })
                })
                .unwrap_or_else(Command::none),
            SelectorWidgetMessage::Submit => self
                .selector
                .as_ref()
                .and_then(|widget| widget.on_submit.as_ref())
                .cloned()
                .map_or_else(Command::none, |message| self.handle_message(message)),
            SelectorWidgetMessage::Cancel => self
                .selector
                .as_ref()
                .and_then(|widget| widget.on_cancel.as_ref())
                .cloned()
                .map_or_else(Command::none, |message| self.handle_message(message)),
        }
    }
}

struct SelectorInternalMessage<'a> {
    depth: usize,
    content: SelectorMessage,
    attributes: &'a [Attribute],
}

impl<'a> HandleMessage<SelectorInternalMessage<'a>, App> for Selector {
    fn handle_message(
        &mut self,
        message: SelectorInternalMessage<'a>,
    ) -> Command<<App as Application>::Message> {
        match self {
            Self::Attribute(selector) => selector.handle_message(message),
            Self::Value(selector) => selector.handle_message(message),
            Self::Condition(selector) => selector.handle_message(message),
        }
    }
}

pub trait IntoSelectorMessage: Sized {
    fn into_selector_message(self, depth: usize) -> SelectorWidgetMessage;

    fn into_message(self, depth: usize) -> Message {
        Message::Selector(self.into_selector_message(depth))
    }
}
