use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The different types of armor in the game.
#[cfg_attr(test, derive(enum_map::Enum))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ArmorType {
    /// Cloth Armor
    Cloth,
    /// Light Armor
    Light,
    /// Medium Armor
    Medium,
    /// Heavy Armor
    Heavy,
}

impl Display for ArmorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmorType::Cloth => write!(f, "Cloth"),
            ArmorType::Light => write!(f, "Light"),
            ArmorType::Medium => write!(f, "Medium"),
            ArmorType::Heavy => write!(f, "Heavy"),
        }
    }
}
