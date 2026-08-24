use core::iter::once;

use itertools::{chain, Itertools};

use crate::{
    attribute::Attribute,
    bonus::{
        traits::{ToAttribute, ToValue},
        Bonus, BonusCondition, BonusSource, BonusType, BonusValue,
    },
    traits::IterValues,
    types::{
        armor_class::ArmorClass,
        damage_type::DamageType,
        epic_past_life::{EpicPastLife, EpicSphere},
        health::PlayerHealth,
        level::PlayerLevel,
        player_class::PlayerClass,
        player_race::PlayerRace,
        sheltering::Sheltering,
        skill::Skill,
    },
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
    Epic(EpicPastLife),
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
            // Racial Completionist Skills
            {
                PlayerRace::values().filter_map(|race| {
                    race.past_life_skill().map(|skill| {
                        Bonus::new(
                            skill.attribute(),
                            val!(1),
                            BonusType::Stacking,
                            BonusSource::Attribute(race.past_life().attribute()),
                        )
                        .with_show_condition(
                            race.past_life()
                                .attribute()
                                .to_value()
                                .greater_or_equal_to(val!(1)),
                        )
                    })
                })
            },
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
            },
            // ---- Epic Past Lives:
            // Arcane
            {
                fn to_value(epls: impl Iterator<Item = EpicPastLife>) -> BonusValue {
                    BonusValue::iter_sum(
                        epls.map(|epl| epl.past_life().attribute().to_value().min(val!(3))),
                    )
                    .unwrap_or(val!(0))
                }

                let value = to_value(EpicSphere::Arcane.past_lives());
                chain!(
                    // Arcane
                    [
                        DamageType::Acid,
                        DamageType::Cold,
                        DamageType::Electric,
                        DamageType::Fire,
                    ]
                    .map(move |damage_type| {
                        Bonus::new(
                            Attribute::Absorption(damage_type),
                            value.clone(),
                            BonusType::Stacking,
                            BonusSource::Custom("Arcane Past Lives".to_owned()),
                        )
                        .with_show_condition(value.clone().is_some())
                    }),
                    [
                        // Divine
                        {
                            let value = to_value(EpicSphere::Divine.past_lives());
                            Bonus::new(
                                Sheltering::Physical.attribute(),
                                value.clone() * val!(3),
                                BonusType::Stacking,
                                BonusSource::Custom("Divine Past Lives".to_owned()),
                            )
                            .with_show_condition(value.is_some())
                        },
                        // Primal
                        {
                            let value = to_value(EpicSphere::Primal.past_lives());
                            Bonus::new(
                                PlayerHealth::Bonus.attribute(),
                                (value.clone() * val!(3))
                                    + (value.clone()
                                        * val!(4)
                                        * (PlayerLevel::Total.attribute().to_value() / val!(10))
                                            .floor()),
                                BonusType::Stacking,
                                BonusSource::Custom("Primal Past Lives".to_owned()),
                            )
                            .with_show_condition(value.is_some())
                        },
                        // Martial
                        {
                            let value = to_value(EpicSphere::Martial.past_lives());

                            Bonus::new(
                                ArmorClass::Bonus.attribute(),
                                (value.clone() * val!(2))
                                    + (value.clone()
                                        * (PlayerLevel::Total.attribute().to_value() / val!(10))
                                            .floor()),
                                BonusType::Stacking,
                                BonusSource::Custom("Martial Past Lives".to_owned()),
                            )
                            .with_show_condition(value.is_some())
                        }
                    ]
                )
            },
        )
    }
}

impl IterValues for PastLife {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            PlayerRace::values().map(Into::into),
            PlayerClass::values().map(Into::into),
            EpicPastLife::values().map(Into::into),
        )
    }
}
