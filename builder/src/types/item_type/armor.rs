use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, ToAttribute},
    types::flag::{Flag, ToFlag},
};

/// The different types of armor in the game.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ArmorType {
    /// Cloth Armor
    #[serde(rename = "c", alias = "Cloth")]
    Cloth,
    /// Light Armor
    #[serde(rename = "l", alias = "Light")]
    Light,
    /// Medium Armor
    #[serde(rename = "m", alias = "Medium")]
    Medium,
    /// Heavy Armor
    #[serde(rename = "h", alias = "Heavy")]
    Heavy,
}

impl Display for ArmorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl StaticOptions for ArmorType {
    fn get_static() -> impl Iterator<Item = Self> {
        [Self::Cloth, Self::Light, Self::Medium, Self::Heavy].into_iter()
    }
}
