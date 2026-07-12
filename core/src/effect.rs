use crate::{
    bonus::{Bonus, BonusCondition, BonusSource, BonusType, BonusValue},
    stat::Stat,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Effect {
    stat: Stat,
    value: BonusValue,
    r#type: BonusType,
    condition: Option<BonusCondition>,
    source: BonusSource,
}

impl Effect {
    pub fn new<A, V, T, S>(stat: A, value: V, bonus_type: T, source: S) -> Self
    where
        A: Into<Stat>,
        V: Into<BonusValue>,
        T: Into<BonusType>,
        S: Into<BonusSource>,
    {
        Self {
            stat: stat.into(),
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
        self.stat
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
