use crate::attribute::sub::{Ability, Flag, SavingThrow, WeaponHand};
use crate::attribute::GetCloned;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

/// Flags that indicate that an ability can be added to something else
#[derive(Copy, Clone, PartialEq, Eq, Hash, Enum, Serialize, Deserialize, Debug)]
pub enum AbilityFlag {
    /// An Ability can be used for a Saving Throw
    AbilityToSavingThrow(Ability, SavingThrow),
    /// An ability can be used to attack for a given hand
    AbilityToAttack(Ability, WeaponHand),
    /// An ability can be used to damage for a given hand
    AbilityToDamage(Ability, WeaponHand),
}

impl ToString for AbilityFlag {
    fn to_string(&self) -> String {
        match self {
            Self::AbilityToSavingThrow(ability, saving_throw) => format!(
                "{} to {} saving throw",
                ability.to_string(),
                saving_throw.to_string()
            ),
            Self::AbilityToAttack(ability, hand) => {
                format!("{} to {} Attack", ability.to_string(), hand.to_string())
            }
            Self::AbilityToDamage(ability, hand) => {
                format!("{} to {} Damage", ability.to_string(), hand.to_string())
            }
        }
    }
}

impl GetCloned<Flag> for AbilityFlag {
    fn get_cloned(&self) -> Option<Vec<Flag>> {
        match self {
            Self::AbilityToAttack(Ability::All, hand) => Some(
                Ability::VALUES
                    .map(|ability| Self::AbilityToAttack(ability, *hand).into())
                    .to_vec(),
            ),
            Self::AbilityToAttack(ability, WeaponHand::Both) => Some(
                WeaponHand::VALUES
                    .map(|hand| Self::AbilityToAttack(*ability, hand).into())
                    .to_vec(),
            ),
            Self::AbilityToDamage(Ability::All, hand) => Some(
                Ability::VALUES
                    .map(|ability| Self::AbilityToDamage(ability, *hand).into())
                    .to_vec(),
            ),
            Self::AbilityToDamage(ability, WeaponHand::Both) => Some(
                WeaponHand::VALUES
                    .map(|hand| Self::AbilityToDamage(*ability, hand).into())
                    .to_vec(),
            ),
            _ => None
        }
    }
}
