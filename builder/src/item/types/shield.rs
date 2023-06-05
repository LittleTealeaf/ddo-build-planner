use std::fmt::Display;

use enum_map::Enum;


#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum ShieldType {
    Buckler,
    SmallShield,
    LargeShield,
    TowerShield,
    Orb,
}

impl Display for ShieldType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShieldType::Buckler => write!(f, "Buckler"),
            ShieldType::SmallShield => write!(f, "Small Shield"),
            ShieldType::LargeShield => write!(f, "Large Shield"),
            ShieldType::TowerShield => write!(f, "Tower Shield"),
            ShieldType::Orb => write!(f, "Orb"),
        }
    }
}
