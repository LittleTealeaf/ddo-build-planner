//! Monster Types
use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

/// Different Monster Types that the character may encounter
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MonsterType {
    /// Orcs
    Orc,
    /// Goblinoids
    Goblinoid,
    /// Giants
    Giant,
    /// Outsiders
    Outsiders,
}

impl MonsterType {
    /// All possible values
    pub const VALUES: [Self; 4] = [Self::Orc, Self::Goblinoid, Self::Giant, Self::Outsiders];
}

impl Display for MonsterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Orc => write!(f, "Orc"),
            Self::Goblinoid => write!(f, "Goblinoid"),
            Self::Outsiders => write!(f, "Outsider"),
            Self::Giant => write!(f, "Giant"),
        }
    }
}

impl StaticValues for MonsterType {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
