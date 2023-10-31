mod shield_class;
mod weapon_class;

use serde::{Deserialize, Serialize};
pub use shield_class::*;
pub use weapon_class::*;

use std::fmt::Display;

use crate::equipment::item::types::WeaponType;

/// Proficiencies for Weapons and Armor
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Proficiency {
    /// Proficiency for each weapon.
    WeaponProficiency(WeaponType),
    /// Proficiency for Simple Weapons
    SimpleWeaponProficiency,
    /// Proficiency for Martial Weapons
    MartialWeaponProficiency,
    /// Shield Proficiency
    Shield(ShieldProficiency),
    /// Rune Arm Proficiency
    RuneArm,
}

impl Display for Proficiency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WeaponProficiency(weapon) => write!(f, "{weapon} Proficiency"),
            Self::SimpleWeaponProficiency => write!(f, "Simple Weapon Proficiency"),
            Self::MartialWeaponProficiency => write!(f, "Martial Weapon Proficiency"),
            Self::Shield(shield) => write!(f, "{shield} Shield Proficiency"),
            Self::RuneArm => write!(f, "Rune Arm Proficiency"),
        }
    }
}

impl From<WeaponType> for Proficiency {
    fn from(value: WeaponType) -> Self {
        Self::WeaponProficiency(value)
    }
}

impl From<ShieldProficiency> for Proficiency {
    fn from(value: ShieldProficiency) -> Self {
        Self::Shield(value)
    }
}
