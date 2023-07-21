use iced::{
    executor, theme,
    widget::{button, column},
    Application, Command, Settings,
};
use iced_aw::menu::{MenuBar, MenuTree};

mod git;

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
        let menu_file = MenuTree::with_children(
            button("File"),
            vec![
                MenuTree::new(button("Open")),
                MenuTree::new(button("Close")),
            ],
        );

        let menu_help = MenuTree::with_children(
            button("Help"),
            vec![
                MenuTree::new(button("Open")),
                MenuTree::new(button("Close")),
            ],
        );

        let menu_bar = MenuBar::new(vec![menu_file, menu_help]);

        column![menu_bar].into()
    }
}
