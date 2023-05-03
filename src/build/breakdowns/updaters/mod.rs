use crate::build::{attribute::Attribute, bonus::Bonus};

use self::ability::{get_ability_updates, get_ability_modifier_updates};

mod ability;


pub fn get_updates(attribute: Attribute, value: f32) -> Vec<Bonus> {
    match attribute {
        Attribute::Ability(ability) => get_ability_updates(ability, value),
        Attribute::AbilityModifier(ability) => get_ability_modifier_updates(ability, value),
        _ => Vec::new()
    }
}
