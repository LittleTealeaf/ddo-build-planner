use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{
        flags::{Flag, OffHandType},
        types::Sheltering,
        Attribute, GetBonuses,
    },
    bonus::{Bonus, BonusType},
};

/// The types of shields.
#[cfg_attr(test, derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ShieldType {
    /// Buckler shields
    Buckler,
    /// Small Shields
    SmallShield,
    /// Large Shields
    LargeShield,
    /// Tower Shields
    TowerShield,
    /// Orbs
    ///
    /// While not technically shields, they fit just as well in this category.
    Orb,
}

impl Display for ShieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShieldType::Buckler => write!(f, "Buckler"),
            ShieldType::SmallShield => write!(f, "Small Shield"),
            ShieldType::LargeShield => write!(f, "Large Shield"),
            ShieldType::TowerShield => write!(f, "Tower Shield"),
            ShieldType::Orb => write!(f, "Orb"),
        }
    }
}

impl GetBonuses for ShieldType {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        (value > 0f32).then(|| match self {
            ShieldType::SmallShield => Some(vec![Bonus::new(
                Sheltering::Physical.into(),
                BonusType::Stacking,
                5f32.into(),
                Attribute::from(Flag::from(OffHandType::from(ShieldType::SmallShield))).into(),
                None,
            )]),
            ShieldType::LargeShield => Some(vec![Bonus::new(
                Sheltering::Physical.into(),
                BonusType::Stacking,
                10f32.into(),
                Attribute::from(Flag::from(OffHandType::from(ShieldType::LargeShield))).into(),
                None,
            )]),
            ShieldType::TowerShield => Some(vec![Bonus::new(
                Sheltering::Physical.into(),
                BonusType::Stacking,
                15f32.into(),
                Attribute::from(Flag::from(OffHandType::from(ShieldType::TowerShield))).into(),
                None,
            )]),
            _ => None,
        })?
    }
}
