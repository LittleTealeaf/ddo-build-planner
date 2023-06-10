//! A Bonus is an individual bonus to an attribute, increasing or decreasing it by a certain amount.
mod bonus_type;
mod condition;
mod deserialize;
mod source;
mod traits;
mod value;

use std::fmt::Display;

use crate::attribute::{flags::Flag, Attribute};

pub use bonus_type::*;
pub use condition::*;
use serde::{Deserialize, Serialize};
pub use source::*;
pub use traits::*;
pub use value::*;

/// Represents a given bonus to some [`Attribute`].
///
/// A bonus contains the [`Attribute`], a [`BonusType`], a [`BonusValue`], a [`BonusSource`], and
/// an optional [`Condition`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "deserialize::DeserializedBonus")]
pub struct Bonus {
    #[serde(rename = "attr")]
    attribute: Attribute,
    #[serde(rename = "type")]
    bonus_type: BonusType,
    #[serde(rename = "val")]
    value: BonusValue,
    #[serde(rename = "src")]
    source: BonusSource,
    #[serde(rename = "cond", skip_serializing_if = "Option::is_none")]
    condition: Option<Condition>,
    #[serde(skip)]
    dependencies: Option<Vec<Attribute>>,
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
        value: BonusValue,
        source: BonusSource,
        condition: Option<Condition>,
    ) -> Self {
        let mut deps = Vec::new();

        if let Some(condition) = &condition {
            for dep in condition.get_dependencies() {
                if !deps.contains(&dep) {
                    deps.push(dep);
                }
            }
        }

        if let Some(value_deps) = value.get_dependencies() {
            for dep in value_deps {
                if !deps.contains(&dep) {
                    deps.push(dep);
                }
            }
        }

        Self {
            attribute,
            bonus_type,
            value,
            source,
            condition,
            dependencies: (!deps.is_empty()).then_some(deps),
        }
    }

    /// Creates a [`Attribute::Dummy`] with a given [`BonusSource`].
    ///
    /// Most values are kept default, since this bonus is never actually tracked.
    ///
    /// # Example
    ///
    /// ```
    /// use builder::{bonus::{Bonus, BonusSource, BonusType, BonusValue}, attribute::Attribute};
    ///
    /// let dummy = Bonus::dummy(BonusSource::Base);
    /// assert_eq!(dummy.get_attribute(), Attribute::Dummy);
    /// assert_eq!(dummy.get_type(), BonusType::Stacking);
    /// assert_eq!(dummy.get_value(), BonusValue::Value(0f32));
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
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, BonusValue}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Stacking, BonusValue::Value(10f32),
    /// BonusSource::Base, None);
    /// assert_eq!(bonus.get_value(), BonusValue::Value(10f32));
    /// ```
    pub fn get_value(&self) -> BonusValue {
        self.value
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
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Condition},
    /// attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Quality, 10f32.into(),
    /// BonusSource::Base, Some(Condition::Has(Attribute::Dummy)));
    /// assert!(matches!(bonus.get_condition(), Some(Condition::Has(Attribute::Dummy))));
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
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Condition, BonusValue},
    /// attribute::{types::Ability, Attribute}};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Quality, 10f32.into(),
    /// BonusSource::Base, None);
    ///
    /// let new_bonus = bonus.clone_into_attribute(Attribute::Ability(Ability::All));
    /// assert_eq!(new_bonus.get_attribute(), Attribute::Ability(Ability::All));
    /// assert_eq!(new_bonus.get_type(), BonusType::Quality);
    /// assert_eq!(new_bonus.get_value(), BonusValue::Value(10f32));
    /// assert_eq!(new_bonus.get_source(), BonusSource::Base);
    /// assert!(new_bonus.get_condition().is_none());
    /// ```
    pub fn clone_into_attribute(&self, attribute: Attribute) -> Bonus {
        Bonus::new(
            attribute,
            self.bonus_type,
            self.value,
            self.source,
            self.condition.clone(),
        )
    }

    /// Returns all other [`Attributes`] that this bonus references
    ///
    /// # Example
    ///
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Condition, BonusValue},
    /// attribute::{types::Ability, Attribute}};
    ///
    /// let bonus = Bonus::new(Attribute::Ability(Ability::Strength), BonusType::Stacking,
    /// BonusValue::FromAttribute(Attribute::Ability(Ability::Constitution)),
    /// BonusSource::Attribute(Attribute::Ability(Ability::Wisdom)),
    /// Some(Condition::Has(Attribute::Ability(Ability::Dexterity))));
    ///
    /// let deps = bonus.get_dependencies();
    ///
    /// assert!(deps.is_some());
    /// let dependencies = deps.unwrap();
    ///
    /// assert!(dependencies.contains(&Attribute::Ability(Ability::Constitution)));
    /// assert!(dependencies.contains(&Attribute::Ability(Ability::Dexterity)));
    ///
    /// assert!(!dependencies.contains(&Attribute::Ability(Ability::Wisdom)));
    /// assert!(!dependencies.contains(&Attribute::Ability(Ability::Strength)));
    /// ```
    /// Attributes referenced in the [`BonusValue`] (see [`BonusValue::get_dependencies()`]) and
    /// [`Condition`] (see [`Condition::get_dependencies()`]) are included. However, the bonus
    /// [`Attribute`] and [`BonusSource`] are not included.
    ///
    ///
    /// [`BonusValue::get_dependencies()`]: crate::bonus::BonusValue::get_dependencies
    /// [`Condition::get_dependencies()`]: crate::bonus::Condition::get_dependencies
    /// [`Attributes`]: crate::attribute::Attribute
    pub fn get_dependencies(&self) -> Option<Vec<Attribute>> {
        self.dependencies.clone()
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
            BonusValue::Value(10f32),
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
