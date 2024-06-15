use core::fmt;

use fmt::Display;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{BonusTemplate, BonusType, Condition, ConditionFold, Value},
    feat::{Feat, ToFeat},
    types::{
        armor_class::ArmorClass,
        flag::MainHandType,
        immunity::Immunity,
        item_type::WeaponType,
        monster_type::MonsterType,
        saving_throw::SavingThrow,
        skill::Skill,
        toggle::AttackingTarget,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
    val,
};

/// Feats granted from different races.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RacialFeat {
    /// Small Size Bonus
    SmallSizeBonus,
    /// Gnomish Proficiencies
    GnomishProficiencies,
    /// Immunity to Sleep
    ImmunityToSleep,
    /// Enchantment Save Bonus
    EnchantmentSaveBonus,
    /// Elven Keen Senses
    ElvenKeenSenses,
    /// Racial Spell Resistance
    RacialSpellResistance,
    /// Dwarven Stability
    DwarvenStability,
    /// Giant Evasion
    GiantEvasion,
    /// Orc and Goblin Bonus
    OrcAndGoblinBonus,
    /// Dwarven Stonecunning
    DwarvenStonecunning,
    /// Spell Save Bonus
    SpellSaveBonus,
    /// Poison Save Bonus
    PoisonSaveBonus,
    /// Halfling Agility
    HalflingAgility,
    /// Halfling Bravery
    HalflingBravery,
    /// Halfling Keen Ears
    HalflingKeenEars,
    /// Halfling Luck
    HalflingLuck,
    /// Halfling Thrown Weapon Focus
    HalflingThrownWeaponFocus,
}

impl Display for RacialFeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SmallSizeBonus => write!(f, "Small Size Bonus"),
            Self::GnomishProficiencies => write!(f, "Gnomish Proficiencies"),
            Self::ImmunityToSleep => write!(f, "Immunity to Sleep"),
            Self::EnchantmentSaveBonus => write!(f, "Enchantment Save Bonus"),
            Self::ElvenKeenSenses => write!(f, "Elven Keen Senses"),
            Self::RacialSpellResistance => write!(f, "Racial Spell Resistance"),
            Self::DwarvenStability => write!(f, "Dwarven Stability"),
            Self::GiantEvasion => write!(f, "Giant Evasion"),
            Self::OrcAndGoblinBonus => write!(f, "Orc and Goblin Bonus"),
            Self::DwarvenStonecunning => write!(f, "Dwarven Stonecunning"),
            Self::SpellSaveBonus => write!(f, "Spell Save Bonus"),
            Self::PoisonSaveBonus => write!(f, "Poison Save Bonus"),
            Self::HalflingAgility => write!(f, "Halfling Agility"),
            Self::HalflingBravery => write!(f, "Halfling Bravery"),
            Self::HalflingKeenEars => write!(f, "Halfling Keen Ears"),
            Self::HalflingLuck => write!(f, "Halfling Luck"),
            Self::HalflingThrownWeaponFocus => write!(f, "Halfling Thrown Weapon Focus"),
        }
    }
}

impl GetBonuses for RacialFeat {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        (value > Decimal::ZERO).then(|| match self {
            Self::SmallSizeBonus => {
                vec![
                    BonusTemplate::new(
                        (WeaponHand::Both, WeaponStat::Attack),
                        BonusType::Size,
                        Value::ONE,
                    ),
                    BonusTemplate::new(ArmorClass::Bonus, BonusType::Size, Value::ONE),
                    BonusTemplate::new(Skill::Hide, BonusType::Size, val!(4)),
                ]
            }
            Self::GnomishProficiencies => {
                vec![
                    BonusTemplate::new(Skill::Haggle, BonusType::Stacking, Value::TWO),
                    BonusTemplate::new(Skill::UseMagicalDevice, BonusType::Stacking, Value::TWO),
                ]
            }
            Self::ImmunityToSleep => {
                vec![BonusTemplate::flag(Immunity::Sleep)]
            }
            Self::EnchantmentSaveBonus => {
                vec![BonusTemplate::new(
                    SavingThrow::Enchantment,
                    BonusType::Stacking,
                    Value::TWO,
                )]
            }
            Self::ElvenKeenSenses => {
                vec![
                    BonusTemplate::new(Skill::Listen, BonusType::Stacking, Value::TWO),
                    BonusTemplate::new(Skill::Search, BonusType::Stacking, Value::TWO),
                    BonusTemplate::new(Skill::Spot, BonusType::Stacking, Value::TWO),
                ]
            }
            Self::RacialSpellResistance => {
                vec![BonusTemplate::new(
                    Attribute::SpellResistance,
                    BonusType::Stacking,
                    val!(6),
                )]
            }
            Self::DwarvenStability => {
                vec![BonusTemplate::new(
                    Skill::Balance,
                    BonusType::Stacking,
                    val!(4),
                )]
            }
            Self::GiantEvasion => {
                vec![
                    BonusTemplate::toggle(AttackingTarget::MonsterType(MonsterType::Giant)),
                    BonusTemplate::new(ArmorClass::Bonus, BonusType::Dodge, val!(4))
                        .with_condition(Condition::toggled(AttackingTarget::MonsterType(
                            MonsterType::Giant,
                        ))),
                ]
            }
            Self::OrcAndGoblinBonus => {
                vec![
                    BonusTemplate::toggle(AttackingTarget::MonsterType(MonsterType::Orc)),
                    BonusTemplate::toggle(AttackingTarget::MonsterType(MonsterType::Goblinoid)),
                    BonusTemplate::new(
                        (WeaponHand::Both, WeaponStat::Attack),
                        BonusType::Racial,
                        Value::ONE,
                    )
                    .with_condition(
                        Condition::toggled(AttackingTarget::MonsterType(MonsterType::Orc))
                            | Condition::toggled(AttackingTarget::MonsterType(
                                MonsterType::Goblinoid,
                            )),
                    ),
                ]
            }
            Self::DwarvenStonecunning => {
                vec![BonusTemplate::new(
                    Skill::Search,
                    BonusType::Stacking,
                    Value::TWO,
                )]
            }
            Self::SpellSaveBonus => {
                vec![BonusTemplate::new(
                    SavingThrow::Spell,
                    BonusType::Stacking,
                    Value::TWO,
                )]
            }
            Self::PoisonSaveBonus => {
                vec![BonusTemplate::new(
                    SavingThrow::Poison,
                    BonusType::Stacking,
                    Value::TWO,
                )]
            }
            Self::HalflingAgility => vec![
                BonusTemplate::new(Skill::Jump, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Skill::MoveSilently, BonusType::Stacking, Value::TWO),
            ],
            Self::HalflingBravery => {
                vec![BonusTemplate::new(
                    SavingThrow::Fear,
                    BonusType::Morale,
                    Value::TWO,
                )]
            }
            Self::HalflingKeenEars => {
                vec![BonusTemplate::new(
                    Skill::Listen,
                    BonusType::Stacking,
                    Value::TWO,
                )]
            }
            Self::HalflingLuck => vec![BonusTemplate::new(
                SavingThrow::All,
                BonusType::Luck,
                Value::ONE,
            )],
            Self::HalflingThrownWeaponFocus => {
                vec![BonusTemplate::new(
                    (WeaponHand::Main, WeaponStat::Attack),
                    BonusType::Stacking,
                    Value::ONE,
                )
                .with_condition(
                    WeaponType::THROWING_WEAPONS
                        .map(|wt| Condition::has(MainHandType::Weapon(wt)))
                        .cond_any()
                        .expect("Expected Condition"),
                )]
            }
        })
    }
}

impl ToFeat for RacialFeat {
    fn to_feat(self) -> Feat {
        Feat::RacialFeat(self)
    }
}

impl StaticValues for RacialFeat {
    fn values() -> impl Iterator<Item = Self> {
        [
            Self::SmallSizeBonus,
            Self::GnomishProficiencies,
            Self::ImmunityToSleep,
            Self::EnchantmentSaveBonus,
            Self::ElvenKeenSenses,
            Self::RacialSpellResistance,
            Self::DwarvenStability,
            Self::GiantEvasion,
            Self::OrcAndGoblinBonus,
            Self::DwarvenStonecunning,
            Self::SpellSaveBonus,
            Self::PoisonSaveBonus,
            Self::HalflingAgility,
            Self::HalflingBravery,
            Self::HalflingKeenEars,
            Self::HalflingLuck,
            Self::HalflingThrownWeaponFocus,
        ]
        .into_iter()
    }
}
