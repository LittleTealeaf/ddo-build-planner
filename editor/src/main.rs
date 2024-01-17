//! Editor Application

mod data_utils;
mod tabs;

use iced::{executor, font, Application, Command, Settings, Theme};
use iced_aw::graphics::icons::ICON_FONT_BYTES;
use tabs::{MSetBonuses, SetBonuses};
use ui::HandleMessage;

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Clone, Debug)]
struct Editor {
    icons_loaded: bool,
    set_bonuses: SetBonuses,
}

#[derive(Clone, Debug)]
enum Message {
    IconsLoaded,
    Error(String),
    SetBonuses(MSetBonuses),
}

impl Application for Editor {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut app = Self {
            icons_loaded: false,
            set_bonuses: SetBonuses::default(),
        };
        let command = Command::batch([
            app.handle_message(MSetBonuses::LoadSets),
            font::load(ICON_FONT_BYTES).map(|res| {
                res.map_or_else(
                    |e| Message::Error(format!("{e:?}")),
                    |()| Message::IconsLoaded,
                )
            }),
        ]);

        (app, command)
    }

    fn title(&self) -> String {
        String::from("DDO Build Planner Editor")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        self.handle_message(message)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        todo!()
    }
}

impl HandleMessage<Message> for Editor {
    fn handle_message(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::IconsLoaded => {
                self.icons_loaded = true;
                Command::none()
            }
            Message::Error(err) => panic!("{err}"),
            Message::SetBonuses(message) => self.handle_message(message),
        }
    }
}
