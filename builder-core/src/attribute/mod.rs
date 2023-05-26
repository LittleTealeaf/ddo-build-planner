//! Represents each attribute that a character can have
pub mod flags;
pub mod toggles;
mod traits;
pub mod types;

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
    SpellPower(SpellPower),
    SpellCriticalChance(SpellPower),
    SpellCriticalDamage(SpellPower),
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Dummy => write!(f, "Dummy"),
            Attribute::Ability(ability) => write!(f, "{} Score", ability),
            Attribute::AbilityModifier(ability) => write!(f, "{} Modifier", ability),
            Attribute::Skill(skill) => skill.fmt(f),
            Attribute::Toggle(toggle) => toggle.fmt(f),
            Attribute::SpellPower(sp) => write!(f, "{} Spell Power", sp),
            Attribute::SpellCriticalChance(sp) => write!(f, "{} Spell Critical Chance", sp),
            Attribute::SpellCriticalDamage(sp) => write!(f, "{} Spell Critical Damage", sp),
        }
    }
}

impl Attribute {
    pub fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Attribute::AbilityModifier(ability) => {
                GetBonuses::<_AbilityModifier>::get_bonuses(ability, value)
            }
            Attribute::Ability(ability) => GetBonuses::<_AbilityScore>::get_bonuses(ability, value),
            Attribute::Skill(skill) => skill.get_bonuses(value),
            Attribute::Toggle(toggle) => toggle.get_bonuses(value),
            Attribute::SpellPower(sp) => GetBonuses::<_SpellPower>::get_bonuses(sp, value),
            Attribute::SpellCriticalChance(sp) => {
                GetBonuses::<_SpellCriticalChance>::get_bonuses(sp, value)
            }
            Attribute::SpellCriticalDamage(sp) => {
                GetBonuses::<_SpellCriticalDamage>::get_bonuses(sp, value)
            }
            _ => None,
        }
    }
}
