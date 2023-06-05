use std::fmt::Display;

use enum_map::Enum;

use crate::item::types::{WeaponType, ShieldType};


#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum OffHandType {
    Weapon(WeaponType),
    Shield(ShieldType),
    RuneArm
}

impl Display for OffHandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OffHandType::Weapon(weapon) => weapon.fmt(f),
            OffHandType::Shield(shield) => shield.fmt(f),
            OffHandType::RuneArm => write!(f, "Rune Arm"),
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
