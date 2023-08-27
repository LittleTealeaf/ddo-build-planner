use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::types::Alignment;

/// Different types of energy resistance or absorption that the user can have
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EnergyResistance {
    /// Acid Energy
    Acid,
    /// Cold Energy
    Cold,
    /// Electric Energy
    Electric,
    /// Fire Energy
    Fire,
    /// Force
    Force,
    /// Light
    Light,
    /// Negative Energy
    Negative,
    /// Poison
    Poison,
    /// Sonic
    Sonic,
    /// Alignment based
    Alignment(Alignment),
}

impl Display for EnergyResistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Acid => write!(f, "Acid"),
            Self::Cold => write!(f, "Cold"),
            Self::Electric => write!(f, "Electric"),
            Self::Fire => write!(f, "Fire"),
            Self::Force => write!(f, "Force"),
            Self::Light => write!(f, "Light"),
            Self::Negative => write!(f, "Negative"),
            Self::Poison => write!(f, "Poison"),
            Self::Sonic => write!(f, "Sonic"),
            Self::Alignment(alignment) => alignment.fmt(f),
        }
    }
}
