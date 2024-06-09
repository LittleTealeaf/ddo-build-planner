use builder::attribute::Attribute;
use fuzzy_filter::matches;
use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, scrollable, text, text_input},
    Application, Command, Element, Length, Renderer,
};
use ui::{HandleMessage, HandleView};
use utils::from_into::FromInto;

use crate::{App, Message};

#[derive(Clone, Debug)]
pub struct AttributeSelector {
    attributes: Vec<Attribute>,
    selected: Option<usize>,
    title: Option<String>,
    filter: String,
    on_submit: Option<Message>,
    on_cancel: Option<Message>,
}

impl App {
    pub fn select_attribute(&self) -> AttributeSelector {
        AttributeSelector::new(self.data.generate_attributes())
    }
}

impl AttributeSelector {
    fn new<I>(attributes: I) -> Self
    where
        I: IntoIterator<Item = Attribute>,
    {
        Self {
            attributes: attributes.into_iter().collect(),
            selected: None,
            title: None,
            filter: String::new(),
            on_submit: None,
            on_cancel: None,
        }
    }

    pub fn select<A>(self, attribute: A) -> Self
    where
        A: Into<Option<Attribute>>,
    {
        let attribute = Option::<Attribute>::from_into(attribute);
        Self {
            selected: attribute.and_then(|attribute| {
                self.attributes
                    .iter()
                    .enumerate()
                    .find_map(|(index, a)| a.eq(&attribute).then_some(index))
            }),
            ..self
        }
    }

    pub fn title<S>(self, title: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    pub fn on_submit<M>(self, message: M) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_submit: Some(message.into()),
            ..self
        }
    }

    pub fn on_submit_maybe<M>(self, message: Option<M>) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_submit: message.map(Into::into),
            ..self
        }
    }

    pub fn on_cancel<M>(self, message: M) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_cancel: Some(message.into()),
            ..self
        }
    }

    pub fn on_cancel_maybe<M>(self, message: Option<M>) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_cancel: message.map(Into::into),
            ..self
        }
    }

    pub fn get_attribute(&self) -> Option<Attribute> {
        self.attributes.get(self.selected?).cloned()
    }
}

#[derive(Debug, Clone)]
pub enum AttributeSelectorMessage {
    Filter(String),
    Select(usize),
    Clear,
    Submit,
    Cancel,
}

impl From<AttributeSelectorMessage> for Message {
    fn from(value: AttributeSelectorMessage) -> Self {
        Self::AttributeSelector(value)
    }
}

impl HandleMessage<AttributeSelectorMessage> for App {
    fn handle_message(
        &mut self,
        message: AttributeSelectorMessage,
    ) -> Command<<Self as Application>::Message> {
        let Some(sel) = &mut self.attribute_selector else {
            return Command::none();
        };

        match message {
            AttributeSelectorMessage::Filter(filter) => {
                sel.filter = filter;
                Command::none()
            }
            AttributeSelectorMessage::Select(index) => {
                sel.selected = Some(index);
                Command::none()
            }
            AttributeSelectorMessage::Submit => {
                let command = sel
                    .on_submit
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.attribute_selector = None;
                command
            }
            AttributeSelectorMessage::Cancel => {
                let command = sel
                    .on_cancel
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.attribute_selector = None;
                command
            }
            AttributeSelectorMessage::Clear => {
                sel.selected = None;
                Command::none()
            }
        }
    }
}

impl HandleView<App> for AttributeSelector {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        let filter = self.filter.to_lowercase();
        let selected = self.selected.unwrap_or(self.attributes.len());

        column!(
            text_input("Filter...", &self.filter)
                .on_input(|filter| { AttributeSelectorMessage::Filter(filter).into() }),
            scrollable(column(
                self.attributes
                    .iter()
                    .enumerate()
                    .map(|(index, attribute)| (index, attribute.to_string()))
                    .filter(|(_, str)| matches(&filter, str.to_lowercase().as_ref()))
                    .map(|(index, attr)| {
                        container(
                            button(text(attr))
                                .on_press(AttributeSelectorMessage::Select(index).into())
                                .style(if selected == index {
                                    theme::Button::Primary
                                } else {
                                    theme::Button::Text
                                }),
                        )
                        .width(Length::Fill)
                        .into()
                    })
            )).height(Length::Fill),
            row!(
                horizontal_space().width(Length::Fill),
                button(text("Cancel"))
                    .style(theme::Button::Secondary)
                    .on_press(AttributeSelectorMessage::Cancel.into()),
                button(text("Submit"))
                    .style(theme::Button::Primary)
                    .on_press_maybe(
                        self.selected
                            .is_some()
                            .then_some(AttributeSelectorMessage::Submit.into())
                    )
            )
        )
        .into()
    }
}
