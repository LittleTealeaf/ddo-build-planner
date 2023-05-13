use crate::{bonus::Bonus, simple_enum};

use super::{Ability, SavingThrow, Skill, Toggle, WeaponHand};

#[derive(Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub enum Flag {
    AbilityToDamage(WeaponHand, Ability),
    AbilityToAttack(WeaponHand, Ability),
    AbilityToSavingThrow(Ability, SavingThrow),
    Centered,
    SkillUse(Skill),
    Toggle(Toggle),
    Simple(SimpleFlag),
}

impl Flag {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Toggle(toggle) => toggle.get_toggled_attribute_bonuses(value),
            _ => None,
        }
    }
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
            Flag::SkillUse(skill) => format!("Skill Use: {}", skill.to_string()),
            Flag::Toggle(toggle) => format!("{} Toggle", toggle.to_string()),
            Flag::Simple(simple) => simple.to_string(),
        }
    }
}

simple_enum!(SimpleFlag, (Centered "Centered", VulkoorCunningProc "On Vorpal: Apply Vulkoorim Constitution Poison"));
