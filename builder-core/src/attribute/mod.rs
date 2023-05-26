//! Represents each attribute that a character can have
mod traits;
pub mod types;
pub mod toggles;

pub use traits::*;

use crate::bonus::Bonus;
use enum_map::Enum;
use std::fmt::Display;
use types::*;

use self::toggles::Toggle;

#[derive(Copy, Clone, Enum, Eq, PartialEq, Debug)]
pub enum Attribute {
    Dummy,
    Toggle(Toggle),
    Ability(Ability),
    AbilityModifier(Ability),
    Skill(Skill),
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Dummy => write!(f, "Dummy"),
            Attribute::Ability(ability) => write!(f, "{} Score", ability),
            Attribute::AbilityModifier(ability) => write!(f, "{} Modifier", ability),
            Attribute::Skill(skill) => skill.fmt(f),
            Attribute::Toggle(toggle) => toggle.fmt(f),
        }
    }
}

impl Attribute {
    pub fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Attribute::AbilityModifier(ability) => Some(ability.get_modifier_bonuses(value)),
            Attribute::Ability(ability) => Some(ability.get_score_bonuses(value)),
            Attribute::Skill(skill) => skill.get_bonuses(value),
            Attribute::Toggle(toggle) => toggle.get_bonuses(value),
            _ => None,
        }
    }
}
