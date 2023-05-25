use enum_map::Enum;
use serde::{Deserialize, Serialize};
use crate::item::types::{ArmorType, ShieldType, WeaponType};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Enum)]
pub enum Proficiency {
    Weapon(WeaponType),
    Armor(ArmorType),
    Shield(ShieldType)
}

impl ToString for Proficiency {
    fn to_string(&self) -> String {
        match self {
            Proficiency::Weapon(weapon) => format!("{} Weapon Proficiency", weapon.to_string()),
            Proficiency::Armor(armor) => format!("{} Armor Proficiency", armor.to_string()),
            Proficiency::Shield(ShieldType::RuneArm) => String::from("Rune Arm Proficiency"),
            Proficiency::Shield(shield) => format!("{} Shield Proficiency", shield.to_string()),
        }
    }
}


impl From<WeaponType> for Proficiency {
    fn from(value: WeaponType) -> Self {
        Self::Weapon(value)
    }
}

impl From<ArmorType> for Proficiency {
    fn from(value: ArmorType) -> Self {
        Self::Armor(value)
    }
}

impl From<ShieldType> for Proficiency {
    fn from(value: ShieldType) -> Self {
        Self::Shield(value)
    }
}
