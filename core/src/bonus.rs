mod bonus_type;
mod condition;
mod source;
pub mod traits;
mod value;

use core::fmt::Display;

pub use bonus_type::*;
pub use condition::*;
use getset::{CopyGetters, Getters, Setters, WithSetters};
use serde::{Deserialize, Serialize};
pub use source::*;
pub use value::*;

use crate::{attribute::Attribute, bonus::traits::ContainsAttribute};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Getters,
    Setters,
    WithSetters,
    CopyGetters,
)]
#[getset(set = "pub", set_with = "pub")]
pub struct Bonus {
    #[getset(get = "pub")]
    attribute: Attribute,

    #[getset(get = "pub")]
    value: BonusValue,

    #[allow(clippy::struct_field_names)]
    #[getset(get_copy = "pub")]
    bonus_type: BonusType,

    #[getset(get = "pub")]
    condition: Option<BonusCondition>,

    #[getset(get = "pub")]
    show_condition: Option<BonusCondition>,

    #[getset(get = "pub")]
    source: BonusSource,
}

impl Display for Bonus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "+({}) {} bonus to {}",
            self.value, self.bonus_type, self.attribute
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
            bonus_type: bonus_type.into(),
            condition: None,
            show_condition: None,
            source: source.into(),
        }
    }

    #[must_use]
    pub fn with_condition_maybe(self, condition: Option<BonusCondition>) -> Self {
        if let Some(condition) = condition {
            self.with_condition(Some(condition))
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
    pub fn with_show_condition_maybe(self, condition: Option<BonusCondition>) -> Self {
        if let Some(condition) = condition {
            self.with_show_condition(Some(condition))
        } else {
            self
        }
    }

    #[must_use]
    pub fn with_show_condition_and<C>(self, condition: C) -> Self
    where
        C: Into<BonusCondition>,
    {
        let condition = condition.into();
        Self {
            show_condition: match self.show_condition {
                Some(cond) => Some(cond & condition),
                None => Some(condition),
            },
            ..self
        }
    }

    #[must_use]
    pub fn with_show_condition_or<C>(self, condition: C) -> Self
    where
        C: Into<BonusCondition>,
    {
        let condition = condition.into();
        Self {
            show_condition: match self.show_condition {
                Some(cond) => Some(cond | condition),
                None => Some(condition),
            },
            ..self
        }
    }
}

impl ContainsAttribute for Bonus {
    fn any_attribute<'a, F>(&'a self, fun: &F) -> bool
    where
        F: Fn(&'a Attribute) -> bool,
    {
        self.value.any_attribute(fun)
            || self
                .condition()
                .as_ref()
                .is_some_and(|cond| cond.any_attribute(fun))
    }
}
