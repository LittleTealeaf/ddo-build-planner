use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{
        flags::{Flag, MainHandType, OffHandType},
        toggles::{AttackingTarget, Toggle},
        Attribute, GetBonuses,
    },
    bonus::{Bonus, BonusType, Condition},
    equipment::item::types::WeaponType,
    feat::Feat,
    types::{ArmorClass, Immunity, MonsterType, SavingThrow, Skill, WeaponHand, WeaponStat},
};

/// Feats granted from different races.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (value > 0f32).then(|| match self {
            Self::SmallSizeBonus => {
                vec![
                    Bonus::new(
                        (WeaponHand::Both, WeaponStat::Attack).into(),
                        BonusType::Size,
                        1f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::SmallSizeBonus)).into(),
                        None,
                    ),
                    Bonus::new(
                        ArmorClass::Bonus.into(),
                        BonusType::Size,
                        1f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::SmallSizeBonus)).into(),
                        None,
                    ),
                    Bonus::new(
                        Skill::Hide.into(),
                        BonusType::Size,
                        4f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::SmallSizeBonus)).into(),
                        None,
                    ),
                ]
            }
            Self::GnomishProficiencies => {
                vec![
                    Bonus::new(
                        Skill::Haggle.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::GnomishProficiencies)).into(),
                        None,
                    ),
                    Bonus::new(
                        Skill::UseMagicalDevice.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::GnomishProficiencies)).into(),
                        None,
                    ),
                ]
            }
            Self::ImmunityToSleep => {
                vec![Bonus::new(
                    Immunity::Sleep.into(),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::from(Feat::RacialFeat(Self::ImmunityToSleep)).into(),
                    None,
                )]
            }
            Self::EnchantmentSaveBonus => {
                vec![Bonus::new(
                    SavingThrow::Enchantment.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::RacialFeat(Self::EnchantmentSaveBonus)).into(),
                    None,
                )]
            }
            Self::ElvenKeenSenses => {
                vec![
                    Bonus::new(
                        Skill::Listen.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::ElvenKeenSenses)).into(),
                        None,
                    ),
                    Bonus::new(
                        Skill::Search.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::ElvenKeenSenses)).into(),
                        None,
                    ),
                    Bonus::new(
                        Skill::Spot.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::ElvenKeenSenses)).into(),
                        None,
                    ),
                ]
            }
            Self::RacialSpellResistance => {
                vec![Bonus::new(
                    Attribute::SpellResistance,
                    BonusType::Stacking,
                    6f32.into(),
                    Attribute::from(Feat::RacialFeat(Self::RacialSpellResistance)).into(),
                    None,
                )]
            }
            Self::DwarvenStability => {
                vec![Bonus::new(
                    Skill::Balance.into(),
                    BonusType::Stacking,
                    4f32.into(),
                    Attribute::from(Feat::RacialFeat(Self::DwarvenStability)).into(),
                    None,
                )]
            }
            Self::GiantEvasion => {
                vec![
                    Bonus::flag(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Giant)).into(),
                        Attribute::from(Feat::RacialFeat(Self::GiantEvasion)).into(),
                    ),
                    Bonus::new(
                        ArmorClass::Bonus.into(),
                        BonusType::Dodge,
                        4f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::GiantEvasion)).into(),
                        Some(Condition::has(
                            Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Giant))
                                .into(),
                        )),
                    ),
                ]
            }
            Self::OrcAndGoblinBonus => {
                vec![
                    Bonus::flag(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Orc)).into(),
                        Attribute::from(Feat::RacialFeat(Self::OrcAndGoblinBonus)).into(),
                    ),
                    Bonus::flag(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Goblinoid))
                            .into(),
                        Attribute::from(Feat::RacialFeat(Self::OrcAndGoblinBonus)).into(),
                    ),
                    Bonus::new(
                        (WeaponHand::Both, WeaponStat::Attack).into(),
                        BonusType::Racial,
                        1f32.into(),
                        Attribute::from(Feat::RacialFeat(Self::OrcAndGoblinBonus)).into(),
                        Some(Condition::Any(vec![
                            Condition::has(
                                Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Orc))
                                    .into(),
                            ),
                            Condition::has(
                                Toggle::Attacking(AttackingTarget::MonsterType(
                                    MonsterType::Goblinoid,
                                ))
                                .into(),
                            ),
                        ])),
                    ),
                ]
            }
            Self::DwarvenStonecunning => {
                vec![Bonus::new(
                    Skill::Search.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::RacialFeat(Self::DwarvenStonecunning)).into(),
                    None,
                )]
            }
            Self::SpellSaveBonus => {
                vec![Bonus::new(
                    SavingThrow::Spell.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::from(Self::SpellSaveBonus)).into(),
                    None,
                )]
            }
            Self::PoisonSaveBonus => {
                vec![Bonus::new(
                    SavingThrow::Poison.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::from(Self::PoisonSaveBonus)).into(),
                    None,
                )]
            }
            Self::HalflingAgility => vec![
                Bonus::new(
                    Skill::Jump.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::from(Self::HalflingAgility)).into(),
                    None,
                ),
                Bonus::new(
                    Skill::MoveSilently.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::from(Self::HalflingAgility)).into(),
                    None,
                ),
            ],
            Self::HalflingBravery => vec![Bonus::new(
                SavingThrow::Fear.into(),
                BonusType::Morale,
                2f32.into(),
                Attribute::from(Feat::from(Self::HalflingBravery)).into(),
                None,
            )],
            Self::HalflingKeenEars => vec![Bonus::new(
                Skill::Listen.into(),
                BonusType::Stacking,
                2f32.into(),
                Attribute::from(Feat::from(Self::HalflingKeenEars)).into(),
                None,
            )],
            Self::HalflingLuck => vec![Bonus::new(
                SavingThrow::All.into(),
                BonusType::Luck,
                1f32.into(),
                Attribute::from(Feat::from(Self::HalflingLuck)).into(),
                None,
            )],
            Self::HalflingThrownWeaponFocus => vec![
                Bonus::new(
                    (WeaponHand::Main, WeaponStat::Attack).into(),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::from(Feat::from(Self::HalflingThrownWeaponFocus)).into(),
                    Some(Condition::Any(
                        WeaponType::THROWING_WEAPONS
                            .map(|wt| Condition::has(Flag::from(MainHandType::Weapon(wt)).into()))
                            .to_vec(),
                    )),
                ),
                Bonus::new(
                    (WeaponHand::Off, WeaponStat::Attack).into(),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::from(Feat::from(Self::HalflingThrownWeaponFocus)).into(),
                    Some(Condition::Any(
                        WeaponType::THROWING_WEAPONS
                            .map(|wt| Condition::has(Flag::from(OffHandType::Weapon(wt)).into()))
                            .to_vec(),
                    )),
                ),
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::bonus::BonusSource;

    use super::*;

    use enum_map::Enum;

    #[test]
    fn one_value_returns_bonuses() {
        for feat in (0..RacialFeat::LENGTH).map(RacialFeat::from_usize) {
            assert!(feat.get_bonuses(1f32).is_some());
        }
    }

    #[test]
    fn source_matches_up() {
        for feat in (0..RacialFeat::LENGTH).map(RacialFeat::from_usize) {
            if let Some(bonuses) = feat.get_bonuses(1f32) {
                for bonus in bonuses {
                    let source = bonus.get_source();
                    let expected = BonusSource::Attribute(Attribute::Feat(Feat::RacialFeat(feat)));
                    assert_eq!(source, expected, "Expected [{expected}], found [{source}]");
                }
            }
        }
    }
}
