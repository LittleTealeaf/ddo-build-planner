use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::equipment::item::types::{ShieldType, WeaponType};

/// Represents the different types of items the character can wield in their off hand
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OffHandType {
    /// The character is wielding a weapon in their off hand
    Weapon(WeaponType),
    /// The character is wielding a shield in their off-hand
    Shield(ShieldType),
    /// The character is wielding a runearm in their off-hand
    RuneArm,
}

impl Display for OffHandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Weapon(weapon) => weapon.fmt(f),
            Self::Shield(shield) => shield.fmt(f),
            Self::RuneArm => write!(f, "Rune Arm"),
        }
    }
}

impl From<WeaponType> for OffHandType {
    fn from(value: WeaponType) -> Self {
        Self::Weapon(value)
    }
}

impl From<ShieldType> for OffHandType {
    fn from(value: ShieldType) -> Self {
        Self::Shield(value)
    }
}
