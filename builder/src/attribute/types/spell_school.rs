use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The different schools that a spell might belong to
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpellSchool {
    /// Abjuration Spells
    Abjuration,
    /// Conjuration Spells
    Conjuration,
    /// Divination Spells
    Divination,
    /// Enchantment Spells
    Enchantment,
    /// Evocation Spells
    Evocation,
    /// Illusion Spells
    Illusion,
    /// Necromancy Spells
    Necromancy,
    /// Transmutation Spells
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
