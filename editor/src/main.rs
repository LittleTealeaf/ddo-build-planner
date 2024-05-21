//! Editor Application
mod data;
mod tabs;
mod widgets;

use data::{container::DataContainerMessage, Data, DataMessage};
use iced::{executor, font, Application, Command, Element, Renderer, Settings, Theme};
use tabs::{
    home::TabHome,
    item_sets::{TabSetBonuses, TabSetBonusesMessage},
    Tab,
};
use ui::{font::NERD_FONT_BYTES, HandleMessage, HandleView};
use widgets::selector::{SelectorWidget, SelectorWidgetMessage};

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Clone, Debug)]
struct App {
    data: Data,
    tab_home: TabHome,
    tab_item_sets: TabSetBonuses,
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
    DebugOpenCondition,
    DebugOpenValue,
    DebugOpenAttribute,
    DebugClose,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new((): Self::Flags) -> (Self, Command<Self::Message>) {
        let mut editor = Self {
            data: Data::new(),
            tab_home: TabHome::default(),
            icons_loaded: false,
            selected_tab: Tab::Home,
            tab_item_sets: TabSetBonuses::default(),
            selector: None,
        };

        let command = Command::batch([
            editor.handle_message(DataMessage::SetBonuses(DataContainerMessage::Load)),
            load_font(),
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
        self.selector.as_ref().map_or_else(
            || self.selected_tab.handle_view(self),
            |selector| selector.handle_view(self),
        )
    }
}

impl HandleMessage<Message> for App {
    fn handle_message(&mut self, message: Message) -> Command<<Self as Application>::Message> {
        match message {
            Message::DebugClose => {
                if let Some(selector) = &self.selector {
                    println!("{:?}", selector.get_result());
                }
                self.selector = None;
                Command::none()
            }
            Message::DebugOpenCondition => {
                self.selector = Some(
                    SelectorWidget::new(self.data.generate_attributes())
                        .with_select_condition(None)
                        .with_on_submit(Message::DebugClose)
                        .with_on_cancel(Message::DebugClose),
                );
                Command::none()
            }

            Message::DebugOpenValue => {
                self.selector = Some(
                    SelectorWidget::new(self.data.generate_attributes())
                        .with_select_value(None)
                        .with_on_submit(Message::DebugClose)
                        .with_on_cancel(Message::DebugClose),
                );
                Command::none()
            }
            Message::DebugOpenAttribute => {
                self.selector = Some(
                    SelectorWidget::new(self.data.generate_attributes())
                        .with_select_attribute(None)
                        .with_on_submit(Message::DebugClose)
                        .with_on_cancel(Message::DebugClose),
                );
                Command::none()
            }
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

fn load_font() -> Command<Message> {
    font::load(NERD_FONT_BYTES).map(|res| {
        res.map_or_else(
            |e| Message::Error(format!("Font: {e:?}")),
            |()| Message::IconsLoaded,
        )
    })
}
