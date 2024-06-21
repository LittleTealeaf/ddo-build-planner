//! Editor Application
mod data;
mod modals;
mod tabs;

use data::{container::DataContainerMessage, Data, DataMessage};
use iced::{executor, font, Application, Command, Element, Renderer, Settings, Theme};
use modals::{
    attribute::{ModalAttribute, ModalAttributeMessage},
    bonus_template::{ModalBonus, ModalBonusMessage},
    expression::{ModalExpression, ModalExpressionMessage},
};
use tabs::{
    home::TabHome,
    item_sets::{TabItemSets, TabSetBonusesMessage},
    Tab,
};
use ui::{
    error,
    font::NERD_FONT_BYTES,
    log::{Log, Severity},
    HandleMessage, HandleView,
};

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
    modal_attribute: Option<ModalAttribute>,
    modal_expression: Option<ModalExpression>,
    modal_bonus: Option<ModalBonus>,
    theme: Theme,
}

#[derive(Clone, Debug)]
enum Message {
    None,
    IconsLoaded,
    Data(DataMessage),
    Log(Log),
    ChangeTab(Tab),
    TabSetBonuses(TabSetBonusesMessage),
    ModalAttribute(ModalAttributeMessage),
    ModalExpression(ModalExpressionMessage),
    ModalBonus(ModalBonusMessage),
    SetTheme(Theme),
    DebugOpenAttribute,
    DebugOpenCondition,
    DebugOpenValue,
    DebugOpenBonus,
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
            modal_attribute: None,
            modal_expression: None,
            modal_bonus: None,
            theme: Theme::CatppuccinMocha,
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
        match (
            &self.modal_attribute,
            &self.modal_expression,
            &self.modal_bonus,
        ) {
            (Some(modal), _, _) => modal.handle_view(self),
            (_, Some(modal), _) => modal.handle_view(self),
            (_, _, Some(modal)) => modal.handle_view(self),
            _ => self.selected_tab.handle_view(self),
        }
    }
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}

impl HandleMessage<Message> for App {
    fn handle_message(&mut self, message: Message) -> Command<<Self as Application>::Message> {
        match message {
            Message::None => Command::none(),
            Message::SetTheme(theme) => {
                self.theme = theme;
                Command::none()
            }
            Message::IconsLoaded => {
                self.icons_loaded = true;
                Command::none()
            }
            Message::Data(message) => self.handle_message(message),
            Message::ChangeTab(tab) => {
                self.selected_tab = tab;
                Command::none()
            }
            Message::TabSetBonuses(message) => self.handle_message(message),
            Message::ModalAttribute(message) => self.handle_message(message),
            Message::ModalExpression(message) => self.handle_message(message),
            Message::ModalBonus(message) => self.handle_message(message),
            Message::DebugOpenAttribute => {
                self.modal_attribute = Some(
                    self.select_attribute()
                        .title("Debug")
                        .on_submit(Message::DebugSubmit),
                );
                Command::none()
            }
            Message::DebugOpenCondition => {
                self.modal_expression = Some(
                    ModalExpression::condition(None)
                        .on_submit(Message::DebugSubmit)
                        .title("Debug Condition"),
                );
                Command::none()
            }
            Message::DebugOpenValue => {
                self.modal_expression = Some(
                    ModalExpression::value(None)
                        .on_submit(Message::DebugSubmit)
                        .title("Debug Submit"),
                );
                Command::none()
            }
            Message::DebugOpenBonus => {
                self.modal_bonus = Some(
                    ModalBonus::new(None)
                        .on_submit(Message::DebugSubmit)
                        .title("Debug Bonus"),
                );
                Command::none()
            }
            Message::DebugSubmit => {
                if let Some(attr) = &self.modal_attribute {
                    if let Some(attr) = attr.get_attribute() {
                        println!("{attr}");
                    }
                }

                if let Some(sel) = &self.modal_expression {
                    if let Some(value) = sel.get_value() {
                        println!("{value}");
                    }
                    if let Some(cond) = sel.get_condition() {
                        println!("{cond}");
                    }
                }

                if let Some(modal) = &self.modal_bonus {
                    if let Some(bonus) = modal.get_bonus() {
                        println!("{bonus:?}");
                    }
                }

                Command::none()
            }
            Message::Log(log) => self.handle_message(log),
        }
    }
}

impl From<Log> for Message {
    fn from(value: Log) -> Self {
        Self::Log(value)
    }
}

impl HandleMessage<Log> for App {
    fn handle_message(&mut self, message: Log) -> Command<<Self as Application>::Message> {
        println!("{message}");

        if matches!(message.severity, Severity::Crash) {
            panic!();
        }

        Command::none()
    }
}

fn load_font() -> Command<Message> {
    font::load(NERD_FONT_BYTES).map(|res| {
        res.map_or_else(
            |e| error!("Font Load: {e:?}").into(),
            |()| Message::IconsLoaded,
        )
    })
}
