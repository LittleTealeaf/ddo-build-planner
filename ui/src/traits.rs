use iced::{Application, Command, Element, Renderer};

/// Provide a new `handle_view` function
pub trait HandleView<App>
where
    App: Application + Sized,
{
    /// Handes the view with a reference to the application
    fn handle_view<'a>(&'a self, app: &'a App) -> Element<'_, App::Message, App::Theme, Renderer>;
}

/// Provides a new `handle_message` function
pub trait HandleMessage<T, A = Self>: Sized
where
    A: Application + Sized,
{
    /// Handles the incoming message
    fn handle_message(&mut self, message: T) -> Command<A::Message>;
}
