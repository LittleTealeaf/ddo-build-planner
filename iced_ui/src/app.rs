use iced::{application, theme::Base, widget::text, Application, Element, Program, Task};

pub trait App: Sized + 'static {
    type Message: Send + 'static;
    fn boot() -> (Self, Option<Task<Self::Message>>);

    fn application() -> Application<impl Program<Theme = impl Base>> {
        application(
            || {
                let (app, maybe_task) = Self::boot();
                (app, maybe_task.unwrap_or_else(|| Task::none()))
            },
            |app: &mut Self, message: Self::Message| todo!(),
            |app: &Self| Element::<'_, Self::Message>>::from(text("Hi")),
        )
    }
}
