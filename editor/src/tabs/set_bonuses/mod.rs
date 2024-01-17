use builder::equipment::set_bonus::SetBonus;
use iced::Command;
use ui::HandleMessage;

use crate::{
    data_utils::{catch_async, load_data, save_data},
    Editor, Message,
};

const DATA_PATH: &str = "./data/data/set_bonuses.ron";

#[derive(Debug, Clone, Default)]
pub struct SetBonuses {
    sets: Option<Vec<SetBonus>>,
    saving: bool,
    editing: Option<EditingSet>,
    modified: bool,
}

#[derive(Debug, Clone)]
pub struct EditingSet {
    index: Option<usize>,
    set: SetBonus,
}

#[derive(Debug, Clone)]
pub enum MSetBonuses {
    LoadSets,
    OnLoadSets(Vec<SetBonus>),
    SaveSets,
    OnSaveSets,
    Edit(EditingSet),
    CancelEdit,
    SaveEdit,
}

impl HandleMessage<MSetBonuses> for Editor {
    fn handle_message(&mut self, message: MSetBonuses) -> iced::Command<Self::Message> {
        match message {
            MSetBonuses::LoadSets => {
                self.set_bonuses.sets = None;
                Command::perform(
                    load_data(DATA_PATH),
                    catch_async(|data| Message::SetBonuses(MSetBonuses::OnLoadSets(data))),
                )
            }
            MSetBonuses::OnLoadSets(sets) => {
                self.set_bonuses.sets = Some(sets);
                Command::none()
            }
            MSetBonuses::SaveSets => {
                self.set_bonuses.saving = true;
                self.set_bonuses
                    .sets
                    .as_ref()
                    .map_or_else(Command::none, |sets| {
                        Command::perform(
                            save_data(DATA_PATH, sets.clone()),
                            catch_async(|()| Message::SetBonuses(MSetBonuses::OnSaveSets)),
                        )
                    })
            }
            MSetBonuses::OnSaveSets => {
                self.set_bonuses.saving = false;
                self.set_bonuses.modified = false;
                Command::none()
            }
            MSetBonuses::Edit(set) => {
                self.set_bonuses.editing = Some(set);
                Command::none()
            }
            MSetBonuses::CancelEdit => {
                self.set_bonuses.editing = None;
                Command::none()
            }
            MSetBonuses::SaveEdit => {
                if let Some(sets) = &mut self.set_bonuses.sets {
                    if let Some(edit) = &self.set_bonuses.editing {
                        self.set_bonuses.modified = true;
                        if let Some(index) = edit.index {
                            sets[index] = edit.set.clone();
                        } else {
                            sets.push(edit.set.clone());
                        }
                    }
                }
                self.set_bonuses.editing = None;
                Command::none()
            }
        }
    }
}
