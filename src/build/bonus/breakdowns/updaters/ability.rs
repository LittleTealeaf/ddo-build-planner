use crate::build::{
    attribute::{ability::Ability, skill::Skill, Attribute},
    bonus::{source::Source, types::BonusType, Bonus},
};

use crate::mod_bonus;

pub fn get_ability_updates(ability: Ability, value: f32) -> Vec<Bonus> {
    vec![Bonus::new(
        Attribute::AbilityModifier(ability),
        BonusType::AbilityScore,
        (value - 10f32) / 2f32,
        Source::Attribute(Attribute::Ability(ability)),
        None,
    )]
}

pub fn get_ability_modifier_updates(ability: Ability, value: f32) -> Vec<Bonus> {
    match ability {
        Ability::Strength => get_strength_modifier_bonuses(value),
        Ability::Dexterity => get_dexterity_modifier_bonuses(value),
        Ability::Constitution => get_constitution_modifier_bonuses(value),
        Ability::Intelligence => get_intelligence_modifier_bonuses(value),
        Ability::Wisdom => get_wisdom_modifier_bonuses(value),
        Ability::Charisma => get_charisma_modifier_bonuses(value),
    }
}

fn get_strength_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![]
}

fn get_dexterity_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![]
}

fn get_constitution_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![]
}

fn get_intelligence_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![]
}

fn get_wisdom_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![
        mod_bonus!(Ability::Wisdom, Attribute::Skill(Skill::Heal), value),
        mod_bonus!(Ability::Wisdom, Attribute::Skill(Skill::Listen), value),
        mod_bonus!(Ability::Wisdom, Attribute::Skill(Skill::Spot), value),
    ]
}

fn get_charisma_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![]
}

#[macro_use]
mod macros {

    #[macro_export]
    macro_rules! mod_bonus {
        ($ability:expr, $attribute:expr, $value:expr) => {
            Bonus::new(
                $attribute,
                BonusType::AbilityModifier,
                $value,
                Source::Attribute(Attribute::AbilityModifier($ability)),
                None,
            )
        };
    }
}
