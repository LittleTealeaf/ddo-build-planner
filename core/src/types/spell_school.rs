use serde::{Deserialize, Serialize};

use crate::{attribute::Attribute, types::spell_selector::SpellSelector};

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
    strum::VariantArray,
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
    #[must_use]
    pub const fn spell_dc(self) -> Attribute {
        SpellSelector::School(self).spell_dc()
    }

    #[must_use]
    pub const fn caster_level(self) -> Attribute {
        SpellSelector::School(self).caster_level()
    }
}
