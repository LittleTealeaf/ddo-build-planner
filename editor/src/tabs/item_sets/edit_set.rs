use builder::equipment::set_bonus::ItemSet;
use iced::{Application, Command};
use ui::HandleMessage;

use crate::App;

#[derive(Debug, Clone)]
pub struct ItemSetEditor {
    item_set: ItemSet,
}

impl ItemSetEditor {
    pub const fn new(item_set: ItemSet) -> Self {
        Self { item_set }
    }
}

#[derive(Debug, Clone)]
pub enum ItemSetEditorMessage {}

impl HandleMessage<ItemSetEditorMessage> for App {
    fn handle_message(
        &mut self,
        _message: ItemSetEditorMessage,
    ) -> Command<<Self as Application>::Message> {
        Command::none()
    }
}
