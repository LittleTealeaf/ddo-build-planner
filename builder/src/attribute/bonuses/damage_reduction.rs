use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::types::Alignment;

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
            Self::Adamantine => write!(f, "Adamantine"),
            Self::Byeshk => write!(f, "Byeshk"),
            Self::ColdIron => write!(f, "Cold Iron"),
            Self::Crystal => write!(f, "Crystal"),
            Self::Mithral => write!(f, "Mithral"),
            Self::Silver => write!(f, "Silver"),
            Self::Bludgeon => write!(f, "Bludgeon"),
            Self::Pierce => write!(f, "Pierce"),
            Self::Slash => write!(f, "Slash"),
            Self::Alignment(alignment) => alignment.fmt(f),
        }
    }
}
