use components::crash::Crash;
use iced::{
    executor, theme,
    widget::{button, column, container, text},
    Application, Command, Length, Settings,
};
use libs::git::GitRepo;
use utils::iced::{HandleMessageOld, HandleView};

mod components;
mod libs;

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

pub enum Editor {
    Crashed(Crash),
    Loaded(State),
}

pub struct State {
    repository: GitRepo,
}

#[derive(Debug, Clone)]
pub enum Message {
    CustomCrash(String),
    CrashMessage(components::crash::Message),
}

impl Application for Editor {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let repository = match GitRepo::open() {
            Err(e) => Self::Crashed(e.into()),
            Ok(repository) => Self::Loaded(State { repository }),
        };

        (repository, Command::none())
    }

    fn title(&self) -> String {
        "Editor".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CustomCrash(crash_message) => {
                *self = Self::Crashed(Crash::SimpleError(crash_message));
                Command::none()
            }
            Message::CrashMessage(message) => {
                if let Self::Crashed(crash) = self {
                    crash.update(message)
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self {
            Self::Crashed(crash) => crash.view(),
            Self::Loaded(_state) => container(column(vec![
                text("Hi world, it loaded".to_string()).into(),
                button("Cause Crash")
                    .on_press(Message::CustomCrash("I Crashed".to_string()))
                    .into(),
            ]))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into(),
        }
    }
}
