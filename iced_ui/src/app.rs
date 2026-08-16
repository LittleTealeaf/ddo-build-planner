use iced::{application, Application, Element, Program, Task};

use crate::effect::Effect;

pub trait App: Sized + 'static {
    type Message: Send + 'static;
    fn boot() -> (Self, Option<Task<Self::Message>>);

    fn title(&self) -> String;

    fn view(&self) -> Element<'_, Self::Message>;

    /// # Errors
    /// Errors when given update process fails
    fn update(&mut self, message: Self::Message) -> anyhow::Result<Effect<Self::Message, ()>>;

    fn application() -> Application<impl Program> {
        application(
            || {
                let (app, maybe_task) = Self::boot();
                (app, maybe_task.unwrap_or_else(Task::none))
            },
            |app: &mut Self, message: Self::Message| {
                app.process_message(message).unwrap_or_default()
            },
            Self::view,
        )
        .title(Self::title)
    }

    /// # Errors
    /// Errors when given update process fails
    fn process_message(&mut self, message: Self::Message) -> anyhow::Result<Task<Self::Message>> {
        self.update(message)
            .and_then(|effect| self.process_effect(effect))
    }

    /// # Errors
    /// Errors when given update process fails
    fn process_effect(
        &mut self,
        effect: Effect<Self::Message, ()>,
    ) -> anyhow::Result<Task<Self::Message>> {
        match effect {
            Effect::Out(()) | Effect::Done => Ok(Task::none()),
            Effect::Task(task) => Ok(task),
            Effect::Msg(message) => self.process_message(message),
            Effect::Batch(effects) => {
                let mut errors = Vec::new();
                let mut tasks = Vec::new();
                for effect in effects {
                    match self.process_effect(effect) {
                        Ok(task) => tasks.push(task),
                        Err(error) => errors.push(error),
                    }
                }

                if errors.is_empty() {
                    Ok(Task::batch(tasks))
                } else {
                    Err(anyhow::anyhow!("Multiple Errors Occurred: {errors:?}"))
                }
            }
            Effect::Sequence(effects) => {
                let mut task = Task::none();
                for effect in effects {
                    task = task.chain(self.process_effect(effect)?);
                }
                Ok(task)
            }
            Effect::OnError(effect, on_error) => {
                let result = self.process_effect(*effect);
                match result {
                    Ok(task) => Ok(task),
                    Err(error) => {
                        eprintln!("Gracefully caught error: {error:?}");
                        on_error.map_or_else(
                            || Ok(Task::none()),
                            |message| self.process_message(message),
                        )
                    }
                }
            }
        }
    }
}
