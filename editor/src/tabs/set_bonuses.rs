use builder::equipment::set_bonus::SetBonus;
use fuzzy_filter::FuzzyFilter;
use iced::{
    theme,
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input,
        vertical_space,
    },
    Alignment, Command, Length,
};
use iced_aw::{floating_element, graphics::icons::icon_to_char, Icon, ICON_FONT};

use crate::{Editor, EditorUpdate, EditorView, Message};

#[derive(Clone, Debug, Default)]
pub struct TabSetBonuses {
    opened: Option<OpenedSet>,
    set_search_term: String,
}

#[derive(Clone, Debug)]
pub struct OpenedSet {
    index: Option<usize>,
    set: SetBonus,
}

#[derive(Debug, Clone)]
pub enum MessageSetBonuses {
    OpenSet(usize),
    DeleteSet(usize),
    CreateSet,
    SaveChanges,
    CancelEdit,
    SearchSets(String),
    EditSet(MessageEditSetBonus),
}

impl From<MessageSetBonuses> for Message {
    fn from(value: MessageSetBonuses) -> Self {
        Self::SetBonuses(value)
    }
}

#[derive(Debug, Clone)]
pub enum MessageEditSetBonus {
    SetName(String),
}

impl From<MessageEditSetBonus> for Message {
    fn from(value: MessageEditSetBonus) -> Self {
        MessageSetBonuses::EditSet(value).into()
    }
}

impl EditorUpdate<MessageSetBonuses> for Editor {
    fn handle_update(&mut self, message: MessageSetBonuses) -> iced::Command<Self::Message> {
        match &mut self.set_bonuses {
            None => Command::none(),
            Some(set_bonuses) => match message {
                MessageSetBonuses::OpenSet(index) => {
                    self.tab_set_bonuses.opened =
                        set_bonuses.get(index).cloned().map(|set| OpenedSet {
                            index: Some(index),
                            set,
                        });
                    Command::none()
                }
                MessageSetBonuses::CancelEdit => {
                    self.tab_set_bonuses.opened = None;
                    Command::none()
                }
                MessageSetBonuses::DeleteSet(set) => {
                    if set_bonuses.len() > set {
                        set_bonuses.remove(set);
                    }
                    Command::none()
                }
                MessageSetBonuses::CreateSet => {
                    self.tab_set_bonuses.opened = Some(OpenedSet {
                        index: None,
                        set: SetBonus::new(String::new()),
                    });
                    Command::none()
                }
                MessageSetBonuses::SearchSets(search) => {
                    self.tab_set_bonuses.set_search_term = search;
                    Command::none()
                }
                MessageSetBonuses::SaveChanges => {
                    if let Some(opened) = &self.tab_set_bonuses.opened {
                        if let Some(index) = opened.index {
                            set_bonuses[index] = opened.set.clone();
                        } else {
                            set_bonuses.push(opened.set.clone());
                        }
                    }
                    self.handle_update(MessageSetBonuses::CancelEdit)
                }
                MessageSetBonuses::EditSet(edit) => match &mut self.tab_set_bonuses.opened {
                    None => Command::none(),
                    Some(opened) => match edit {
                        MessageEditSetBonus::SetName(name) => {
                            opened.set.set_name(name);
                            Command::none()
                        }
                    },
                },
            },
        }
    }
}

impl EditorView<TabSetBonuses> for Editor {
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
                self.tab_set_bonuses.opened.as_ref().map_or_else(
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
                            button(text(icon_to_char(Icon::Plus)).font(ICON_FONT).size(35))
                                .on_press(MessageSetBonuses::CreateSet.into()),
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into()
                    },
                    |set| {
                        column!(
                            text_input("Set Name...", set.set.name())
                                .on_input(|i| MessageEditSetBonus::SetName(i).into())
                                .size(20),
                            vertical_space(Length::Fill),
                            row!(
                                horizontal_space(Length::Fill),
                                button(text("Cancel"))
                                    .style(theme::Button::Secondary)
                                    .on_press(MessageSetBonuses::CancelEdit.into()),
                                button(text("Save"))
                                    .style(theme::Button::Primary)
                                    .on_press(MessageSetBonuses::SaveChanges.into())
                            )
                        )
                        .into()
                    },
                )
            },
        )
    }
}
