use getset::{Getters, Setters, WithSetters};
use serde::{Deserialize, Serialize};

use crate::{
    bonus::{Bonus, BonusCondition, BonusSource, BonusType, BonusValue},
    property::Property,
};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Getters, Setters, WithSetters,
)]
#[getset(set = "pub", get = "pub")]
pub struct Effect {
    #[getset(set_with = "pub")]
    property: Property,
    #[getset(set_with = "pub")]
    value: BonusValue,
    #[getset(set_with = "pub")]
    r#type: BonusType,
    condition: Option<BonusCondition>,
    show_condition: Option<BonusCondition>,
    #[getset(set_with = "pub")]
    source: BonusSource,
}

impl Effect {
    pub fn new<A, V, T, S>(stat: A, value: V, bonus_type: T, source: S) -> Self
    where
        A: Into<Property>,
        V: Into<BonusValue>,
        T: Into<BonusType>,
        S: Into<BonusSource>,
    {
        Self {
            property: stat.into(),
            value: value.into(),
            r#type: bonus_type.into(),
            condition: None,
            source: source.into(),
            show_condition: None
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
}

impl Effect {
    pub fn into_bonuses(self) -> impl Iterator<Item = Bonus> {
        self.property
            .into_attributes()
            .into_iter()
            .map(move |attribute| {
                Bonus::new(
                    attribute,
                    self.value.clone(),
                    self.r#type,
                    self.source.clone(),
                )
                .with_condition_maybe(self.condition.clone())
            })
    }
}
