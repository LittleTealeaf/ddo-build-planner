use builder::attribute::Attribute;
use iced::{Application, Command};
use ui::HandleMessage;

use crate::Editor;

use super::{Selector, SelectorMessage};

#[derive(Debug, Clone)]
pub struct ValueSelector {
    selector: Option<Box<Selector>>,
}

#[derive(Debug, Clone)]
pub enum ValueSelectorMessage {}

impl HandleMessage<(usize, SelectorMessage, &[Attribute]), Editor> for ValueSelector {
    fn handle_message(
        &mut self,
        _message: (usize, SelectorMessage, &[Attribute]),
    ) -> Command<<Editor as Application>::Message> {
        todo!()
    }
}
