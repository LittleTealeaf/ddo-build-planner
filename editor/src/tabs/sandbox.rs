use builder::{attribute::Attribute, breakdowns::Breakdowns, equipment::set_bonus::ItemSet};
use iced::{widget::text, Application, Command, Element, Renderer};
use ui::{error, ExecuteMessage, HandleMessage, HandleView};

use crate::{App, Message};

#[derive(Debug, Clone)]
pub struct TabSandbox {
    breakdowns: Breakdowns,
}

impl TabSandbox {
    pub fn new() -> Self {
        Self {
            breakdowns: Breakdowns::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TabSandboxMessage {
    NewBreakdowns,
    RefreshItemSets,
    TrackAttribute(Attribute),
    UntrackAttribute(Attribute),
}

impl From<TabSandboxMessage> for Message {
    fn from(value: TabSandboxMessage) -> Self {
        Self::TabSandbox(value)
    }
}

impl HandleMessage<TabSandboxMessage> for App {
    fn handle_message(
        &mut self,
        message: TabSandboxMessage,
    ) -> Command<<Self as Application>::Message> {
        let tab = &mut self.tab_sandbox;

        match message {
            TabSandboxMessage::NewBreakdowns => {
                tab.breakdowns = Breakdowns::new();
                self.handle_message(TabSandboxMessage::RefreshItemSets)
            }
            TabSandboxMessage::RefreshItemSets => {
                let Some(item_sets) = self.data.item_sets.get() else {
                    return Command::message(error!("Item Sets Not Loaded"));
                };

                let dynamic_bonuses = item_sets.iter().cloned().map(ItemSet::to_dynamic_bonus);
                tab.breakdowns.import_dynamic_bonuses(dynamic_bonuses);

                Command::none()
            }
            TabSandboxMessage::TrackAttribute(_) => todo!(),
            TabSandboxMessage::UntrackAttribute(_) => todo!(),
        }
    }
}

impl HandleView<App> for TabSandbox {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        text("Hi").into()
    }
}
