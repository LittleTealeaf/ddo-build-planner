use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::Alignment;

/// Describes different types of damage possible in Dungeons & Dragons Online
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
    /// Repair Damage/Heaing
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            Self::Aligned(alignment) => alignment.fmt(f)
        }
    }
}
