use itertools::{chain, Itertools};

use crate::{
    bonus::{
        traits::{ToAttribute, ToValue},
        Bonus, BonusCondition, BonusSource, BonusType, BonusValue,
    },
    traits::IterValues,
    types::{player_class::PlayerClass, player_race::PlayerRace, skill::Skill},
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
    derive_more::From,
    derive_more::Display,
)]
pub enum PastLife {
    Racial(PlayerRace),
    Class(PlayerClass),
}

impl PastLife {
    pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
        chain!(
            // Racial Parent Lifes
            PlayerRace::values().filter_map(|race| {
                race.get_parent_race().map(|parent| {
                    Bonus::new(
                        parent.past_life(),
                        race.past_life().attribute().to_value(),
                        BonusType::Stacking,
                        BonusSource::Attribute(race.past_life().attribute()),
                    )
                })
            }),
            // Racial Completionist
            {
                let condition = BonusCondition::all(
                    PlayerRace::values()
                        .filter(|race| race.get_parent_race().is_none() && !race.is_iconic())
                        .map(|race| {
                            race.past_life()
                                .attribute()
                                .to_value()
                                .greater_or_equal_to(val!(3))
                        }),
                );

                Skill::values().map(move |skill| {
                    Bonus::new(
                        skill.attribute(),
                        val![2],
                        BonusType::Stacking,
                        BonusSource::Custom("Racial Completionist".to_owned()),
                    )
                    .with_show_condition(condition.clone())
                })
            },
            // Heroic Completionist
            {
                let condition = BonusCondition::all(
                    PlayerClass::values()
                        .into_grouping_map_by(|cls| cls.parent_class().unwrap_or(*cls))
                        .aggregate(|a, _, c| {
                            a.map_or_else(
                                || Some(c.past_life().attribute().to_value()),
                                |prev| Some(prev + c.past_life().attribute().to_value()),
                            )
                        })
                        .into_values()
                        .map(BonusValue::is_some),
                );

                Skill::values().map(move |skill| {
                    Bonus::new(
                        skill.attribute(),
                        val![2],
                        BonusType::Stacking,
                        BonusSource::Custom("Heroic Completionist".to_owned()),
                    )
                    .with_show_condition(condition.clone())
                })
            }
        )
    }
}

impl IterValues for PastLife {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            PlayerRace::values().map(Into::into),
            PlayerClass::values().map(Into::into)
        )
    }
}
