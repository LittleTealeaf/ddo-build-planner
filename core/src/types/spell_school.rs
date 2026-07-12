use serde::{Deserialize, Serialize};

use crate::traits::IterValues;

#[derive(
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    derive_more::Display,
)]
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

impl IterValues for SpellSchool {
    fn values() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}
