//! Indicates different item types

use serde::{Deserialize, Serialize};
use utils::public_modules;

public_modules!(armor, weapon, shield);

/// The types that an item can be
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ItemType {
    /// Weapons
    #[serde(rename = "w", alias = "Weapon")]
    Weapon(WeaponType),
    /// Shields
    #[serde(rename = "s", alias = "Shield")]
    Shield(ShieldType),
    /// Armor
    #[serde(rename = "a", alias = "Armor")]
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
