use iced::{widget::text, Application, Element, Renderer};
use ui::HandleView;

use crate::Editor;

pub struct THome;

impl HandleView<Editor> for THome {
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer<<Editor as Application>::Theme>>
    {
        text("hi").into()
    }
}
