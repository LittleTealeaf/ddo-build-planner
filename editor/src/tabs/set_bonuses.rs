use iced::widget::text;

use crate::{EditorView, Editor};



#[derive(Clone, Debug, Default)]
pub struct TabSetBonuses {

}

impl EditorView<TabSetBonuses> for Editor {
    fn handle_view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text("Set Bonuses").into()
    }
}


