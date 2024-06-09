use builder::{
    attribute::Attribute,
    bonus::{BonusSource, BonusTemplate, BonusType, Condition, Value},
};
use iced::{
    theme,
    widget::{button, column, horizontal_space, pick_list, row, text},
    Application, Command,
};
use ui::{HandleMessage, HandleView};
use utils::enums::StaticOptions;

use crate::{App, Message};

use super::expression::ModalExpression;

#[derive(Debug, Clone)]
pub struct ModalBonus {
    title: Option<String>,
    attribute: Option<Attribute>,
    bonus_type: BonusType,
    value: Option<Value>,
    condition: Option<Condition>,
    display_source: Option<BonusSource>,
    on_cancel: Option<Message>,
    on_submit: Option<Message>,
}

impl ModalBonus {
    pub fn new(bonus: Option<&BonusTemplate>) -> Self {
        Self {
            title: None,
            attribute: bonus.as_ref().map(|b| b.attribute().clone()),
            bonus_type: bonus.as_ref().map(|b| *b.bonus_type()).unwrap_or_default(),
            display_source: bonus.as_ref().and_then(|b| b.display_source().cloned()),
            condition: bonus.as_ref().and_then(|b| b.condition().cloned()),
            on_cancel: None,
            on_submit: None,
            value: bonus.as_ref().map(|b| b.value().clone()),
        }
    }

    pub fn title<S>(self, title: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    pub fn title_maybe<S>(self, title: Option<S>) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: title.map(Into::into),
            ..self
        }
    }

    pub fn on_submit<M>(self, message: M) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_submit: Some(message.into()),
            ..self
        }
    }

    pub fn on_submit_maybe<M>(self, message: Option<M>) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_submit: message.map(Into::into),
            ..self
        }
    }

    pub fn on_cancel<M>(self, message: M) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_cancel: Some(message.into()),
            ..self
        }
    }

    pub fn on_cancel_maybe<M>(self, message: Option<M>) -> Self
    where
        M: Into<Message>,
    {
        Self {
            on_cancel: message.map(Into::into),
            ..self
        }
    }

    pub fn get_bonus(&self) -> Option<BonusTemplate> {
        Some(
            BonusTemplate::new(
                self.attribute.clone()?,
                self.bonus_type,
                self.value.clone()?,
            )
            .with_condition(self.condition.clone())
            .with_display_source_maybe(self.display_source.clone()),
        )
    }
}

#[derive(Debug, Clone)]
pub enum ModalBonusMessage {
    Submit,
    Cancel,
    OpenAttributeModal,
    OnAttributeSelected,
    SetBonusType(BonusType),
    OpenValueModal,
    OnValueSelected,
    OpenConditionModal,
    ClearCondition,
    OnConditionSelected,
    SetDisplaySource(Option<BonusSource>),
}

impl From<ModalBonusMessage> for Message {
    fn from(value: ModalBonusMessage) -> Self {
        Self::ModalBonus(value)
    }
}

impl HandleMessage<ModalBonusMessage> for App {
    fn handle_message(
        &mut self,
        message: ModalBonusMessage,
    ) -> Command<<Self as Application>::Message> {
        let Some(modal) = &mut self.modal_bonus else {
            return Command::none();
        };

        match message {
            ModalBonusMessage::Submit => {
                let command = modal
                    .on_submit
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.modal_bonus = None;
                command
            }
            ModalBonusMessage::Cancel => {
                let command = modal
                    .on_cancel
                    .clone()
                    .map_or_else(Command::none, |message| self.handle_message(message));
                self.modal_bonus = None;
                command
            }
            ModalBonusMessage::OpenAttributeModal => {
                let attribute = modal.attribute.clone();
                self.modal_attribute = Some(
                    self.select_attribute()
                        .title("Bonus Attribute")
                        .on_submit(ModalBonusMessage::OnAttributeSelected)
                        .select_maybe(attribute),
                );
                Command::none()
            }
            ModalBonusMessage::OnAttributeSelected => {
                if let Some(modal_attribute) = &self.modal_attribute {
                    modal.attribute = modal_attribute.get_attribute();
                }
                Command::none()
            }
            ModalBonusMessage::SetBonusType(bonus_type) => {
                modal.bonus_type = bonus_type;
                Command::none()
            }
            ModalBonusMessage::OpenValueModal => {
                self.modal_expression = Some(
                    ModalExpression::value(modal.value.clone())
                        .on_submit(ModalBonusMessage::OnValueSelected)
                        .title("Bonus Value"),
                );
                Command::none()
            }
            ModalBonusMessage::OnValueSelected => {
                if let Some(modal_expression) = &self.modal_expression {
                    modal.value = modal_expression.get_value();
                }
                Command::none()
            }
            ModalBonusMessage::OpenConditionModal => {
                self.modal_expression = Some(
                    ModalExpression::condition(modal.condition.clone())
                        .on_submit(ModalBonusMessage::OnConditionSelected)
                        .title("Bonus Value"),
                );
                Command::none()
            }
            ModalBonusMessage::OnConditionSelected => {
                if let Some(modal_expression) = &self.modal_expression {
                    modal.condition = modal_expression.get_condition();
                }
                Command::none()
            }
            ModalBonusMessage::ClearCondition => {
                modal.condition = None;
                Command::none()
            }
            ModalBonusMessage::SetDisplaySource(source) => {
                modal.display_source = source;
                Command::none()
            }
        }
    }
}

impl HandleView<App> for ModalBonus {
    fn handle_view<'a>(
        &'a self,
        _app: &'a App,
    ) -> iced::Element<'_, <App as Application>::Message, <App as Application>::Theme, iced::Renderer>
    {
        column!(
            row!(
                text(
                    self.title
                        .as_ref()
                        .unwrap_or(&String::from("Configure Bonus"))
                ),
                horizontal_space(),
                button(text("Cancel"))
                    .style(theme::Button::Secondary)
                    .on_press(ModalBonusMessage::Cancel.into()),
                button(text("Submit")) // TODO: only when valid
                    .style(theme::Button::Primary)
                    .on_press(ModalBonusMessage::Submit.into()),
            ),
            text("Attribute: ").size(20),
            button(
                self.attribute
                    .as_ref()
                    .map_or_else(|| text("None"), |attribute| text(attribute.to_string()))
            )
            .on_press(ModalBonusMessage::OpenAttributeModal.into()),
            text("Bonus Type").size(20),
            pick_list(BonusType::ALL, Some(self.bonus_type), |selected| {
                ModalBonusMessage::SetBonusType(selected).into()
            }),
            text("Value").size(20),
            self.value
                .as_ref()
                .map_or_else(|| text("None Set"), |value| text(value.to_string())),
            button("Set Value").on_press(ModalBonusMessage::OpenValueModal.into()),
            text("Condition").size(20),
            self.condition
                .as_ref()
                .map_or_else(|| text("None Set"), |condition| text(condition.to_string())),
            row!(
                button("Set Condition").on_press(ModalBonusMessage::OpenConditionModal.into()),
                button("Clear").on_press(ModalBonusMessage::ClearCondition.into())
            ),
        )
        .into()
    }
}
