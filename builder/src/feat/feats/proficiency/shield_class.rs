use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl StaticOptions for ShieldProficiency {
    fn get_static() -> impl Iterator<Item = Self> {
        [
            Self::Orb,
            Self::Shield,
            Self::TowerShield
        ].into_iter()
    }
}
