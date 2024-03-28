use iced::{
    widget::{button, column, text},
    Application, Element,
};
use ui::HandleView;

use crate::{App, Message};

#[allow(clippy::empty_structs_with_brackets)]
#[derive(Debug, Clone, Default)]
pub struct TabHome {}

impl HandleView<App> for TabHome {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, iced::Renderer>
    {
        column!(
            button(text("Attribute")).on_press(Message::DebugOpenAttribute),
            button(text("Value")).on_press(Message::DebugOpenValue),
            button(text("Condition")).on_press(Message::DebugOpenCondition),
        )
        .into()
    }
}
