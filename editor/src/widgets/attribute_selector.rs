use std::marker::PhantomData;

use builder::attribute::Attribute;
use fuzzy_filter::matches;
use iced::{
    widget::{
        column, container, horizontal_space, row, scrollable, text, text_input, vertical_space,
        Column,
    },
    Application, Command, Element, Length, Renderer,
};
use iced_aw::card;
use itertools::Itertools;
use ui::{HandleMessage, HandleView};

use crate::{Editor, Message};

#[derive(Clone, Debug)]
pub struct AttributeSelector<T> {
    filter: String,
    on_return: Message,
    on_cancel: Message,
    attributes: Vec<Attribute>,
    selected: Option<usize>,
    _phantom: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub enum MAttributeSelector {
    Select(Option<usize>),
    Filter(String),
}

impl<T> AttributeSelector<T> {
    pub fn new(
        attributes: impl IntoIterator<Item = Attribute>,
        on_return: Message,
        on_cancel: Message,
    ) -> Self {
        Self {
            on_return,
            on_cancel,
            filter: String::new(),
            selected: None,
            _phantom: PhantomData,
            attributes: attributes
                .into_iter()
                .sorted_by_cached_key(|attribute| format!("{attribute}"))
                .collect(),
        }
    }

    pub fn get_selected(&self) -> Option<Attribute> {
        self.selected
            .and_then(|index| self.attributes.get(index))
            .cloned()
    }

    pub fn set_selected(&mut self, index: Option<usize>) {
        self.selected = index;
    }
}

impl<T> HandleMessage<MAttributeSelector, Editor> for AttributeSelector<T> {
    fn handle_message(
        &mut self,
        message: MAttributeSelector,
    ) -> iced::Command<<Editor as Application>::Message> {
        match message {
            MAttributeSelector::Select(selection) => {
                self.selected = selection;
                Command::none()
            }
            MAttributeSelector::Filter(filter) => {
                self.filter = filter;
                Command::none()
            }
        }
    }
}

impl<T> HandleView<Editor> for AttributeSelector<T>
where
    T: From<MAttributeSelector>,
    Message: From<T>,
{
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer<<Editor as Application>::Theme>>
    {
        let filter = self.filter.to_lowercase();

        card(
            text("Attribute Selector"),
            Column::new()
                .push(
                    text_input("Filter...", &self.filter)
                        .on_input(|filter| T::from(MAttributeSelector::Filter(filter)).into()),
                )
                .push(
                    scrollable(column(
                        self.attributes
                            .iter()
                            .map(|attribute| (format!("{attribute}"), attribute))
                            .filter(|(attr, _)| matches(&filter, attr))
                            .map(|(attr, _)| container(text(attr)).into())
                            .collect(),
                    ))
                    .height(Length::from(100)),
                ),
        )
        .into()
    }
}
