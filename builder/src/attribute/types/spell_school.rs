use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The different schools that a spell might belong to
#[cfg_attr(test, derive(enum_map::Enum))]
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
            SpellSchool::Abjuration => write!(f, "Abjuration"),
            SpellSchool::Conjuration => write!(f, "Conjuration"),
            SpellSchool::Divination => write!(f, "Divination"),
            SpellSchool::Enchantment => write!(f, "Enchantment"),
            SpellSchool::Evocation => write!(f, "Evocation"),
            SpellSchool::Illusion => write!(f, "Illusion"),
            SpellSchool::Necromancy => write!(f, "Necromancy"),
            SpellSchool::Transmutation => write!(f, "Transmutation"),
        }
    }
}
