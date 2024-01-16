//! Application Messages

use iced::Command;
use utils::public_modules;

use crate::EditorApp;

public_modules!(data);

/// Messages used within the application
#[derive(Debug, Clone)]
pub enum Message {
    /// Messages for loading / saving data
    Data(DataMessage),
    /// Errors
    Error(String),
}

/// Handles messages passed from the application
pub trait HandleMessage {
    /// Handle a message recieved
    fn handle(self, app: &mut EditorApp) -> Command<Message>;
}

impl HandleMessage for Message {
    fn handle(self, app: &mut EditorApp) -> Command<Message> {
        match self {
            Self::Data(message) => message.handle(app),
            Self::Error(error) => panic!("{error}"),
        }
    }
}
