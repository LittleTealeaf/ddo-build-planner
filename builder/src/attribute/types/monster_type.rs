use std::fmt::Display;

use enum_map::Enum;
use serde::{Serialize, Deserialize};

/// Different Monster Types that the character may encounter
#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, PartialOrd, Ord, Serialize, Deserialize)]
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
