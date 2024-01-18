use iced::{Application, Command, Element, Renderer};

/// Provide a new `hanle_view` function
pub trait HandleView<A: Application + Sized> {
    /// Handes the view with a reference to the application
    fn handle_view<'a>(&'a self, app: &'a A) -> Element<'_, A::Message, Renderer<A::Theme>>;
}

/// Provides a new `handle_message` function
pub trait HandleMessage<T>: Application + Sized {
    /// Handles the incoming message
    fn handle_message(&mut self, message: T) -> Command<Self::Message>;
}

/// Indicates that this can be converted to another message
pub trait ToMessage<M> {
    /// Converts to another message
    fn to_message(self) -> M;
}
