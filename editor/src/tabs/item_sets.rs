mod edit_set;

use builder::equipment::set_bonus::ItemSet;
use fuzzy_filter::matches;
use iced::{
    widget::{button, column, row, scrollable, text, text_input, Column, Row},
    Application, Command, Element, Renderer,
};
use ui::{error, font::nf_icon, HandleMessage, HandleView};

use crate::{
    data::{container::DataContainerMessage, DataMessage},
    App, Message,
};

use self::edit_set::{ItemSetEditor, ItemSetEditorMessage};

#[derive(Debug, Clone, Default)]
pub struct TabItemSets {
    filter: String,
    editing: Option<ItemSetEditor>,
}

#[derive(Debug, Clone)]
pub enum TabSetBonusesMessage {
    Filter(String),
    NewSet,
    Edit(usize),
    CancelEdit,
    SaveEdit,
    Editing(ItemSetEditorMessage),
}

impl From<TabSetBonusesMessage> for Message {
    fn from(value: TabSetBonusesMessage) -> Self {
        Self::TabSetBonuses(value)
    }
}

impl HandleMessage<TabSetBonusesMessage> for App {
    fn handle_message(
        &mut self,
        message: TabSetBonusesMessage,
    ) -> Command<<Self as Application>::Message> {
        match message {
            TabSetBonusesMessage::Filter(filter) => {
                self.tab_item_sets.filter = filter;
                Command::none()
            }
            TabSetBonusesMessage::NewSet => {
                self.tab_item_sets.editing = Some(ItemSetEditor::new(
                    ItemSet::new(self.tab_item_sets.filter.clone()),
                    None,
                ));
                Command::none()
            }
            TabSetBonusesMessage::Edit(index) => {
                let Some(item_sets) = self.data.item_sets.get() else {
                    return self.handle_message(error!("Item Sets Not Loaded"));
                };

                let Some(set) = item_sets.get(index) else {
                    return self.handle_message(error!("Invalid Index: {index}"));
                };

                self.tab_item_sets.editing = Some(ItemSetEditor::new(set.clone(), Some(index)));

                Command::none()
            }
            TabSetBonusesMessage::CancelEdit => {
                self.tab_item_sets.editing = None;
                Command::none()
            }
            TabSetBonusesMessage::Editing(message) => self.handle_message(message),
            TabSetBonusesMessage::SaveEdit => {
                let Some(item_sets) = self.data.item_sets.get_mut() else {
                    return self.handle_message(error!("Item Sets Not Loaded"));
                };

                let Some(editor) = &self.tab_item_sets.editing else {
                    return self.handle_message(error!("No Editing Item Sets Open"));
                };

                if let Some(index) = editor.index {
                    let Some(pointer) = item_sets.get_mut(index) else {
                        return self.handle_message(error!("Invalid Index: {index}"));
                    };

                    *pointer = editor.item_set.clone();
                } else {
                    item_sets.push(editor.item_set.clone());
                }

                Command::batch([
                    self.handle_message(DataMessage::SetBonuses(DataContainerMessage::Modified)),
                    self.handle_message(TabSetBonusesMessage::CancelEdit),
                ])
            }
        }
    }
}

impl HandleView<App> for TabItemSets {
    fn handle_view<'a>(
        &'a self,
        app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        let ui = || {
            Column::new()
                .push(
                    Row::new()
                        .push(
                            text_input("Search...", &self.filter)
                                .on_input(|filter| TabSetBonusesMessage::Filter(filter).into()),
                        )
                        .push(
                            button(nf_icon("󰝒")).on_press_maybe(
                                (!&self.filter.is_empty())
                                    .then_some(TabSetBonusesMessage::NewSet.into()),
                            ),
                        ),
                )
                .push_maybe(app.data.item_sets.get().map(|sets| {
                    scrollable(column(
                        sets.iter()
                            .enumerate()
                            .filter(|(_, set)| matches(&self.filter, set.name()))
                            .map(|(index, set)| {
                                row!(
                                    button(nf_icon(""))
                                        .on_press(TabSetBonusesMessage::Edit(index).into()),
                                    text(set.name())
                                )
                                .into()
                            }),
                    ))
                }))
                .into()
        };

        self.editing
            .as_ref()
            .map_or_else(ui, |editor| editor.handle_view(app))
    }
}
