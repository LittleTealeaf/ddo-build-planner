use crate::item::types::WeaponType;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Enum)]
pub enum WeaponCategory {
    Bludgeoning,
    Piercing,
    Ranged,
    Slashing,
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
