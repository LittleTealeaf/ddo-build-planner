use crate::effect::Effect;

pub trait Service {
    type Message;
    type OutMessage;

    /// # Errors
    /// Errors when the implement process fails
    fn run(
        &mut self,
        message: Self::Message,
    ) -> anyhow::Result<Effect<Self::Message, Self::OutMessage>>;
}
