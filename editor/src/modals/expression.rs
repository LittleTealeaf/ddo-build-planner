use core::convert::Into;

use builder::bonus::{Condition, Value};
use condition::ConditionSelector;
use value::ValueSelector;

use crate::Message;

mod condition;
mod value;

#[derive(Debug, Clone)]
pub struct ExpressionSelector {
    title: Option<String>,
    selector: InternalSelector,
    on_submit: Option<Message>,
    on_cancel: Option<Message>,
}

impl ExpressionSelector {
    pub fn value<V>(value: V) -> Self
    where
        V: Into<Option<Value>>,
    {
        Self {
            title: None,
            selector: InternalSelector::Value(ValueSelector::new(value)),
            on_cancel: None,
            on_submit: None,
        }
    }
    pub fn condition<C>(condition: C) -> Self
    where
        C: Into<Option<Condition>>,
    {
        Self {
            title: None,
            selector: InternalSelector::Condition(ConditionSelector::new(condition)),
            on_cancel: None,
            on_submit: None,
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
}

#[derive(Debug, Clone)]
enum InternalSelector {
    Value(ValueSelector),
    Condition(ConditionSelector),
}
