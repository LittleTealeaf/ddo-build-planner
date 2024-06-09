use iced::{
    theme,
    widget::{
        button, column, container, horizontal_space, pick_list, row, scrollable, text, text_input,
        vertical_space, Column, Row,
    },
    Application, Element, Length, Renderer,
};
use ui::HandleView;

use crate::App;

use super::{
    condition::ConditionType, value::ValueType, ModalExpression, ModalExpressionInternalMessage,
    ModalExpressionMessage, ModalExpressionType,
};

impl ModalExpression {
    fn handle_internal_view(
        &self,
        id: usize,
    ) -> Option<Element<'_, <App as Application>::Message, <App as Application>::Theme, Renderer>>
    {
        let selector = self.selectors.get(&id)?;

        let cond_a = selector
            .condition_a
            .and_then(|id| self.handle_internal_view(id));
        let cond_b = selector
            .condition_b
            .and_then(|id| self.handle_internal_view(id));
        let val_a = selector
            .value_a
            .and_then(|id| self.handle_internal_view(id));
        let val_b = selector
            .value_b
            .and_then(|id| self.handle_internal_view(id));

        Some(
            match selector.selector_type {
                ModalExpressionType::Value(value_type) => container(column([
                    pick_list(ValueType::TYPES, Some(value_type), move |value_type| {
                        (
                            id,
                            ModalExpressionInternalMessage::SetType(ModalExpressionType::Value(
                                value_type,
                            )),
                        )
                            .into()
                    })
                    .into(),
                    row([
                        text("   ").into(),
                        match value_type {
                            ValueType::Const => container(
                                text_input("Constant", selector.constant_str.as_str()).on_input(
                                    move |string| {
                                        (id, ModalExpressionInternalMessage::ConstInput(string))
                                            .into()
                                    },
                                ),
                            ),
                            ValueType::Attribute => container(
                                button(selector.attribute.as_ref().map_or_else(
                                    || text("None"),
                                    |attribute| text(attribute.to_string()),
                                ))
                                .on_press(
                                    (id, ModalExpressionInternalMessage::SelectAttribute).into(),
                                ),
                            ),
                            ValueType::Min => {
                                container(column([text("Min").into(), val_a?, val_b?]))
                            }
                            ValueType::Max => {
                                container(column([text("Max").into(), val_a?, val_b?]))
                            }
                            ValueType::Floor => container(column([text("Floor").into(), val_a?])),
                            ValueType::Ceil => container(column([text("Ceil").into(), val_a?])),
                            ValueType::Round => container(column([text("Round").into(), val_a?])),
                            ValueType::Abs => container(column([text("Abs").into(), val_a?])),
                            ValueType::Add => {
                                container(column([text("Add").into(), val_a?, val_b?]))
                            }
                            ValueType::Sub => {
                                container(column([text("Subtract").into(), val_a?, val_b?]))
                            }
                            ValueType::Mul => {
                                container(column([text("Multiply").into(), val_a?, val_b?]))
                            }
                            ValueType::Div => {
                                container(column([text("Divide").into(), val_a?, val_b?]))
                            }
                            ValueType::Rem => {
                                container(column([text("Remainder").into(), val_a?, val_b?]))
                            }
                            ValueType::If => container(column([
                                text("If").into(),
                                cond_a?,
                                text("Then").into(),
                                val_a?,
                                text("Else").into(),
                                val_b?,
                            ])),
                            ValueType::Dice => container(column([
                                text("Count").into(),
                                val_a?,
                                text("Size").into(),
                                val_b?,
                            ])),
                        }
                        .into(),
                    ])
                    .into(),
                ])),
                ModalExpressionType::Condition(condition_type) => container(column!(
                    pick_list(
                        ConditionType::TYPES,
                        Some(condition_type),
                        move |condition_type| {
                            (
                                id,
                                ModalExpressionInternalMessage::SetType(
                                    ModalExpressionType::Condition(condition_type),
                                ),
                            )
                                .into()
                        }
                    ),
                    row!(
                        text("   "),
                        match condition_type {
                            ConditionType::Not => container(column!(text("Not"), cond_a?)),
                            ConditionType::GreaterThan =>
                                container(column!(val_a?, text("Greater Than"), val_b?)),
                            ConditionType::LessThan =>
                                container(column!(val_a?, text("Less Than"), val_b?)),
                            ConditionType::EqualTo =>
                                container(column!(val_a?, text("Equal To"), val_b?)),
                            ConditionType::True => container(text("True")),
                            ConditionType::False => container(text("False")),
                            ConditionType::And => container(column!(cond_a?, text("And"), cond_b?)),
                            ConditionType::Or => container(column!(cond_a?, text("Or"), cond_b?)),
                            ConditionType::Xor =>
                                container(column!(cond_a?, text("Exclusive Or"), cond_b?)),
                        }
                    )
                )),
            }
            .into(),
        )
    }
}

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
            scrollable(
                self.handle_internal_view(self.base)
                    .unwrap_or_else(|| text("No Base Expression Set").into())
            )
            .width(Length::Fill),
            vertical_space(),
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
