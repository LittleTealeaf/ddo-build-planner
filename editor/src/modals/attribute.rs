use core::ops::Not;
use std::string::ToString;

use builder::attribute::Attribute;
use fuzzy_filter::matches;
use iced::{
    theme,
    widget::{button, column, container, row, scrollable, text, text_input},
    Application, Command, Element, Length, Renderer,
};
use im::OrdSet;
use ui::{error, HandleMessage, HandleView};
use utils::from_into::FromInto;

use crate::{App, Message};

#[derive(Clone, Debug)]
pub struct ModalAttribute {
    attributes: Vec<Attribute>,
    selected: OrdSet<usize>,
    multiselect: bool,
    title: Option<String>,
    filter: String,
    on_submit: Option<Message>,
    on_cancel: Option<Message>,
}

// TODO: Filter Strings on Back-End

impl App {
    pub fn select_attribute(&self) -> ModalAttribute {
        ModalAttribute::new(self.data.generate_attributes())
    }
}

impl ModalAttribute {
    fn new<I>(attributes: I) -> Self
    where
        I: IntoIterator<Item = Attribute>,
    {
        let mut attributes = attributes.into_iter().collect::<Vec<_>>();
        attributes.sort_by_cached_key(ToString::to_string);

        Self {
            attributes,
            selected: OrdSet::new(),
            title: None,
            multiselect: false,
            filter: String::new(),
            on_submit: None,
            on_cancel: None,
        }
    }

    pub fn select<A>(mut self, attribute: A) -> Self
    where
        A: Into<Attribute>,
    {
        let attribute = Attribute::from_into(attribute);
        if self.multiselect {
            if let Some(index) = self.lookup(&attribute) {
                self.selected.insert(index);
            }
            self
        } else {
            Self {
                selected: self.lookup(&attribute).into_iter().collect(),
                ..self
            }
        }
    }

    pub fn select_maybe<A>(self, attribute: Option<A>) -> Self
    where
        A: Into<Attribute>,
    {
        if let Some(attribute) = attribute {
            self.select(attribute)
        } else {
            self
        }
    }

    pub fn multiselect(self, enabled: bool) -> Self {
        Self {
            multiselect: enabled,
            ..self
        }
    }

    pub fn select_all<I, A>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        let indexes = attributes
            .into_iter()
            .map(Into::into)
            .filter_map(|a| self.lookup(&a))
            .collect::<Vec<_>>();

        self.selected.extend(indexes);

        self
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
        (!self.multiselect).then(|| {
            let index = self.selected.get_min()?;
            self.attributes.get(*index).cloned()
        })?
    }

    pub fn get_attributes(&self) -> Option<Vec<Attribute>> {
        (self.multiselect).then(|| {
            self.selected
                .iter()
                .filter_map(|index| self.attributes.get(*index))
                .cloned()
                .collect()
        })
    }
}

impl ModalAttribute {
    fn lookup(&self, attribute: &Attribute) -> Option<usize> {
        self.attributes
            .iter()
            .enumerate()
            .find_map(|(index, a)| a.eq(attribute).then_some(index))
    }
}

#[derive(Debug, Clone)]
pub enum ModalAttributeMessage {
    Filter(String),
    Select(usize),
    Clear,
    Submit,
    Cancel,
}

impl From<ModalAttributeMessage> for Message {
    fn from(value: ModalAttributeMessage) -> Self {
        Self::ModalAttribute(value)
    }
}

impl HandleMessage<ModalAttributeMessage> for App {
    fn handle_message(
        &mut self,
        message: ModalAttributeMessage,
    ) -> Command<<Self as Application>::Message> {
        let Some(sel) = &mut self.modal_attribute else {
            return self.handle_message(error!("Modal does not exist"));
        };

        match message {
            ModalAttributeMessage::Filter(filter) => {
                sel.filter = filter;
                Command::none()
            }
            ModalAttributeMessage::Select(index) => {
                if !sel.multiselect {
                    sel.selected.clear();
                }

                if sel.selected.contains(&index) {
                    sel.selected.remove(&index);
                } else {
                    sel.selected.insert(index);
                }

                Command::none()
            }
            ModalAttributeMessage::Submit => {
                let command = sel
                    .on_submit
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.modal_attribute = None;
                command
            }
            ModalAttributeMessage::Cancel => {
                let command = sel
                    .on_cancel
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.modal_attribute = None;
                command
            }
            ModalAttributeMessage::Clear => {
                sel.selected.clear();
                Command::none()
            }
        }
    }
}

impl HandleView<App> for ModalAttribute {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        let filter = self.filter.to_lowercase();

        column!(
            row!(
                text_input("Filter...", &self.filter)
                    .on_input(|filter| { ModalAttributeMessage::Filter(filter).into() })
                    .width(Length::Fill),
                button(text("Cancel"))
                    .style(theme::Button::Secondary)
                    .on_press(ModalAttributeMessage::Cancel.into()),
                button(text("Submit"))
                    .style(theme::Button::Primary)
                    .on_press_maybe(
                        self.selected
                            .is_empty()
                            .not()
                            .then_some(ModalAttributeMessage::Submit.into())
                    )
            ),
            scrollable(column(
                self.attributes
                    .iter()
                    .enumerate()
                    .map(|(index, attribute)| (index, attribute.to_string()))
                    .filter(|(_, str)| matches(&filter, str.to_lowercase().as_ref()))
                    .map(|(index, attr)| {
                        container(
                            button(text(attr))
                                .on_press(ModalAttributeMessage::Select(index).into())
                                .style(if self.selected.contains(&index) {
                                    theme::Button::Primary
                                } else {
                                    theme::Button::Text
                                }),
                        )
                        .width(Length::Fill)
                        .into()
                    })
            ))
            .height(Length::Fill),
        )
        .into()
    }
}
