//! Guild Attributes

use core::iter::once;

use itertools::chain;
use rust_decimal::prelude::Decimal;
use rust_decimal_macros::dec;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{BonusTemplate, BonusType, Condition, ConditionFold, ToValue, Value},
    types::{
        ability::Ability,
        heal_amp::HealingAmplification,
        saving_throw::SavingThrow,
        skill::Skill,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
    val,
};

use super::{damage_type::DamageType, toggle::GuildAmenity};

/// Represents the current guild's level
pub struct GuildLevel;

impl GetBonuses for GuildLevel {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        fn scale_with_level<A, B, C>(a: A, b: B, c: C) -> Value
        where
            A: Into<Value>,
            B: Into<Value>,
            C: Into<Value>,
        {
            Value::condition(
                Attribute::TotalCharacterLevel
                    .to_value()
                    .greater_than(val!(20)),
                c,
                Value::condition(
                    Attribute::TotalCharacterLevel
                        .to_value()
                        .greater_than(val!(10)),
                    b,
                    a,
                ),
            )
        }

        fn amenity<I>(amenity: GuildAmenity, bonuses: I) -> impl Iterator<Item = BonusTemplate>
        where
            I: IntoIterator<Item = BonusTemplate>,
        {
            chain!(
                once(BonusTemplate::toggle(amenity)),
                bonuses
                    .into_iter()
                    .map(move |bonus| bonus.with_condition_and(Condition::toggled(amenity)))
            )
        }

        fn amenity_with_alternates<A, I>(
            amenity: GuildAmenity,
            alternates: A,
            bonuses: I,
        ) -> impl Iterator<Item = BonusTemplate>
        where
            A: IntoIterator<Item = GuildAmenity>,
            I: IntoIterator<Item = BonusTemplate>,
        {
            let condition = chain!(once(amenity), alternates)
                .map(Condition::toggled)
                .cond_any()
                .unwrap();
            chain!(
                once(BonusTemplate::toggle(amenity)),
                bonuses
                    .into_iter()
                    .map(move |bonus| bonus.with_condition_and(condition.clone()))
            )
        }

        if value < dec!(10) {
            return None;
        }

        let mut bonuses = Vec::new();

        bonuses.extend(chain!(
            [
                BonusTemplate::toggle(GuildAmenity::SignOfTheSilverFlameI),
                BonusTemplate::toggle(GuildAmenity::ShrineToTheDevourerI),
                BonusTemplate::toggle(GuildAmenity::StormreaverMemorialI),
                BonusTemplate::toggle(GuildAmenity::ShrineOfExperienceI),
                BonusTemplate::toggle(GuildAmenity::TheOrienExpress),
                BonusTemplate::toggle(GuildAmenity::ThreeFingerThads),
            ],
            [
                BonusTemplate::new(
                    Attribute::Resistance(DamageType::Fire),
                    BonusType::Guild,
                    scale_with_level(val!(5), val!(10), val!(15)),
                ),
                BonusTemplate::new(
                    Attribute::spell_power(DamageType::Fire),
                    BonusType::Guild,
                    scale_with_level(val!(5), val!(10), val!(15)),
                ),
                BonusTemplate::new(
                    Attribute::spell_power(DamageType::Light),
                    BonusType::Guild,
                    scale_with_level(val!(5), val!(10), val!(15)),
                ),
                BonusTemplate::new(
                    Attribute::spell_power(DamageType::Alignment),
                    BonusType::Guild,
                    scale_with_level(val!(5), val!(10), val!(15)),
                ),
            ]
            .into_iter()
            .map(|bonus| {
                bonus.with_condition(
                    [
                        GuildAmenity::SignOfTheSilverFlameI,
                        GuildAmenity::SignOfTheSilverFlameII,
                        GuildAmenity::SignOfTheSilverFlameIII,
                        GuildAmenity::SignOfTheSilverFlameIV,
                        GuildAmenity::GrandReliquaryI,
                        GuildAmenity::GrandReliquaryII,
                        GuildAmenity::GrandReliquaryIII,
                        GuildAmenity::GrandReliquaryIV,
                    ]
                    .map(Condition::toggled)
                    .cond_any(),
                )
            }),
            [DamageType::Acid, DamageType::Cold]
                .into_iter()
                .flat_map(|dt| {
                    [
                        BonusTemplate::new(
                            Attribute::spell_power(dt),
                            BonusType::Guild,
                            scale_with_level(val!(5), val!(10), val!(15)),
                        ),
                        BonusTemplate::new(
                            Attribute::Resistance(dt),
                            BonusType::Guild,
                            scale_with_level(val!(5), val!(10), val!(15)),
                        ),
                    ]
                })
                .map(|bonus| {
                    bonus.with_condition(
                        [
                            GuildAmenity::ShrineToTheDevourerI,
                            GuildAmenity::ShrineToTheDevourerII,
                            GuildAmenity::ShrineToTheDevourerIII,
                            GuildAmenity::ShrineToTheDevourerIV,
                            GuildAmenity::GrandReliquaryI,
                            GuildAmenity::GrandReliquaryII,
                            GuildAmenity::GrandReliquaryIII,
                            GuildAmenity::GrandReliquaryIV,
                        ]
                        .map(Condition::toggled)
                        .cond_any(),
                    )
                }),
            [DamageType::Sonic, DamageType::Electric]
                .into_iter()
                .flat_map(|dt| {
                    [
                        BonusTemplate::new(
                            Attribute::spell_power(dt),
                            BonusType::Guild,
                            scale_with_level(val!(5), val!(10), val!(15)),
                        ),
                        BonusTemplate::new(
                            Attribute::Resistance(dt),
                            BonusType::Guild,
                            scale_with_level(val!(5), val!(10), val!(15)),
                        ),
                    ]
                })
                .map(|bonus| {
                    bonus.with_condition(
                        [
                            GuildAmenity::StormreaverMemorialI,
                            GuildAmenity::StormreaverMemorialII,
                            GuildAmenity::StormreaverMemorialIII,
                            GuildAmenity::StormreaverMemorialIV,
                            GuildAmenity::GrandReliquaryI,
                            GuildAmenity::GrandReliquaryII,
                            GuildAmenity::GrandReliquaryIII,
                            GuildAmenity::GrandReliquaryIV,
                        ]
                        .map(Condition::toggled)
                        .cond_any(),
                    )
                })
        ));

        if value < dec!(11) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::FarshiftersChambers));

        if value < dec!(12) {
            return Some(bonuses);
        }

        bonuses.extend(chain!(
            [BonusTemplate::toggle(GuildAmenity::Chronoscope)],
            [
                BonusTemplate::new(
                    SavingThrow::Reflex,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3))
                ),
                BonusTemplate::new(Attribute::MovementSpeed, BonusType::Guild, val!(40)),
            ]
            .map(|bonus| bonus.with_condition(Condition::toggled(GuildAmenity::Chronoscope)))
        ));

        if value < dec!(13) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::SellswordsTavern));
        // TODO: +4/8/12 mrr/prr of hires

        if value < dec!(14) {
            return Some(bonuses);
        }

        bonuses.extend(chain!(
            once(BonusTemplate::toggle(GuildAmenity::BathHouse)),
            [
                BonusTemplate::new(HealingAmplification::All, BonusType::Guild, val!(20)),
                // TODO: uncon range +5/10/15
                // TODO: -10% damage helpless
            ]
            .map(|bonus| bonus.with_condition(Condition::toggled(GuildAmenity::BathHouse)))
        ));

        if value < dec!(15) {
            return Some(bonuses);
        }

        bonuses.extend(chain!(
            once(BonusTemplate::toggle(GuildAmenity::FloatingRockGarden)),
            [
                BonusTemplate::new(Ability::Strength, BonusType::Guild, val!(2)),
                BonusTemplate::new(Ability::Wisdom, BonusType::Guild, val!(2)),
            ]
            .map(|bonus| bonus.with_condition(Condition::toggled(GuildAmenity::BathHouse)))
        ));

        if value < dec!(16) {
            return Some(bonuses);
        }

        bonuses.extend(chain!(
            once(BonusTemplate::toggle(GuildAmenity::ParadoxicalPuzzleBox)),
            [
                BonusTemplate::new(Ability::Dexterity, BonusType::Guild, val!(2)),
                BonusTemplate::new(Ability::Intelligence, BonusType::Guild, val!(2)),
            ]
            .map(|bonus| bonus
                .with_condition(Condition::toggled(GuildAmenity::ParadoxicalPuzzleBox)))
        ));

        if value < dec!(17) {
            return Some(bonuses);
        }

        bonuses.extend(chain!(
            once(BonusTemplate::toggle(GuildAmenity::OldSullysGrogCellar)),
            [
                BonusTemplate::new(Ability::Constitution, BonusType::Guild, val!(2)),
                BonusTemplate::new(Ability::Charisma, BonusType::Guild, val!(2)),
            ]
            .map(
                |bonus| bonus.with_condition(Condition::toggled(GuildAmenity::OldSullysGrogCellar))
            )
        ));

        if value < dec!(18) {
            return Some(bonuses);
        }

        bonuses.extend(chain!(
            once(BonusTemplate::toggle(GuildAmenity::ThroneRoom)),
            [
                Skill::Bluff,
                Skill::Diplomacy,
                Skill::Haggle,
                Skill::Intimidate,
                Skill::Listen
            ]
            .map(|skill| {
                BonusTemplate::new(
                    skill,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3)),
                )
                .with_condition(Condition::toggled(GuildAmenity::ThroneRoom))
            })
        ));

        // if value == dec!(18) {
        //     return Some(bonuses);
        // }
        //
        // // bonuses.push(BonusTemplate::toggle(GuildAmenity::GuildStorageI));
        //
        // if value == dec!(19) {
        //     return Some(bonuses);
        // }

        if value < dec!(21) {
            return Some(bonuses);
        }

        bonuses.extend(chain!(
            once(BonusTemplate::toggle(GuildAmenity::TacticalTrainingRoom)),
            [
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::CriticalDamage),
                    BonusType::Guild,
                    scale_with_level(val!(2), val!(4), val!(6))
                ),
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::Attack),
                    BonusType::Guild,
                    val!(2)
                ),
                // TODO: +1 DCs of Trip, Sunder, Slicing Blow
            ]
            .map(|bonus| bonus
                .with_condition(Condition::toggled(GuildAmenity::TacticalTrainingRoom)))
        ));

        if value < dec!(22) {
            return Some(bonuses);
        }

        bonuses.extend(chain!(
            once(BonusTemplate::toggle(GuildAmenity::DangerRoom)),
            [
                Skill::DisableDevice,
                Skill::Hide,
                Skill::OpenLock,
                Skill::Search,
                Skill::Spot
            ]
            .map(|skill| {
                BonusTemplate::new(
                    skill,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3)),
                )
                .with_condition(Condition::toggled(GuildAmenity::DangerRoom))
            })
        ));

        if value < dec!(23) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::ForbiddenLibrary,
            [
                Skill::Concentration,
                Skill::Heal,
                Skill::Repair,
                Skill::Spellcraft,
                Skill::UseMagicalDevice,
            ]
            .map(|skill| {
                BonusTemplate::new(
                    skill,
                    BonusType::Guild,
                    scale_with_level(val![1], val![2], val![3]),
                )
            }),
        ));

        Some(bonuses)
    }
}

//
// use core::fmt;
//
// use serde::{Deserialize, Serialize};
// use utils::public_modules;
//
// use crate::attribute::{Attribute, ToAttribute};
//
// public_modules!(amenities);
//
// /// Guild-focused attributes
// #[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
// pub enum Guild {
//     /// Guild Level
//     Level,
//     /// Guild Amenities
//     Amenity(GuildAmenityOld),
// }
//
// impl ToAttribute for Guild {
//     fn to_attribute(self) -> Attribute {
//         Attribute::Guild(self)
//     }
// }
//
// impl fmt::Display for Guild {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Level => write!(f, "Guild Level"),
//             Self::Amenity(a) => write!(f, "Guild Amenity: {a}"),
//         }
//     }
// }
