mod proficiency_class;

pub use proficiency_class::*;

use std::fmt::Display;

use enum_map::Enum;

use crate::{item::types::WeaponType, attribute::Attribute};

use super::Feat;

/// Proficiencies for Weapons and Armor
#[derive(Clone, Copy, PartialEq, Eq, Debug, Enum)]
pub enum Proficiency {
    /// Proficiency for each weapon.
    WeaponProficiency(WeaponType),
    /// Proficiency for Simple Weapons
    SimpleWeaponProficiency,
    /// Proficiency for Martial Weapons
    MartialWeaponProficiency,
}

impl Display for Proficiency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Proficiency::WeaponProficiency(weapon) => write!(f, "{} Proficiency", weapon),
            Proficiency::SimpleWeaponProficiency => write!(f, "Simple Weapon Proficiency"),
            Proficiency::MartialWeaponProficiency => write!(f, "Martial Weapon Proficiency"),
        }
    }
}

impl From<WeaponType> for Proficiency {
    fn from(value: WeaponType) -> Self {
        Self::WeaponProficiency(value)
    }
}


impl From<Proficiency> for Attribute {
    fn from(value: Proficiency) -> Self {
        Feat::from(value).into()
    }
}
