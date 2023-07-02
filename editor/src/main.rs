use iced::{executor, theme, Application, Command, Settings};

fn main() -> iced::Result {
    EditorState::run(Settings::default())
}

#[derive(Debug)]
struct EditorState {}

#[derive(Debug, Clone, Copy)]
enum EditorMessage {}

impl Application for EditorState {
    type Executor = executor::Default;

    type Message = EditorMessage;

    type Theme = theme::Theme;

    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        "Hello World".into()
    }
}
