use super::{Ability, WeaponHand};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Flag {
    AbilityToDamage(WeaponHand, Ability),
    AbilityToAttack(WeaponHand, Ability),
}

impl ToString for Flag {
    fn to_string(&self) -> String {
        String::from(match self {
            Flag::AbilityToAttack(weapon_hand, ability) => format!(
                "{} {} to attack",
                weapon_hand.to_string(),
                ability.to_string()
            ),
            Flag::AbilityToDamage(weapon_hand, ability) => format!(
                "{} {} to damage",
                weapon_hand.to_string(),
                ability.to_string()
            ),
        })
    }
}
