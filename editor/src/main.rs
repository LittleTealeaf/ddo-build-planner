//! Application starting point for the Editor Application

use builder::equipment::set_bonus::SetBonus;
use editor::data::load_set_bonuses;
use iced::{
    executor, theme,
    widget::{Scrollable, Text},
    Application, Command, Settings, Subscription,
};

fn main() -> iced::Result {
    EditorState::run(Settings::default())
}

#[derive(Debug)]
struct EditorState {
    set_bonuses: Option<Vec<SetBonus>>,
}

#[derive(Debug, Clone)]
enum EditorMessage {
    LoadedSetBonuses(Vec<SetBonus>),
}

impl Application for EditorState {
    type Executor = executor::Default;

    type Message = EditorMessage;

    type Theme = theme::Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self { set_bonuses: None },
            Command::perform(async { load_set_bonuses().await }, |data| {
                EditorMessage::LoadedSetBonuses(data.unwrap_or_default())
            }),
        )
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            EditorMessage::LoadedSetBonuses(set_bonuses) => self.set_bonuses = Some(set_bonuses),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Scrollable::new(Text::new(if self.set_bonuses.is_some() {
            "loaded"
        } else {
            "not loaded"
        }))
        .into()
    }
}
