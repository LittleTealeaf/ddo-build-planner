use iced::{
    alignment::Vertical,
    theme,
    widget::{button, column, horizontal_space, row, text, text_input, vertical_space, Column},
    Application, Element, Length, Renderer,
};
use ui::HandleView;

use crate::{widgets::selector::IntoSelectorMessage, App, Message};

use super::{message::ValueSelectorMessage, types::ValueType, ValueSelector, ValueSubSelector};

impl HandleView<App> for ValueSelector {
    fn handle_view<'a>(
        &'a self,
        app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        if let Some(selector) = &self.selector {
            return match selector {
                ValueSubSelector::Attribute(selector) => selector.handle_view(app),
                ValueSubSelector::Condition(selector) => selector.handle_view(app),
                ValueSubSelector::ValueA(selector) | ValueSubSelector::ValueB(selector) => {
                    selector.handle_view(app)
                }
            };
        }

        column!(
            row!(
                column(ValueType::TYPES.map(|value| {
                    button(text(format!("{value}")).vertical_alignment(Vertical::Center))
                        .on_press(ValueSelectorMessage::SetType(value).into_message(self.depth))
                        .style(if value == self.val {
                            theme::Button::Primary
                        } else {
                            theme::Button::Text
                        })
                        .into()
                })),
                Column::new()
                    .push_maybe(self.val.show_const().then(|| {
                        text_input("Constant", &self.constant_string).on_input(|s| {
                            ValueSelectorMessage::UpdateDecimalString(s).into_message(self.depth)
                        })
                    }))
                    .push_maybe(self.val.show_condition().then(|| {
                        row!(
                            button(text("Condition")).on_press(
                                ValueSelectorMessage::EditCondition.into_message(self.depth)
                            ),
                            self.condition
                                .as_ref()
                                .map_or_else(|| text("None Selected"), text)
                        )
                    }))
                    .push_maybe(self.val.show_value_a().then(|| {
                        row!(
                            button(text("Value A")).on_press(
                                ValueSelectorMessage::EditValueA.into_message(self.depth)
                            ),
                            self.value_a
                                .as_ref()
                                .map_or_else(|| text("None Selected"), text)
                        )
                    }))
                    .push_maybe(self.val.show_value_b().then(|| {
                        row!(
                            button(text("Value B")).on_press(
                                ValueSelectorMessage::EditValueB.into_message(self.depth)
                            ),
                            self.value_b
                                .as_ref()
                                .map_or_else(|| text("None Selected"), text)
                        )
                    }))
                    .push_maybe(self.val.show_attribute().then(|| {
                        row!(
                            button(text("Attribute")).on_press(
                                ValueSelectorMessage::EditAttribute.into_message(self.depth)
                            ),
                            self.attribute
                                .as_ref()
                                .map_or_else(|| text("None Selected"), text)
                        )
                    }))
                    .width(Length::Fill)
            ),
            vertical_space(),
            row!(
                horizontal_space(),
                button(text("Cancel"))
                    .style(theme::Button::Secondary)
                    .on_press(Message::Selector(self.on_cancel.clone())),
                button(text("Submit"))
                    .style(theme::Button::Primary)
                    .on_press(Message::Selector(self.on_submit.clone())),
            )
        )
        .into()
    }
}
