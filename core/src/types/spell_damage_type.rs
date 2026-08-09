use itertools::chain;

use crate::{
    attribute::Attribute,
    traits::IterValues,
    types::{alignment::Alignment, damage_type::DamageType},
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
pub enum SpellDamageType {
    Universal,
    Damage(DamageType),
}

impl SpellDamageType {
    pub const SPELL_POWERS: [Self; 15] = [
        Self::Damage(DamageType::Acid),
        Self::Damage(DamageType::Fire),
        Self::Damage(DamageType::Cold),
        Self::Damage(DamageType::Electric),
        Self::Damage(DamageType::Sonic),
        Self::Damage(DamageType::Positive),
        Self::Damage(DamageType::Negative),
        Self::Damage(DamageType::Poison),
        Self::Damage(DamageType::Repair),
        Self::Damage(DamageType::Rust),
        Self::Damage(DamageType::Aligned(Alignment::Good)),
        Self::Damage(DamageType::Aligned(Alignment::Evil)),
        Self::Damage(DamageType::Aligned(Alignment::Lawful)),
        Self::Damage(DamageType::Aligned(Alignment::Chaotic)),
        Self::Damage(DamageType::Light),
    ];

    #[must_use]
    pub const fn spell_power(self) -> Attribute {
        Attribute::SpellPower(self)
    }

    #[must_use]
    pub const fn spell_critical_chance(self) -> Attribute {
        Attribute::SpellCriticalChance(self)
    }

    #[must_use]
    pub const fn spell_critical_damage(self) -> Attribute {
        Attribute::SpellCriticalDamage(self)
    }
}

impl IterValues for SpellDamageType {
    fn values() -> impl Iterator<Item = Self> {
        chain!([Self::Universal], DamageType::values().map(Self::Damage))
    }
}
