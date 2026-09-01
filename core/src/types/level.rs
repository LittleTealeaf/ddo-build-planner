use itertools::chain;

use crate::{
    bonus::{
        traits::{ToAttribute, ToValue},
        Bonus, BonusSource, BonusType, BonusValue,
    },
    traits::IterValues,
    types::player_class::PlayerClass,
    val,
};

#[derive(
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::From,
)]
pub enum PlayerLevel {
    Heroic(PlayerClass),
    Epic,
    Legendary,
    Total,
}

impl PlayerLevel {
    pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
        [
            Bonus::new(
                Self::Total,
                BonusValue::iter_sum(
                    PlayerClass::values().map(|cls| Self::Heroic(cls).attribute().to_value()),
                )
                .unwrap_or(val!(0))
                .min(val!(20)),
                BonusType::Stacking,
                BonusSource::Custom("Heroic Levels".to_owned()),
            ),
            Bonus::new(
                Self::Total,
                Self::Epic.attribute().to_value(),
                BonusType::Stacking,
                BonusSource::Attribute(Self::Epic.attribute()),
            ),
            Bonus::new(
                Self::Total,
                Self::Legendary.attribute().to_value(),
                BonusType::Stacking,
                BonusSource::Attribute(Self::Legendary.attribute()),
            ),
        ]
        .into_iter()
    }
}

impl IterValues for PlayerLevel {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            PlayerClass::values().map(Into::into),
            [Self::Epic, Self::Legendary, Self::Total]
        )
    }
}
