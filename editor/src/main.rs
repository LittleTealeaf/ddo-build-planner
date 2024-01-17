//! Editor Application

mod data_load;
mod tabs;

use builder::equipment::set_bonus::SetBonus;
use data_load::DataMessage;
use iced::{
    executor, font,
    widget::{column, container, text},
    Application, Command, Settings, Theme,
};
use iced_aw::{graphics::icons::ICON_FONT_BYTES, TabBar, TabLabel};
use tabs::{MessageSetBonuses, TabHome, TabSetBonuses};
use ui::{HandleMessage, HandleView};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Debug, Clone, Default)]
struct Editor {
    set_bonuses: Option<Vec<SetBonus>>,
    font_loaded: bool,
    tab_home: TabHome,
    tab_set_bonuses: TabSetBonuses,
    current_tab: Tab,
}

#[derive(Debug, Clone)]
enum Message {
    Data(DataMessage),
    Error(String),
    SetTab(Tab),
    SetBonuses(MessageSetBonuses),
    FontLoaded,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
enum Tab {
    #[default]
    Home,
    SetBonuses,
}

impl Application for Editor {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut app = Self {
            font_loaded: false,
            ..Default::default()
        };
        let command = Command::batch([
            app.handle_message(Message::Data(DataMessage::LoadSetBonuses)),
            font::load(ICON_FONT_BYTES).map(|res| {
                res.map_or_else(
                    |e| Message::Error(format!("{e:?}")),
                    |()| Message::FontLoaded,
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
        if self.font_loaded {
            column!(
                [(Tab::Home, "Home"), (Tab::SetBonuses, "Set Bonuses")]
                    .into_iter()
                    .fold(TabBar::new(Message::SetTab), |bar, (id, label)| {
                        bar.push(id, TabLabel::Text(label.to_owned()))
                    })
                    .set_active_tab(&self.current_tab),
                match self.current_tab {
                    Tab::Home => HandleView::<TabHome>::handle_view(self),
                    Tab::SetBonuses => HandleView::<TabSetBonuses>::handle_view(self),
                }
            )
            .into()
        } else {
            container(text("Loading...").size(10))
                .center_x()
                .center_y()
                .into()
        }
    }
}

impl HandleMessage<Message> for Editor {
    fn handle_message(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Data(message) => self.handle_message(message),
            Message::Error(error) => panic!("{error}"),
            Message::SetTab(tab) => {
                self.current_tab = tab;
                Command::none()
            }
            Message::SetBonuses(message) => self.handle_message(message),
            Message::FontLoaded => {
                self.font_loaded = true;
                Command::none()
            }
        }
    }
}
