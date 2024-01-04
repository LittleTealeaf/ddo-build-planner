use std::fmt::Display;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType, Condition, ConditionFold},
    feat::Feat,
    types::{
        armor_class::ArmorClass,
        flag::{Flag, MainHandType, OffHandType},
        immunity::Immunity,
        item::WeaponType,
        monster_type::MonsterType,
        saving_throw::SavingThrow,
        skill::Skill,
        toggle::{AttackingTarget, Toggle},
        weapon_attribute::{WeaponHand, WeaponStat},
    },
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<Bonus>> {
        (value > Decimal::ZERO).then(|| match self {
            Self::SmallSizeBonus => {
                vec![
                    Bonus::new(
                        (WeaponHand::Both, WeaponStat::Attack),
                        BonusType::Size,
                        1,
                        Attribute::from(Feat::RacialFeat(Self::SmallSizeBonus)),
                        None,
                    ),
                    Bonus::new(
                        ArmorClass::Bonus,
                        BonusType::Size,
                        1,
                        Attribute::from(Feat::RacialFeat(Self::SmallSizeBonus)),
                        None,
                    ),
                    Bonus::new(
                        Skill::Hide,
                        BonusType::Size,
                        4,
                        Attribute::from(Feat::RacialFeat(Self::SmallSizeBonus)),
                        None,
                    ),
                ]
            }
            Self::GnomishProficiencies => {
                vec![
                    Bonus::new(
                        Skill::Haggle,
                        BonusType::Stacking,
                        2,
                        Attribute::from(Feat::RacialFeat(Self::GnomishProficiencies)),
                        None,
                    ),
                    Bonus::new(
                        Skill::UseMagicalDevice,
                        BonusType::Stacking,
                        22,
                        Attribute::from(Feat::RacialFeat(Self::GnomishProficiencies)),
                        None,
                    ),
                ]
            }
            Self::ImmunityToSleep => {
                vec![Bonus::new(
                    Immunity::Sleep,
                    BonusType::Stacking,
                    1,
                    Attribute::from(Feat::RacialFeat(Self::ImmunityToSleep)),
                    None,
                )]
            }
            Self::EnchantmentSaveBonus => {
                vec![Bonus::new(
                    SavingThrow::Enchantment,
                    BonusType::Stacking,
                    2,
                    Attribute::from(Feat::RacialFeat(Self::EnchantmentSaveBonus)),
                    None,
                )]
            }
            Self::ElvenKeenSenses => {
                vec![
                    Bonus::new(
                        Skill::Listen,
                        BonusType::Stacking,
                        2,
                        Attribute::from(Feat::RacialFeat(Self::ElvenKeenSenses)),
                        None,
                    ),
                    Bonus::new(
                        Skill::Search,
                        BonusType::Stacking,
                        2,
                        Attribute::from(Feat::RacialFeat(Self::ElvenKeenSenses)),
                        None,
                    ),
                    Bonus::new(
                        Skill::Spot,
                        BonusType::Stacking,
                        2,
                        Attribute::from(Feat::RacialFeat(Self::ElvenKeenSenses)),
                        None,
                    ),
                ]
            }
            Self::RacialSpellResistance => {
                vec![Bonus::new(
                    Attribute::SpellResistance,
                    BonusType::Stacking,
                    6,
                    Attribute::from(Feat::RacialFeat(Self::RacialSpellResistance)),
                    None,
                )]
            }
            Self::DwarvenStability => {
                vec![Bonus::new(
                    Skill::Balance,
                    BonusType::Stacking,
                    4,
                    Attribute::from(Feat::RacialFeat(Self::DwarvenStability)),
                    None,
                )]
            }
            Self::GiantEvasion => {
                vec![
                    Bonus::flag(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Giant)),
                        Attribute::from(Feat::RacialFeat(Self::GiantEvasion)),
                    ),
                    Bonus::new(
                        ArmorClass::Bonus,
                        BonusType::Dodge,
                        4,
                        Attribute::from(Feat::RacialFeat(Self::GiantEvasion)),
                        Some(Condition::has(Toggle::Attacking(
                            AttackingTarget::MonsterType(MonsterType::Giant),
                        ))),
                    ),
                ]
            }
            Self::OrcAndGoblinBonus => {
                vec![
                    Bonus::flag(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Orc)),
                        Attribute::from(Feat::RacialFeat(Self::OrcAndGoblinBonus)),
                    ),
                    Bonus::flag(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Goblinoid)),
                        Attribute::from(Feat::RacialFeat(Self::OrcAndGoblinBonus)),
                    ),
                    Bonus::new(
                        (WeaponHand::Both, WeaponStat::Attack),
                        BonusType::Racial,
                        1,
                        Attribute::from(Feat::RacialFeat(Self::OrcAndGoblinBonus)),
                        Some(
                            Condition::has(Toggle::Attacking(AttackingTarget::MonsterType(
                                MonsterType::Orc,
                            ))) | Condition::has(Toggle::Attacking(AttackingTarget::MonsterType(
                                MonsterType::Goblinoid,
                            ))),
                        ),
                    ),
                ]
            }
            Self::DwarvenStonecunning => {
                vec![Bonus::new(
                    Skill::Search,
                    BonusType::Stacking,
                    2,
                    Attribute::from(Feat::RacialFeat(Self::DwarvenStonecunning)),
                    None,
                )]
            }
            Self::SpellSaveBonus => {
                vec![Bonus::new(
                    SavingThrow::Spell,
                    BonusType::Stacking,
                    2,
                    Attribute::from(Feat::from(Self::SpellSaveBonus)),
                    None,
                )]
            }
            Self::PoisonSaveBonus => {
                vec![Bonus::new(
                    SavingThrow::Poison,
                    BonusType::Stacking,
                    2,
                    Attribute::from(Feat::from(Self::PoisonSaveBonus)),
                    None,
                )]
            }
            Self::HalflingAgility => vec![
                Bonus::new(
                    Skill::Jump,
                    BonusType::Stacking,
                    2,
                    Attribute::from(Feat::from(Self::HalflingAgility)),
                    None,
                ),
                Bonus::new(
                    Skill::MoveSilently,
                    BonusType::Stacking,
                    2,
                    Attribute::from(Feat::from(Self::HalflingAgility)),
                    None,
                ),
            ],
            Self::HalflingBravery => vec![Bonus::new(
                SavingThrow::Fear,
                BonusType::Morale,
                2,
                Attribute::from(Feat::from(Self::HalflingBravery)),
                None,
            )],
            Self::HalflingKeenEars => vec![Bonus::new(
                Skill::Listen,
                BonusType::Stacking,
                2,
                Attribute::from(Feat::from(Self::HalflingKeenEars)),
                None,
            )],
            Self::HalflingLuck => vec![Bonus::new(
                SavingThrow::All,
                BonusType::Luck,
                1,
                Attribute::from(Feat::from(Self::HalflingLuck)),
                None,
            )],
            Self::HalflingThrownWeaponFocus => {
                vec![
                    Bonus::new(
                        (WeaponHand::Main, WeaponStat::Attack),
                        BonusType::Stacking,
                        1,
                        Attribute::from(Feat::from(Self::HalflingThrownWeaponFocus)),
                        WeaponType::THROWING_WEAPONS
                            .map(|wt| Condition::has(Flag::from(MainHandType::Weapon(wt))))
                            .cond_any(),
                    ),
                    Bonus::new(
                        (WeaponHand::Off, WeaponStat::Attack),
                        BonusType::Stacking,
                        1,
                        Attribute::from(Feat::from(Self::HalflingThrownWeaponFocus)),
                        WeaponType::THROWING_WEAPONS
                            .map(|wt| Condition::has(Flag::from(OffHandType::Weapon(wt))))
                            .cond_any(),
                    ),
                ]
            }
        })
    }
}
