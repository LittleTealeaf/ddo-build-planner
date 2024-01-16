//! Handles Application State and misc. logic

use builder::equipment::set_bonus::SetBonus;

use iced::{
    executor, theme,
    widget::{Scrollable, Text},
    Application as IcedApplication, Command,
};

use crate::messages::{DataIOMessage, DataMessage, HandleMessage, Message};

/// Application state and additional logic
#[derive(Debug, Clone)]
pub struct Application {
    pub(crate) set_bonuses: Option<Vec<SetBonus>>,
}

impl IcedApplication for Application {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut app = Self { set_bonuses: None };

        let command = Command::batch([app.update(Message::Data(DataMessage::SetBonuses(
            DataIOMessage::StartLoad,
        )))]);

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        message.handle(self)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Scrollable::new(self.set_bonuses.as_ref().map_or_else(
            || Text::new("none"),
            |set_bonuses| Text::new(format!("{set_bonuses:?}")),
        ))
        .into()
    }
}
