use iced::{
    theme,
    widget::{button, column, horizontal_space, row, scrollable, text},
    Application, Element, Length, Renderer,
};
use ui::HandleView;

use crate::App;

use super::{InternalSelector, ModalExpression, ModalExpressionMessage};

impl HandleView<App> for ModalExpression {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        column!(
            text(
                self.title
                    .as_ref()
                    .unwrap_or(&String::from("Configure Expression"))
            ),
            scrollable(text("THIS IS THE INTER OAJWE OFIAJ WEOFIJAW")).height(Length::Fill),
            row!(
                horizontal_space().width(Length::Fill),
                button("Cancel")
                    .on_press(ModalExpressionMessage::Cancel.into())
                    .style(theme::Button::Secondary),
                button("Submit")
                    .on_press_maybe(
                        (self.cached_value.is_some() || self.cached_condition.is_some())
                            .then_some(ModalExpressionMessage::Submit.into())
                    )
                    .style(theme::Button::Primary)
            )
        )
        .into()
    }
}

impl HandleView<App> for InternalSelector {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        todo!()
    }
}
