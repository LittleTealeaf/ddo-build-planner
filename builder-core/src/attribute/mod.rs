//! Represents each attribute that a character can have
mod traits;
pub mod types;

pub use traits::*;

use crate::bonus::Bonus;
use enum_map::Enum;
use std::fmt::Display;
use types::*;

#[derive(Copy, Clone, Enum, Eq, PartialEq)]
pub enum Attribute {
    Dummy,
    Ability(Ability),
    AbilityModifier(Ability),
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Dummy => write!(f, "Dummy"),
            Attribute::Ability(ability) => write!(f, "{} Score", ability),
            Attribute::AbilityModifier(ability) => write!(f, "{} Modifier", ability),
        }
    }
}

impl Attribute {
    pub fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Attribute::AbilityModifier(ability) => Some(ability.get_modifier_bonuses(value)),
            Attribute::Ability(ability) => Some(ability.get_score_bonuses(value)),
            _ => None,
        }
    }
}
