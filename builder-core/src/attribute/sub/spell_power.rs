use crate::{
    attribute::{Attribute, GetCloned},
    simple_enum,
};

simple_enum!(SpellPower, "", (Universal "Universal", Acid "Acid", Light "Light", Cold "Cold", Electric "Electric", Fire "Fire", Force "Force", Negative "Negative", Physical "Physical", Poison "Poison", Positive "Positive", Repair "Repair", Rust "Rust", Sonic "Sonic", All "All"));

impl SpellPower {
    /// Converts to [`Attribute::SpellPower`]
    pub fn into_spell_power(self) -> Attribute {
        Attribute::SpellPower(self)
    }

    /// Converts to [`Attribute::SpellCriticalChance`]
    pub fn into_spell_critical_chance(self) -> Attribute {
        Attribute::SpellCriticalChance(self)
    }

    /// Converts to [`Attribute::SpellCriticalDamage`]
    pub fn into_spell_critical_damage(self) -> Attribute {
        Attribute::SpellCriticalDamage(self)
    }
}

impl GetCloned<SpellPower> for SpellPower {
    fn get_cloned(&self) -> Option<Vec<SpellPower>> {
        match self {
            SpellPower::All => Some(vec![
                SpellPower::Acid,
                SpellPower::Light,
                SpellPower::Cold,
                SpellPower::Electric,
                SpellPower::Fire,
                SpellPower::Force,
                SpellPower::Negative,
                SpellPower::Physical,
                SpellPower::Poison,
                SpellPower::Positive,
                SpellPower::Repair,
                SpellPower::Rust,
                SpellPower::Sonic,
            ]),
            _ => None,
        }
    }
}
