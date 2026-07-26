use serde::{Deserialize, Serialize};

use crate::{attribute::Attribute, traits::IterValues, types::spell_selector::SpellSelector};

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
    pub const VALUES: [Self; 8] = [
        Self::Abjuration,
        Self::Conjuration,
        Self::Divination,
        Self::Enchantment,
        Self::Evocation,
        Self::Illusion,
        Self::Necromancy,
        Self::Transmutation,
    ];

    #[must_use]
    pub const fn spell_dc(self) -> Attribute {
        SpellSelector::School(self).spell_dc()
    }

    #[must_use]
    pub const fn caster_level(self) -> Attribute {
        SpellSelector::School(self).caster_level()
    }
}

impl IterValues for SpellSchool {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
