//! Spell School
use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

/// The different schools that a spell might belong to
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpellSchool {
    /// Abjuration Spells
    #[serde(rename = "Ab", alias = "Abjuration")]
    Abjuration,
    /// Conjuration Spells
    #[serde(rename = "Co", alias = "Conjuration")]
    Conjuration,
    /// Divination Spells
    #[serde(rename = "Di", alias = "Divination")]
    Divination,
    /// Enchantment Spells
    #[serde(rename = "En", alias = "Enchantment")]
    Enchantment,
    /// Evocation Spells
    #[serde(rename = "Ev", alias = "Evocation")]
    Evocation,
    /// Illusion Spells
    #[serde(rename = "Il", alias = "Illusion")]
    Illusion,
    /// Necromancy Spells
    #[serde(rename = "Ne", alias = "Necromancy")]
    Necromancy,
    /// Transmutation Spells
    #[serde(rename = "Tr", alias = "Transmutation")]
    Transmutation,
}

impl SpellSchool {
    /// Returns all of the Spell School values as an array.
    pub const ALL: [Self; 8] = [
        Self::Abjuration,
        Self::Conjuration,
        Self::Divination,
        Self::Enchantment,
        Self::Evocation,
        Self::Illusion,
        Self::Necromancy,
        Self::Transmutation,
    ];
}

impl Display for SpellSchool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abjuration => write!(f, "Abjuration"),
            Self::Conjuration => write!(f, "Conjuration"),
            Self::Divination => write!(f, "Divination"),
            Self::Enchantment => write!(f, "Enchantment"),
            Self::Evocation => write!(f, "Evocation"),
            Self::Illusion => write!(f, "Illusion"),
            Self::Necromancy => write!(f, "Necromancy"),
            Self::Transmutation => write!(f, "Transmutation"),
        }
    }
}

impl StaticValues for SpellSchool {
    fn values() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}
