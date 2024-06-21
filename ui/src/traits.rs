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

/// Adds the `message` method to be used to execute a command that immediately returns some message
pub trait ExecuteMessage<Msg>
where
    Msg: 'static + Send + Sync,
{
    /// Runs a message
    fn run_message(message: Msg) -> Command<Msg>;

    /// Creates a command that executes a delayed message
    fn message<M>(message: M) -> Command<Msg>
    where
        M: Into<Msg>,
    {
        Self::run_message(message.into())
    }
}

impl<M> ExecuteMessage<M> for Command<M>
where
    M: 'static + Send + Sync,
{
    fn run_message(message: M) -> Self {
        Self::perform(async {}, |()| message)
    }
}
