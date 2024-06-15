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
    Physical,
    /// Magical Damage Type
    Magical,
    /// Force damage type, such as spells
    Force,
    /// Slash damage
    Slash,
    /// Pierce damage
    Pierce,
    /// Bludgeoning damage
    Bludgeon,
    /// Acid Damage
    Acid,
    /// Fire Damage
    Fire,
    /// Cold Damage
    Cold,
    /// Electric Damage
    Electric,
    /// Sonic Damage
    Sonic,
    /// Positive Damage / Healing
    Positive,
    /// Negative Damage / Healing
    Negative,
    /// Poison Damage
    Poison,
    /// Repair Damage/Healing
    Repair,
    /// Rust Damage
    Rust,
    /// Alignment Damage
    Alignment,
    /// Light Damage
    Light,
    /// Specific Alignment Damage
    Aligned(Alignment),
    /// Untyped Damage
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
