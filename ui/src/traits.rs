use iced::{Application, Command, Element, Renderer};

/// Provide a new `hanle_view` function
pub trait HandleView<App: Application + Sized> {
    /// Handes the view with a reference to the application
    fn handle_view<'a>(&'a self, app: &'a App) -> Element<'_, App::Message, Renderer>;
}

/// Provides a new `handle_message` function
pub trait HandleMessage<T, A = Self>: Sized
where
    A: Application + Sized,
{
    /// Handles the incoming message
    fn handle_message(&mut self, message: T) -> Command<A::Message>;
}

/// Indicates that this can be converted to another message
pub trait ToMessage<M> {
    /// Converts to another message
    fn to_message(self) -> M;
}
