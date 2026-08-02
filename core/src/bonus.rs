mod bonus_type;
mod condition;
mod source;
pub mod traits;
mod value;

use core::fmt::Display;

pub use bonus_type::*;
pub use condition::*;
pub use source::*;
pub use value::*;

use crate::{attribute::Attribute, bonus::traits::ContainsAttribute};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Bonus {
    attribute: Attribute,
    value: BonusValue,
    r#type: BonusType,
    condition: Option<BonusCondition>,
    source: BonusSource,
}

impl Display for Bonus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "+({}) {} bonus to {}",
            self.value, self.r#type, self.attribute
        )?;
        if let Some(condition) = &self.condition {
            write!(f, " if {condition}")?;
        }
        write!(f, " [{}]", self.source)?;
        Ok(())
    }
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

    #[must_use]
    pub const fn bonus_type(&self) -> &BonusType {
        &self.r#type
    }

    #[must_use]
    pub const fn value(&self) -> &BonusValue {
        &self.value
    }

    #[must_use]
    pub const fn attribute(&self) -> &Attribute {
        &self.attribute
    }

    #[must_use]
    pub const fn source(&self) -> &BonusSource {
        &self.source
    }
}

impl ContainsAttribute for Bonus {
    fn contains_attribute(&self, attribute: &Attribute) -> bool {
        self.value.contains_attribute(attribute)
            || self
                .condition()
                .as_ref()
                .is_some_and(|cond| cond.contains_attribute(attribute))
    }
}
