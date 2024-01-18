use builder::equipment::set_bonus::SetBonus;
use iced::{
    alignment::{Horizontal, Vertical},
    theme,
    widget::{button, column, horizontal_space, row, text, text_input, vertical_space},
    Alignment, Command, Length,
};
use iced_aw::{card, modal};
use ui::{HandleMessage, HandleView};

use crate::{Editor, Message};

use super::MSetBonuses;

#[derive(Debug, Clone)]
pub struct EditingSet {
    index: Option<usize>,
    set: SetBonus,
    confirm_delete: bool,
}

impl EditingSet {
    pub const fn from_index(index: usize, set: SetBonus) -> Self {
        Self {
            index: Some(index),
            set,
            confirm_delete: false,
        }
    }

    pub const fn new(set: SetBonus) -> Self {
        Self {
            index: None,
            set,
            confirm_delete: false,
        }
    }

    pub const fn index(&self) -> &Option<usize> {
        &self.index
    }

    pub const fn set(&self) -> &SetBonus {
        &self.set
    }
}

#[derive(Debug, Clone)]
pub enum MEditingSet {
    SetName(String),
    Delete,
    CancelDelete,
    ConfirmDelete,
}

impl From<MEditingSet> for Message {
    fn from(value: MEditingSet) -> Self {
        MSetBonuses::Edit(value).into()
    }
}

impl HandleMessage<MEditingSet> for Editor {
    fn handle_message(&mut self, message: MEditingSet) -> iced::Command<Self::Message> {
        if let Some(editing) = &mut self.set_bonuses.editing {
            match message {
                MEditingSet::SetName(name) => {
                    editing.set.set_name(name);
                    Command::none()
                }
                MEditingSet::Delete => {
                    editing.confirm_delete = true;
                    Command::none()
                }
                MEditingSet::CancelDelete => {
                    editing.confirm_delete = false;
                    Command::none()
                }
                MEditingSet::ConfirmDelete => editing.index.map_or_else(Command::none, |index| {
                    self.handle_message(MSetBonuses::DeleteSet(index))
                }),
            }
        } else {
            Command::none()
        }
    }
}

impl HandleView<Editor> for EditingSet {
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> iced::Element<
        '_,
        <Editor as iced::Application>::Message,
        iced::Renderer<<Editor as iced::Application>::Theme>,
    > {
        let content = column!(
            text_input("Set Name", self.set.name())
                .size(30)
                .on_input(|name| MEditingSet::SetName(name).into()),
            vertical_space(Length::Fill),
            row!(
                button(text("Delete").horizontal_alignment(Horizontal::Center))
                    .on_press_maybe(self.index.is_some().then_some(MEditingSet::Delete.into()))
                    .style(theme::Button::Destructive),
                horizontal_space(Length::Fill),
                row!(
                    button(text("Cancel").horizontal_alignment(Horizontal::Center))
                        .width(Length::Fill)
                        .on_press(MSetBonuses::CancelEdit.into())
                        .style(theme::Button::Secondary),
                    button(text("Save").horizontal_alignment(Horizontal::Center))
                        .width(Length::Fill)
                        .on_press(MSetBonuses::SaveEdit.into())
                        .style(theme::Button::Primary),
                )
                .width(Length::Fill),
            )
            .align_items(Alignment::Center)
        )
        .padding(10);

        let confirm_delete_modal = self.confirm_delete.then(|| {
            row!(
                horizontal_space(Length::FillPortion(2)),
                card(
                    text(format!("Delete {}?", self.set.name())),
                    text(format!(
                    "Do you really want to delete {}? If you save, this set will no longer exist",
                    self.set.name()
                )),
                )
                .foot(row!(
                    horizontal_space(Length::Fill),
                    button(text("Cancel"))
                        .on_press(MEditingSet::CancelDelete.into())
                        .style(theme::Button::Primary),
                    horizontal_space(10.0),
                    button(text("Delete"))
                        .on_press(MEditingSet::ConfirmDelete.into())
                        .style(theme::Button::Destructive),
                ))
                .width(Length::FillPortion(6)),
                horizontal_space(Length::FillPortion(2)),
            )
        });

        modal(content, confirm_delete_modal)
            .backdrop(MEditingSet::CancelDelete.into())
            .on_esc(MEditingSet::CancelDelete.into())
            .align_y(Vertical::Center)
            .into()
    }
}
