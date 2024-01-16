//! Handles Application State and misc. logic

use builder::equipment::set_bonus::SetBonus;

use iced::{
    executor, theme,
    widget::{Scrollable, Text},
    Application as IcedApplication, Command,
};

use crate::messages::{HandleMessage, Message};

/// Application state and additional logic
#[derive(Debug, Clone)]
pub struct Application {
    set_bonuses: Option<Vec<SetBonus>>,
}

impl IcedApplication for Application {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self { set_bonuses: None },
            Command::none(), // Command::perform(load_set_bonuses(), |data| {
                             //     EditorMessage::LoadedSetBonuses(data.unwrap_or_default())
                             // }),
        )
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        message.handle(self)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Scrollable::new(Text::new("Hi")).into()
    }
}
