//! Editor Application
mod data;
mod tabs;
mod widgets;

use data::{container::DataContainerMessage, Data, DataMessage};
use iced::{
    executor, font, widget::text, Application, Command, Element, Renderer, Settings, Theme,
};
use tabs::{
    home::TabHome,
    set_bonuses::{TabSetBonuses, TabSetBonusesMessage},
    Tab,
};
use ui::{font::NERD_FONT_BYTES, HandleMessage};
use widgets::selector::{SelectorWidget, SelectorWidgetMessage};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Clone, Debug)]
struct Editor {
    data: Data,
    tab_home: TabHome,
    tab_set_bonuses: TabSetBonuses,
    icons_loaded: bool,
    selected_tab: Tab,
    selector: Option<SelectorWidget>,
}

#[derive(Clone, Debug)]
enum Message {
    IconsLoaded,
    Data(DataMessage),
    Error(String),
    ChangeTab(Tab),
    TabSetBonuses(TabSetBonusesMessage),
    Selector(SelectorWidgetMessage),
}

impl Application for Editor {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, Command<Self::Message>) {
        let mut editor = Self {
            data: Data::default(),
            tab_home: TabHome::default(),
            icons_loaded: false,
            selected_tab: Tab::Home,
            tab_set_bonuses: TabSetBonuses::default(),
            selector: None,
        };

        editor.selector = Some(SelectorWidget::new(editor.data.generate_attributes()));

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

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
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
            Message::ChangeTab(tab) => {
                self.selected_tab = tab;
                Command::none()
            }
            Message::TabSetBonuses(message) => self.handle_message(message),
            Message::Selector(message) => self.handle_message(message),
        }
    }
}
