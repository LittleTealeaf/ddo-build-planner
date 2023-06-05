use std::fmt::Display;

use enum_map::Enum;

use crate::item::types::WeaponType;




#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum MainHandType {
    Wand,
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
