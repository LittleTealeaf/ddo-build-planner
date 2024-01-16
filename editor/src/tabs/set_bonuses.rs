use iced::widget::text;

use crate::EditorTab;

#[derive(Debug, Clone, Default)]
pub struct TabSetBonuses {}

#[derive(Debug, Clone)]
pub enum TabSetBonusesMessage {}

impl EditorTab for TabSetBonuses {
    type Message = TabSetBonusesMessage;

    fn update(
        &mut self,
        _state: &mut crate::state::AppState,
        _message: Self::Message,
    ) -> iced::Command<crate::Message> {
        todo!()
    }

    fn view(
        &self,
        _state: &crate::state::AppState,
    ) -> iced::Element<'_, crate::Message, iced::Renderer<iced::Theme>> {
        text("I am a bird").into()
    }
}
