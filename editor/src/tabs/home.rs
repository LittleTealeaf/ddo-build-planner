use iced::{widget::text, Application, Element};
use ui::HandleView;

use crate::App;

#[derive(Debug, Clone, Default)]
pub struct TabHome {}

impl HandleView<App> for TabHome {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, iced::Renderer>
    {
        text("hi world").into()
    }
}
