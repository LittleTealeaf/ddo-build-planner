use core::fmt::Debug;

use iced::{application, Application, Element, Program, Task};

use crate::signal::Signal;

pub trait App: Sized + 'static {
    type Message: Send + 'static + Debug;
    fn boot() -> (Self, Option<Task<Self::Message>>);

    fn title(&self) -> String;

    fn view(&self) -> Element<'_, Self::Message>;

    /// # Errors
    /// Errors when given update process fails
    fn update(&mut self, message: Self::Message) -> anyhow::Result<Signal<Self::Message, ()>>;

    fn on_error(&mut self, error: String) {
        eprintln!("Error: {error}");
    }

    fn application() -> Application<impl Program> {
        application(
            || {
                let (app, maybe_task) = Self::boot();
                (app, maybe_task.unwrap_or_else(Task::none))
            },
            |app: &mut Self, message: Self::Message| match process_message(app, message) {
                Ok(task) => task,
                Err(error) => {
                    app.on_error(format!("{error}"));
                    Task::none()
                }
            },
            Self::view,
        )
        .title(Self::title)
    }
}

fn process_message<A>(app: &mut A, message: A::Message) -> anyhow::Result<Task<A::Message>>
where
    A: App,
{
    #[cfg(debug_assertions)]
    {
        println!("Msg: {message:?}");
    }
    let result = app.update(message)?;
    process_effect(app, result)
}

fn process_effect<A>(
    app: &mut A,
    effect: Signal<A::Message, ()>,
) -> anyhow::Result<Task<A::Message>>
where
    A: App,
{
    match effect {
        Signal::Out(()) | Signal::Done => Ok(Task::none()),
        Signal::Task(task) => Ok(task),
        Signal::Message(message) => process_message(app, message),
        Signal::Batch(effects) => {
            let mut errors = Vec::new();
            let mut tasks = Vec::new();
            for effect in effects {
                match process_effect(app, effect) {
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
        Signal::Sequence(effects) => {
            let mut task = Task::none();
            for effect in effects {
                task = task.chain(process_effect(app, effect)?);
            }
            Ok(task)
        }
        Signal::OnError(message, on_error) => {
            let result = process_effect(app, *message);
            match result {
                Ok(task) => Ok(task),
                Err(error) => {
                    eprintln!("Gracefully caught error: {error:?}");
                    on_error
                        .map_or_else(|| Ok(Task::none()), |message| process_message(app, message))
                }
            }
        }
    }
}
