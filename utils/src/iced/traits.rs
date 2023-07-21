use std::fmt::Debug;

use iced::{Application, Command};

/// Provides the ability to render based on a given application
pub trait HandleView<App>
where
    App: Application,
{
    /// Handles the view
    fn view(&self) -> iced::Element<'_, App::Message, iced::Renderer<App::Theme>>;
}

/// Provides the ability to handle messages forwarded from the application.
pub trait HandleMessage<App>
where
    App: Application,
{
    /// The type of the messages here
    type Message: Debug + Send;

    /// Update method to consume the messages
    fn update(&mut self, message: Self::Message) -> Command<App::Message>;
}
