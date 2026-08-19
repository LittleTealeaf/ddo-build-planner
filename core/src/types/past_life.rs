use itertools::chain;

use crate::{
    bonus::{
        traits::{ToAttribute, ToValue},
        Bonus, BonusSource, BonusType,
    },
    traits::IterValues,
    types::player_race::PlayerRace,
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
    derive_more::From,
    derive_more::Display,
)]
pub enum PastLife {
    Racial(PlayerRace),
}

impl PastLife {
    pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
        chain!(PlayerRace::values().filter_map(|race| {
            race.get_parent_race().map(|parent| {
                Bonus::new(
                    parent.past_life(),
                    race.past_life().attribute().to_value(),
                    BonusType::Stacking,
                    BonusSource::Attribute(race.past_life().attribute()),
                )
            })
        }))
    }
}

impl IterValues for PastLife {
    fn values() -> impl Iterator<Item = Self> {
        chain!(PlayerRace::values().map(Into::into),)
    }
}
