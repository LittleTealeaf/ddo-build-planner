use super::{Ability, SavingThrow};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
    AbilityToAttack(Ability),
    AbilityToDamage(Ability),
    AbilityToSavingThrow(Ability, SavingThrow),
}

impl ToString for Flag {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::AbilityToAttack(ability) => format!("{} to Attack", ability.to_string()),
            Self::AbilityToDamage(ability) => format!("{} to Damage", ability.to_string()),
            Self::AbilityToSavingThrow(ability, saving_throw) => format!("{} to {} Saving Throw", ability.to_string(), saving_throw.to_string())
        })
    }
}
