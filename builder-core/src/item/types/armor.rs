use enum_map::Enum;
use serde::{Deserialize, Serialize};

/// The different types of armor that a character can wear
#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Enum)]
pub enum ArmorType {
    /// Cloth or Robe armor
    Cloth,
    /// Light Armor
    Light,
    /// Medium Armor
    Medium,
    /// Heavy Armor
    Heavy,
}

impl ToString for ArmorType {
    fn to_string(&self) -> String {
        match self {
            ArmorType::Cloth => String::from("Cloth"),
            ArmorType::Light => String::from("Light"),
            ArmorType::Medium => String::from("Medium"),
            ArmorType::Heavy => String::from("Heavy"),
        }
    }
}