use builder::attribute::Attribute;
use fuzzy_filter::matches;
use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, scrollable, text, text_input},
    Length,
};
use iced_aw::card;

use super::BonusSelectorTrait;

#[derive(Clone, Debug)]
pub struct AttributeSelector {
    selected: Option<usize>,
    filter: String,
}

#[derive(Clone, Debug)]
pub enum MAttributeSelector {
    Select(usize),
    Clear,
    Filter(String),
}

impl BonusSelectorTrait for AttributeSelector {
    type Message = MAttributeSelector;

    type Output = Attribute;

    fn new() -> Self {
        Self {
            selected: None,
            filter: String::new(),
        }
    }

    fn set_value(
        mut self,
        value: &Self::Output,
        attributes: &[builder::attribute::Attribute],
    ) -> Self {
        self.selected = attributes
            .iter()
            .enumerate()
            .find_map(|(index, attr)| attr.eq(value).then_some(index))
            .or(self.selected);
        self
    }

    fn get_value(&self, attributes: &[builder::attribute::Attribute]) -> Option<Self::Output> {
        self.selected
            .and_then(|index| attributes.get(index))
            .cloned()
    }

    fn message(
        &mut self,
        message: Self::Message,
        attributes: &[builder::attribute::Attribute],
    ) -> iced::Command<crate::Message> {
        match message {
            MAttributeSelector::Select(index) => {
                if index < attributes.len() {
                    self.selected = Some(index);
                }
                iced::Command::none()
            }
            MAttributeSelector::Filter(filter) => {
                self.filter = filter;
                iced::Command::none()
            }
            MAttributeSelector::Clear => {
                self.selected = None;
                iced::Command::none()
            }
        }
    }

    fn view<'a, FSubmit, FConvert>(
        &'a self,
        submit: FSubmit,
        convert: FConvert,
        attributes: &[Attribute],
    ) -> iced::Element<'_, crate::Message, iced::Renderer<crate::AppTheme>>
    where
        FSubmit: Fn(Option<Self::Output>) -> crate::Message + 'a + Clone,
        FConvert: Fn(Self::Message) -> crate::Message + 'a + Clone,
    {
        let filter = self.filter.to_lowercase();
        let selected = self.selected.unwrap_or(attributes.len());

        card(
            text("Attribute Selector"),
            column!(
                text_input("Filter...", &self.filter).on_input({
                    let convert = convert.clone();
                    move |filter| convert(MAttributeSelector::Filter(filter))
                }),
                scrollable(column(
                    attributes
                        .iter()
                        .enumerate()
                        .map(|(index, attribute)| (index, format!("{attribute}")))
                        .filter(|(_, str)| matches(&filter, str.to_lowercase().as_ref()))
                        .map(|(index, attr)| {
                            container(
                                button(text(attr))
                                    .on_press(convert(MAttributeSelector::Select(index)))
                                    .style(if selected == index {
                                        theme::Button::Primary
                                    } else {
                                        theme::Button::Text
                                    }),
                            )
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
                .on_press(submit(None)),
            horizontal_space(10),
            button(text("Submit"))
                .style(theme::Button::Primary)
                .on_press_maybe(
                    self.selected
                        .and_then(|index| attributes.get(index))
                        .cloned()
                        .map(|attribute| submit(Some(attribute)))
                )
        ))
        .into()
    }
}
