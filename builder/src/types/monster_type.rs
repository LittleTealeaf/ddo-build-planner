//! Monster Types
use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

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

impl StaticOptions for MonsterType {
    fn get_static() -> impl Iterator<Item = Self> {
        [Self::Orc, Self::Goblinoid, Self::Giant, Self::Outsiders].into_iter()
    }
}
