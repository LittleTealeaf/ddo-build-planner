//! Guild Attributes

use std::iter::once;

use itertools::chain;
use rust_decimal::prelude::Decimal;
use rust_decimal_macros::dec;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{BonusTemplate, BonusType, Condition, ConditionFold, ToValue, Value},
    types::{heal_amp::HealingAmplification, saving_throw::SavingThrow},
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

        if value < dec!(10) {
            return None;
        }

        let mut bonuses = Vec::new();

        bonuses.extend(chain!(
            [
                BonusTemplate::toggle(GuildAmenity::SignOfTheSilverFlameI),
                BonusTemplate::toggle(GuildAmenity::ShrineToTheDevourerII),
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

        if value == dec!(10) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::FarshiftersChambers));

        if value == dec!(11) {
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

        if value == dec!(12) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::SellswordsTavern));
        // TODO: +4/8/12 mrr/prr of hires

        if value == dec!(13) {
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

        if value == dec!(14) {
            return Some(bonuses);
        }

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
