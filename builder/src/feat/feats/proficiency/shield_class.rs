use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::types::item_type::ShieldType;


/// Shield Proficiencies
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
            Self::Shield => write!(f, "Shield"),
            Self::TowerShield => write!(f, "Tower Shield"),
            Self::Orb => write!(f, "Orb"),
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
