use std::path::{Path, PathBuf};

use builder::{attribute::Attribute, equipment::set_bonus::ItemSet};
use iced::{Application, Command};
use itertools::chain;
use ui::HandleMessage;
use utils::enums::StaticOptions;

use crate::{Editor, Message};

use self::container::{DataContainer, DataContainerMessage};

pub mod container;

#[derive(Clone, Debug)]
pub struct Data {
    pub set_bonuses: DataContainer<Vec<ItemSet>>,
}

impl Data {
    pub fn generate_attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        let set_bonuses = self.set_bonuses.data.iter().flat_map(|sets| {
            sets.iter()
                .map(|set| Attribute::ItemSet(set.name().clone()))
        });
        chain!(set_bonuses, Attribute::get_static())
    }
}

impl Default for Data {
    fn default() -> Self {
        fn base() -> PathBuf {
            Path::new(".").join("data").join("data")
        }

        Self {
            set_bonuses: DataContainer::new(base().join("item_sets.ron")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum DataMessage {
    SetBonuses(DataContainerMessage<Vec<ItemSet>>),
}

impl HandleMessage<DataMessage> for Editor {
    fn handle_message(&mut self, message: DataMessage) -> Command<<Self as Application>::Message> {
        self.data.handle_message(message)
    }
}

impl HandleMessage<DataMessage, Editor> for Data {
    fn handle_message(
        &mut self,
        message: DataMessage,
    ) -> Command<<Editor as Application>::Message> {
        match message {
            DataMessage::SetBonuses(message) => self.set_bonuses.handle_message(message),
        }
    }
}

impl From<DataContainerMessage<Vec<ItemSet>>> for DataMessage {
    fn from(value: DataContainerMessage<Vec<ItemSet>>) -> Self {
        Self::SetBonuses(value)
    }
}

impl From<DataMessage> for Message {
    fn from(value: DataMessage) -> Self {
        Self::Data(value)
    }
}
