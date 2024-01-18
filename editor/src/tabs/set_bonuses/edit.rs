use builder::equipment::set_bonus::SetBonus;
use iced::{
    widget::{button, column, text},
    Command,
};
use ui::{HandleMessage, HandleView};

use crate::{Editor, Message};

use super::MSetBonuses;

#[derive(Debug, Clone)]
pub struct EditingSet {
    index: Option<usize>,
    set: SetBonus,
}

impl EditingSet {
    pub const fn from_index(index: usize, set: SetBonus) -> Self {
        Self {
            index: Some(index),
            set,
        }
    }

    pub const fn new(set: SetBonus) -> Self {
        Self { index: None, set }
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
            }
        } else {
            Command::none()
        }
    }
}

impl HandleView<Editor> for EditingSet {
    fn handle_view<'a>(
        &'a self,
        app: &'a Editor,
    ) -> iced::Element<
        '_,
        <Editor as iced::Application>::Message,
        iced::Renderer<<Editor as iced::Application>::Theme>,
    > {
        column!(
            button(text("Back")).on_press(MSetBonuses::CancelEdit.into()),
            text(self.set.name())
        )
        .into()
    }
}
