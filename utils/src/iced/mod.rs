//! Helper functions for iced

use iced::{Application, Command, Element, Renderer};


/// Provides implementations for an application child
pub trait ApplicationChild<T> where T: Application {
    /// See [`Application::Message`]
    type Message;


    /// See [`Application::update`]
    fn update(&mut self, message: Self::Message) -> Command<T::Message>;

    /// See [`Application::view`]
    fn view(&self) -> Element<'_, T::Message, Renderer<T::Theme>>;
}
