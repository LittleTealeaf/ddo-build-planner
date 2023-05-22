use crate::{
    attribute::{Attribute, GetBonuses, GetCloned},
    bonus::Bonus,
    simple_enum,
};

use super::{Ability, SavingThrow, Toggle, WeaponHand};

simple_enum!(
    Flag, "", (
        Centered() String::from("Centered"),
        Toggle(toggle: Toggle) format!("Toggled: {}", toggle.to_string()),
        AbilityToSavingThrow(ability: Ability, savingthrow: SavingThrow) format!("{} to {} saving throw", ability.to_string(), savingthrow.to_string()),
        AbilityToAttack(ability: Ability, hand: WeaponHand) format!("{} to {} Attack", ability.to_string(), hand.to_string()),
        AbilityToDamage(ability: Ability, hand: WeaponHand) format!("{} to {} Damage", ability.to_string(), hand.to_string()),
        ReligiousLoreToQualityMagicalSheltering() String::from("Religious Lore to Quality Magical Sheltering"),
        ReligiousLoreToQualityPhysicalSheltering() String::from("Religious Lore to Quality Physical Sheltering"),
        TrueSeeing() String::from("True Seeing")
    )
);

impl GetBonuses for Flag {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Toggle(toggle) => toggle.get_toggled_bonuses(value),
            _ => None,
        }
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

impl From<Flag> for Attribute {
    #[inline(always)]
    fn from(value: Flag) -> Attribute {
        Attribute::Flag(value)
    }
}
