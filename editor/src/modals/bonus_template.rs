use builder::{
    attribute::Attribute,
    bonus::{BonusSource, BonusTemplate, BonusType, Condition, Value},
};
use iced::{Application, Command};
use ui::{HandleMessage, HandleView};

use crate::{App, Message};

use super::expression::ModalExpression;

#[derive(Debug, Clone)]
pub struct ModalBonus {
    attribute: Option<Attribute>,
    bonus_type: BonusType,
    value: Option<Value>,
    condition: Option<Condition>,
    display_source: Option<BonusSource>,
    on_cancel: Option<Message>,
    on_submit: Option<Message>,
}

impl ModalBonus {
    pub fn new<B>(bonus: Option<B>) -> Self
    where
        B: Into<BonusTemplate>,
    {
        let bonus: Option<BonusTemplate> = bonus.map(Into::into);

        Self {
            attribute: bonus.as_ref().map(|b| b.attribute().clone()),
            bonus_type: bonus
                .as_ref()
                .map(|b| b.bonus_type().clone())
                .unwrap_or_default(),
            display_source: bonus.as_ref().and_then(|b| b.display_source().cloned()),
            condition: bonus.as_ref().and_then(|b| b.condition().cloned()),
            on_cancel: None,
            on_submit: None,
            value: bonus.as_ref().map(|b| b.value().clone()),
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
        let Some(modal) = &self.modal_bonus else {
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
                self.modal_attribute = Some(
                    self.select_attribute()
                        .title("Bonus Attribute")
                        .on_submit(ModalBonusMessage::OnAttributeSelected)
                        .select_maybe(modal.attribute.clone()),
                );
                Command::none()
            }
            ModalBonusMessage::OnAttributeSelected => {
                if let (Some(modal_attribute), Some(modal)) =
                    (&self.modal_attribute, &mut self.modal_bonus)
                {
                    modal.attribute = modal_attribute.get_attribute();
                }
                Command::none()
            }
            ModalBonusMessage::SetBonusType(bonus_type) => {
                if let Some(modal) = &mut self.modal_bonus {
                    modal.bonus_type = bonus_type;
                }
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
                if let (Some(modal_expression), Some(modal)) =
                    (&self.modal_expression, &mut self.modal_bonus)
                {
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
                if let (Some(modal_expression), Some(modal)) =
                    (&self.modal_expression, &mut self.modal_bonus)
                {
                    modal.condition = modal_expression.get_condition();
                }
                Command::none()
            }
            ModalBonusMessage::ClearCondition => {
                if let Some(modal) = &mut self.modal_bonus {
                    modal.condition = None;
                }
                Command::none()
            }
            ModalBonusMessage::SetDisplaySource(source) => {
                if let Some(modal) = &mut self.modal_bonus {
                    modal.display_source = source;
                }
                Command::none()
            }
        }
    }
}

impl HandleView<App> for ModalBonus {
    fn handle_view<'a>(
        &'a self,
        app: &'a App,
    ) -> iced::Element<'_, <App as Application>::Message, <App as Application>::Theme, iced::Renderer>
    {
        todo!()
    }
}
