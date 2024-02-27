use iced::{
    widget::{button, text},
    Application, Command, Element, Renderer,
};
use iced_aw::modal;
use ui::{HandleMessage, HandleView};

use crate::{
    widgets::modals::attribute::{AttributeSelector, MAttributeSelector},
    Editor, Message,
};

#[derive(Debug, Clone, Default)]
pub struct THome {
    selector: Option<AttributeSelector>,
}

#[derive(Debug, Clone)]
pub enum MHome {
    Selector(MAttributeSelector),
    OpenSelection,
    SubmitSelection,
    CloseSelection,
}

impl From<MHome> for Message {
    fn from(value: MHome) -> Self {
        Self::Home(value)
    }
}

impl HandleMessage<MHome> for Editor {
    fn handle_message(&mut self, message: MHome) -> Command<<Self as Application>::Message> {
        match message {
            MHome::Selector(message) => self
                .home
                .selector
                .as_mut()
                .map_or_else(Command::none, |selector| selector.message(message)),
            MHome::OpenSelection => {
                self.home.selector = Some(
                    AttributeSelector::new(self.generate_attributes())
                        .on_submit(MHome::SubmitSelection.into())
                        .on_cancel(MHome::CloseSelection.into()),
                );
                Command::none()
            }
            MHome::CloseSelection => {
                self.home.selector = None;
                Command::none()
            }
            MHome::SubmitSelection => {
                println!(
                    "{:?}",
                    self.home
                        .selector
                        .as_ref()
                        .and_then(|selector| selector.selected())
                );
                self.handle_message(MHome::CloseSelection)
            }
        }
    }
}

impl HandleView<Editor> for THome {
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer>
    {
        modal(
            button(text("Open Selection")).on_press(MHome::OpenSelection.into()),
            self.selector
                .as_ref()
                .map(|selector| selector.view(|message| MHome::Selector(message).into())),
        )
        .on_esc(MHome::CloseSelection.into())
        .backdrop(MHome::CloseSelection.into())
        .into()
    }
}
