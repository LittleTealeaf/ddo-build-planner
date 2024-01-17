use iced::{Application, Command, Element, Renderer};

/// Provides a new `handle_view` function
pub trait HandleView<T>: Application + Sized {
    /// Handles the view by returning a new element
    fn handle_view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>>;
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
