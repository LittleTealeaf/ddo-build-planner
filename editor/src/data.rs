use std::path::PathBuf;

use builder::{attribute::Attribute, equipment::set_bonus::ItemSet};
use iced::{Application, Command};
use itertools::chain;
use ui::HandleMessage;
use utils::enums::StaticValues;

use crate::{App, Message};

use self::container::{DataContainer, DataContainerMessage};

pub mod container;

type ItemSetsType = Vec<ItemSet>;

#[derive(Clone, Debug)]
pub struct Data {
    pub item_sets: DataContainer<ItemSetsType>,
}

impl Data {
    pub fn new() -> Self {
        let root: PathBuf = PathBuf::from_iter([".", "data", "daata"]);

        Self {
            item_sets: DataContainer::new(root.join("item_sets.ron")),
        }
    }

    pub fn generate_attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        let set_bonuses = self.item_sets.get().into_iter().flat_map(|sets| {
            sets.iter()
                .map(|set| Attribute::ItemSet(set.name().clone()))
        });

        chain!(set_bonuses, Attribute::values())
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub enum DataMessage {
    SetBonuses(DataContainerMessage<ItemSetsType>),
}

impl HandleMessage<DataMessage> for App {
    fn handle_message(&mut self, message: DataMessage) -> Command<<Self as Application>::Message> {
        self.data.handle_message(message)
    }
}

impl HandleMessage<DataMessage, App> for Data {
    fn handle_message(&mut self, message: DataMessage) -> Command<<App as Application>::Message> {
        match message {
            DataMessage::SetBonuses(message) => self.item_sets.handle_message(message),
        }
    }
}

impl From<DataContainerMessage<ItemSetsType>> for DataMessage {
    fn from(value: DataContainerMessage<ItemSetsType>) -> Self {
        Self::SetBonuses(value)
    }
}

impl From<DataMessage> for Message {
    fn from(value: DataMessage) -> Self {
        Self::Data(value)
    }
}
