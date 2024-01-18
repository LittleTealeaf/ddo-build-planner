use iced::widget::text;
use ui::HandleView;

use crate::Editor;



pub struct THome;

impl HandleView<THome> for Editor {
    fn handle_view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text("hi").into()
    }
}
