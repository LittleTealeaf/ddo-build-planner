use iced_futures::MaybeSend;

use crate::signal::Signal;

pub trait Model {
    type Message;
    type OutMessage;
    type Context<'a>
    where
        Self: 'a;

    /// # Errors
    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>>;

    /// # Errors
    fn map_update<'a, M, O, F>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
        map_out: F,
    ) -> anyhow::Result<Signal<M, O>>
    where
        F: Fn(Self::OutMessage) -> anyhow::Result<Signal<M, O>>,
        Self::Message: Into<M> + 'static + MaybeSend,
        M: Send + MaybeSend + 'static,
    {
        self.update(message, context)?.map(map_out)
    }

    /// # Errors
    fn empty_update<'a, M, O>(
        &'a mut self,
        message: Self::Message,
        context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<M, O>>
    where
        Self: Model<OutMessage = ()>,
        M: Send + MaybeSend + 'static,
        Self::Message: Into<M> + Send + 'static,
    {
        self.update(message, context)?.map_empty()
    }
}
