use crate::item::types::WeaponType;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

/// Indicates the category that a weapon belongs to
#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Enum)]
pub enum WeaponCategory {
    /// Weapons that deal Bludgeoning damage
    Bludgeoning,
    /// Weapons that deal Piercing Damage
    Piercing,
    /// Ranged Weapons
    Ranged,
    /// Weapons that deal Slashing damage
    Slashing,
    /// Thrown Weapons
    Thrown,
}

impl ToString for WeaponCategory {
    fn to_string(&self) -> String {
        match self {
            WeaponCategory::Bludgeoning => String::from("Bludgeoning"),
            WeaponCategory::Piercing => String::from("Piercing"),
            WeaponCategory::Ranged => String::from("Ranged"),
            WeaponCategory::Slashing => String::from("Slashing"),
            WeaponCategory::Thrown => String::from("Thrown"),
        }
    }
}
