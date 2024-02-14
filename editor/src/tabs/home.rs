use iced::{widget::text, Application, Command, Element, Renderer};
use ui::{HandleMessage, HandleView};

use crate::{Editor, Message};

#[derive(Debug, Clone, Default)]
pub struct THome {}

#[derive(Debug, Clone)]
pub enum MHome {
    Hi,
}

impl From<MHome> for Message {
    fn from(value: MHome) -> Self {
        Self::Home(value)
    }
}

impl HandleMessage<MHome> for Editor {
    fn handle_message(&mut self, message: MHome) -> Command<<Self as Application>::Message> {
        match message {
            MHome::Hi => Command::none(),
        }
    }
}

impl HandleView<Editor> for THome {
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer<<Editor as Application>::Theme>>
    {
        text("hi").into()
        // modal(
        //     button(text("Open Selection")).on_press(MHome::OpenSelection.into()),
        //     self.selector.as_ref().map(|selector| {
        //         selector.view(
        //             |val| MHome::SubmitSelection(val).into(),
        //             |message| MHome::MSelector(Box::new(message)).into(),
        //         )
        //     }),
        // )
        // .into()
    }
}
