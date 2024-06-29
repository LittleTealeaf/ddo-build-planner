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
        slider::Slider,
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
    #[serde(
        rename = "c",
        alias = "cond",
        alias = "condition",
        skip_serializing_if = "Option::is_none"
    )]
    condition: Option<Condition>,
    #[serde(rename = "s", alias = "src", alias = "source")]
    source: BonusSource,
    #[serde(
        rename = "d",
        alias = "display_source",
        skip_serializing_if = "Option::is_none"
    )]
    display_source: Option<BonusSource>,
}

impl Bonus {
    /// Creates a new [`Bonus`] with the required fields.
    ///
    /// Additional optional fields can be assigned using the `with_` methods, such as
    /// [`Bonus::with_condition`]
    pub fn new<A, T, V, S>(attribute: A, bonus_type: T, value: V, source: S) -> Self
    where
        A: Into<Attribute>,
        T: Into<BonusType>,
        V: Into<Value>,
        S: Into<BonusSource>,
    {
        Self {
            attribute: attribute.into(),
            bonus_type: bonus_type.into(),
            value: value.into(),
            source: source.into(),
            condition: None,
            display_source: None,
        }
    }

    /// Creates a dummy bonus
    pub fn dummy<S>(source: S) -> Self
    where
        S: Into<BonusSource>,
    {
        Self::new(Attribute::Dummy, BonusType::Stacking, Value::ZERO, source)
    }

    /// Creates a [`Bonus`] that provides a +1 [`BonusType::Stacking`] bonus to a [`Flag`]
    pub fn flag<F, S>(flag: F, source: S) -> Self
    where
        F: Into<Flag>,
        S: Into<BonusSource>,
    {
        Self::new(flag.into(), BonusType::Stacking, Value::ONE, source)
    }

    /// Creates a [`Bonus`] that provides a +1 [`BonusType::Stacking`] bonus to a [`Feat`]
    pub fn feat<F, S>(feat: F, source: S) -> Self
    where
        F: Into<Feat>,
        S: Into<BonusSource>,
    {
        Self::new(feat.into(), BonusType::Stacking, Value::ONE, source)
    }

    /// Creates a [`Bonus`] that provides the use of a [`Toggle`]
    pub fn toggle<T, S>(toggle: T, source: S) -> Self
    where
        T: Into<Toggle>,
        S: Into<BonusSource>,
    {
        Self::new(
            Toggle::from_into(toggle).to_flag(),
            BonusType::Stacking,
            Value::ONE,
            source,
        )
    }

    /// Creates a new [`Bonus`] that provides the user with the ability to use a [`Slider`]
    pub fn slider<T, S>(slider: T, source: S) -> Self
    where
        T: Into<Slider>,
        S: Into<BonusSource>,
    {
        Self::flag(Slider::from_into(slider), source)
    }
}

/// Modifier Constructors
impl Bonus {
    /// Updates the [`Attribute`] and returns the result
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

    /// Updates the [`BonusType`] and returns the result
    #[must_use]
    pub fn with_type<T>(self, bonus_type: T) -> Self
    where
        T: Into<BonusType>,
    {
        Self {
            bonus_type: bonus_type.into(),
            ..self
        }
    }

    /// Updates the [`BonusSource`] and returns the result
    #[must_use]
    pub fn with_source<S>(self, source: S) -> Self
    where
        S: Into<BonusSource>,
    {
        Self {
            source: source.into(),
            ..self
        }
    }

    /// Updates the [`Value`] and returns the result
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

    /// Updates the [`Condition`] and returns the result
    #[must_use]
    pub fn with_condition<C>(self, condition: C) -> Self
    where
        C: Into<Option<Condition>>,
    {
        Self {
            condition: condition.into(),
            ..self
        }
    }

    /// Updates the [`Condition`] and returns the result.
    ///
    /// If both the current condition and the provided condition exist, this will set the condition
    /// as the AND product of both conditions
    #[must_use]
    pub fn with_condition_and<C>(self, condition: C) -> Self
    where
        C: Into<Option<Condition>>,
    {
        Self {
            condition: match (self.condition, condition.into()) {
                (Some(a), Some(b)) => Some(a & b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            ..self
        }
    }

    /// Updates the [`Condition`] and returns the result.
    ///
    /// If both the current condition and the provided condition exist, this will set the condition
    /// as the OR product of both conditions
    #[must_use]
    pub fn with_condition_or<C>(self, condition: C) -> Self
    where
        C: Into<Option<Condition>>,
    {
        Self {
            condition: match (self.condition, condition.into()) {
                (Some(a), Some(b)) => Some(a | b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            ..self
        }
    }

    /// Updates the [`Condition`] and returns the result.
    ///
    /// If both the current condition and the provided condition exist, this will set the condition
    /// as the XOR product of both conditions
    #[must_use]
    pub fn with_condition_xor<C>(self, condition: C) -> Self
    where
        C: Into<Option<Condition>>,
    {
        Self {
            condition: match (self.condition, condition.into()) {
                (Some(a), Some(b)) => Some(a ^ b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            ..self
        }
    }

    /// Sets the displayed [`BonusSource`]
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

    /// Clears the displayed [`BonusSource`]
    #[must_use]
    pub fn without_display_source(self) -> Self {
        Self {
            display_source: None,
            ..self
        }
    }
}

/// Paramter Values
impl Bonus {
    /// Returns the attribute that the bonus applies to.
    ///
    /// # Example
    /// ```
    /// use builder::{bonus::{Bonus, BonusType, BonusSource, Value}, attribute::Attribute};
    ///
    /// let bonus = Bonus::new(Attribute::Dummy, BonusType::Stacking, Value::from(10),
    /// BonusSource::Base);
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
    /// BonusSource::Base);
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
    /// BonusSource::Base);
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
    /// BonusSource::Base);
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
    /// ).with_condition(
    ///     Condition::GreaterThan(Value::Attribute(Attribute::Dummy), Value::from(0)),
    /// );
    /// assert!(matches!(bonus.condition(), Some(_)));
    ///
    /// ```
    #[must_use]
    pub const fn condition(&self) -> Option<&Condition> {
        self.condition.as_ref()
    }

    /// TODO: documentation
    #[must_use]
    pub const fn display_source(&self) -> Option<&BonusSource> {
        self.display_source.as_ref()
    }

    /// Returns the displayed source. Either this is [`Self::display_source`], or if the display
    /// source is not set, then [`Self::source`] is returned
    #[must_use]
    pub fn displayed_source(&self) -> &BonusSource {
        self.display_source().unwrap_or(&self.source)
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
    /// BonusSource::Base);
    ///
    /// let new_bonus = bonus.clone_with_attribute(Attribute::Ability(Ability::All));
    /// assert_eq!(new_bonus.attribute(), &Attribute::Ability(Ability::All));
    /// assert_eq!(new_bonus.bonus_type(), &BonusType::Quality);
    /// assert_eq!(new_bonus.value(), &Value::from(10));
    /// assert_eq!(new_bonus.source(), &BonusSource::Base);
    /// assert!(new_bonus.condition().is_none());
    /// ```
    #[must_use]
    pub fn clone_with_attribute<A>(&self, attribute: A) -> Self
    where
        A: Into<Attribute>,
    {
        Self {
            attribute: attribute.into(),
            bonus_type: self.bonus_type,
            value: self.value.clone(),
            source: self.source.clone(),
            display_source: self.display_source.clone(),
            condition: self.condition.clone(),
        }
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
    use crate::{types::ability::Ability, val};

    use super::*;

    #[test]
    fn serializes_and_deserializes_correct() {
        let bonus = Bonus::new(
            Ability::Strength,
            BonusType::Profane,
            val!(10),
            BonusSource::Debug(3),
        );

        let serialized = ron::to_string(&bonus).unwrap();
        let deserialized: Bonus = ron::from_str(&serialized).unwrap();

        assert_eq!(bonus.attribute(), deserialized.attribute());
        assert!(deserialized.condition.is_none());
        assert_eq!(bonus.bonus_type, deserialized.bonus_type);
    }
}
