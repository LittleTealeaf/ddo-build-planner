//! Each of the different damage types in the game
use core::fmt;

use fmt::Display;

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use super::alignment::Alignment;

/// Describes different types of damage possible in Dungeons & Dragons Online
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DamageType {
    /// Physical Damage Type
    #[serde(rename = "p", alias = "Physical")]
    Physical,
    /// Magical Damage Type
    #[serde(rename = "m", alias = "Magical")]
    Magical,
    /// Force damage type, such as spells
    #[serde(rename = "fo", alias = "Force")]
    Force,
    /// Slash damage
    #[serde(rename = "sl", alias = "Slash")]
    Slash,
    /// Pierce damage
    #[serde(rename = "pi", alias = "Pierce")]
    Pierce,
    /// Bludgeoning damage
    #[serde(rename = "bl", alias = "Bludgeon")]
    Bludgeon,
    /// Acid Damage
    #[serde(rename = "ac", alias = "Acid")]
    Acid,
    /// Fire Damage
    #[serde(rename = "fi", alias = "Fire")]
    Fire,
    /// Cold Damage
    #[serde(rename = "co", alias = "Cold")]
    Cold,
    /// Electric Damage
    #[serde(rename = "el", alias = "Electric")]
    Electric,
    /// Sonic Damage
    #[serde(rename = "so", alias = "Sonic")]
    Sonic,
    /// Positive Damage / Healing
    #[serde(rename = "po", alias = "Positive")]
    Positive,
    /// Negative Damage / Healing
    #[serde(rename = "ne", alias = "Negative")]
    Negative,
    /// Poison Damage
    #[serde(rename = "ps", alias = "Poison")]
    Poison,
    /// Repair Damage/Healing
    #[serde(rename = "re", alias = "Repair")]
    Repair,
    /// Rust Damage
    #[serde(rename = "ru", alias = "Rust")]
    Rust,
    /// Alignment Damage
    #[serde(rename = "ali", alias = "Alignment")]
    Alignment,
    /// Light Damage
    #[serde(rename = "l", alias = "Light")]
    Light,
    /// Specific Alignment Damage
    #[serde(rename = "al", alias = "Aligned")]
    Aligned(Alignment),
    /// Untyped Damage
    #[serde(rename = "u", alias = "Untyped")]
    Untyped,
}

impl Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Physical => write!(f, "Physical"),
            Self::Force => write!(f, "Force"),
            Self::Slash => write!(f, "Slash"),
            Self::Pierce => write!(f, "Pierce"),
            Self::Bludgeon => write!(f, "Bludgeon"),
            Self::Acid => write!(f, "Acid"),
            Self::Fire => write!(f, "Fire"),
            Self::Cold => write!(f, "Cold"),
            Self::Electric => write!(f, "Electric"),
            Self::Sonic => write!(f, "Sonic"),
            Self::Positive => write!(f, "Positive"),
            Self::Negative => write!(f, "Negative"),
            Self::Poison => write!(f, "Poison"),
            Self::Repair => write!(f, "Repair"),
            Self::Rust => write!(f, "Rust"),
            Self::Alignment => write!(f, "Alignment"),
            Self::Light => write!(f, "Light"),
            Self::Untyped => write!(f, "Untyped"),
            Self::Magical => write!(f, "Magical"),
            Self::Aligned(alignment) => write!(f, "{alignment} aligned"),
        }
    }
}

impl StaticValues for DamageType {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            [
                Self::Physical,
                Self::Force,
                Self::Slash,
                Self::Pierce,
                Self::Bludgeon,
                Self::Acid,
                Self::Fire,
                Self::Cold,
                Self::Electric,
                Self::Sonic,
                Self::Positive,
                Self::Negative,
                Self::Poison,
                Self::Repair,
                Self::Rust,
                Self::Alignment,
                Self::Light,
                Self::Untyped,
                Self::Magical,
            ],
            Alignment::values().map(Self::Aligned)
        )
    }
}
