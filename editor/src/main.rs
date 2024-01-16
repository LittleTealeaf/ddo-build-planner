//! Handles Application State and misc. logic

mod messages;

use builder::equipment::set_bonus::SetBonus;

use iced::{executor, theme, widget::text, Application as IcedApplication, Command, Settings};
use messages::{DataIOMessage, DataMessage, HandleMessage, Message};

fn main() -> iced::Result {
    Application::run(Settings::default())
}

/// Application state and additional logic
#[derive(Debug, Clone)]
pub struct Application {
    pub(crate) set_bonuses: Option<Vec<SetBonus>>,
    pub(crate) state: AppState,
}

#[derive(Debug, Clone)]
enum AppState {
    Home,
}

impl IcedApplication for Application {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut app = Self {
            set_bonuses: None,
            state: AppState::Home,
        };

        let command = Command::batch([app.update(Message::Data(DataMessage::SetBonuses(
            DataIOMessage::StartLoad,
        )))]);

        (app, command)
    }

    fn title(&self) -> String {
        String::from("DDO Build Planner Data Editor")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        message.handle(self)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text("HI WORLD").size(100).into()
    }
}
