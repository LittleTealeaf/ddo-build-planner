use iced::{
    widget::{button, column, container, text},
    window, Application, Command, Element, Length,
};
use utils::iced::{HandleMessage, HandleView};

use crate::{Editor, Message};

#[derive(Debug)]
pub enum Crash {
    SimpleError(String),
    LoadRepoError(git2::Error),
}

#[derive(Debug, Clone, Copy)]
pub enum CrashMessage {
    CloseApplication,
}

impl From<CrashMessage> for Message {
    fn from(value: CrashMessage) -> Self {
        Self::CrashMessage(value)
    }
}

impl HandleView<Editor> for Crash {
    fn view(
        &self,
    ) -> Element<'_, <Editor as Application>::Message, iced::Renderer<<Editor as Application>::Theme>>
    {
        container(column(vec![
            text("Oh no! It Crashed!").into(),
            button("Close")
                .on_press(CrashMessage::CloseApplication.into())
                .into(),
        ]))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

impl HandleMessage<Editor> for Crash {
    type Message = CrashMessage;

    fn update(
        &mut self,
        message: Self::Message,
    ) -> iced::Command<<Editor as Application>::Message> {
        match message {
            CrashMessage::CloseApplication => window::close(),
        }
    }
}

impl From<git2::Error> for Crash {
    fn from(value: git2::Error) -> Self {
        Self::LoadRepoError(value)
    }
}
