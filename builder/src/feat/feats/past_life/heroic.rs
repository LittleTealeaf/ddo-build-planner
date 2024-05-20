use core::fmt::{self, Display};

use rust_decimal::prelude::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{BonusTemplate, BonusType, Condition, ConditionFold},
    feat::{Feat, ToFeat},
    types::{
        damage_type::DamageType,
        flag::MainHandType,
        heal_amp::HealingAmplification,
        health::Health,
        item_type::WeaponType,
        player_class::PlayerClass,
        saving_throw::SavingThrow,
        sheltering::Sheltering,
        skill::Skill,
        sneak_attack::SneakAttack,
        spell_points::SpellPoints,
        spell_school::SpellSchool,
        tactics::Tactics,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
};

use super::PastLifeFeat;

/// Passive Heroic Past Lives
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HeroicPastLife(pub PlayerClass);

impl Display for HeroicPastLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(class) = self;
        write!(f, "{class} Past Life")
    }
}

impl StaticOptions for HeroicPastLife {
    fn get_static() -> impl Iterator<Item = Self> {
        PlayerClass::get_static().map(Self)
    }
}

impl GetBonuses for HeroicPastLife {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        (value > Decimal::ZERO).then(|| {
            let Self(class) = self;
            let value = value.min(dec!(3));
            match class {
                PlayerClass::Alchemist => vec![
                    BonusTemplate::new(
                        Attribute::SpellDC(SpellSchool::Transmutation.into()),
                        BonusType::Stacking,
                        value,
                    ),
                    BonusTemplate::new(SpellPoints::Base, BonusType::Stacking, dec!(20) * value),
                ],
                PlayerClass::Artificer => [
                    Skill::DisableDevice,
                    Skill::Repair,
                    Skill::Search,
                    Skill::Spellcraft,
                    Skill::UseMagicalDevice,
                ]
                .map(|skill| BonusTemplate::new(skill, BonusType::Stacking, value))
                .to_vec(),
                PlayerClass::Barbarian => vec![BonusTemplate::new(
                    Health::Bonus,
                    BonusType::Stacking,
                    dec!(20) * value,
                )],
                PlayerClass::Fighter => vec![
                    BonusTemplate::new(
                        (WeaponHand::Both, WeaponStat::Attack),
                        BonusType::Stacking,
                        value,
                    ),
                    BonusTemplate::new(Tactics::All, BonusType::Stacking, value),
                ],
                PlayerClass::Monk => vec![BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::Damage),
                    BonusType::Stacking,
                    value,
                )],
                PlayerClass::Rogue => vec![
                    BonusTemplate::new(
                        SavingThrow::Traps,
                        BonusType::Stacking,
                        value * Decimal::TWO,
                    ),
                    // TODO: Sneak Attack
                ],
                PlayerClass::Sorcerer => vec![
                    BonusTemplate::new(
                        Attribute::SpellDC(SpellSchool::Evocation.into()),
                        BonusType::Stacking,
                        value,
                    ),
                    BonusTemplate::new(SpellPoints::Base, BonusType::Stacking, dec!(20) * value),
                ],
                PlayerClass::Wizard => vec![
                    BonusTemplate::new(
                        Attribute::SpellPenetration,
                        BonusType::Stacking,
                        value * Decimal::TWO,
                    ),
                    // TODO: +2 DC to wands
                ],
                PlayerClass::FavoredSoul => vec![
                    BonusTemplate::new(Attribute::SpellPenetration, BonusType::Stacking, value),
                    BonusTemplate::new(SpellPoints::Base, BonusType::Stacking, dec!(20) * value),
                ],
                PlayerClass::Bard => vec![
                    BonusTemplate::new(
                        SavingThrow::Enchantment,
                        BonusType::Stacking,
                        Decimal::TWO * value,
                    ),
                    BonusTemplate::new(
                        SavingThrow::Illusion,
                        BonusType::Stacking,
                        Decimal::TWO * value,
                    ),
                ],
                PlayerClass::Stormsinger => vec![
                    BonusTemplate::new(
                        Attribute::SpellPower(DamageType::Electric.into()),
                        BonusType::Stacking,
                        dec!(5) * value,
                    ),
                    BonusTemplate::new(
                        Attribute::SpellPower(DamageType::Sonic.into()),
                        BonusType::Stacking,
                        dec!(5) * value,
                    ),
                ],
                PlayerClass::Cleric => vec![
                    BonusTemplate::new(
                        Attribute::SpellDC(SpellSchool::Conjuration.into()),
                        BonusType::Stacking,
                        value,
                    ),
                    // TODO: +1 turn undead charge
                ],
                PlayerClass::DarkApostate => vec![
                    // TODO: +5% negative healing amp
                    BonusTemplate::new(
                        Attribute::SpellPower(DamageType::Negative.into()),
                        BonusType::Stacking,
                        dec!(5) * value,
                    ),
                ],
                PlayerClass::Druid => vec![
                    // TODO: +2 stats for companions
                    BonusTemplate::new(Attribute::Debug(0), BonusType::Stacking, 0),
                ],
                PlayerClass::BlightCaster => vec![
                    BonusTemplate::new(
                        Attribute::SpellPower(DamageType::Acid.into()),
                        BonusType::Stacking,
                        dec!(5) * value,
                    ),
                    BonusTemplate::new(
                        Attribute::SpellPower(DamageType::Poison.into()),
                        BonusType::Stacking,
                        dec!(5) * value,
                    ),
                ],
                PlayerClass::Paladin => vec![BonusTemplate::new(
                    HealingAmplification::Positive,
                    BonusType::Stacking,
                    dec!(10) * value,
                )],
                PlayerClass::SacredFist => vec![
                    BonusTemplate::new(
                        HealingAmplification::Positive,
                        BonusType::Stacking,
                        Decimal::TEN * value,
                    ),
                    BonusTemplate::new(
                        Attribute::SpellPower(DamageType::Positive.into()),
                        BonusType::Stacking,
                        dec!(5) * value,
                    ),
                ],
                PlayerClass::Ranger => vec![BonusTemplate::new(
                    (WeaponHand::Main, WeaponStat::Damage),
                    BonusType::Stacking,
                    Decimal::TWO * value,
                )
                .with_condition(
                    WeaponType::RANGED_WEAPONS
                        .map(|wt| Condition::has(MainHandType::from(wt)))
                        .cond_any()
                        .expect("Expected Condition"),
                )],
                PlayerClass::DarkHunter => vec![
                    BonusTemplate::new(SneakAttack::Attack, BonusType::Stacking, 1),
                    BonusTemplate::new(SneakAttack::Damage, BonusType::Stacking, 1),
                ],
                PlayerClass::Warlock => vec![BonusTemplate::new(
                    Sheltering::Magical,
                    BonusType::Stacking,
                    dec!(3) * value,
                )],
                PlayerClass::AcolyteOfTheSkin => vec![
                    BonusTemplate::new(Sheltering::Magical, BonusType::Stacking, value),
                    BonusTemplate::new(
                        Attribute::SpellPower(DamageType::Fire.into()),
                        BonusType::Stacking,
                        dec!(5) * value,
                    ),
                ],
            }
        })
    }
}

impl ToFeat for HeroicPastLife {
    fn to_feat(self) -> Feat {
        PastLifeFeat::Heroic(self).to_feat()
    }
}
