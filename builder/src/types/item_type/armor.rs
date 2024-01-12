use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{types::flag::{Flag, ToFlag}, attribute::{ToAttribute, Attribute}};

/// The different types of armor in the game.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
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
            Self::Cloth => write!(f, "Cloth"),
            Self::Light => write!(f, "Light"),
            Self::Medium => write!(f, "Medium"),
            Self::Heavy => write!(f, "Heavy"),
        }
    }
}

impl ToFlag for ArmorType {
    fn to_flag(self) -> Flag {
        Flag::ArmorType(self)
    }
}

impl ToAttribute for ArmorType {
    fn to_attribute(self) -> Attribute {
        self.to_flag().to_attribute()
    }
}
