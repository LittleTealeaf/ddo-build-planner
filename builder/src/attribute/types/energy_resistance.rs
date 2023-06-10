use std::fmt::Display;

use serde::{Serialize, Deserialize};

use super::Alignment;

/// Different types of energy resistance or absorption that the user can have
#[cfg_attr(test, derive(enum_map::Enum))]
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
            EnergyResistance::Acid => write!(f, "Acid"),
            EnergyResistance::Cold => write!(f, "Cold"),
            EnergyResistance::Electric => write!(f, "Electric"),
            EnergyResistance::Fire => write!(f, "Fire"),
            EnergyResistance::Force => write!(f, "Force"),
            EnergyResistance::Light => write!(f, "Light"),
            EnergyResistance::Negative => write!(f, "Negative"),
            EnergyResistance::Poison => write!(f, "Poison"),
            EnergyResistance::Sonic => write!(f, "Sonic"),
            EnergyResistance::Alignment(alignment) => alignment.fmt(f),
        }
    }
}
