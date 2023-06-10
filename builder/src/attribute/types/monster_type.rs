use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Different Monster Types that the character may encounter
#[cfg_attr(test, derive(enum_map::Enum))]
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
            MonsterType::Orc => write!(f, "Orc"),
            MonsterType::Goblinoid => write!(f, "Goblinoid"),
            MonsterType::Giant => write!(f, "Giant"),
        }
    }
}
