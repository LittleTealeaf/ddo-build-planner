use std::path::{Path, PathBuf};

use builder::{attribute::Attribute, equipment::set_bonus::ItemSet};
use iced::{Application, Command};
use itertools::chain;
use ui::HandleMessage;
use utils::enums::StaticOptions;

use crate::{App, Message};

use self::container::{DataContainer, DataContainerMessage};

pub mod container;

type ItemSetsType = Vec<ItemSet>;

#[derive(Clone, Debug)]
pub struct Data {
    item_sets: DataContainer<ItemSetsType>,
}

impl Data {
    pub fn generate_attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        let set_bonuses = self.item_sets.data.iter().flat_map(|sets| {
            sets.iter()
                .map(|set| Attribute::ItemSet(set.name().clone()))
        });
        chain!(set_bonuses, Attribute::get_static())
    }

    pub const fn item_sets(&self) -> Option<&ItemSetsType> {
        self.item_sets.data.as_ref()
    }

    pub fn item_sets_mut(&mut self) -> Option<&mut ItemSetsType> {
        self.item_sets.data.as_mut()
    }
}

impl Default for Data {
    fn default() -> Self {
        fn base() -> PathBuf {
            [".", "data", "data"].iter().collect()
        }

        Self {
            item_sets: DataContainer::new(base().join("item_sets.ron")),
        }
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
