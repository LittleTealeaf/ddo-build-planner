use crate::types::{
    flag::ToFlag, spell_power::SpellPower, spell_selector::SpellSelector, toggle::ToToggle,
};

use super::Attribute;

/// Helper functions for more complex variants
impl Attribute {
    /// Creates an [`Attribute::Flag`] attribute
    pub fn flag<F>(flag: F) -> Self
    where
        F: ToFlag,
    {
        Self::Flag(flag.to_flag())
    }

    /// Creates an [`Attribute::Toggle`] attribute
    pub fn toggle<T>(toggle: T) -> Self
    where
        T: ToToggle,
    {
        Self::Toggle(toggle.to_toggle())
    }

    /// Creates an [`Attribute::SpellPower`] attribute
    pub fn spell_power<S>(spell_power: S) -> Self
    where
        S: Into<SpellPower>,
    {
        Self::SpellPower(spell_power.into())
    }

    /// Creates an [`Attribute::SpellCriticalChance`] attribute
    pub fn spell_critical_chance<S>(spell_power: S) -> Self
    where
        S: Into<SpellPower>,
    {
        Self::SpellCriticalChance(spell_power.into())
    }

    /// Creates an [`Attribute::SpellCriticalDamage`] attribute
    pub fn spell_critical_damage<S>(spell_power: S) -> Self
    where
        S: Into<SpellPower>,
    {
        Self::SpellCriticalDamage(spell_power.into())
    }

    /// Creates an [`Attribute::CasterLevel`] attribute
    pub fn caster_level<S>(selector: S) -> Self
    where
        S: Into<SpellSelector>,
    {
        Self::CasterLevel(selector.into())
    }

    /// Creates an [`Attribute::MaxCasterLevel`] attribute
    pub fn max_caster_level<S>(selector: S) -> Self
    where
        S: Into<SpellSelector>,
    {
        Self::MaxCasterLevel(selector.into())
    }

    /// Creates an [`Attribute::SpellDC`] attribute
    pub fn spell_dc<S>(selector: S) -> Self
    where
        S: Into<SpellSelector>,
    {
        Self::SpellDC(selector.into())
    }
}
