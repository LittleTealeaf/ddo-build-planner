use std::fmt::Display;

use enum_map::Enum;



#[derive(Clone, Copy, PartialEq, Eq, Debug, Enum)]
pub enum EnergyResistance {
    Acid,
    Chaos,
    Cold,
    Electric,
    Evil,
    Fire,
    Force,
    Good,
    Lawful,
    Light,
    Negative,
    Poison,
    Sonic
}

impl Display for EnergyResistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnergyResistance::Acid => write!(f, "Acid"),
            EnergyResistance::Chaos => write!(f, "Chaos"),
            EnergyResistance::Cold => write!(f, "Cold"),
            EnergyResistance::Electric => write!(f, "Electric"),
            EnergyResistance::Evil => write!(f, "Evil"),
            EnergyResistance::Fire => write!(f, "Fire"),
            EnergyResistance::Force => write!(f, "Force"),
            EnergyResistance::Good => write!(f, "Good"),
            EnergyResistance::Lawful => write!(f, "Lawful"),
            EnergyResistance::Light => write!(f, "Light"),
            EnergyResistance::Negative => write!(f, "Negative"),
            EnergyResistance::Poison => write!(f, "Poison"),
            EnergyResistance::Sonic => write!(f, "Sonic"),
        }
    }
}
