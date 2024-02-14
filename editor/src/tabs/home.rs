use builder::bonus::Condition;
use iced::{
    widget::{button, text},
    Application, Command, Element, Renderer,
};
use iced_aw::modal;
use ui::{HandleMessage, HandleView};

use crate::{
    widgets::bonus_selector::{
        condition::{ConditionSelector, MConditionSelector},
        BonusSelector,
    },
    Editor, Message,
};

#[derive(Debug, Clone, Default)]
pub struct THome {
    selector: Option<BonusSelector<ConditionSelector>>,
}

#[derive(Debug, Clone)]
pub enum MHome {
    OpenSelection,
    SubmitSelection(Option<Condition>),
    MSelector(Box<MConditionSelector>),
}

impl From<MHome> for Message {
    fn from(value: MHome) -> Self {
        Self::Home(value)
    }
}

impl HandleMessage<MHome> for Editor {
    fn handle_message(&mut self, message: MHome) -> Command<<Self as Application>::Message> {
        match message {
            MHome::OpenSelection => {
                self.home.selector = Some(BonusSelector::new(self.generate_attributes()));
                Command::none()
            }
            MHome::MSelector(m) => self
                .home
                .selector
                .as_mut()
                .map_or_else(Command::none, |selector| selector.message(*m)),
            MHome::SubmitSelection(selection) => {
                println!("{selection:?}");
                self.home.selector = None;
                Command::none()
            }
        }
    }
}

impl HandleView<Editor> for THome {
    fn handle_view<'a>(
        &'a self,
        _app: &'a Editor,
    ) -> Element<'_, <Editor as Application>::Message, Renderer<<Editor as Application>::Theme>>
    {
        modal(
            button(text("Open Selection")).on_press(MHome::OpenSelection.into()),
            self.selector.as_ref().map(|selector| {
                selector.view(
                    |val| MHome::SubmitSelection(val).into(),
                    |message| MHome::MSelector(Box::new(message)).into(),
                )
            }),
        )
        .into()
    }
}
