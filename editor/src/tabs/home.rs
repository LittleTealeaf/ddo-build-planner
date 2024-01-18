use iced::{
    widget::{column, scrollable, text},
    Application, Element, Renderer,
};
use ui::HandleView;

use crate::{utils::selectors::attribute::all_attributes, Editor};

pub struct THome;

impl HandleView<Editor> for THome {
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer<<Editor as Application>::Theme>>
    {
        scrollable(column(
            all_attributes()
                .map(|attribute| text(format!("{attribute}")).into())
                .collect(),
        )).into()
    }
}
