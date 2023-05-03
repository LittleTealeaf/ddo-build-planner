use crate::build::{attribute::Attribute, bonus::Bonus};

use self::{ability::{get_ability_updates, get_ability_modifier_updates}, skill::get_skill_updates};

mod ability;
mod skill;


pub fn get_updates(attribute: Attribute, value: f32) -> Vec<Bonus> {
    match attribute {
        Attribute::Ability(ability) => get_ability_updates(ability, value),
        Attribute::AbilityModifier(ability) => get_ability_modifier_updates(ability, value),
        Attribute::Skill(skill) => get_skill_updates(skill, value),
        _ => Vec::new()
    }
}
