use core::{
    fmt::{self, Display},
    iter::once,
};

use itertools::chain;
use rust_decimal::prelude::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{BonusTemplate, BonusType, Condition, ToValue, Value},
    feat::{Feat, ToFeat},
    types::{
        absorption::{Absorption, AbsorptionSource},
        armor_class::ArmorClass,
        damage_type::DamageType,
        epic_sphere::EpicSphere,
        flag::Flag,
        heal_amp::HealingAmplification,
        health::Health,
        saving_throw::SavingThrow,
        sheltering::Sheltering,
        skill::Skill,
        tactics::Tactics,
        toggle::{GetToggleGroup, ToToggle, Toggle},
        toggle_group::ToggleGroup,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
    val,
};

use super::PastLifeFeat;

/// Epic Past Life
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EpicPastLife {
    /// Energy Criticals
    EnergyCriticals,
    /// Enchant Weapon
    EnchantWeapon,
    /// Arcane Alacrity
    ArcaneAlacrity,
    /// Ancient Knowledge
    AncientKnowledge,
    /// Eclipse Power
    EclipsePower,
    /// Power over Life and Death
    PowerOverLifeAndDeath,
    /// Brace
    Brace,
    /// Block Energy
    BlockEnergy,
    /// Ancient Blessings
    AncientBlessngs,
    /// Doublestrike
    Doublestrike,
    /// Skill Mastery
    SkillMastery,
    /// Fortification
    Fortification,
    /// Ancient Tactics
    AncientTactics,
    /// Trap Damage Absorption
    TrapDamageAbsorption,
    /// Doubleshot
    Doubleshot,
    /// Fast Healing
    FastHealing,
    /// Colors of the Queen
    ColorsOfTheQueen,
    /// Ancient Power
    AncientPower,
}

impl EpicPastLife {
    /// All epic past lives
    pub const ALL: [Self; 18] = [
        Self::EnergyCriticals,
        Self::EnchantWeapon,
        Self::ArcaneAlacrity,
        Self::AncientKnowledge,
        Self::EclipsePower,
        Self::PowerOverLifeAndDeath,
        Self::Brace,
        Self::BlockEnergy,
        Self::AncientBlessngs,
        Self::Doublestrike,
        Self::SkillMastery,
        Self::Fortification,
        Self::AncientTactics,
        Self::TrapDamageAbsorption,
        Self::Doubleshot,
        Self::FastHealing,
        Self::ColorsOfTheQueen,
        Self::AncientPower,
    ];

    /// Returns the sphere that the past life belongs to
    #[must_use]
    pub const fn get_sphere(self) -> EpicSphere {
        match self {
            Self::EnergyCriticals
            | Self::EnchantWeapon
            | Self::ArcaneAlacrity
            | Self::AncientKnowledge
            | Self::EclipsePower => EpicSphere::Arcane,
            Self::PowerOverLifeAndDeath
            | Self::Brace
            | Self::BlockEnergy
            | Self::AncientBlessngs => EpicSphere::Divine,
            Self::Doublestrike
            | Self::SkillMastery
            | Self::Fortification
            | Self::AncientTactics
            | Self::TrapDamageAbsorption => EpicSphere::Martial,
            Self::Doubleshot | Self::FastHealing | Self::ColorsOfTheQueen | Self::AncientPower => {
                EpicSphere::Primal
            }
        }
    }
}

impl Display for EpicPastLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnergyCriticals => write!(f, "Energy Criticals"),
            Self::EnchantWeapon => write!(f, "Encahnt Weapon"),
            Self::ArcaneAlacrity => write!(f, "Arcane Alacrity"),
            Self::AncientKnowledge => write!(f, "Ancient Knowledge"),
            Self::EclipsePower => write!(f, "Eclipse Power"),
            Self::PowerOverLifeAndDeath => write!(f, "Power over Life and Death"),
            Self::Brace => write!(f, "Brace"),
            Self::BlockEnergy => write!(f, "Block Energy"),
            Self::AncientBlessngs => write!(f, "Ancient Blessings"),
            Self::Doublestrike => write!(f, "Doublestrike"),
            Self::SkillMastery => write!(f, "Skill Mastery"),
            Self::Fortification => write!(f, "Fortification"),
            Self::AncientTactics => write!(f, "Ancient Tactics"),
            Self::TrapDamageAbsorption => write!(f, "Trap Damage Absorption"),
            Self::Doubleshot => write!(f, "Doubleshot"),
            Self::FastHealing => write!(f, "Fast Healing"),
            Self::ColorsOfTheQueen => write!(f, "Colors of the Queen"),
            Self::AncientPower => write!(f, "Ancient Power"),
        }
    }
}

impl GetBonuses for EpicPastLife {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        if value <= Decimal::ZERO {
            return None;
        }

        let value = value.min(dec!(3));

        let sphere_bonuses = match self.get_sphere() {
            EpicSphere::Arcane => [
                DamageType::Acid,
                DamageType::Cold,
                DamageType::Electric,
                DamageType::Fire,
            ]
            .map(|damage| {
                BonusTemplate::new(
                    Absorption::Bonus(damage, AbsorptionSource::ArcanePastLife),
                    BonusType::Stacking,
                    value,
                )
            })
            .to_vec(),
            EpicSphere::Primal => vec![BonusTemplate::new(
                Health::Bonus,
                BonusType::Stacking,
                value.to_value()
                    * (val!(3)
                        + (val!(4)
                            * ((Attribute::TotalCharacterLevel.to_value()
                                - (Attribute::TotalCharacterLevel.to_value() % Value::TEN))
                                / Value::TEN))),
            )],
            EpicSphere::Divine => vec![BonusTemplate::new(
                Sheltering::Physical,
                BonusType::Stacking,
                value * dec!(3),
            )],
            EpicSphere::Martial => vec![BonusTemplate::new(
                ArmorClass::Bonus,
                BonusType::Stacking,
                value.to_value()
                    * (Value::TWO
                        + ((Attribute::TotalCharacterLevel.to_value()
                            - (Attribute::TotalCharacterLevel.to_value() % Value::TEN))
                            / Value::TEN)),
            )],
        };

        let toggle = BonusTemplate::toggle(*self);

        let toggle_bonuses = match self {
            Self::EnergyCriticals => [
                DamageType::Acid,
                DamageType::Cold,
                DamageType::Electric,
                DamageType::Fire,
                DamageType::Sonic,
            ]
            .map(|damage| {
                BonusTemplate::new(
                    Attribute::SpellCriticalChance(damage.into()),
                    BonusType::Stacking,
                    dec!(3) * value,
                )
            })
            .to_vec(),
            Self::EnchantWeapon => vec![
                BonusTemplate::new(Attribute::Debug(0), BonusType::Stacking, 0),
                // TODO: Weapon Enchantment
            ],
            Self::ArcaneAlacrity => vec![
                BonusTemplate::new(Attribute::Debug(1), BonusType::Stacking, 0),
                // TODO: Arcane Alacrity
            ],
            Self::AncientKnowledge => {
                vec![BonusTemplate::new(
                    Sheltering::Magical,
                    BonusType::Stacking,
                    dec!(3) * value,
                )]
            }
            Self::EclipsePower => {
                vec![BonusTemplate::new(
                    Attribute::SpellPenetration,
                    BonusType::Stacking,
                    value,
                )]
            }
            Self::PowerOverLifeAndDeath => [DamageType::Positive, DamageType::Negative]
                .map(|damage| {
                    BonusTemplate::new(
                        Attribute::SpellPower(damage.into()),
                        BonusType::Stacking,
                        dec!(10) * value,
                    )
                })
                .to_vec(),
            Self::Brace => vec![BonusTemplate::new(
                SavingThrow::All,
                BonusType::Stacking,
                value,
            )],
            Self::BlockEnergy => vec![
                BonusTemplate::new(Attribute::Debug(2), BonusType::Stacking, 0),
                // TODO: Block Energy
            ],
            Self::AncientBlessngs => vec![BonusTemplate::new(
                HealingAmplification::All,
                BonusType::Stacking,
                dec!(5) * value,
            )
            .with_condition(Condition::toggled(*self))],
            Self::Doublestrike => vec![BonusTemplate::new(
                Attribute::Doublestrike,
                BonusType::Stacking,
                dec!(3) * value,
            )],
            Self::SkillMastery => vec![BonusTemplate::new(Skill::All, BonusType::Stacking, value)],
            Self::Fortification => vec![BonusTemplate::new(
                Attribute::Fortification,
                BonusType::Stacking,
                value * Decimal::TEN,
            )],
            Self::AncientTactics => vec![
                BonusTemplate::new(Tactics::Tactics, BonusType::Stacking, dec!(2) * value),
                BonusTemplate::new(Tactics::Assassinate, BonusType::Stacking, value),
            ],
            Self::TrapDamageAbsorption => vec![
                BonusTemplate::new(Attribute::Debug(6), BonusType::Stacking, 0),
                // TODO: Trap Absorption
            ],
            Self::Doubleshot => vec![BonusTemplate::new(
                Attribute::Doubleshot,
                BonusType::Stacking,
                dec!(3) * value,
            )],
            Self::FastHealing => vec![
                BonusTemplate::new(Attribute::Debug(8), BonusType::Stacking, 0),
                // TODO: Fast Healing
            ],
            Self::ColorsOfTheQueen => vec![
                BonusTemplate::new(Attribute::Debug(9), BonusType::Stacking, 0),
                // TODO: Colors of the Queen
            ],
            Self::AncientPower => vec![
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::Attack),
                    BonusType::Stacking,
                    Decimal::TWO * value,
                ),
                BonusTemplate::new(
                    (WeaponHand::Main, WeaponStat::Damage),
                    BonusType::Stacking,
                    Decimal::TWO * value,
                )
                .with_condition(Condition::flag(Flag::TwoHandedFighting)),
            ],
        }
        .into_iter()
        .map(|bonus| bonus.with_condition_and(Condition::toggled(*self)));

        Some(chain!(sphere_bonuses, once(toggle), toggle_bonuses).collect())
    }
}

impl ToToggle for EpicPastLife {
    fn to_toggle(self) -> Toggle {
        Toggle::EpicPastLife(self)
    }
}

impl ToFeat for EpicPastLife {
    fn to_feat(self) -> Feat {
        PastLifeFeat::Epic(self).to_feat()
    }
}

impl GetToggleGroup for EpicPastLife {
    fn custom_toggle_group(&self) -> Option<ToggleGroup> {
        Some(ToggleGroup::EpicPastLife(self.get_sphere()))
    }
}

impl StaticValues for EpicPastLife {
    fn values() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}
