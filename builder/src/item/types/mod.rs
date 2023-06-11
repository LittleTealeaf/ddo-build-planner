//! Different item types
mod armor;
mod shield;
mod weapon;

pub use armor::*;
pub use shield::*;
pub use weapon::*;

/// The types that an item can be
#[cfg_attr(test, derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ItemType {
    /// Weapons
    Weapon(WeaponType),
    /// Shields
    Shield(ShieldType),
    /// Armor
    Armor(ArmorType),
}

impl From<WeaponType> for ItemType {
    fn from(value: WeaponType) -> Self {
        Self::Weapon(value)
    }
}

impl From<ShieldType> for ItemType {
    fn from(value: ShieldType) -> Self {
        Self::Shield(value)
    }
}

impl From<ArmorType> for ItemType {
    fn from(value: ArmorType) -> Self {
        Self::Armor(value)
    }
}
