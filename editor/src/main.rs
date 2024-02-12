//! Editor Application

mod data;
mod tabs;
mod widgets;

use ::utils::enums::StaticOptions;
use builder::attribute::Attribute;
use data::{Data, MData, MDataContainer};
use iced::{executor, font, Application, Command, Element, Renderer, Settings, Theme};
use itertools::chain;
use tabs::{MHome, MSetBonuses, THome, TSetBonuses, Tab};
use ui::{font::NERD_FONT_BYTES, HandleMessage, HandleView};

type AppExecutor = executor::Default;
type AppMessage = Message;
type AppTheme = Theme;
type AppFlags = ();

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Clone, Debug)]
struct Editor {
    data: Data,
    icons_loaded: bool,
    set_bonuses: TSetBonuses,
    home: THome,
    tab: Tab,
}

#[derive(Clone, Debug)]
enum Message {
    Data(MData),
    IconsLoaded,
    Error(String),
    SetBonuses(MSetBonuses),
    Home(MHome),
    ChangeTab(Tab),
}

impl Editor {

    fn generate_attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        let set_bonuses = self.data.set_bonuses.data.iter().flat_map(|sets| {
            sets.iter()
                .map(|set| Attribute::SetBonus(set.name().clone()))
        });

        chain!(set_bonuses, Attribute::get_static())
    }
}

impl Application for Editor {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Self {
            data: Data::default(),
            icons_loaded: false,
            set_bonuses: TSetBonuses::default(),
            home: THome::default(),
            tab: Tab::Home,
        };
        let command = Command::batch([
            app.handle_message(MData::SetBonus(MDataContainer::Load)),
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
    fn handle_message(&mut self, message: Message) -> Command<<Self as Application>::Message> {
        match message {
            Message::Data(m) => self.handle_message(m),
            Message::Home(message) => self.handle_message(message),
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
