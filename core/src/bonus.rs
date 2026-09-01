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
    condition: BonusCondition,

    #[getset(get = "pub")]
    show_condition: BonusCondition,

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
        if !self.condition.is_true() {
            write!(f, " if {}", self.condition)?;
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
            condition: BonusCondition::TRUE,
            show_condition: BonusCondition::TRUE,
            source: source.into(),
        }
    }

    #[must_use]
    pub fn with_condition_maybe(self, condition: Option<BonusCondition>) -> Self {
        self.with_condition(condition.unwrap_or_default())
    }

    #[must_use]
    pub fn with_condition_and(self, condition: BonusCondition) -> Self {
        Self {
            condition: self.condition.and(condition),
            ..self
        }
    }

    #[must_use]
    pub fn with_condition_or(self, condition: BonusCondition) -> Self {
        Self {
            condition: self.condition.or(condition),
            ..self
        }
    }

    #[must_use]
    pub fn with_show_condition_and(self, condition: BonusCondition) -> Self {
        Self {
            show_condition: self.show_condition.and(condition),
            ..self
        }
    }

    #[must_use]
    pub fn with_show_condition_or(self, condition: BonusCondition) -> Self {
        Self {
            show_condition: self.show_condition.or(condition),
            ..self
        }
    }
}

impl ContainsAttribute for Bonus {
    fn any_attribute<'a, F>(&'a self, fun: &F) -> bool
    where
        F: Fn(&'a Attribute) -> bool,
    {
        self.value.any_attribute(fun) || self.condition.any_attribute(fun)
    }
}
