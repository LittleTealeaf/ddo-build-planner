use iced::Element;
use iced_futures::MaybeSend;

use crate::effect::Effect;

pub trait Component {
    type Message;
    type Context<'a>
    where
        Self: 'a;
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message>;
    fn render_into<'a, M>(&'a self, context: Self::Context<'a>) -> Element<'a, M>
    where
        Self::Message: Into<M>,
        M: 'a,
    {
        self.render(context).map(Into::into)
    }
}

pub trait ComponentUpdate: Component {
    type OutMessage;

    /// # Errors
    /// Returns `Err()` if there is any error that occurs during the process
    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>;

    /// # Errors
    /// Returns `Err()` if either the update errors, or the map errors
    fn map_update<'a, M, O, F>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
        map_out: F,
    ) -> anyhow::Result<Effect<M, O>>
    where
        F: Fn(Self::OutMessage) -> anyhow::Result<Effect<M, O>>,
        Self::Message: Into<M> + 'static + MaybeSend,
        M: Send + MaybeSend + 'static,
    {
        self.update(message, context)?.map(map_out)
    }
}
