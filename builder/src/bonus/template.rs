#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use utils::from_into::FromInto;

use crate::{
    attribute::Attribute,
    feat::Feat,
    types::{flag::Flag, toggle::Toggle},
};

use super::{Bonus, BonusSource, BonusType, Condition, Value};

/// Represents a template of a bonus. In other words, a bonus without it's bonus source.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct BonusTemplate {
    #[serde(rename = "a", alias = "attr", alias = "attribute")]
    attribute: Attribute,
    #[serde(rename = "t", alias = "type", alias = "bonus_type")]
    bonus_type: BonusType,
    #[serde(rename = "v", alias = "val", alias = "value")]
    value: Value,
    #[serde(
        rename = "c",
        alias = "cond",
        alias = "condition",
        skip_serializing_if = "Option::is_none"
    )]
    condition: Option<Condition>,
    #[serde(
        rename = "d",
        alias = "display_source",
        skip_serializing_if = "Option::is_none"
    )]
    display_source: Option<BonusSource>,
}

/// Fetching Types
impl BonusTemplate {
    #[must_use]
    pub const fn attribute(&self) -> &Attribute {
        &self.attribute
    }

    #[must_use]
    pub const fn bonus_type(&self) -> &BonusType {
        &self.bonus_type
    }

    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }

    #[must_use]
    pub const fn condition(&self) -> Option<&Condition> {
        self.condition.as_ref()
    }

    #[must_use]
    pub const fn display_source(&self) -> Option<&BonusSource> {
        self.display_source.as_ref()
    }
}

impl BonusTemplate {
    #[must_use]
    pub fn new<A, T, V>(attribute: A, bonus_type: T, value: V) -> Self
    where
        A: Into<Attribute>,
        T: Into<BonusType>,
        V: Into<Value>,
    {
        Self {
            attribute: attribute.into(),
            bonus_type: bonus_type.into(),
            value: value.into(),
            condition: None,
            display_source: None,
        }
    }

    pub fn flag<F>(flag: F) -> Self
    where
        F: Into<Flag>,
    {
        Self::new(flag.into(), BonusType::Stacking, Value::ONE)
    }

    pub fn toggle<T>(toggle: T) -> Self
    where
        T: Into<Toggle>,
    {
        Self::flag(Toggle::from_into(toggle))
    }

    pub fn feat<F>(feat: F) -> Self
    where
        F: Into<Feat>,
    {
        Self::new(feat.into(), BonusType::Stacking, Value::ONE)
    }
}

/// Modifier Constructors
impl BonusTemplate {
    #[must_use]
    pub fn with_attribute<A>(self, attribute: A) -> Self
    where
        A: Into<Attribute>,
    {
        Self {
            attribute: attribute.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_bonus_type<T>(self, bonus_type: T) -> Self
    where
        T: Into<BonusType>,
    {
        Self {
            bonus_type: bonus_type.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_value<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        Self {
            value: value.into(),
            ..self
        }
    }

    #[must_use]
    pub fn with_condition<C>(self, condition: C) -> Self
    where
        C: Into<Condition>,
    {
        Self {
            condition: Some(condition.into()),
            ..self
        }
    }

    #[must_use]
    pub fn with_condition_and<C>(self, condition: C) -> Self
    where
        C: Into<Condition>,
    {
        if let Some(current) = self.condition {
            Self {
                condition: Some(current & condition.into()),
                ..self
            }
        } else {
            self.with_condition(condition)
        }
    }

    #[must_use]
    pub fn with_condition_or<C>(self, condition: C) -> Self
    where
        C: Into<Condition>,
    {
        if let Some(current) = self.condition {
            Self {
                condition: Some(current | condition.into()),
                ..self
            }
        } else {
            self.with_condition(condition)
        }
    }

    #[must_use]
    pub fn with_condition_xor<C>(self, condition: C) -> Self
    where
        C: Into<Condition>,
    {
        if let Some(current) = self.condition {
            Self {
                condition: Some(current ^ condition.into()),
                ..self
            }
        } else {
            self.with_condition(condition)
        }
    }

    #[must_use]
    pub fn without_condition(self) -> Self {
        Self {
            condition: None,
            ..self
        }
    }

    #[must_use]
    pub fn with_dislay_source<S>(self, display_source: S) -> Self
    where
        S: Into<BonusSource>,
    {
        Self {
            display_source: Some(display_source.into()),
            ..self
        }
    }

    #[must_use]
    pub fn without_display_source(self) -> Self {
        Self {
            display_source: None,
            ..self
        }
    }
}

impl BonusTemplate {
    pub fn to_bonus<S>(self, source: S) -> Bonus
    where
        S: Into<BonusSource>,
    {
        Bonus {
            attribute: self.attribute,
            bonus_type: self.bonus_type,
            value: self.value,
            condition: self.condition,
            display_source: self.display_source,
            source: source.into(),
        }
    }
}

impl From<Bonus> for BonusTemplate {
    fn from(value: Bonus) -> Self {
        Self {
            attribute: value.attribute,
            bonus_type: value.bonus_type,
            value: value.value,
            condition: value.condition,
            display_source: value.display_source,
        }
    }
}

impl From<(BonusTemplate, BonusSource)> for Bonus {
    fn from((template, source): (BonusTemplate, BonusSource)) -> Self {
        template.to_bonus(source)
    }
}
