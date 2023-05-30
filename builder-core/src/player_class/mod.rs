use std::fmt::Display;

use enum_map::Enum;

#[derive(Copy, Clone, PartialEq, Eq, Enum, Debug)]
pub enum PlayerClass {
    Alchemist,
    Artificer,
    Barbarian,
    Fighter,
    Monk,
    Rogue,
    Sorcerer,
    Wizard,
    FavoredSoul,
    Bard,
    Stormsinger,
    Cleric,
    DarkApostate,
    Druid,
    BlightCaster,
    Paladin,
    SacredFist,
    Ranger,
    DarkHunter,
    Warlock,
    AcolyteOfTheSkin,
}

impl Display for PlayerClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alchemist => write!(f, "Alchemist"),
            Self::Artificer => write!(f, "Artificer"),
            Self::Barbarian => write!(f, "Barbarian"),
            Self::Fighter => write!(f, "Fighter"),
            Self::Monk => write!(f, "Monk"),
            Self::Rogue => write!(f, "Rogue"),
            Self::Sorcerer => write!(f, "Sorcerer"),
            Self::Wizard => write!(f, "Wizard"),
            Self::FavoredSoul => write!(f, "Favored Soul"),
            Self::Bard => write!(f, "Bard"),
            Self::Stormsinger => write!(f, "Stormsinger"),
            Self::Cleric => write!(f, "Cleric"),
            Self::DarkApostate => write!(f, "Dark Apostate"),
            Self::Paladin => write!(f, "Paladin"),
            Self::SacredFist => write!(f, "Sacred Fist"),
            Self::Ranger => write!(f, "Ranger"),
            Self::DarkHunter => write!(f, "Dark Hunter"),
            Self::Warlock => write!(f, "Warlock"),
            Self::AcolyteOfTheSkin => write!(f, "Acolyte Of The Skin"),
            Self::Druid => write!(f, "Druid"),
            Self::BlightCaster => write!(f, "Blight Caster"),
        }
    }
}
