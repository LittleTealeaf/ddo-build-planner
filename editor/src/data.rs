use builder::equipment::set_bonus::SetBonus;
use iced::{Application, Command};
use ui::HandleMessage;

use crate::{Editor, Message};

use self::container::{DataContainer, DataContainerMessage};

pub mod container;

#[derive(Clone, Debug)]
pub struct Data {
    pub set_bonuses: DataContainer<Vec<SetBonus>>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            set_bonuses: DataContainer::new("./data/data/set_bonuses.ron"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum DataMessage {
    SetBonuses(DataContainerMessage<Vec<SetBonus>>),
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

impl From<DataContainerMessage<Vec<SetBonus>>> for DataMessage {
    fn from(value: DataContainerMessage<Vec<SetBonus>>) -> Self {
        Self::SetBonuses(value)
    }
}

impl From<DataMessage> for Message {
    fn from(value: DataMessage) -> Self {
        Self::Data(value)
    }
}
