use std::fmt::Display;

use enum_map::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum DamageReduction {
    Adamantine,
    Byeshk,
    ColdIron,
    Crystal,
    Mithral,
    Silver,
    Bludgeon,
    Pierce,
    Slash,
    Chaos,
    Evil,
    Good,
    Law,
}

impl Display for DamageReduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageReduction::Adamantine => write!(f, "Adamantine"),
            DamageReduction::Byeshk => write!(f, "Byeshk"),
            DamageReduction::ColdIron => write!(f, "Cold Iron"),
            DamageReduction::Crystal => write!(f, "Crystal"),
            DamageReduction::Mithral => write!(f, "Mithral"),
            DamageReduction::Silver => write!(f, "Silver"),
            DamageReduction::Bludgeon => write!(f, "Bludgeon"),
            DamageReduction::Pierce => write!(f, "Pierce"),
            DamageReduction::Slash => write!(f, "Slash"),
            DamageReduction::Chaos => write!(f, "Chaos"),
            DamageReduction::Evil => write!(f, "Evil"),
            DamageReduction::Good => write!(f, "Good"),
            DamageReduction::Law => write!(f, "Law"),
        }
    }
}
