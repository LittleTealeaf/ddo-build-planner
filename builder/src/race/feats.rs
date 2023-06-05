use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{
        toggles::{AttackingTarget, Toggle},
        types::{ArmorClass, Immunity, MonsterType, SavingThrow, Skill, WeaponHand, WeaponStat},
        Attribute, GetBonuses,
    },
    bonus::{Bonus, BonusType, Condition},
    feat::Feat,
};

/// Feats granted from different races.
#[derive(PartialEq, Eq, Clone, Copy, Enum, Debug)]
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
        }
    }
}

impl GetBonuses for RacialFeat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (value > 0f32).then(|| match self {
            RacialFeat::SmallSizeBonus => Some(vec![
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
            ]),
            RacialFeat::GnomishProficiencies => Some(vec![
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
            ]),
            RacialFeat::ImmunityToSleep => Some(vec![Bonus::new(
                Immunity::Sleep.into(),
                BonusType::Stacking,
                1f32.into(),
                Attribute::from(Feat::RacialFeat(RacialFeat::ImmunityToSleep)).into(),
                None,
            )]),
            RacialFeat::EnchantmentSaveBonus => Some(vec![Bonus::new(
                SavingThrow::Enchantment.into(),
                BonusType::Stacking,
                2f32.into(),
                Attribute::from(Feat::RacialFeat(RacialFeat::EnchantmentSaveBonus)).into(),
                None,
            )]),
            RacialFeat::ElvenKeenSenses => Some(vec![
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
            ]),
            RacialFeat::RacialSpellResistance => Some(vec![Bonus::new(
                Attribute::SpellResistance,
                BonusType::Stacking,
                6f32.into(),
                Attribute::from(Feat::RacialFeat(RacialFeat::RacialSpellResistance)).into(),
                None,
            )]),
            RacialFeat::DwarvenStability => Some(vec![Bonus::new(
                Skill::Balance.into(),
                BonusType::Stacking,
                4f32.into(),
                Attribute::from(Feat::RacialFeat(RacialFeat::DwarvenStability)).into(),
                None,
            )]),
            RacialFeat::GiantEvasion => Some(vec![
                Bonus::flag(
                    Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Giant)).into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::OrcAndGoblinBonus)).into(),
                ),
                Bonus::new(
                    ArmorClass::Bonus.into(),
                    BonusType::Dodge,
                    4f32.into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::OrcAndGoblinBonus)).into(),
                    Some(Condition::Has(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Giant)).into(),
                    )),
                ),
            ]),
            RacialFeat::OrcAndGoblinBonus => Some(vec![
                Bonus::flag(
                    Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Orc)).into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::OrcAndGoblinBonus)).into(),
                ),
                Bonus::flag(
                    Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Goblinoid)).into(),
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
                            Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Goblinoid))
                                .into(),
                        ),
                    ])),
                ),
            ]),
            RacialFeat::DwarvenStonecunning => Some(vec![Bonus::new(
                Skill::Search.into(),
                BonusType::Stacking,
                2f32.into(),
                Attribute::from(Feat::RacialFeat(RacialFeat::DwarvenStonecunning)).into(),
                None,
            )]),
            RacialFeat::SpellSaveBonus => Some(vec![Bonus::new(
                SavingThrow::Spell.into(),
                BonusType::Stacking,
                2f32.into(),
                Attribute::from(Feat::from(RacialFeat::SpellSaveBonus)).into(),
                None,
            )]),
            RacialFeat::PoisonSaveBonus => Some(vec![Bonus::new(
                SavingThrow::Poison.into(),
                BonusType::Stacking,
                2f32.into(),
                Attribute::from(Feat::from(RacialFeat::PoisonSaveBonus)).into(),
                None,
            )]),
        })?
    }
}
