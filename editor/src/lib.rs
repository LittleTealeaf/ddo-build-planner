mod database;
mod message;

use database::Database;
use iced::Task;
use iced_ui::{app::App, model::Model};
use message::Message;

#[derive(Debug, Clone, Default)]
pub struct EditorApp {
    data: Database,
}

impl App for EditorApp {
    type Message = Message;

    fn boot() -> (Self, Option<iced::Task<Self::Message>>) {
        let app = Self::default();
        let tasks = Task::batch([app.data.load_all().map(Into::into)]);

        (app, Some(tasks))
    }

    fn title(&self) -> String {
        "hello".to_owned()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::text("hi").into()
    }

    fn update(
        &mut self,
        message: Self::Message,
    ) -> anyhow::Result<iced_ui::signal::Signal<Self::Message, ()>> {
        match message {
            Message::Database(msg) => self.data.empty_update(msg, ()),
        }
    }
}
