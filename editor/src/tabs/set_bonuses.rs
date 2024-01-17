use builder::equipment::set_bonus::SetBonus;
use fuzzy_filter::FuzzyFilter;
use iced::{
    theme,
    widget::{button, column, container, row, scrollable, text, text_input},
    Alignment, Command, Length,
};
use iced_aw::{floating_element, graphics::icons::icon_to_char, Icon, ICON_FONT};
use ui::{HandleView, HandleMessage};

use crate::{Editor, Message};

#[derive(Clone, Debug, Default)]
pub struct TabSetBonuses {
    open_index: Option<usize>,
    set_search_term: String,
}

#[derive(Debug, Clone)]
pub enum MessageSetBonuses {
    OpenSet(usize),
    DeleteSet(usize),
    CreateSet,
    CloseSet,
    SearchSets(String),
    SetSetName(usize, String),
}

impl From<MessageSetBonuses> for Message {
    fn from(value: MessageSetBonuses) -> Self {
        Self::SetBonuses(value)
    }
}

impl HandleMessage<MessageSetBonuses> for Editor {
    fn handle_message(&mut self, message: MessageSetBonuses) -> iced::Command<Self::Message> {
        match &mut self.set_bonuses {
            None => Command::none(),
            Some(set_bonuses) => match message {
                MessageSetBonuses::OpenSet(set) => {
                    if set_bonuses.len() > set {
                        self.tab_set_bonuses.open_index = Some(set);
                    }
                    Command::none()
                }
                MessageSetBonuses::CloseSet => {
                    self.tab_set_bonuses.open_index = None;
                    Command::none()
                }
                MessageSetBonuses::DeleteSet(set) => {
                    if set_bonuses.len() > set {
                        set_bonuses.remove(set);
                    }
                    Command::none()
                }
                MessageSetBonuses::CreateSet => {
                    let index = set_bonuses.len();
                    set_bonuses.push(SetBonus::new(format!("Set {}", &set_bonuses.len())));
                    self.handle_message(MessageSetBonuses::OpenSet(index))
                }
                MessageSetBonuses::SetSetName(index, name) => {
                    if let Some(set) = set_bonuses.get_mut(index) {
                        set.set_name(name);
                    }
                    Command::none()
                }
                MessageSetBonuses::SearchSets(search) => {
                    self.tab_set_bonuses.set_search_term = search;
                    Command::none()
                }
            },
        }
    }
}

impl HandleView<TabSetBonuses> for Editor {
    fn handle_view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let search_term = self.tab_set_bonuses.set_search_term.to_lowercase();
        let filter = FuzzyFilter::new(&search_term);
        self.set_bonuses.as_ref().map_or_else(
            || {
                container(text("Loading Set Bonuses..."))
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            },
            |set_bonuses| {
                self.tab_set_bonuses.open_index.map_or_else(
                    || {
                        container(floating_element(
                            column!(
                                text_input("Search...", &self.tab_set_bonuses.set_search_term)
                                    .on_input(|i| { MessageSetBonuses::SearchSets(i).into() }),
                                scrollable(
                                    column(
                                        set_bonuses
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, set_bonus)| {
                                                filter.matches(
                                                    set_bonus.name().to_lowercase().as_ref(),
                                                )
                                            })
                                            .map(|(index, set_bonus)| {
                                                row!(
                                                    button(
                                                        text(icon_to_char(Icon::PencilSquare))
                                                            .font(ICON_FONT)
                                                            .size(15)
                                                    )
                                                    .style(theme::Button::Text)
                                                    .on_press(
                                                        MessageSetBonuses::OpenSet(index).into()
                                                    ),
                                                    text(set_bonus.name()),
                                                )
                                                .align_items(Alignment::Center)
                                                .padding(1)
                                                .into()
                                            })
                                            .collect(),
                                    )
                                    .padding(5)
                                ),
                            )
                            .width(Length::Fill)
                            .height(Length::Fill),
                            button(text(icon_to_char(Icon::Plus)).font(ICON_FONT))
                                .on_press(MessageSetBonuses::CreateSet.into()),
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into()
                    },
                    |index| {
                        container(floating_element(
                            column!(text_input("Name", set_bonuses.get(index).unwrap().name()).size(30)
                                .on_input(move |i| MessageSetBonuses::SetSetName(index, i).into())),
                            button(text(icon_to_char(Icon::Back)).font(ICON_FONT))
                                .on_press(MessageSetBonuses::CloseSet.into())
                                .style(theme::Button::Primary),
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into()
                        // column!(
                        //     text(format!("Opened index {index}")),
                        //     button("Back").on_press(MessageSetBonuses::CloseSet.into())
                        // )
                        // .into()
                    },
                )
            },
        )
    }
}
