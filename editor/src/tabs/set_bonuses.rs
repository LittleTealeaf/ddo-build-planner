use iced::{
    widget::{container, text},
    Length,
};

use crate::{Editor, EditorView};

#[derive(Clone, Debug, Default)]
pub struct TabSetBonuses {
    open_index: Option<usize>,
}

impl EditorView<TabSetBonuses> for Editor {
    fn handle_view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self.set_bonuses {
            None => container(text("Loading Set Bonuses..."))
                .height(Length::Fill)
                .width(Length::Fill)
                .center_x()
                .center_y()
                .into(),
            Some(_) => text("Set Bonuses").into(),
        }
    }
}
