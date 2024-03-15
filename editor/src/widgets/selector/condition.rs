use builder::attribute::Attribute;
use iced::{Application, Command};
use ui::HandleMessage;

use crate::Editor;

use super::SelectorMessage;

#[derive(Debug, Clone)]
pub struct ConditionSelector {}

#[derive(Debug, Clone)]
pub enum ConditionSelectorMessage {}

impl HandleMessage<(usize, SelectorMessage, &[Attribute]), Editor> for ConditionSelector {
    fn handle_message(
        &mut self,
        _message: (usize, SelectorMessage, &[Attribute]),
    ) -> Command<<Editor as Application>::Message> {
        todo!()
    }
}
