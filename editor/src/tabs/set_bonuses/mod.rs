mod edit;
use edit::EditingSet;

use builder::equipment::set_bonus::SetBonus;
use fuzzy_filter::matches;
use iced::{
    theme,
    widget::{button, column, container, row, scrollable, text, text_input},
    Alignment, Application, Command, Element, Length, Renderer,
};
use ui::{font::NERD_FONT, HandleMessage, HandleView};

use crate::{
    data_utils::{catch_async, load_data, save_data},
    Editor, Message,
};

use self::edit::MEditingSet;

const DATA_PATH: &str = "./data/data/set_bonuses.ron";

#[derive(Debug, Clone, Default)]
pub struct TSetBonuses {
    sets: Option<Vec<SetBonus>>,
    saving: bool,
    editing: Option<EditingSet>,
    modified: bool,
    filter: String,
}

#[derive(Debug, Clone)]
pub enum MSetBonuses {
    LoadSets,
    OnLoadSets(Vec<SetBonus>),
    SaveSets,
    OnSaveSets,
    OpenSet(usize),
    EditSet(EditingSet),
    CancelEdit,
    SaveEdit,
    Edit(MEditingSet),
    Filter(String),
}

impl From<MSetBonuses> for Message {
    fn from(value: MSetBonuses) -> Self {
        Self::SetBonuses(value)
    }
}

impl HandleMessage<MSetBonuses> for Editor {
    fn handle_message(&mut self, message: MSetBonuses) -> iced::Command<Self::Message> {
        match message {
            MSetBonuses::LoadSets => {
                self.set_bonuses.sets = None;
                Command::perform(
                    load_data(DATA_PATH),
                    catch_async(|data| MSetBonuses::OnLoadSets(data).into()),
                )
            }
            MSetBonuses::OnLoadSets(sets) => {
                self.set_bonuses.sets = Some(sets);
                Command::none()
            }
            MSetBonuses::SaveSets => {
                self.set_bonuses
                    .sets
                    .as_ref()
                    .map_or_else(Command::none, |sets| {
                        self.set_bonuses.saving = true;
                        Command::perform(
                            save_data(DATA_PATH, sets.clone()),
                            catch_async(|()| MSetBonuses::OnSaveSets.into()),
                        )
                    })
            }
            MSetBonuses::OnSaveSets => {
                self.set_bonuses.saving = false;
                self.set_bonuses.modified = false;
                Command::none()
            }

            MSetBonuses::OpenSet(index) => self
                .set_bonuses
                .sets
                .as_ref()
                .and_then(|sets| sets.get(index))
                .cloned()
                .map_or_else(Command::none, |set| {
                    self.handle_message(MSetBonuses::EditSet(EditingSet::from_index(index, set)))
                }),
            MSetBonuses::EditSet(set) => {
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
                        if let Some(index) = edit.index() {
                            sets[*index] = edit.set().clone();
                        } else {
                            sets.push(edit.set().clone());
                        }
                    }
                }
                self.set_bonuses.editing = None;
                Command::none()
            }
            MSetBonuses::Edit(message) => self.handle_message(message),
            MSetBonuses::Filter(search) => {
                self.set_bonuses.filter = search;
                Command::none()
            }
        }
    }
}

impl HandleView<Editor> for TSetBonuses {
    fn handle_view<'a>(
        &'a self,
        app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer<<Editor as Application>::Theme>>
    {
        self.editing.as_ref().map_or_else(
            || {
                column(vec![
                    row!(
                        text_input("Search...", &self.filter)
                            .on_input(|search| MSetBonuses::Filter(search).into()),
                        button(text('󰑓').font(NERD_FONT)).on_press(MSetBonuses::LoadSets.into()),
                        button(text('').font(NERD_FONT)).on_press_maybe(
                            (self.modified && !self.saving).then_some(MSetBonuses::SaveSets.into())
                        ),
                    )
                    .into(),
                    self.sets.as_ref().map_or_else(
                        || {
                            container(text("Loading...").size(30))
                                .center_x()
                                .center_y()
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .into()
                        },
                        |sets| {
                            scrollable(column(
                                sets.iter()
                                    .enumerate()
                                    .filter(|(_, set)| matches(&self.filter, set.name()))
                                    .map(|(index, set)| {
                                        row!(
                                            button(text('').font(NERD_FONT))
                                                .on_press(MSetBonuses::OpenSet(index).into())
                                                .style(theme::Button::Text),
                                            text(set.name())
                                        )
                                        .align_items(Alignment::Center)
                                        .into()
                                    })
                                    .collect(),
                            ))
                            .into()
                        },
                    ),
                ])
                .into()
            },
            |editing| editing.handle_view(app),
        )
    }
}
