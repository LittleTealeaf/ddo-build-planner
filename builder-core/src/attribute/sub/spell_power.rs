use crate::simple_enum;

simple_enum!(SpellPower, (Universal "Universal", Acid "Acid", Light "Light", Cold "Cold", Electric "Electric", Fire "Fire", Force "Force", Negative "Negative", Physical "Physical", Poison "Poison", Positive "Positive", Repair "Repair", Rust "Rust", Sonic "Sonic", All "All"));

impl SpellPower {
    pub fn get_cloned_spellpowers(&self) -> Option<Vec<SpellPower>> {
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
