use serde::{Deserialize, Serialize};

use crate::{
    attribute::Attribute,
    feat::Feat,
    types::{
        flag::{Flag, ToFlag},
        toggle::Toggle,
    },
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
}

impl BonusTemplate {
    /// Creates a new bonus template with the following values
    #[must_use]
    pub fn new<A, T, V, C>(attribute: A, bonus_type: T, value: V, condition: C) -> Self
    where
        A: Into<Attribute>,
        T: Into<BonusType>,
        V: Into<Value>,
        C: Into<Option<Condition>>,
    {
        Self {
            attribute: attribute.into(),
            bonus_type: bonus_type.into(),
            value: value.into(),
            condition: condition.into(),
        }
    }

    /// Provides the use of a given toggle
    #[must_use]
    pub fn toggle<T, C>(toggle: T, condition: C) -> Self
    where
        T: Into<Toggle>,
        C: Into<Option<Condition>>,
    {
        let toggle: Toggle = toggle.into();
        Self::new(toggle.to_flag(), BonusType::Stacking, 1, condition)
    }

    /// Provides the specified flag
    #[must_use]
    pub fn flag<F, C>(flag: F, condition: C) -> Self
    where
        F: Into<Flag>,
        C: Into<Option<Condition>>,
    {
        Self::new(flag.into(), BonusType::Stacking, 1, condition)
    }

    /// Provides the feat
    #[must_use]
    pub fn feat<F, C>(feat: F, condition: C) -> Self
    where
        F: Into<Feat>,
        C: Into<Option<Condition>>,
    {
        Self::new(feat.into(), BonusType::Stacking, 1, condition)
    }

    /// Converts this bonus template into a bonus
    #[must_use]
    pub fn to_bonus<S>(self, source: S) -> Bonus
    where
        S: Into<BonusSource>,
    {
        Bonus::new(
            self.attribute,
            self.bonus_type,
            self.value,
            self.condition,
            source,
        )
    }

    /// Returns a reference to the attribute of this [`BonusTemplate`].
    #[must_use]
    pub const fn attribute(&self) -> &Attribute {
        &self.attribute
    }

    /// Returns a reference to the bonus type of this [`BonusTemplate`].
    #[must_use]
    pub const fn bonus_type(&self) -> &BonusType {
        &self.bonus_type
    }

    /// Returns a reference to the value of this [`BonusTemplate`].
    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }

    /// Returns a reference to the condition of this [`BonusTemplate`].
    #[must_use]
    pub const fn condition(&self) -> &Option<Condition> {
        &self.condition
    }

    /// Sets the attribute of this [`BonusTemplate`].
    pub fn set_attribute<A>(&mut self, attribute: A)
    where
        A: Into<Attribute>,
    {
        self.attribute = attribute.into();
    }

    /// Sets the bonus type of this [`BonusTemplate`].
    pub fn set_bonus_type<T>(&mut self, bonus_type: T)
    where
        T: Into<BonusType>,
    {
        self.bonus_type = bonus_type.into();
    }

    /// Sets the value of this [`BonusTemplate`].
    pub fn set_value<V>(&mut self, value: V)
    where
        V: Into<Value>,
    {
        self.value = value.into();
    }

    /// Sets the condition of this [`BonusTemplate`].
    pub fn set_condition<C>(&mut self, condition: C)
    where
        C: Into<Option<Condition>>,
    {
        self.condition = condition.into();
    }
}

impl From<Bonus> for BonusTemplate {
    fn from(value: Bonus) -> Self {
        Self::new(
            value.attribute,
            value.bonus_type,
            value.value,
            value.condition,
        )
    }
}
