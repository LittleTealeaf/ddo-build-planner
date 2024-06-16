use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

/// The types of shields.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ShieldType {
    /// Buckler shields
    #[serde(rename = "b", alias = "Buckler")]
    Buckler,
    /// Small Shields
    #[serde(rename = "s", alias = "SmallShield")]
    SmallShield,
    /// Large Shields
    #[serde(rename = "l", alias = "LargeShield")]
    LargeShield,
    /// Tower Shields
    #[serde(rename = "t", alias = "TowerShield")]
    TowerShield,
    /// Orbs
    ///
    /// While not technically shields, they fit just as well in this category.
    #[serde(rename = "o", alias = "Orb")]
    Orb,
}

impl Display for ShieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Buckler => write!(f, "Buckler"),
            Self::SmallShield => write!(f, "Small Shield"),
            Self::LargeShield => write!(f, "Large Shield"),
            Self::TowerShield => write!(f, "Tower Shield"),
            Self::Orb => write!(f, "Orb"),
        }
    }
}

impl StaticValues for ShieldType {
    fn values() -> impl Iterator<Item = Self> {
        [
            Self::Buckler,
            Self::SmallShield,
            Self::LargeShield,
            Self::TowerShield,
            Self::Orb,
        ]
        .into_iter()
    }
}
