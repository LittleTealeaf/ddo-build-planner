//! Editor Application

mod state;
mod tabs;

use iced::{executor, widget::Column, Application, Command, Element, Renderer, Settings, Theme};
use iced_aw::{TabBar, TabLabel};
use state::{AppState, AppStateMessage};
use tabs::{
    home::{TabHome, TabHomeMessage},
    set_bonuses::{TabSetBonuses, TabSetBonusesMessage},
};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Debug, Clone, Default)]
struct Editor {
    state: AppState,
    tab_home: TabHome,
    tab_set_bonuses: TabSetBonuses,
    selected_tab: Tab,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
enum Tab {
    #[default]
    Home,
    SetBonuses,
}

#[derive(Debug, Clone)]
enum Message {
    SwitchTab(Tab),
    AppState(AppStateMessage),
    TabHome(TabHomeMessage),
    TabSetBonuses(TabSetBonusesMessage),
    Error(String),
}

impl Application for Editor {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Self::default();
        let command = Command::batch([app.state.update(AppStateMessage::LoadSetBonuses)]);

        (app, command)
    }

    fn title(&self) -> String {
        String::from("DDO Build Planner Data Editor")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SwitchTab(tab) => {
                self.selected_tab = tab;
                Command::none()
            }
            Message::AppState(message) => self.state.update(message),
            Message::TabHome(message) => self.tab_home.update(&mut self.state, message),
            Message::TabSetBonuses(message) => {
                self.tab_set_bonuses.update(&mut self.state, message)
            }
            Message::Error(err) => panic!("{err}"),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Column::new()
            .push({
                [(Tab::Home, "Home"), (Tab::SetBonuses, "Set Bonuses")]
                    .into_iter()
                    .fold(TabBar::new(Message::SwitchTab), |bar, (tab, label)| {
                        bar.push(tab, TabLabel::Text(label.to_owned()))
                    })
                    .set_active_tab(&self.selected_tab)
            })
            .push(match &self.selected_tab {
                Tab::Home => self.tab_home.view(&self.state),
                Tab::SetBonuses => self.tab_set_bonuses.view(&self.state),
            })
            .into()
    }
}

trait EditorTab {
    type Message;

    fn update(&mut self, state: &mut AppState, message: Self::Message) -> Command<Message>;

    fn view(&self, state: &AppState) -> Element<'_, Message, Renderer<Theme>>;
}
