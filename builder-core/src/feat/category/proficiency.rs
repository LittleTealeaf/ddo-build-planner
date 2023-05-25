use crate::bonus::{Bonus, GetBonuses};
use crate::feat::{Feat, FeatTrait};
use crate::item::types::{ArmorType, ShieldType, WeaponType};
use enum_map::Enum;
use serde::{Deserialize, Serialize};

/// Indicates that the character has some proficiency
#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Enum)]
pub enum ProficiencyFeat {
    /// Proficiency with a certain weapon
    Weapon(WeaponType),
    /// Proficiency with a certain armor type
    Armor(ArmorType),
    /// Proficiency with a certain shield type, or rune arm.
    Shield(ShieldType),
}

impl GetBonuses for ProficiencyFeat {
    fn get_bonuses(&self, _: f32) -> Option<Vec<Bonus>> {
        None
    }
}

impl FeatTrait for ProficiencyFeat {
    fn get_description(&self) -> String {
        match self {
            ProficiencyFeat::Weapon(_) => todo!(),
            ProficiencyFeat::Armor(_) => todo!(),
            ProficiencyFeat::Shield(_) => todo!(),
        }
    }
}

impl ToString for ProficiencyFeat {
    fn to_string(&self) -> String {
        match self {
            ProficiencyFeat::Weapon(weapon) => format!("{} Weapon Proficiency", weapon.to_string()),
            ProficiencyFeat::Armor(armor) => format!("{} Armor Proficiency", armor.to_string()),
            ProficiencyFeat::Shield(ShieldType::RuneArm) => String::from("Rune Arm Proficiency"),
            ProficiencyFeat::Shield(shield) => format!("{} Shield Proficiency", shield.to_string()),
        }
    }
}

impl From<WeaponType> for ProficiencyFeat {
    fn from(value: WeaponType) -> Self {
        Self::Weapon(value)
    }
}

impl From<ArmorType> for ProficiencyFeat {
    fn from(value: ArmorType) -> Self {
        Self::Armor(value)
    }
}

impl From<ShieldType> for ProficiencyFeat {
    fn from(value: ShieldType) -> Self {
        Self::Shield(value)
    }
}

impl From<ProficiencyFeat> for Feat {
    fn from(value: ProficiencyFeat) -> Self {
        Feat::Proficiency(value)
    }
}
