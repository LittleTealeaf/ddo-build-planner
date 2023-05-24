use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses, GetCloned},
    bonus::Bonus,
};

use super::{Ability, Immunity, SavingThrow, Toggle, WeaponHand};

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
    /// This flag is useless if the user already has that ability to that saving throw.
    AbilityToSavingThrow(Ability, SavingThrow),
    /// If the user has some ability to attack for a given hand.
    AbilityToAttack(Ability, WeaponHand),
    /// If the user has some ability to damage for a given hand
    AbilityToDamage(Ability, WeaponHand),
    /// Provides bonuses to magical sheltering equal to their religious lore
    ReligiousLoreToQualityMagicalSheltering,
    /// Provides bonuses to physical sheltering equal to their religious lore
    ReligiousLoreToQualityPhysicalSheltering,
    /// If the user has true seeing.
    TrueSeeing,
    /// If the user is immune to something
    Immunity(Immunity),
}

impl ToString for Flag {
    fn to_string(&self) -> String {
        match self {
            Flag::Centered => String::from("Centered"),
            Flag::Toggle(toggle) => format!("Toggled: {}", toggle.to_string()),
            Flag::AbilityToSavingThrow(ability, savingthrow) => format!(
                "{} to {} saving throw",
                ability.to_string(),
                savingthrow.to_string()
            ),
            Flag::AbilityToAttack(ability, hand) => {
                format!("{} to {} Attack", ability.to_string(), hand.to_string())
            }
            Flag::AbilityToDamage(ability, hand) => {
                format!("{} to {} Damage", ability.to_string(), hand.to_string())
            }
            Flag::ReligiousLoreToQualityMagicalSheltering => {
                String::from("Religious Lore to Quality Magical Sheltering")
            }
            Flag::ReligiousLoreToQualityPhysicalSheltering => {
                String::from("Religious Lore to Quality Physical Sheltering")
            }
            Flag::TrueSeeing => String::from("True Seeing"),
            Flag::Immunity(immunity) => format!("Immunity to {}", immunity.to_string()),
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
            Flag::AbilityToAttack(Ability::All, hand) => Some(
                Ability::VALUES
                    .map(|ability| Flag::AbilityToAttack(ability, *hand))
                    .to_vec(),
            ),
            Flag::AbilityToAttack(ability, WeaponHand::Both) => Some(
                WeaponHand::VALUES
                    .map(|hand| Flag::AbilityToAttack(*ability, hand))
                    .to_vec(),
            ),
            Flag::AbilityToDamage(Ability::All, hand) => Some(
                Ability::VALUES
                    .map(|ability| Flag::AbilityToDamage(ability, *hand))
                    .to_vec(),
            ),
            Flag::AbilityToDamage(ability, WeaponHand::Both) => Some(
                WeaponHand::VALUES
                    .map(|hand| Flag::AbilityToDamage(*ability, hand))
                    .to_vec(),
            ),
            _ => None,
        }
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

impl From<Flag> for Attribute {
    #[inline(always)]
    fn from(value: Flag) -> Attribute {
        Attribute::Flag(value)
    }
}
