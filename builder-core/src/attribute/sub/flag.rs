use crate::{attribute::{GetCloned}, bonus::Bonus, simple_enum};

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

impl Flag {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Toggle(toggle) => toggle.get_toggled_bonuses(value),
            _ => None,
        }
    }
}

impl GetCloned<Flag> for Flag {
    fn get_cloned(&self) -> Option<Vec<Flag>> {
        match self {
            Flag::AbilityToAttack(ability, WeaponHand::Both) => Some(vec![
                Flag::AbilityToAttack(*ability, WeaponHand::Main),
                Flag::AbilityToAttack(*ability, WeaponHand::Off),
            ]),
            Flag::AbilityToDamage(ability, WeaponHand::Both) => Some(vec![
                Flag::AbilityToDamage(*ability, WeaponHand::Main),
                Flag::AbilityToDamage(*ability, WeaponHand::Off),
            ]),
            _ => None,
        }
    }
}
