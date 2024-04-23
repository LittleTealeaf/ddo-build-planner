use iced::{widget::column, Application, Element, Renderer};

use ui::HandleView;

use crate::App;

use super::{ConditionSelector, ConditionSubSelector};

impl HandleView<App> for ConditionSelector {
    fn handle_view<'a>(
        &'a self,
        app: &'a App,
    ) -> Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer> {
        if let Some(selector) = &self.selector {
            return match selector {
                ConditionSubSelector::ConditionA(selector)
                | ConditionSubSelector::ConditionB(selector) => selector.handle_view(app),
                ConditionSubSelector::ValueA(selector) | ConditionSubSelector::ValueB(selector) => {
                    selector.handle_view(app)
                }
            };
        }

        column!(
            row!(
                column(ConditionType::TYPES.map(|condition| {
                    button(
                        text(format!("{condition}"))
                            .vertical_alignment(Vertical::Center)
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .on_press(ConditionSelectorMessage::SetType(condition).into_message(self.depth))
                    .style(if condition == self.cond {
                        theme::Button::Primary
                    } else {
                        theme::Button::Text
                    })
                    .into()
                })),
                Column::new()
                    .push_maybe(self.cond.show_value_a().then(|| {
                        row!(
                            button(text("Value A")).on_press(
                                ConditionSelectorMessage::EditValueA.into_message(self.depth)
                            ),
                            self.value_a
                                .as_ref()
                                .map_or_else(|| text("None Selected"), text)
                        )
                    }))
                    .push_maybe(self.cond.show_value_b().then(|| {
                        row!(
                            button(text("Value B")).on_press(
                                ConditionSelectorMessage::EditValueB.into_message(self.depth)
                            ),
                            self.value_b
                                .as_ref()
                                .map_or_else(|| text("None Selected"), text)
                        )
                    }))
                    .push_maybe(self.cond.show_condition_a().then(|| {
                        row!(
                            button(text("Condition A")).on_press(
                                ConditionSelectorMessage::EditConditionA.into_message(self.depth)
                            ),
                            self.condition_a
                                .as_ref()
                                .map_or_else(|| text("None Selected"), text)
                        )
                    }))
                    .push_maybe(self.cond.show_condition_b().then(|| {
                        row!(
                            button(text("Condition B")).on_press(
                                ConditionSelectorMessage::EditConditionB.into_message(self.depth)
                            ),
                            self.condition_b
                                .as_ref()
                                .map_or_else(|| text("None Selected"), text)
                        )
                    }))
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
