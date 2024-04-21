mod edit_set;

use fuzzy_filter::matches;
use iced::{
    theme,
    widget::{button, column, scrollable, text, text_input, Column, Row},
    Application, Command, Element, Renderer,
};
use ui::{font::NERD_FONT, HandleMessage, HandleView};

use crate::{App, Message};

use self::edit_set::{ItemSetEditor, ItemSetEditorMessage};

#[derive(Debug, Clone, Default)]
pub struct TabSetBonuses {
    filter: String,
    editing: Option<ItemSetEditor>,
}

#[derive(Debug, Clone)]
pub enum TabSetBonusesMessage {
    Filter(String),
    NewSet,
    Edit(usize),
    CancelEdit,
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
            TabSetBonusesMessage::NewSet => todo!(),
            TabSetBonusesMessage::Edit(index) => {
                if let Some(set) = self.data.item_sets().and_then(|sets| sets.get(index)) {
                    self.tab_item_sets.editing = Some(ItemSetEditor::new(set.clone()));
                }
                Command::none()
            }
            TabSetBonusesMessage::CancelEdit => {
                self.tab_item_sets.editing = None;
                Command::none()
            }
            TabSetBonusesMessage::Editing(message) => self.handle_message(message),
        }
    }
}

impl HandleView<App> for TabSetBonuses {
    fn handle_view<'a>(
        &'a self,
        app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        Column::new()
            .push(
                Row::new()
                    .push(
                        text_input("Search...", &self.filter)
                            .on_input(|filter| TabSetBonusesMessage::Filter(filter).into()),
                    )
                    .push(button(text("󰝒").font(NERD_FONT)).on_press_maybe(
                        (!&self.filter.is_empty()).then_some(TabSetBonusesMessage::NewSet.into()),
                    )),
            )
            .push_maybe(app.data.item_sets().map(|sets| {
                scrollable(column(
                    sets.iter()
                        .enumerate()
                        .filter(|(_, set)| matches(&self.filter, set.name()))
                        .map(|(_index, set)| {
                            button(text(set.name())).style(theme::Button::Text).into()
                        }),
                ))
            }))
            .into()
    }
}
