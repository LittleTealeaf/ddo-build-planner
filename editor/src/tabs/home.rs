use iced::widget::text;

use crate::{Editor, EditorView};

#[derive(Default, Clone, Debug)]
pub struct TabHome;

impl EditorView<TabHome> for Editor {
    fn handle_view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text("Home").into()
    }
}
