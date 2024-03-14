//! Editor Application
mod data;

use data::{container::DataContainerMessage, Data, DataMessage};
use iced::{executor, font, widget::text, Application, Command, Settings, Theme};
use ui::{font::NERD_FONT_BYTES, HandleMessage};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Clone, Debug)]
struct Editor {
    data: Data,
    icons_loaded: bool,
}

#[derive(Clone, Debug)]
enum Message {
    IconsLoaded,
    Data(DataMessage),
    Error(String),
}

impl Application for Editor {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, Command<Self::Message>) {
        let mut editor = Self {
            data: Data::default(),
            icons_loaded: false,
        };

        let command = Command::batch([
            editor.handle_message(DataMessage::SetBonuses(DataContainerMessage::Load)),
            font::load(NERD_FONT_BYTES).map(|res| {
                res.map_or_else(
                    |e| Message::Error(format!("{e:?}")),
                    |()| Message::IconsLoaded,
                )
            }),
        ]);

        (editor, command)
    }

    fn title(&self) -> String {
        String::from("DDO Build Planner Editor")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.handle_message(message)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        text(format!(
            "Icons Loaded: {}, data loaded: {:?}",
            self.icons_loaded, self.data.set_bonuses.data
        ))
        .into()
    }
}

impl HandleMessage<Message> for Editor {
    fn handle_message(&mut self, message: Message) -> Command<<Self as Application>::Message> {
        match message {
            Message::IconsLoaded => {
                self.icons_loaded = true;
                Command::none()
            }
            Message::Data(message) => self.handle_message(message),
            Message::Error(err) => panic!("{err}"),
        }
    }
}
