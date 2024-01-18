use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::all::AllStatic;

/// The types of shields.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
            Self::Buckler => write!(f, "Buckler"),
            Self::SmallShield => write!(f, "Small Shield"),
            Self::LargeShield => write!(f, "Large Shield"),
            Self::TowerShield => write!(f, "Tower Shield"),
            Self::Orb => write!(f, "Orb"),
        }
    }
}

impl AllStatic for ShieldType {
    fn all() -> impl Iterator<Item = Self> {
        [
            Self::Buckler,
            Self::SmallShield,
            Self::LargeShield,
            Self::TowerShield,
            Self::Orb,
        ].into_iter()
    }
}
