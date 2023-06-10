use std::fmt::Display;

use enum_map::Enum;

use crate::item::types::WeaponType;

/// Represents the different options that the character can have in their main hand.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, PartialOrd, Ord)]
pub enum MainHandType {
    /// The character is wielding a wand in their main hand.
    Wand,
    /// The character is wielding some weapon in their main hand.
    Weapon(WeaponType),
}

impl Display for MainHandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainHandType::Wand => write!(f, "Wand"),
            MainHandType::Weapon(weapon) => weapon.fmt(f),
        }
    }
}

impl From<WeaponType> for MainHandType {
    fn from(value: WeaponType) -> Self {
        Self::Weapon(value)
    }
}
