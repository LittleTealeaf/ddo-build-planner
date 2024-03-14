use iced::{Application, Command};
use ui::HandleMessage;

use crate::Editor;

#[derive(Debug, Clone, Default)]
pub struct TabSetBonuses {
    filter: String,
}

#[derive(Debug, Clone)]
pub enum TabSetBonusesMessage {}

impl HandleMessage<TabSetBonusesMessage> for Editor {
    fn handle_message(
        &mut self,
        _message: TabSetBonusesMessage,
    ) -> Command<<Self as Application>::Message> {
        todo!()
    }
}
