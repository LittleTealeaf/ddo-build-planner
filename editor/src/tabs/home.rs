use iced::{
    widget::{button, text},
    Application, Command, Element, Renderer,
};
use iced_aw::modal;
use ui::{HandleMessage, HandleView};

use crate::{
    widgets::attribute_selector::{AttributeSelector, MAttributeSelector},
    Editor, Message,
};

#[derive(Debug, Clone, Default)]
pub struct THome {
    selector: Option<AttributeSelector<MHome>>,
}

#[derive(Debug, Clone)]
pub enum MHome {
    Selector(MAttributeSelector),
    OpenSelection,
    CancelSelection,
}

impl From<MHome> for Message {
    fn from(value: MHome) -> Self {
        Self::Home(value)
    }
}

impl From<MAttributeSelector> for MHome {
    fn from(value: MAttributeSelector) -> Self {
        Self::Selector(value)
    }
}

impl HandleMessage<MHome> for Editor {
    fn handle_message(&mut self, message: MHome) -> Command<<Self as Application>::Message> {
        match message {
            MHome::Selector(message) => self
                .home
                .selector
                .as_mut()
                .map_or_else(Command::none, |selector| selector.handle_message(message)),
            MHome::OpenSelection => {
                self.home.selector = Some(AttributeSelector::new(
                    self.generate_attributes(),
                    None,
                    MHome::CancelSelection.into(),
                    MHome::CancelSelection.into(),
                ));
                Command::none()
            }
            MHome::CancelSelection => {
                self.home.selector = None;
                Command::none()
            }
        }
    }
}

impl HandleView<Editor> for THome {
    fn handle_view<'a>(
        &'a self,
        app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer<<Editor as Application>::Theme>>
    {
        modal(
            button(text("Open Selection")).on_press(MHome::OpenSelection.into()),
            self.selector
                .as_ref()
                .map(|selector| selector.handle_view(app)),
        )
        .on_esc(MHome::CancelSelection.into())
        .backdrop(MHome::CancelSelection.into())
        .into()
    }
}
