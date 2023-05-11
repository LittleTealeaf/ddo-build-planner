use super::{Ability, SavingThrow, WeaponHand};

#[derive(Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Flag {
    AbilityToDamage(WeaponHand, Ability),
    AbilityToAttack(WeaponHand, Ability),
    AbilityToSavingThrow(Ability, SavingThrow),
    Centered,
}

impl ToString for Flag {
    fn to_string(&self) -> String {
        match self {
            Flag::AbilityToAttack(weapon_hand, ability) => format!(
                "{}{} to attack",
                weapon_hand.to_string(),
                ability.to_string()
            ),
            Flag::AbilityToDamage(weapon_hand, ability) => format!(
                "{}{} to damage",
                weapon_hand.to_string(),
                ability.to_string()
            ),
            Flag::AbilityToSavingThrow(ability, saving_throw) => {
                format!("{} to {}", ability.to_string(), saving_throw.to_string())
            }
            Flag::Centered => String::from("Centered"),
        }
    }
}
