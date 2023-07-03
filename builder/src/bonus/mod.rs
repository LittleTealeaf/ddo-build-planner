//! A Bonus is an individual bonus to an attribute, increasing or decreasing it by a certain amount.
mod bonus_type;
mod condition;
mod deserialize;
mod source;
mod traits;
mod value;

use std::fmt::Display;

use crate::attribute::{flags::Flag, Attribute, AttributeDependencies};

pub use bonus_type::*;
pub use condition::*;
use serde::{Deserialize, Serialize};
pub use source::*;
pub use traits::*;
pub use value::*;

/// Represents a given bonus to some [`Attribute`].
///
/// A bonus contains the [`Attribute`], a [`BonusType`], a [`Value`], a [`BonusSource`], and
/// an optional [`Condition`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "deserialize::DeserializedBonus")]
pub struct Bonus {
    #[serde(rename = "attr")]
    attribute: Attribute,
    #[serde(rename = "type")]
    bonus_type: BonusType,
    #[serde(rename = "val")]
    value: Value,
    #[serde(rename = "src")]
    source: BonusSource,
    #[serde(rename = "cond", skip_serializing_if = "Option::is_none")]
    condition: Option<Condition>,
}

impl Bonus {
    /// Creates a new bonus with the provided values.
    ///
    /// Many of the custom parameters implement as many [`From`] traits as needed, so many times
    /// the [`Into::into`] function can be used.
    ///
    /// # Example
    ///
    /// ```
    /// use builder::{bonus::{BonusType, Bonus}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy.into(), BonusType::Stacking, 1f32.into(),
    /// Attribute::Dummy.into(), None);
    /// ```
    /// If you are unsure about a parameter, looking at it's type will tell you what you can enter.
    pub fn new(
        attribute: Attribute,
        bonus_type: BonusType,
        value: Value,
        source: BonusSource,
        condition: Option<Condition>,
    ) -> Self {
        Self {
            attribute,
            bonus_type,
            value,
            source,
            condition,
        }
    }

    /// Creates a [`Attribute::Dummy`] with a given [`BonusSource`].
    ///
    /// Most values are kept default, since this bonus is never actually tracked.
    ///
    /// # Example
    ///
    /// ```
    /// use builder::{bonus::{Bonus, BonusSource, BonusType, Value}, attribute::Attribute};
    ///
    /// let dummy = Bonus::dummy(BonusSource::Base);
    /// assert_eq!(dummy.get_attribute(), Attribute::Dummy);
    /// assert_eq!(dummy.get_type(), BonusType::Stacking);
    /// assert_eq!(dummy.get_value(), Value::Value(0f32));
    /// assert_eq!(dummy.get_source(), BonusSource::Base);
    /// assert!(dummy.get_condition().is_none());
    /// ```
    pub fn dummy(source: BonusSource) -> Bonus {
        Self::new(
            Attribute::Dummy,
            BonusType::Stacking,
            0f32.into(),
            source,
            None,
        )
    }

    /// Returns a bonus that gives the character some [`Flag`].
    pub fn flag(flag: Flag, source: BonusSource) -> Bonus {
        Self::new(
            Attribute::Flag(flag),
            BonusType::Stacking,
            1f32.into(),
            source,
            None,
        )
    }

    /// Returns the attribute that the bonus applies to.
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Stacking, 10f32.into(),
    /// BonusSource::Base, None);
    /// assert_eq!(bonus.get_attribute(), Attribute::Dummy);
    /// ```
    pub fn get_attribute(&self) -> Attribute {
        self.attribute
    }

    /// Returns the type that the bonus is
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Enhancement, 10f32.into(),
    /// BonusSource::Base, None);
    /// assert_eq!(bonus.get_type(), BonusType::Enhancement);
    /// ```
    pub fn get_type(&self) -> BonusType {
        self.bonus_type
    }

    /// Returns the value of the bonus
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Value}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Stacking, Value::Value(10f32),
    /// BonusSource::Base, None);
    /// assert_eq!(bonus.get_value(), Value::Value(10f32));
    /// ```
    pub fn get_value(&self) -> Value {
        self.value.clone()
    }

    /// Returns the source of the bonus
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Enhancement, 10f32.into(),
    /// BonusSource::Base, None);
    /// assert_eq!(bonus.get_source(), BonusSource::Base);
    /// ```
    pub fn get_source(&self) -> BonusSource {
        self.source
    }

    /// Returns the condition of the bonus
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Condition, Value},
    /// attribute::Attribute};
    ///
    /// let bonus = Bonus::new(
    ///     Attribute::Dummy,
    ///     BonusType::Quality,
    ///     10f32.into(),
    ///     BonusSource::Base,
    ///     Some(Condition::GreaterThan(Value::Attribute(Attribute::Dummy), Value::Value(0f32)))
    /// );
    /// assert!(matches!(bonus.get_condition(), Some(_)));
    ///
    /// ```
    pub fn get_condition(&self) -> Option<Condition> {
        self.condition.clone()
    }

    /// Clones all of the bonuse's values, replacing the attribute.
    ///
    /// Returns a new [`Bonus`] instance.
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Condition, Value},
    /// attribute::{types::Ability, Attribute}};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Quality, 10f32.into(),
    /// BonusSource::Base, None);
    ///
    /// let new_bonus = bonus.clone_into_attribute(Attribute::Ability(Ability::All));
    /// assert_eq!(new_bonus.get_attribute(), Attribute::Ability(Ability::All));
    /// assert_eq!(new_bonus.get_type(), BonusType::Quality);
    /// assert_eq!(new_bonus.get_value(), Value::Value(10f32));
    /// assert_eq!(new_bonus.get_source(), BonusSource::Base);
    /// assert!(new_bonus.get_condition().is_none());
    /// ```
    pub fn clone_into_attribute(&self, attribute: Attribute) -> Bonus {
        Bonus::new(
            attribute,
            self.bonus_type,
            self.value.clone(),
            self.source,
            self.condition.clone(),
        )
    }
}

impl AttributeDependencies for Bonus {
    fn has_attr_dependency(&self, attribute: Attribute) -> bool {
        self.value.has_attr_dependency(attribute)
            || self
                .condition
                .as_ref()
                .map(|cond| cond.has_attr_dependency(attribute))
                .unwrap_or(false)
    }
}

impl Display for Bonus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(condition) = &self.condition {
            write!(
                f,
                "{} {} bonus to {} if {}",
                self.value, self.bonus_type, self.attribute, condition
            )
        } else {
            write!(
                f,
                "{} {} bonus to {}",
                self.value, self.bonus_type, self.attribute
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attribute::types::Ability;

    use super::*;

    #[test]
    fn serializes_and_deserializes_correct() {
        let bonus = Bonus::new(
            Attribute::Ability(Ability::Strength),
            BonusType::Profane,
            Value::Value(10f32),
            BonusSource::Debug(3),
            None,
        );

        let serialized = ron::to_string(&bonus).unwrap();
        let deserialized: Bonus = ron::from_str(&serialized).unwrap();

        assert_eq!(bonus.get_attribute(), deserialized.get_attribute());
        assert!(deserialized.condition.is_none());
        assert_eq!(bonus.bonus_type, deserialized.bonus_type);
    }
}
