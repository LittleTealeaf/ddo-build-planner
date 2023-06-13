use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{
        flags::{Flag, MainHandType, OffHandType},
        toggles::{AttackingTarget, Toggle},
        types::{ArmorClass, Immunity, MonsterType, SavingThrow, Skill, WeaponHand, WeaponStat},
        Attribute, GetBonuses,
    },
    bonus::{Bonus, BonusType, Condition},
    feat::Feat,
    item::types::WeaponType,
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
            RacialFeat::SmallSizeBonus => write!(f, "Small Size Bonus"),
            RacialFeat::GnomishProficiencies => write!(f, "Gnomish Proficiencies"),
            RacialFeat::ImmunityToSleep => write!(f, "Immunity to Sleep"),
            RacialFeat::EnchantmentSaveBonus => write!(f, "Enchantment Save Bonus"),
            RacialFeat::ElvenKeenSenses => write!(f, "Elven Keen Senses"),
            RacialFeat::RacialSpellResistance => write!(f, "Racial Spell Resistance"),
            RacialFeat::DwarvenStability => write!(f, "Dwarven Stability"),
            RacialFeat::GiantEvasion => write!(f, "Giant Evasion"),
            RacialFeat::OrcAndGoblinBonus => write!(f, "Orc and Goblin Bonus"),
            RacialFeat::DwarvenStonecunning => write!(f, "Dwarven Stonecunning"),
            RacialFeat::SpellSaveBonus => write!(f, "Spell Save Bonus"),
            RacialFeat::PoisonSaveBonus => write!(f, "Poison Save Bonus"),
            RacialFeat::HalflingAgility => write!(f, "Halfling Agility"),
            RacialFeat::HalflingBravery => write!(f, "Halfling Bravery"),
            RacialFeat::HalflingKeenEars => write!(f, "Halfling Keen Ears"),
            RacialFeat::HalflingLuck => write!(f, "Halfling Luck"),
            RacialFeat::HalflingThrownWeaponFocus => write!(f, "Halfling Thrown Weapon Focus"),
        }
    }
}

impl GetBonuses for RacialFeat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (value > 0f32).then(|| match self {
            RacialFeat::SmallSizeBonus => {
                vec![
                    Bonus::new(
                        (WeaponHand::Both, WeaponStat::Attack).into(),
                        BonusType::Size,
                        1f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::SmallSizeBonus)).into(),
                        None,
                    ),
                    Bonus::new(
                        ArmorClass::Bonus.into(),
                        BonusType::Size,
                        1f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::SmallSizeBonus)).into(),
                        None,
                    ),
                    Bonus::new(
                        Skill::Hide.into(),
                        BonusType::Size,
                        4f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::SmallSizeBonus)).into(),
                        None,
                    ),
                ]
            }
            RacialFeat::GnomishProficiencies => {
                vec![
                    Bonus::new(
                        Skill::Haggle.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::GnomishProficiencies)).into(),
                        None,
                    ),
                    Bonus::new(
                        Skill::UseMagicalDevice.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::GnomishProficiencies)).into(),
                        None,
                    ),
                ]
            }
            RacialFeat::ImmunityToSleep => {
                vec![Bonus::new(
                    Immunity::Sleep.into(),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::ImmunityToSleep)).into(),
                    None,
                )]
            }
            RacialFeat::EnchantmentSaveBonus => {
                vec![Bonus::new(
                    SavingThrow::Enchantment.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::EnchantmentSaveBonus)).into(),
                    None,
                )]
            }
            RacialFeat::ElvenKeenSenses => {
                vec![
                    Bonus::new(
                        Skill::Listen.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::ElvenKeenSenses)).into(),
                        None,
                    ),
                    Bonus::new(
                        Skill::Search.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::ElvenKeenSenses)).into(),
                        None,
                    ),
                    Bonus::new(
                        Skill::Spot.into(),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::ElvenKeenSenses)).into(),
                        None,
                    ),
                ]
            }
            RacialFeat::RacialSpellResistance => {
                vec![Bonus::new(
                    Attribute::SpellResistance,
                    BonusType::Stacking,
                    6f32.into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::RacialSpellResistance)).into(),
                    None,
                )]
            }
            RacialFeat::DwarvenStability => {
                vec![Bonus::new(
                    Skill::Balance.into(),
                    BonusType::Stacking,
                    4f32.into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::DwarvenStability)).into(),
                    None,
                )]
            }
            RacialFeat::GiantEvasion => {
                vec![
                    Bonus::flag(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Giant)).into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::GiantEvasion)).into(),
                    ),
                    Bonus::new(
                        ArmorClass::Bonus.into(),
                        BonusType::Dodge,
                        4f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::GiantEvasion)).into(),
                        Some(Condition::Has(
                            Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Giant))
                                .into(),
                        )),
                    ),
                ]
            }
            RacialFeat::OrcAndGoblinBonus => {
                vec![
                    Bonus::flag(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Orc)).into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::OrcAndGoblinBonus)).into(),
                    ),
                    Bonus::flag(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Goblinoid))
                            .into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::OrcAndGoblinBonus)).into(),
                    ),
                    Bonus::new(
                        (WeaponHand::Both, WeaponStat::Attack).into(),
                        BonusType::Racial,
                        1f32.into(),
                        Attribute::from(Feat::RacialFeat(RacialFeat::OrcAndGoblinBonus)).into(),
                        Some(Condition::Any(vec![
                            Condition::Has(
                                Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Orc))
                                    .into(),
                            ),
                            Condition::Has(
                                Toggle::Attacking(AttackingTarget::MonsterType(
                                    MonsterType::Goblinoid,
                                ))
                                .into(),
                            ),
                        ])),
                    ),
                ]
            }
            RacialFeat::DwarvenStonecunning => {
                vec![Bonus::new(
                    Skill::Search.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::DwarvenStonecunning)).into(),
                    None,
                )]
            }
            RacialFeat::SpellSaveBonus => {
                vec![Bonus::new(
                    SavingThrow::Spell.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::from(RacialFeat::SpellSaveBonus)).into(),
                    None,
                )]
            }
            RacialFeat::PoisonSaveBonus => {
                vec![Bonus::new(
                    SavingThrow::Poison.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::from(RacialFeat::PoisonSaveBonus)).into(),
                    None,
                )]
            }
            RacialFeat::HalflingAgility => vec![
                Bonus::new(
                    Skill::Jump.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::from(RacialFeat::HalflingAgility)).into(),
                    None,
                ),
                Bonus::new(
                    Skill::MoveSilently.into(),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::from(Feat::from(RacialFeat::HalflingAgility)).into(),
                    None,
                ),
            ],
            RacialFeat::HalflingBravery => vec![Bonus::new(
                SavingThrow::Fear.into(),
                BonusType::Morale,
                2f32.into(),
                Attribute::from(Feat::from(RacialFeat::HalflingBravery)).into(),
                None,
            )],
            RacialFeat::HalflingKeenEars => vec![Bonus::new(
                Skill::Listen.into(),
                BonusType::Stacking,
                2f32.into(),
                Attribute::from(Feat::from(RacialFeat::HalflingKeenEars)).into(),
                None,
            )],
            RacialFeat::HalflingLuck => vec![Bonus::new(
                SavingThrow::All.into(),
                BonusType::Luck,
                1f32.into(),
                Attribute::from(Feat::from(RacialFeat::HalflingLuck)).into(),
                None,
            )],
            RacialFeat::HalflingThrownWeaponFocus => vec![
                Bonus::new(
                    (WeaponHand::Main, WeaponStat::Attack).into(),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::from(Feat::from(RacialFeat::HalflingThrownWeaponFocus)).into(),
                    Some(Condition::Any(
                        WeaponType::THROWING_WEAPONS
                            .map(|wt| Condition::Has(Flag::from(MainHandType::Weapon(wt)).into()))
                            .to_vec(),
                    )),
                ),
                Bonus::new(
                    (WeaponHand::Off, WeaponStat::Attack).into(),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::from(Feat::from(RacialFeat::HalflingThrownWeaponFocus)).into(),
                    Some(Condition::Any(
                        WeaponType::THROWING_WEAPONS
                            .map(|wt| Condition::Has(Flag::from(OffHandType::Weapon(wt)).into()))
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
                    assert_eq!(
                        source, expected,
                        "Expected [{}], found [{}]",
                        expected, source
                    );
                }
            }
        }
    }
}
