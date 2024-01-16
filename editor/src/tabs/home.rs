use iced::widget::text;

use crate::{EditorTab, state::AppState};

#[derive(Clone, Debug, Default)]
pub struct TabHome {

}

#[derive(Debug, Clone)]
pub enum TabHomeMessage {}

impl EditorTab for TabHome {
    type Message = TabHomeMessage;

    fn update(&mut self, _state: &mut crate::state::AppState, _message: Self::Message) -> iced::Command<crate::Message> {
        todo!()
    }

    fn view(&self, _state: &AppState) -> iced::Element<'_, crate::Message, iced::Renderer<iced::Theme>> {
        text("HELLO WORLD").size(100).into()
    }
}
