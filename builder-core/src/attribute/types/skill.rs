use std::fmt::Display;

use enum_map::Enum;

use crate::bonus::Bonus;

#[derive(Clone, Copy, PartialEq, Eq, Enum)]
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

    pub fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        todo!()
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
