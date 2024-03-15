use builder::attribute::Attribute;
use iced::{Application, Command};
use ui::HandleMessage;

use crate::Editor;

use super::{SelectorMessage, SelectorWidgetMessage};

#[derive(Debug, Clone)]
pub struct AttributeSelector<'a> {
    depth: usize,
    attributes: &'a [Attribute],
    selected: Option<usize>,
    filter: String,
    on_submit: SelectorWidgetMessage,
    on_cancel: SelectorWidgetMessage,
}

impl<'a> AttributeSelector<'a> {
    pub const fn new(
        depth: usize,
        attributes: &'a [Attribute],
        selected: Option<usize>,
        on_submit: SelectorWidgetMessage,
        on_cancel: SelectorWidgetMessage,
    ) -> Self {
        Self {
            depth,
            attributes,
            selected,
            on_submit,
            on_cancel,
            filter: String::new(),
        }
    }

    pub fn get_selected(&self) -> Option<&'a Attribute> {
        self.selected.map(|index| &self.attributes[index])
    }
}

#[derive(Debug, Clone)]
pub enum AttributeSelectorMessage {
    Select(usize),
    Clear,
    Filter(String),
}

impl<'a> HandleMessage<(usize, SelectorMessage), Editor> for AttributeSelector<'a> {
    fn handle_message(
        &mut self,
        (depth, message): (usize, SelectorMessage),
    ) -> Command<<Editor as Application>::Message> {
        if depth == self.depth {
            match message {
                SelectorMessage::Attribute(message) => match message {
                    AttributeSelectorMessage::Select(selected) => {
                        self.selected = Some(selected);
                        Command::none()
                    }
                    AttributeSelectorMessage::Clear => {
                        self.selected = None;
                        Command::none()
                    }
                    AttributeSelectorMessage::Filter(filter) => {
                        self.filter = filter;
                        Command::none()
                    }
                },
                _ => Command::none(),
            }
        } else {
            Command::none()
        }
    }
}
