use std::fmt::Display;

use enum_map::Enum;

use crate::item::types::ShieldType;

/// Shield Proficiencies
#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum ShieldProficiency {
    /// Orbs
    Orb,
    /// General Shield Proficiency
    Shield,
    /// Tower Shield Proficiency
    TowerShield,
}

impl Display for ShieldProficiency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShieldProficiency::Shield => write!(f, "Shield"),
            ShieldProficiency::TowerShield => write!(f, "Tower Shield"),
            ShieldProficiency::Orb => write!(f, "Orb"),
        }
    }
}

impl From<ShieldType> for ShieldProficiency {
    fn from(value: ShieldType) -> Self {
        match value {
            ShieldType::Buckler | ShieldType::SmallShield | ShieldType::LargeShield => Self::Shield,
            ShieldType::TowerShield => Self::TowerShield,
            ShieldType::Orb => Self::Orb,
        }
    }
}
