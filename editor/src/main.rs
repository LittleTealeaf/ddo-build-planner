//! Editor Application

mod data_load;
mod tabs;

use builder::equipment::set_bonus::SetBonus;
use data_load::DataMessage;
use iced::{executor, widget::Column, Application, Command, Element, Renderer, Settings, Theme};
use iced_aw::{TabBar, TabLabel};
use tabs::{TabHome, TabSetBonuses};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Debug, Clone, Default)]
struct Editor {
    set_bonuses: Option<Vec<SetBonus>>,
    tab_home: TabHome,
    current_tab: Tab,
}

#[derive(Debug, Clone)]
enum Message {
    Data(DataMessage),
    Error(String),
    SetTab(Tab),
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
        let mut app = Self::default();
        let command = Command::batch([app.update(Message::Data(DataMessage::LoadSetBonuses))]);

        (app, command)
    }

    fn title(&self) -> String {
        String::from("DDO Build Planner Editor")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Data(message) => self.handle_update(message),
            Message::Error(error) => panic!("{error}"),
            Message::SetTab(tab) => {
                self.current_tab = tab;
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Column::new()
            .push(
                [(Tab::Home, "Home"), (Tab::SetBonuses, "Set Bonuses")]
                    .into_iter()
                    .fold(TabBar::new(Message::SetTab), |bar, (id, label)| {
                        bar.push(id, TabLabel::Text(label.to_owned()))
                    })
                    .set_active_tab(&self.current_tab),
            )
            .push(match self.current_tab {
                Tab::Home => EditorView::<TabHome>::handle_view(self),
                Tab::SetBonuses => EditorView::<TabSetBonuses>::handle_view(self),
            })
            .into()
    }
}

trait EditorView<T>: Sized + Application {
    fn handle_view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>>;
}

trait EditorUpdate<M>: Sized + Application {
    fn handle_update(&mut self, message: M) -> Command<Self::Message>;
}
