mod ability_flags;
pub use ability_flags::*;

use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::item::types::{ArmorType, WeaponCategory, WeaponType};
use crate::{
    attribute::{Attribute, GetBonuses, GetCloned},
    bonus::Bonus,
};

use super::{Immunity, Toggle, WeaponHand};

/// Defines any flags that the user can have.
///
/// In short, a "flag" indicates that the user has some trait. It could be having some ability to a saving throw, like with [`Self::AbilityToSavingThrow`], or it could be as simple as wearing some item.
///
/// The main use of flags is to give some form of "attribute" that can be checked for certain traits, such as bonuses to health only if the user is wearing heavy armor.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Enum, Serialize, Deserialize, Debug)]
pub enum Flag {
    /// This is a special flag used when the user has "interacted" with a toggle. For example, if the user has the reaper toggle on.
    Toggle(Toggle),
    /// If the user is centered
    Centered,
    /// If the user has some ability to a saving throw.
    ///
    AbilityFlag(AbilityFlag),
    /// Provides bonuses to magical sheltering equal to their religious lore
    ReligiousLoreToQualityMagicalSheltering,
    /// Provides bonuses to physical sheltering equal to their religious lore
    ReligiousLoreToQualityPhysicalSheltering,
    /// If the user has true seeing.
    TrueSeeing,
    /// If the user is immune to something
    Immunity(Immunity),
    /// If the character is wearing a certain type of armor
    WearingArmor(ArmorType),
    /// Indicates if the character is wielding a weapon in a given hand
    WeaponEquipped(WeaponHand, WeaponType),
    /// Indicates if a certain weapon category is equipped
    WeaponCategoryEquipped(WeaponHand, WeaponCategory),
}

impl ToString for Flag {
    fn to_string(&self) -> String {
        match self {
            Flag::Centered => String::from("Centered"),
            Flag::Toggle(toggle) => format!("Toggled: {}", toggle.to_string()),

            Flag::ReligiousLoreToQualityMagicalSheltering => {
                String::from("Religious Lore to Quality Magical Sheltering")
            }
            Flag::ReligiousLoreToQualityPhysicalSheltering => {
                String::from("Religious Lore to Quality Physical Sheltering")
            }
            Flag::TrueSeeing => String::from("True Seeing"),
            Flag::Immunity(immunity) => format!("Immunity to {}", immunity.to_string()),
            Flag::WearingArmor(armor) => format!("Wearing {} Armor", armor.to_string()),
            Flag::WeaponEquipped(hand, weapon_type) => {
                format!("{} in {} hand", weapon_type.to_string(), hand.to_string())
            }
            Flag::WeaponCategoryEquipped(hand, weapon_category) => format!(
                "{} in {} hand",
                weapon_category.to_string(),
                hand.to_string()
            ),
            Flag::AbilityFlag(ability_flag) => ability_flag.to_string(),
        }
    }
}

impl GetBonuses for Flag {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        None
    }
}

impl GetCloned<Flag> for Flag {
    #[inline(always)]
    fn get_cloned(&self) -> Option<Vec<Flag>> {
        match self {
            Self::AbilityFlag(flag) => flag.get_cloned(),
            Flag::WeaponEquipped(hand, weapon_type) => {
                Some(vec![(*hand, WeaponCategory::from(*weapon_type)).into()])
            }
            _ => None,
        }
    }
}

impl From<AbilityFlag> for Flag {
    fn from(value: AbilityFlag) -> Self {
        Self::AbilityFlag(value)
    }
}

impl From<Immunity> for Flag {
    fn from(value: Immunity) -> Flag {
        Flag::Immunity(value)
    }
}

impl From<Toggle> for Flag {
    fn from(value: Toggle) -> Flag {
        Flag::Toggle(value)
    }
}

impl From<ArmorType> for Flag {
    fn from(value: ArmorType) -> Self {
        Self::WearingArmor(value)
    }
}

impl From<(WeaponHand, WeaponType)> for Flag {
    fn from((hand, weapon_type): (WeaponHand, WeaponType)) -> Self {
        Self::WeaponEquipped(hand, weapon_type)
    }
}

impl From<(WeaponHand, WeaponCategory)> for Flag {
    fn from((hand, weapon_category): (WeaponHand, WeaponCategory)) -> Self {
        Self::WeaponCategoryEquipped(hand, weapon_category)
    }
}

impl From<Flag> for Attribute {
    #[inline(always)]
    fn from(value: Flag) -> Attribute {
        Attribute::Flag(value)
    }
}
