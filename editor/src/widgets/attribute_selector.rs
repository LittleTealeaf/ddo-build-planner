use std::marker::PhantomData;

use builder::attribute::Attribute;
use fuzzy_filter::matches;
use iced::{
    theme,
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input, Column,
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
    on_submit: Message,
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
        selected: Option<Attribute>,
        on_submit: Message,
        on_cancel: Message,
    ) -> Self {
        let attributes = attributes
            .into_iter()
            .sorted_by_cached_key(|attribute| format!("{attribute}"))
            .collect::<Vec<_>>();

        let selected = selected.and_then(|selected| {
            attributes
                .iter()
                .enumerate()
                .find_map(|(index, attr)| selected.eq(attr).then_some(index))
        });

        Self {
            on_submit,
            on_cancel,
            filter: String::new(),
            selected,
            _phantom: PhantomData,
            attributes,
        }
    }

    pub fn get_selected(&self) -> Option<&Attribute> {
        self.selected.and_then(|index| self.attributes.get(index))
    }

    pub fn set_selected(&mut self, index: Option<usize>) {
        self.selected = index;
    }
}

impl<T> HandleMessage<MAttributeSelector, Editor> for AttributeSelector<T> {
    fn handle_message(
        &mut self,
        message: MAttributeSelector,
    ) -> Command<<Editor as Application>::Message> {
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
    T: From<MAttributeSelector> + Clone,
    Message: From<T>,
{
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer<<Editor as Application>::Theme>>
    {
        let filter = self.filter.to_lowercase();
        let selected = self.selected.map_or(self.attributes.len(), |index| index);

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
                            .enumerate()
                            .map(|(index, attribute)| (index, format!("{attribute}")))
                            .filter(|(_, str)| matches(&filter, str.to_lowercase().as_ref()))
                            .map(|(index, attr)| {
                                container(
                                    button(text(attr))
                                        .on_press(
                                            T::from(MAttributeSelector::Select(Some(index))).into(),
                                        )
                                        .style(if selected == index {
                                            theme::Button::Primary
                                        } else {
                                            theme::Button::Text
                                        }),
                                )
                                .into()
                            })
                            .collect(),
                    ))
                    .width(Length::Fill)
                    .height(Length::from(400)),
                ),
        )
        .foot(row!(
            horizontal_space(Length::Fill),
            button(text("Cancel"))
                .style(theme::Button::Secondary)
                .on_press(self.on_cancel.clone()),
            horizontal_space(10),
            button(text("Submit"))
                .style(theme::Button::Primary)
                .on_press_maybe(self.selected.map(|_| self.on_submit.clone()))
        ))
        .max_width(500.0)
        .into()
    }
}
