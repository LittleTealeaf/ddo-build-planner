mod bonus_source;
pub use bonus_source::*;
mod bonus_types;
pub use bonus_types::*;
mod condition;
pub use condition::*;
mod traits;
use itertools::Itertools;
pub use traits::*;

use crate::attribute::sub::{Flag, Toggle};

use super::attribute::Attribute;

/// Describes a bonus for an attribute by it's type, value, source, and any conditions.
///
/// A bonus that consists of the following features:
///
/// **Attribute**: The [Attribute] that the bonus applies to.
///
/// **Bonus Type**: The [BonusType] that the bonus is. Bonuses of the same [BonusType] will not stack (unless it is [BonusType::Stacking]), but bonuses of different types will.
///
/// **Value**: The value of the bonus. This is stored as a [f32].
///
/// **Source**: The original source of the bonus, represented with [BonusSource]. This is used to automatically remove / replace bonuses of the same source.
///
/// **Conditions**: Any [Conditions](Condition) that must be met for this bonus to apply. Stored as a `Vec<Condition>`.
///
/// # Examples
///
/// ```
/// use builder_core::{
///     bonus::{Bonus, BonusType, BonusSource},
///     attribute::Attribute
/// };
///
/// let bonus = Bonus::new(
///     Attribute::Dodge(),
///     BonusType::Stacking,
///     5f32,
///     BonusSource::Unique(0),
///     None
/// );
///
/// assert_eq!(Attribute::Dodge(), bonus.get_attribute());
/// assert_eq!(BonusType::Stacking, bonus.get_bonus_type());
/// assert!(5f32 == bonus.get_value());
/// assert_eq!(BonusSource::Unique(0), bonus.get_source());
/// assert_eq!(None, bonus.get_conditions());
/// ```
///
#[derive(PartialEq, Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Bonus {
    #[serde(rename = "atr")]
    attribute: Attribute,
    #[serde(rename = "ty")]
    bonus_type: BonusType,
    #[serde(rename = "val")]
    value: f32,
    #[serde(rename = "src")]
    source: BonusSource,
    #[serde(rename = "cond", skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<Condition>>,
}

impl ToString for Bonus {
    fn to_string(&self) -> String {
        if let Some(conditions) = &self.conditions {
            format!(
                "{} {} bonus to {} when {}",
                self.value,
                self.bonus_type.to_string(),
                self.attribute.to_string(),
                conditions.iter().map(Condition::to_string).join(", ")
            )
        } else {
            format!(
                "{} {} bonus to {}",
                self.value,
                self.bonus_type.to_string(),
                self.attribute.to_string()
            )
        }
    }
}

impl Bonus {
    /// Creates a new [Bonus].
    #[inline(always)]
    pub fn new(
        attribute: Attribute,
        bonus_type: BonusType,
        value: f32,
        source: BonusSource,
        conditions: Option<Vec<Condition>>,
    ) -> Self {
        Self {
            attribute,
            bonus_type,
            value,
            source,
            conditions,
        }
    }
    /// Creates a dummy bonus.
    ///
    /// The bonus will be for the [Attribute::Dummy()] attribute, have a stacking value of `0f32`, no conditions, and the provided source.
    #[inline(always)]
    pub fn dummy(source: BonusSource) -> Bonus {
        Self {
            attribute: Attribute::Dummy(),
            bonus_type: BonusType::Stacking,
            value: 0f32,
            source,
            conditions: None,
        }
    }

    /// Creates a simple flag bonus that gives the user that flag.
    #[inline(always)]
    pub fn flag(flag: Flag, source: BonusSource) -> Bonus {
        Self {
            attribute: Attribute::Flag(flag),
            bonus_type: BonusType::Stacking,
            value: 1f32,
            source,
            conditions: None,
        }
    }

    /// Creates a simple toggle bonus that gives the user that toggle.
    #[inline(always)]
    pub fn toggle(toggle: Toggle, source: BonusSource) -> Bonus {
        Self {
            attribute: Attribute::Toggle(toggle),
            bonus_type: BonusType::Stacking,
            value: 1f32,
            source,
            conditions: None,
        }
    }

    /// Returns the attribute of the bonus
    pub fn get_attribute(&self) -> Attribute {
        self.attribute
    }

    /// Returns the value of the bonus
    pub fn get_value(&self) -> f32 {
        self.value
    }

    /// Returns the bonus type of the bonus
    pub fn get_bonus_type(&self) -> BonusType {
        self.bonus_type
    }

    /// Returns the source of the bonus
    pub fn get_source(&self) -> BonusSource {
        self.source
    }

    /// Returns the conditions of the bonus, if any.
    ///
    /// If there are no conditions, [`None`] is returned.
    pub fn get_conditions(&self) -> Option<Vec<Condition>> {
        self.conditions.clone()
    }
}
