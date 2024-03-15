use builder::attribute::Attribute;
use fuzzy_filter::matches;
use iced::{
    theme,
    widget::{button, column, container, scrollable, text, text_input},
    Application, Command, Element, Renderer,
};
use ui::{HandleMessage, HandleView};

use crate::{Editor, Message};

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

impl AttributeSelectorMessage {
    const fn into_message(self, depth: usize) -> Message {
        Message::Selector(SelectorWidgetMessage::Selector(
            depth,
            SelectorMessage::Attribute(self),
        ))
    }
}

impl<'s> HandleView<Editor> for AttributeSelector<'s> {
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, <Editor as Application>::Theme, Renderer>
    {
        let filter = self.filter.to_lowercase();
        let selected = self.selected.unwrap_or(self.attributes.len());

        column!(
            text_input("Filter...", &self.filter).on_input(|filter| {
                AttributeSelectorMessage::Filter(filter).into_message(self.depth)
            }),
            scrollable(column(
                self.attributes
                    .iter()
                    .enumerate()
                    .map(|(index, attribute)| (index, format!("{attribute}")))
                    .filter(|(_, str)| matches(&filter, str.to_lowercase().as_ref()))
                    .map(|(index, attr)| {
                        container(
                            button(text(attr))
                                .on_press(
                                    AttributeSelectorMessage::Select(index)
                                        .into_message(self.depth),
                                )
                                .style(if selected == index {
                                    theme::Button::Primary
                                } else {
                                    theme::Button::Text
                                }),
                        )
                        .into()
                    })
            ))
        )
        .into()
    }
}
