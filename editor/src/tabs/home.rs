use iced::widget::text;
use ui::HandleView;

use crate::Editor;

#[derive(Default, Clone, Debug)]
pub struct TabHome;

impl HandleView<TabHome> for Editor {
    fn handle_view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text("Home").into()
    }
}
