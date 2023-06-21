use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{flags::Flag, types::Sheltering, Attribute, GetBonuses},
    bonus::{Bonus, BonusType, BonusValue},
};

/// The different types of armor in the game.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ArmorType {
    /// Cloth Armor
    Cloth,
    /// Light Armor
    Light,
    /// Medium Armor
    Medium,
    /// Heavy Armor
    Heavy,
}

impl Display for ArmorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmorType::Cloth => write!(f, "Cloth"),
            ArmorType::Light => write!(f, "Light"),
            ArmorType::Medium => write!(f, "Medium"),
            ArmorType::Heavy => write!(f, "Heavy"),
        }
    }
}

impl GetBonuses for ArmorType {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        (value > 0f32).then(|| match self {
            ArmorType::Cloth => None,
            ArmorType::Light => Some(vec![Bonus::new(
                Sheltering::Physical.into(),
                BonusType::Stacking,
                Attribute::BaseAttackBonus.into(),
                Attribute::from(Flag::from(*self)).into(),
                None,
            )]),
            ArmorType::Medium => Some(vec![Bonus::new(
                Sheltering::Physical.into(),
                BonusType::Stacking,
                BonusValue::Product(vec![Attribute::BaseAttackBonus.into(), 1.5f32.into()]),
                Attribute::from(Flag::from(*self)).into(),
                None,
            )]),
            ArmorType::Heavy => Some(vec![Bonus::new(
                Sheltering::Physical.into(),
                BonusType::Stacking,
                BonusValue::Product(vec![Attribute::BaseAttackBonus.into(), 2f32.into()]),
                Attribute::from(Flag::from(*self)).into(),
                None,
            )]),
        })?
    }
}
