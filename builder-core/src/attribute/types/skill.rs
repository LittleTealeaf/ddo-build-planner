use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusSource, BonusType},
};

use super::SpellPower;

#[derive(Clone, Copy, PartialEq, Eq, Enum, Debug)]
pub enum Skill {
    Balance,
    Bluff,
    Concentration,
    Diplomacy,
    DisableDevice,
    Haggle,
    Heal,
    Hide,
    Intimidate,
    Jump,
    Listen,
    MoveSilently,
    OpenLock,
    Perform,
    Repair,
    Search,
    Spellcraft,
    Spot,
    Swim,
    Tumble,
    UseMagicalDevice,
}

impl Skill {
    pub const ALL: [Skill; 21] = [
        Skill::Balance,
        Skill::Bluff,
        Skill::Concentration,
        Skill::Diplomacy,
        Skill::DisableDevice,
        Skill::Haggle,
        Skill::Heal,
        Skill::Hide,
        Skill::Intimidate,
        Skill::Jump,
        Skill::Listen,
        Skill::MoveSilently,
        Skill::OpenLock,
        Skill::Perform,
        Skill::Repair,
        Skill::Search,
        Skill::Spellcraft,
        Skill::Spot,
        Skill::Swim,
        Skill::Tumble,
        Skill::UseMagicalDevice,
    ];

    fn spell_power_bonus(&self, sp: SpellPower, value: f32) -> Bonus {
        Bonus::new(
            Attribute::SpellPower(sp),
            BonusType::Stacking,
            value.into(),
            Attribute::Skill(*self).into(),
            None,
        )
    }
}

impl GetBonuses for Skill {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Skill::Heal => Some(vec![
                self.spell_power_bonus(SpellPower::Positive, value),
                self.spell_power_bonus(SpellPower::Negative, value),
            ]),
            Skill::Perform => Some(vec![self.spell_power_bonus(SpellPower::Sonic, value)]),
            Skill::Spellcraft => Some(vec![
                self.spell_power_bonus(SpellPower::Acid, value),
                self.spell_power_bonus(SpellPower::Cold, value),
                self.spell_power_bonus(SpellPower::Electric, value),
                self.spell_power_bonus(SpellPower::Fire, value),
                self.spell_power_bonus(SpellPower::Force, value),
                self.spell_power_bonus(SpellPower::Light, value),
                self.spell_power_bonus(SpellPower::Poison, value),
            ]),
            _ => None,
        }
    }
}

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Skill::Balance => "Balance",
                Skill::Bluff => "Bluff",
                Skill::Concentration => "Concentration",
                Skill::Diplomacy => "Diplomacy",
                Skill::DisableDevice => "Disable Device",
                Skill::Haggle => "Haggle",
                Skill::Heal => "Heal",
                Skill::Hide => "Hide",
                Skill::Intimidate => "Intimidate",
                Skill::Jump => "Jump",
                Skill::Listen => "Listen",
                Skill::MoveSilently => "Move Silently",
                Skill::OpenLock => "Open Lock",
                Skill::Perform => "Perform",
                Skill::Repair => "Repair",
                Skill::Search => "Search",
                Skill::Spellcraft => "Spellcraft",
                Skill::Spot => "Spot",
                Skill::Swim => "Swim",
                Skill::Tumble => "Tumble",
                Skill::UseMagicalDevice => "Use Magical Device",
            }
        )
    }
}

impl From<Skill> for Attribute {
    fn from(value: Skill) -> Self {
        Attribute::Skill(value)
    }
}
