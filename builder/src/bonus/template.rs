use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

use super::{Bonus, BonusSource, BonusType, Condition, Value};

/// Represents a template of a bonus. In other words, a bonus without it's bonus source.
#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct BonusTemplate {
    #[serde(rename = "attr")]
    attribute: Attribute,
    #[serde(rename = "type")]
    bonus_type: BonusType,
    #[serde(rename = "val")]
    value: Value,
    #[serde(rename = "cond", skip_serializing_if = "Option::is_none")]
    condition: Option<Condition>,
}

impl BonusTemplate {
    /// Creates a new bonus template with the following values
    #[must_use]
    pub fn new(
        attribute: impl Into<Attribute>,
        bonus_type: impl Into<BonusType>,
        value: impl Into<Value>,
        condition: impl Into<Option<Condition>>,
    ) -> Self {
        Self {
            attribute: attribute.into(),
            bonus_type: bonus_type.into(),
            value: value.into(),
            condition: condition.into(),
        }
    }

    /// Converts this bonus template into a bonus
    #[must_use]
    pub fn to_bonus(self, source: BonusSource) -> Bonus {
        Bonus::new(
            self.attribute,
            self.bonus_type,
            self.value,
            source,
            self.condition,
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
    pub fn set_attribute(&mut self, attribute: impl Into<Attribute>) {
        self.attribute = attribute.into();
    }

    /// Sets the bonus type of this [`BonusTemplate`].
    pub fn set_bonus_type(&mut self, bonus_type: impl Into<BonusType>) {
        self.bonus_type = bonus_type.into();
    }

    /// Sets the value of this [`BonusTemplate`].
    pub fn set_value(&mut self, value: impl Into<Value>) {
        self.value = value.into();
    }

    /// Sets the condition of this [`BonusTemplate`].
    pub fn set_condition(&mut self, condition: impl Into<Option<Condition>>) {
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
