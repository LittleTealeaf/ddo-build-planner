mod utils;

use core::fmt::Debug;

use builder::equipment::set_bonus::SetBonus;
use iced::{Application, Command};
use ui::HandleMessage;

use crate::{Editor, Message};

use self::utils::{catch_async, load_data, save_data};

const PATH_SET_BONUSES: &str = "./data/data/set_bonuses.ron";

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub set_bonuses: DataContainer<Vec<SetBonus>>,
}

#[derive(Debug, Clone)]
pub enum MData {
    SetBonus(MDataContainer<Vec<SetBonus>>),
}

impl From<MData> for Message {
    fn from(value: MData) -> Self {
        Self::Data(value)
    }
}

#[derive(Debug, Clone)]
pub struct DataContainer<T> {
    pub data: Option<T>,
    pub modified: bool,
    pub saving: bool,
}

impl<T> Default for DataContainer<T> {
    fn default() -> Self {
        Self {
            data: None,
            modified: false,
            saving: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MDataContainer<T>
where
    T: Debug + Clone,
{
    Load,
    OnLoad(T),
    Save,
    OnSaved,
    Modified,
}

impl HandleMessage<MData> for Editor {
    fn handle_message(&mut self, message: MData) -> iced::Command<<Self as Application>::Message> {
        match message {
            MData::SetBonus(m) => match m {
                MDataContainer::Load => {
                    self.data.set_bonuses.modified = false;
                    self.data.set_bonuses.data = None;
                    Command::perform(
                        load_data(PATH_SET_BONUSES),
                        catch_async(|data| MData::SetBonus(MDataContainer::OnLoad(data))),
                    )
                }
                MDataContainer::OnLoad(data) => {
                    self.data.set_bonuses.data = Some(data);
                    Command::none()
                }
                MDataContainer::Save => {
                    self.data
                        .set_bonuses
                        .data
                        .as_ref()
                        .map_or_else(Command::none, |sets| {
                            self.data.set_bonuses.saving = true;
                            Command::perform(
                                save_data(PATH_SET_BONUSES, sets.clone()),
                                catch_async(|()| MData::SetBonus(MDataContainer::OnSaved)),
                            )
                        })
                }
                MDataContainer::OnSaved => {
                    self.data.set_bonuses.saving = false;
                    Command::none()
                }
                MDataContainer::Modified => {
                    self.data.set_bonuses.modified = true;
                    Command::none()
                }
            },
        }
    }
}
