use builder::attribute::Attribute;
use fuzzy_filter::matches;
use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, scrollable, text, text_input},
    Command, Element, Length, Renderer,
};
use iced_aw::card;
use itertools::Itertools;

use crate::{AppMessage, AppTheme, Message};

#[derive(Clone, Debug)]
pub struct AttributeSelector {
    attributes: Vec<Attribute>,
    on_submit: Option<Message>,
    on_cancel: Option<Message>,
    selected: Option<usize>,
    filter: String,
}

#[derive(Clone, Debug)]
pub enum MAttributeSelector {
    Select(usize),
    Clear,
    Filter(String),
}

impl AttributeSelector {
    pub fn new(attributes: impl IntoIterator<Item = Attribute>) -> Self {
        let attributes = attributes
            .into_iter()
            .sorted_by_cached_key(|attribute| format!("{attribute}"))
            .collect();

        Self {
            attributes,
            selected: None,
            on_submit: None,
            on_cancel: None,
            filter: String::new(),
        }
    }

    pub fn select(mut self, selected: &Attribute) -> Self {
        self.selected = self
            .attributes
            .iter()
            .enumerate()
            .find_map(|(index, attribute)| attribute.eq(selected).then_some(index));
        self
    }

    pub fn selected(&self) -> Option<&Attribute> {
        self.selected.and_then(|index| self.attributes.get(index))
    }

    pub fn filter(mut self, filter: String) -> Self {
        self.filter = filter;
        self
    }

    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    pub fn on_cancel(mut self, message: Message) -> Self {
        self.on_cancel = Some(message);
        self
    }

    pub fn message(&mut self, message: MAttributeSelector) -> Command<Message> {
        match message {
            MAttributeSelector::Select(index) => {
                if index < self.attributes.len() {
                    self.selected = Some(index);
                }
                Command::none()
            }
            MAttributeSelector::Filter(filter) => {
                self.filter = filter;
                Command::none()
            }
            MAttributeSelector::Clear => {
                self.selected = None;
                Command::none()
            }
        }
    }

    pub fn view<'a>(
        &'a self,
        convert_message: impl Fn(MAttributeSelector) -> AppMessage + 'a + Clone,
    ) -> Element<'_, AppMessage, Renderer<AppTheme>> {
        let filter = self.filter.to_lowercase();
        let selected = self.selected.unwrap_or(self.attributes.len());

        let on_filter_convert = convert_message.clone();
        let on_filter = move |filter| on_filter_convert(MAttributeSelector::Filter(filter));

        let on_select_convert = convert_message;
        let on_select = move |index| on_select_convert(MAttributeSelector::Select(index));

        card(
            text("Attribute Selector"),
            column!(
                text_input("Filter...", &self.filter).on_input(on_filter),
                scrollable(column(
                    self.attributes
                        .iter()
                        .enumerate()
                        .map(|(index, attribute)| (index, format!("{attribute}")))
                        .filter(|(_, str)| matches(&filter, str.to_lowercase().as_ref()))
                        .map(|(index, attr)| {
                            container(button(text(attr)).on_press(on_select(index)).style(
                                if selected == index {
                                    theme::Button::Primary
                                } else {
                                    theme::Button::Text
                                },
                            ))
                            .into()
                        })
                        .collect()
                ))
                .width(Length::Fill)
                .height(Length::from(400))
            ),
        )
        .foot(row!(
            horizontal_space(Length::Fill),
            button(text("Cancel"))
                .style(theme::Button::Secondary)
                .on_press_maybe(self.on_cancel.clone()),
            horizontal_space(10),
            button(text("Submit"))
                .style(theme::Button::Primary)
                .on_press_maybe(self.selected.and_then(|_| self.on_submit.clone()))
        ))
        .into()
    }
}