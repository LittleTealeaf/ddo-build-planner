//! A Bonus is an individual bonus to an attribute, increasing or decreasing it by a certain amount.
mod bonus_type;
mod condition;
mod source;
mod template;
mod traits;
mod value;

use core::fmt::{self, Display};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utils::from_into::FromInto;

use crate::{
    attribute::{Attribute, AttributeDependencies},
    feat::Feat,
    types::{
        flag::{Flag, ToFlag},
        toggle::Toggle,
    },
};

pub use bonus_type::*;
pub use condition::*;
pub use source::*;
pub use template::*;
pub use traits::*;
pub use value::*;

/// Represents a given bonus to some [`Attribute`].
///
/// A bonus contains the [`Attribute`], a [`BonusType`], a [`Value`], a [`BonusSource`], and
/// an optional [`Condition`].
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Bonus {
    #[serde(rename = "a", alias = "attr", alias = "attribute")]
    attribute: Attribute,
    #[serde(rename = "t", alias = "type", alias = "bonus_type")]
    bonus_type: BonusType,
    #[serde(rename = "v", alias = "val", alias = "value")]
    value: Value,
    #[serde(rename = "s", alias = "src", alias = "source")]
    source: BonusSource,
    #[serde(
        rename = "c",
        alias = "cond",
        alias = "condition",
        skip_serializing_if = "Option::is_none"
    )]
    condition: Option<Condition>,
}

/// Constructors
impl Bonus {
    /// Creates a new bonus with the provided values.
    ///
    /// # Example
    ///
    /// ```
    /// use builder::{bonus::{BonusType, Bonus, Value}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Stacking, Value::from(1),
    /// Attribute::Dummy, None);
    /// ```
    /// If you are unsure about a parameter, looking at it's type will tell you what you can enter.
    #[must_use]
    pub fn new<A, T, V, S, C>(
        attribute: A,
        bonus_type: T,
        value: V,
        source: S,
        condition: C,
    ) -> Self
    where
        A: Into<Attribute>,
        T: Into<BonusType>,
        V: Into<Value>,
        S: Into<BonusSource>,
        C: Into<Option<Condition>>,
    {
        Self {
            attribute: attribute.into(),
            bonus_type: bonus_type.into(),
            value: value.into(),
            source: source.into(),
            condition: condition.into(),
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
    /// assert_eq!(dummy.attribute(), &Attribute::Dummy);
    /// assert_eq!(dummy.bonus_type(), &BonusType::Stacking);
    /// assert_eq!(dummy.value(), &Value::from(0));
    /// assert_eq!(dummy.source(), &BonusSource::Base);
    /// assert!(dummy.condition().is_none());
    /// ```
    #[must_use]
    pub fn dummy<S>(source: S) -> Self
    where
        S: Into<BonusSource>,
    {
        Self::new(Attribute::Dummy, BonusType::Stacking, 0, source, None)
    }

    /// Returns a bonus that gives the character some [`Flag`].
    #[must_use]
    pub fn flag<F, S, C>(flag: F, source: S, condition: C) -> Self
    where
        F: Into<Flag>,
        S: Into<BonusSource>,
        C: Into<Option<Condition>>,
    {
        Self::new(flag.into(), BonusType::Stacking, 1, source, condition)
    }

    /// Returns a bonus that gives the character some [`Feat`]
    #[must_use]
    pub fn feat<F, S, C>(feat: F, source: S, condition: C) -> Self
    where
        F: Into<Feat>,
        S: Into<BonusSource>,
        C: Into<Option<Condition>>,
    {
        Self::new(feat.into(), BonusType::Stacking, 1, source, condition)
    }

    /// Provides the use of a toggle
    #[must_use]
    pub fn toggle<T, S, C>(toggle: T, source: S, condition: C) -> Self
    where
        T: Into<Toggle>,
        S: Into<BonusSource>,
        C: Into<Option<Condition>>,
    {
        Self::new(
            Toggle::from_into(toggle).to_flag(),
            BonusType::Stacking,
            1,
            source,
            condition,
        )
    }
}

/// Methods
impl Bonus {
    /// Returns the attribute that the bonus applies to.
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Value}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Stacking, Value::from(10),
    /// BonusSource::Base, None);
    /// assert_eq!(bonus.attribute(), &Attribute::Dummy);
    /// ```
    #[must_use]
    pub const fn attribute(&self) -> &Attribute {
        &self.attribute
    }

    /// Returns the type that the bonus is
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Value}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Enhancement, Value::from(10),
    /// BonusSource::Base, None);
    /// assert_eq!(bonus.bonus_type(), &BonusType::Enhancement);
    /// ```
    #[must_use]
    pub const fn bonus_type(&self) -> &BonusType {
        &self.bonus_type
    }

    /// Returns the value of the bonus
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Value}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Stacking, Value::from(10),
    /// BonusSource::Base, None);
    /// assert_eq!(bonus.value(), &Value::from(10));
    /// ```
    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }

    /// Returns the source of the bonus
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Value}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Enhancement, Value::from(10),
    /// BonusSource::Base, None);
    /// assert_eq!(bonus.source(), &BonusSource::Base);
    /// ```
    #[must_use]
    pub const fn source(&self) -> &BonusSource {
        &self.source
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
    ///     Value::from(10),
    ///     BonusSource::Base,
    ///     Some(Condition::GreaterThan(Value::Attribute(Attribute::Dummy), Value::from(0)))
    /// );
    /// assert!(matches!(bonus.condition(), Some(_)));
    ///
    /// ```
    #[must_use]
    pub const fn condition(&self) -> Option<&Condition> {
        self.condition.as_ref()
    }

    /// Clones all the bonuses values, replacing the attribute.
    ///
    /// Returns a new [`Bonus`] instance.
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Condition, Value},
    /// attribute::{Attribute}, types::ability::Ability};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Quality, Value::from(10),
    /// BonusSource::Base, None);
    ///
    /// let new_bonus = bonus.clone_into_attribute(Attribute::Ability(Ability::All));
    /// assert_eq!(new_bonus.attribute(), &Attribute::Ability(Ability::All));
    /// assert_eq!(new_bonus.bonus_type(), &BonusType::Quality);
    /// assert_eq!(new_bonus.value(), &Value::from(10));
    /// assert_eq!(new_bonus.source(), &BonusSource::Base);
    /// assert!(new_bonus.condition().is_none());
    /// ```
    #[must_use]
    pub fn clone_into_attribute<A>(&self, attribute: A) -> Self
    where
        A: Into<Attribute>,
    {
        Self::new(
            attribute,
            self.bonus_type,
            self.value.clone(),
            self.source.clone(),
            self.condition.clone(),
        )
    }
}

impl HasDice for Bonus {
    fn has_dice(&self) -> bool {
        self.value.has_dice() || self.condition.as_ref().is_some_and(HasDice::has_dice)
    }
}

impl AttributeDependencies for Bonus {
    fn has_attr_dependency(&self, attribute: &Attribute) -> bool {
        self.value.has_attr_dependency(attribute)
            || self
                .condition
                .as_ref()
                .map_or(false, |cond| cond.has_attr_dependency(attribute))
    }

    fn include_attr_dependency(&self, set: &mut HashSet<Attribute>) {
        self.value.include_attr_dependency(set);
        if let Some(condition) = &self.condition {
            condition.include_attr_dependency(set);
        }
    }
}

impl Display for Bonus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl From<Bonus> for Value {
    fn from(value: Bonus) -> Self {
        value.value
    }
}

impl From<Bonus> for Attribute {
    fn from(value: Bonus) -> Self {
        value.attribute
    }
}

#[cfg(test)]
mod tests {
    use crate::types::ability::Ability;

    use super::*;

    #[test]
    fn serializes_and_deserializes_correct() {
        let bonus = Bonus::new(
            Attribute::Ability(Ability::Strength),
            BonusType::Profane,
            Value::Const(10.into()),
            BonusSource::Debug(3),
            None,
        );

        let serialized = ron::to_string(&bonus).unwrap();
        let deserialized: Bonus = ron::from_str(&serialized).unwrap();

        assert_eq!(bonus.attribute(), deserialized.attribute());
        assert!(deserialized.condition.is_none());
        assert_eq!(bonus.bonus_type, deserialized.bonus_type);
    }
}
