mod edit;
use edit::EditingSet;

use builder::equipment::set_bonus::SetBonus;
use fuzzy_filter::matches;
use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, scrollable, text, text_input},
    Alignment, Application, Command, Element, Length, Renderer,
};
use ui::{font::NERD_FONT, HandleMessage, HandleView};

use crate::{
    data::{MData, MDataContainer},
    Editor, Message,
};

use self::edit::MEditingSet;

const DATA_PATH: &str = "./data/data/set_bonuses.ron";

#[derive(Debug, Clone, Default)]
pub struct TSetBonuses {
    editing: Option<EditingSet>,
    filter: String,
}

#[derive(Debug, Clone)]
pub enum MSetBonuses {
    OpenSet(usize),
    NewSet,
    EditSet(EditingSet),
    DeleteSet(usize),
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
    fn handle_message(&mut self, message: MSetBonuses) -> Command<<Self as Application>::Message> {
        match message {
            MSetBonuses::NewSet => self.handle_message(MSetBonuses::EditSet(EditingSet::new(
                SetBonus::new(self.set_bonuses.filter.clone()),
            ))),
            MSetBonuses::OpenSet(index) => self
                .data
                .set_bonuses
                .data
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
                if let Some(sets) = &mut self.data.set_bonuses.data {
                    if let Some(edit) = &self.set_bonuses.editing {
                        self.data.set_bonuses.modified = true;
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
            MSetBonuses::DeleteSet(index) => {
                if let Some(sets) = &mut self.data.set_bonuses.data {
                    if index < sets.len() {
                        sets.remove(index);
                    }
                }
                self.set_bonuses.editing = None;
                self.data.set_bonuses.modified = true;

                Command::none()
            }
        }
    }
}

impl HandleView<Editor> for TSetBonuses {
    fn handle_view<'a>(
        &'a self,
        app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer>
    {

        self.editing.as_ref().map_or_else(
            || {
                column(vec![
                    row!(
                        text_input("Search...", &self.filter)
                            .on_input(|search| MSetBonuses::Filter(search).into()),
                        button(text('').font(NERD_FONT)).on_press_maybe(
                            (!&self.filter.is_empty()).then_some(MSetBonuses::NewSet.into())
                        ),
                        horizontal_space().width(10.0),
                        button(text('󰑓').font(NERD_FONT))
                            .on_press(MData::SetBonus(MDataContainer::Load).into()),
                        horizontal_space().width(5.0),
                        button(text('').font(NERD_FONT)).on_press_maybe(
                            (app.data.set_bonuses.modified && !app.data.set_bonuses.saving)
                                .then_some(MData::SetBonus(MDataContainer::Save).into())
                        ),
                        horizontal_space().width(2.0),
                    )
                    .into(),
                    app.data.set_bonuses.data.as_ref().map_or_else(
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
