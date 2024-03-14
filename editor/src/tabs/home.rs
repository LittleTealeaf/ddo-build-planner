use iced::{Application, Element};
use ui::HandleView;

use crate::Editor;

#[derive(Debug, Clone, Default)]
pub struct TabHome {}

impl HandleView<Editor> for TabHome {
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, <Editor as Application>::Theme, iced::Renderer>
    {
        todo!()
    }
}
