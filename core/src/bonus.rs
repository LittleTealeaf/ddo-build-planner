mod bonus_type;
mod condition;
mod source;
pub mod traits;
mod value;

pub use bonus_type::*;
pub use condition::*;
pub use source::*;
pub use value::*;

use crate::attribute::Attribute;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Bonus {
    attribute: Attribute,
    value: BonusValue,
    r#type: BonusType,
    condition: Option<BonusCondition>,
    source: BonusSource,
}

impl Bonus {
    pub fn new<A, V, T, S>(attribute: A, value: V, bonus_type: T, source: S) -> Self
    where
        A: Into<Attribute>,
        V: Into<BonusValue>,
        T: Into<BonusType>,
        S: Into<BonusSource>,
    {
        Self {
            attribute: attribute.into(),
            value: value.into(),
            r#type: bonus_type.into(),
            condition: None,
            source: source.into(),
        }
    }

    #[must_use]
    pub fn with_condition<C>(self, condition: C) -> Self
    where
        C: Into<BonusCondition>,
    {
        Self {
            condition: Some(condition.into()),
            ..self
        }
    }

    #[must_use]
    pub fn with_condition_maybe(self, condition: Option<BonusCondition>) -> Self {
        if let Some(condition) = condition {
            self.with_condition(condition)
        } else {
            self
        }
    }

    #[must_use]
    pub fn with_condition_and<C>(self, condition: C) -> Self
    where
        C: Into<BonusCondition>,
    {
        let condition = condition.into();
        Self {
            condition: match self.condition {
                Some(cond) => Some(cond & condition),
                None => Some(condition),
            },
            ..self
        }
    }

    #[must_use]
    pub fn with_condition_or<C>(self, condition: C) -> Self
    where
        C: Into<BonusCondition>,
    {
        let condition = condition.into();
        Self {
            condition: match self.condition {
                Some(cond) => Some(cond | condition),
                None => Some(condition),
            },
            ..self
        }
    }

    #[must_use]
    pub const fn condition(&self) -> &Option<BonusCondition> {
        &self.condition
    }
}
