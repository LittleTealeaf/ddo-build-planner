//! Monster Types
use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Different Monster Types that the character may encounter
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MonsterType {
    /// Orcs
    Orc,
    /// Goblinoids
    Goblinoid,
    /// Giants
    Giant,
}

impl Display for MonsterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Orc => write!(f, "Orc"),
            Self::Goblinoid => write!(f, "Goblinoid"),
            Self::Giant => write!(f, "Giant"),
        }
    }
}
