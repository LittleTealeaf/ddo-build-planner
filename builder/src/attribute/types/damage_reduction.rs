use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::Alignment;

/// Indicates a type of Damage Reduction that something might have
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DamageReduction {
    /// Adamantine Damage Reduction
    Adamantine,
    /// Byeshk Damage Reduction
    Byeshk,
    /// Cold Iron Damage Reduction
    ColdIron,
    /// Crystal Damage Reduction
    Crystal,
    /// Mithral Damage Reduction
    Mithral,
    /// Silver Damage Reduction
    Silver,
    /// Bludgeoning Damage Reduction
    Bludgeon,
    /// Piercing Damage Reduction
    Pierce,
    /// Slashing Damage Reduction
    Slash,
    /// Alignment-based Damage Reduction
    Alignment(Alignment),
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
            DamageReduction::Alignment(alignment) => alignment.fmt(f),
        }
    }
}
