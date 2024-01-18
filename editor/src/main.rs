//! Editor Application

mod tabs;
mod utils;

use iced::{executor, font, Application, Command, Element, Renderer, Settings, Theme};
use tabs::{MSetBonuses, TSetBonuses, Tab};
use ui::{font::NERD_FONT_BYTES, HandleMessage, HandleView};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Clone, Debug)]
struct Editor {
    icons_loaded: bool,
    set_bonuses: TSetBonuses,
    tab: Tab,
}

#[derive(Clone, Debug)]
enum Message {
    IconsLoaded,
    Error(String),
    SetBonuses(MSetBonuses),
    ChangeTab(Tab),
}

impl Application for Editor {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Self {
            icons_loaded: false,
            set_bonuses: TSetBonuses::default(),
            tab: Tab::Home,
        };
        let command = Command::batch([
            app.handle_message(MSetBonuses::LoadSets),
            font::load(NERD_FONT_BYTES).map(|res| {
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

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.handle_message(message)
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        self.tab.handle_view(self)
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
            Message::ChangeTab(tab) => {
                self.tab = tab;
                Command::none()
            }
        }
    }
}
