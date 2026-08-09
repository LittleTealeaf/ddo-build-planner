use itertools::chain;

use crate::{
    traits::IterValues,
    types::{alignment::Alignment, spell_damage_type::SpellDamageType},
};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    derive_more::Display,
    serde::Deserialize,
    serde::Serialize,
    derive_more::From,
)]
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
    /// Light Damage
    Light,
    /// Specific Alignment Damage
    Aligned(Alignment),
    /// Untyped Damage
    Untyped,
}

impl DamageType {
    #[must_use]
    pub const fn spell_damage(self) -> SpellDamageType {
        SpellDamageType::Damage(self)
    }
}

impl IterValues for DamageType {
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
                Self::Light,
                Self::Untyped,
                Self::Magical,
            ],
            Alignment::values().map(Self::Aligned)
        )
    }
}
