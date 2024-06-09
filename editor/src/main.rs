//! Editor Application
mod data;
mod modals;
mod tabs;

use data::{container::DataContainerMessage, Data, DataMessage};
use iced::{executor, font, Application, Command, Element, Renderer, Settings, Theme};
use modals::{
    attribute::{AttributeSelector, AttributeSelectorMessage},
    expression::{ModalExpression, ModalExpressionMessage},
};
use tabs::{
    home::TabHome,
    item_sets::{TabItemSets, TabSetBonusesMessage},
    Tab,
};
use ui::{font::NERD_FONT_BYTES, HandleMessage, HandleView};

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Clone, Debug)]
struct App {
    data: Data,
    tab_home: TabHome,
    tab_item_sets: TabItemSets,
    icons_loaded: bool,
    selected_tab: Tab,
    attribute_selector: Option<AttributeSelector>,
    expression_selector: Option<ModalExpression>,
}

#[derive(Clone, Debug)]
enum Message {
    IconsLoaded,
    Data(DataMessage),
    Error(String),
    ChangeTab(Tab),
    TabSetBonuses(TabSetBonusesMessage),
    AttributeSelector(AttributeSelectorMessage),
    ExpressionSelector(ModalExpressionMessage),
    DebugOpenAttribute,
    DebugOpenCondition,
    DebugOpenValue,
    DebugSubmit,
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
            tab_item_sets: TabItemSets::default(),
            attribute_selector: None,
            expression_selector: None,
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
        self.attribute_selector.as_ref().map_or_else(
            || {
                self.expression_selector.as_ref().map_or_else(
                    || self.selected_tab.handle_view(self),
                    |selector| selector.handle_view(self),
                )
            },
            |selector| selector.handle_view(self),
        )
    }
}

impl HandleMessage<Message> for App {
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
            Message::AttributeSelector(message) => self.handle_message(message),
            Message::ExpressionSelector(message) => self.handle_message(message),
            Message::DebugOpenAttribute => {
                self.attribute_selector = Some(
                    self.select_attribute()
                        .title("Debug")
                        .on_submit(Message::DebugSubmit),
                );
                Command::none()
            }
            Message::DebugOpenCondition => {
                self.expression_selector = Some(
                    ModalExpression::condition(None)
                        .on_submit(Message::DebugSubmit)
                        .title("Debug Condition"),
                );
                Command::none()
            }
            Message::DebugOpenValue => {
                self.expression_selector = Some(
                    ModalExpression::value(None)
                        .on_submit(Message::DebugSubmit)
                        .title("Debug Submit"),
                );
                Command::none()
            }
            Message::DebugSubmit => {
                if let Some(attr) = &self.attribute_selector {
                    if let Some(attr) = attr.get_attribute() {
                        println!("{attr}");
                    }
                }

                if let Some(sel) = &self.expression_selector {
                    if let Some(value) = sel.get_value() {
                        println!("{value}");
                    }
                    if let Some(cond) = sel.get_condition() {
                        println!("{cond}");
                    }
                }

                Command::none()
            }
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
