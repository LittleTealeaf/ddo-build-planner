//! Guild Attributes

use core::iter::{empty, once};

use itertools::chain;
use rust_decimal::prelude::Decimal;
use rust_decimal_macros::dec;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{BonusTemplate, BonusType, Condition, ConditionFold, ToValue, Value},
    types::{
        ability::Ability,
        absorption::{Absorption, AbsorptionSource},
        armor_class::ArmorClass,
        heal_amp::HealingAmplification,
        health::Health,
        saving_throw::SavingThrow,
        sheltering::Sheltering,
        skill::Skill,
        spell_points::SpellPoints,
        spell_selector::SpellSelector,
        summoned_attribute::SummonedAttribute,
        tactics::Tactics,
        toggle::Toggle,
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

        fn amenity_with_alternates<A, B>(
            amenity: GuildAmenity,
            alternates: A,
            bonuses: B,
        ) -> impl Iterator<Item = BonusTemplate>
        where
            A: IntoIterator<Item = GuildAmenity>,
            B: IntoIterator<Item = BonusTemplate>,
        {
            let condition = chain!(once(amenity), alternates)
                .map(Condition::toggled)
                .cond_any()
                .unwrap();

            bonuses
                .into_iter()
                .map(move |bonus: BonusTemplate| bonus.with_condition_and(condition.clone()))
                .chain(once(BonusTemplate::toggle(amenity)))
        }

        fn amenity<I>(amenity: GuildAmenity, bonuses: I) -> impl Iterator<Item = BonusTemplate>
        where
            I: IntoIterator<Item = BonusTemplate>,
        {
            amenity_with_alternates(amenity, empty(), bonuses)
        }

        if value < dec!(10) {
            return None;
        }

        let mut bonuses = Vec::new();

        bonuses.extend(chain!(
            amenity_with_alternates(
                GuildAmenity::SignOfTheSilverFlameI,
                [
                    GuildAmenity::SignOfTheSilverFlameII,
                    GuildAmenity::SignOfTheSilverFlameIII,
                    GuildAmenity::SignOfTheSilverFlameIV,
                    GuildAmenity::GrandReliquaryI,
                    GuildAmenity::GrandReliquaryII,
                    GuildAmenity::GrandReliquaryIII,
                    GuildAmenity::GrandReliquaryIV,
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
            ),
            amenity_with_alternates(
                GuildAmenity::ShrineToTheDevourerI,
                [
                    GuildAmenity::ShrineToTheDevourerII,
                    GuildAmenity::ShrineToTheDevourerIII,
                    GuildAmenity::ShrineToTheDevourerIV,
                    GuildAmenity::GrandReliquaryI,
                    GuildAmenity::GrandReliquaryII,
                    GuildAmenity::GrandReliquaryIII,
                    GuildAmenity::GrandReliquaryIV,
                ],
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
                    }),
            ),
            amenity_with_alternates(
                GuildAmenity::StormreaverMemorialI,
                [
                    GuildAmenity::StormreaverMemorialII,
                    GuildAmenity::StormreaverMemorialIII,
                    GuildAmenity::StormreaverMemorialIV,
                    GuildAmenity::GrandReliquaryI,
                    GuildAmenity::GrandReliquaryII,
                    GuildAmenity::GrandReliquaryIII,
                    GuildAmenity::GrandReliquaryIV,
                ],
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
                    }),
            ),
        ));

        // if value < dec!(11) {
        //     return Some(bonuses);
        // }

        // bonuses.push(BonusTemplate::toggle(GuildAmenity::FarshiftersChambers));

        if value < dec!(12) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::Chronoscope,
            [
                BonusTemplate::new(
                    SavingThrow::Reflex,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3)),
                ),
                BonusTemplate::new(Attribute::MovementSpeed, BonusType::Guild, val!(40)),
            ],
        ));

        if value < dec!(13) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::SellswordsTavern,
            once(BonusTemplate::new(
                SummonedAttribute::Sheltering(Sheltering::Both),
                BonusType::Guild,
                scale_with_level(val!(4), val!(8), val!(12)),
            )),
        ));

        if value < dec!(14) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::BathHouse,
            [BonusTemplate::new(
                HealingAmplification::All,
                BonusType::Guild,
                val!(20),
            )],
        ));

        if value < dec!(15) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::FloatingRockGarden,
            [
                BonusTemplate::new(Ability::Strength, BonusType::Guild, val!(2)),
                BonusTemplate::new(Ability::Wisdom, BonusType::Guild, val!(2)),
            ],
        ));

        if value < dec!(16) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::ParadoxicalPuzzleBox,
            [
                BonusTemplate::new(Ability::Dexterity, BonusType::Guild, val!(2)),
                BonusTemplate::new(Ability::Intelligence, BonusType::Guild, val!(2)),
            ],
        ));

        if value < dec!(17) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::OldSullysGrogCellar,
            [
                BonusTemplate::new(Ability::Constitution, BonusType::Guild, val!(2)),
                BonusTemplate::new(Ability::Charisma, BonusType::Guild, val!(2)),
            ],
        ));

        if value < dec!(18) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::ThroneRoom,
            [
                Skill::Bluff,
                Skill::Diplomacy,
                Skill::Haggle,
                Skill::Intimidate,
                Skill::Listen,
            ]
            .map(|skill| {
                BonusTemplate::new(
                    skill,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3)),
                )
            }),
        ));

        if value < dec!(21) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::TacticalTrainingRoom,
            once(GuildAmenity::ProvingGround),
            [
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::CriticalDamage),
                    BonusType::Guild,
                    scale_with_level(val!(2), val!(4), val!(6)),
                ),
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::Attack),
                    BonusType::Guild,
                    val!(2),
                ),
                BonusTemplate::new(Tactics::Trip, BonusType::Guild, Value::ONE),
                BonusTemplate::new(Tactics::Sunder, BonusType::Guild, Value::ONE),
                BonusTemplate::new(Tactics::SlicingBlow, BonusType::Guild, Value::ONE),
            ],
        ));

        if value < dec!(22) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::DangerRoom,
            [
                Skill::DisableDevice,
                Skill::Hide,
                Skill::OpenLock,
                Skill::Search,
                Skill::Spot,
            ]
            .map(|skill| {
                BonusTemplate::new(
                    skill,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3)),
                )
            }),
        ));

        if value < dec!(23) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::DangerRoom,
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
                    scale_with_level(val!(1), val!(2), val!(3)),
                )
            }),
        ));

        if value < dec!(24) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::ArcheryRange,
            once(GuildAmenity::ProvingGround),
            [
                // TODO: +2% doubleshot
        ],
        ));

        if value < dec!(25) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::Armory,
            once(GuildAmenity::ProvingGround),
            [
                BonusTemplate::new(
                    ArmorClass::Bonus,
                    BonusType::Guild,
                    scale_with_level(val!(2), val!(4), val!(6)),
                ),
                BonusTemplate::new(
                    Attribute::Fortification,
                    BonusType::Guild,
                    scale_with_level(val!(5), val!(10), val!(15)),
                ),
            ],
        ));

        if value < dec!(26) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::OttosIrresistableDancehall,
            [
                Skill::Balance,
                Skill::Jump,
                Skill::MoveSilently,
                Skill::Perform,
                Skill::Swim,
                Skill::Tumble,
            ]
            .map(|skill| {
                BonusTemplate::new(
                    skill,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3)),
                )
            }),
        ));

        if value < dec!(27) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::CrusadersChapel,
            once(GuildAmenity::CollegiumOfTheTwelve),
            [DamageType::Positive, DamageType::Negative].map(|dt| {
                BonusTemplate::new(
                    Attribute::spell_power(dt),
                    BonusType::Guild,
                    scale_with_level(val!(5), val!(10), val!(15)),
                )
            }),
        ));

        if value < dec!(28) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::ArcaneSanctum,
            once(GuildAmenity::CollegiumOfTheTwelve),
            [
                BonusTemplate::new(SavingThrow::Enchantment, BonusType::Guild, val!(1)),
                BonusTemplate::new(SpellPoints::Base, BonusType::Guild, val!(25)),
                BonusTemplate::new(Attribute::SpellPenetration, BonusType::Guild, val!(1)),
            ],
        ));

        if value < dec!(29) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::TrapsmithsWorkshop,
            once(GuildAmenity::CollegiumOfTheTwelve),
            [
                // TODO: +5% fortification bypass
        ],
        ));

        // TODO: wild grove
        // once(GuildAmenity::CollegiumOfTheTwelve),

        if value < dec!(32) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::GrandmastersDojo,
            once(GuildAmenity::ProvingGround),
            [
                BonusTemplate::new(SavingThrow::Will, BonusType::Guild, val!(2)),
                BonusTemplate::new(Tactics::Stun, BonusType::Guild, Value::ONE),
                // Sap and Hamstring have no DC
            ],
        ));

        if value < dec!(34) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::ProvingGround));

        if value < dec!(35) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::CollegiumOfTheTwelve));

        if value < dec!(33) {
            return Some(bonuses);
        }

        // TODO: Black Abbots Shadow

        if value < dec!(38) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::ConcertHall,
            [
                BonusTemplate::new(SavingThrow::Enchantment, BonusType::Guild, val!(1)),
                // TODO: +1 bard song
                // TODO: +1 action boost
            ],
        ));

        if value < dec!(39) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::Archwizard,
            once(BonusTemplate::new(
                Attribute::spell_dc(SpellSelector::All),
                BonusType::Guild,
                val!(1),
            )),
        ));

        if value < dec!(42) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::GameHunter,
            [
                BonusTemplate::new(
                    SavingThrow::Fortitude,
                    BonusType::Guild,
                    scale_with_level(val!(1), val!(2), val!(3)),
                ),
                // TODO: +5% helpless damage
            ],
        ));

        if value < dec!(43) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::FencingMaster,
            [
                // TODO: max dodge
                BonusTemplate::new(ArmorClass::ArmorMaxDex, BonusType::Guild, val!(1)),
            ],
        ));

        if value < dec!(44) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::NinjaAssassin,
            [
                // TODO: +0.25[W] damage
                BonusTemplate::toggle(Toggle::Flanking),
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::Attack),
                    BonusType::Guild,
                    val!(6),
                )
                .with_condition(Condition::toggled(Toggle::Flanking)),
            ],
        ));

        if value < dec!(45) {
            return Some(bonuses);
        }

        bonuses.extend(amenity(
            GuildAmenity::HagApothecary,
            [
                BonusTemplate::new(Health::Bonus, BonusType::Guild, val!(20)),
                BonusTemplate::new(SavingThrow::Poison, BonusType::Guild, val!(1)),
                BonusTemplate::new(SavingThrow::Disease, BonusType::Guild, val!(1)),
            ],
        ));

        if value < dec!(55) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::GrandReliquaryI));

        if value < dec!(65) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::SignOfTheSilverFlameII,
            [
                GuildAmenity::SignOfTheSilverFlameIII,
                GuildAmenity::SignOfTheSilverFlameIV,
                GuildAmenity::GrandReliquaryII,
                GuildAmenity::GrandReliquaryIII,
                GuildAmenity::GrandReliquaryIV,
            ],
            once(BonusTemplate::new(
                Absorption::Bonus(DamageType::Fire, AbsorptionSource::Guild),
                BonusType::Stacking,
                Value::condition(
                    Condition::toggled(GuildAmenity::SignOfTheSilverFlameIV)
                        | Condition::toggled(GuildAmenity::GrandReliquaryIV),
                    val!(0.15),
                    Value::condition(
                        Condition::toggled(GuildAmenity::SignOfTheSilverFlameIII)
                            | Condition::toggled(GuildAmenity::GrandReliquaryIII),
                        val!(0.1),
                        val!(0.05),
                    ),
                ),
            )),
        ));

        if value < dec!(70) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::ShrineToTheDevourerII,
            [
                GuildAmenity::ShrineToTheDevourerIII,
                GuildAmenity::ShrineToTheDevourerIV,
                GuildAmenity::GrandReliquaryII,
                GuildAmenity::GrandReliquaryIII,
                GuildAmenity::GrandReliquaryIV,
            ],
            [DamageType::Acid, DamageType::Cold].map(|dt| {
                BonusTemplate::new(
                    Absorption::Bonus(dt, AbsorptionSource::Guild),
                    BonusType::Stacking,
                    Value::condition(
                        Condition::toggled(GuildAmenity::ShrineToTheDevourerIV)
                            | Condition::toggled(GuildAmenity::GrandReliquaryIV),
                        val!(0.15),
                        Value::condition(
                            Condition::toggled(GuildAmenity::ShrineToTheDevourerIII)
                                | Condition::toggled(GuildAmenity::GrandReliquaryIII),
                            val!(0.1),
                            val!(0.05),
                        ),
                    ),
                )
            }),
        ));

        if value < dec!(80) {
            return Some(bonuses);
        }

        bonuses.extend(amenity_with_alternates(
            GuildAmenity::StormreaverMemorialII,
            [
                GuildAmenity::StormreaverMemorialIII,
                GuildAmenity::StormreaverMemorialIV,
                GuildAmenity::GrandReliquaryII,
                GuildAmenity::GrandReliquaryIII,
                GuildAmenity::GrandReliquaryIV,
            ],
            [DamageType::Sonic, DamageType::Electric].map(|dt| {
                BonusTemplate::new(
                    Absorption::Bonus(dt, AbsorptionSource::Guild),
                    BonusType::Stacking,
                    Value::condition(
                        Condition::toggled(GuildAmenity::StormreaverMemorialIV)
                            | Condition::toggled(GuildAmenity::GrandReliquaryIV),
                        val!(0.15),
                        Value::condition(
                            Condition::toggled(GuildAmenity::StormreaverMemorialIII)
                                | Condition::toggled(GuildAmenity::GrandReliquaryIII),
                            val!(0.10),
                            val!(0.05),
                        ),
                    ),
                )
            }),
        ));

        if value < dec!(85) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::GrandReliquaryII));

        if value < dec!(90) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::SignOfTheSilverFlameIII));

        if value < dec!(95) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::ShrineToTheDevourerIII));

        if value < dec!(110) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::StormreaverMemorialIII));

        if value < dec!(120) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::GrandReliquaryIII));

        if value < dec!(125) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::SignOfTheSilverFlameIV));

        if value < dec!(130) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::ShrineToTheDevourerIV));

        if value < dec!(140) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::StormreaverMemorialIV));

        if value < dec!(150) {
            return Some(bonuses);
        }

        bonuses.push(BonusTemplate::toggle(GuildAmenity::GrandReliquaryIV));

        Some(bonuses)
    }
}
