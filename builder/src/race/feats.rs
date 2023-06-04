use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{
        types::{ArmorClass, Immunity, SavingThrow, Skill, WeaponHand, WeaponStat},
        Attribute, GetBonuses,
    },
    bonus::{Bonus, BonusType},
    feat::{Feat, Proficiency},
    item::types::WeaponType,
};

#[derive(PartialEq, Eq, Clone, Copy, Enum, Debug)]
pub enum RacialFeat {
    SmallSizeBonus,
    GnomishProficiencies,
    ImmunityToSleep,
    EnchantmentSaveBonus,
    ElvenKeenSenses,
    RacialSpellResistance,
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
        }
    }
}

impl GetBonuses for RacialFeat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (value > 0f32).then(|| {
            match self {
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
                RacialFeat::RacialSpellResistance => Some(vec![
                    // TODO: Add Racial Spell Resistance
                ]),
            }
        })?
    }
}
