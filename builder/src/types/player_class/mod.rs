//! Each of the playable player classes and archetypes
public_modules!(bonuses);

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::public_modules;

/// The different Player Classes that are in the game
#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlayerClass {
    /// Alchemist Class
    Alchemist,
    /// Artificer Class
    Artificer,
    /// Barbarian Class
    Barbarian,
    /// Fighter Class
    Fighter,
    /// Monk Class
    Monk,
    /// Rogue Class
    Rogue,
    /// Sorcerer Class
    Sorcerer,
    /// Wizard Class
    Wizard,
    /// Favored Soul Class
    FavoredSoul,
    /// Bard Class
    Bard,
    /// Stormsinger Class
    Stormsinger,
    /// Cleric Class
    Cleric,
    /// Dark Apostate Class
    DarkApostate,
    /// Druid Class
    Druid,
    /// Blight Caster Class
    BlightCaster,
    /// Paladin Class
    Paladin,
    /// Sacred Fist Class
    SacredFist,
    /// Ranger Class
    Ranger,
    /// Dark Hunter Class
    DarkHunter,
    /// Warlock Class
    Warlock,
    /// Acolyte Of The Skin Class
    AcolyteOfTheSkin,
}

impl PlayerClass {
    /// All valid classes currently in the game
    pub const CLASSES: [Self; 21] = [
        Self::Alchemist,
        Self::Artificer,
        Self::Barbarian,
        Self::Fighter,
        Self::Monk,
        Self::Rogue,
        Self::Sorcerer,
        Self::Wizard,
        Self::FavoredSoul,
        Self::Bard,
        Self::Stormsinger,
        Self::Cleric,
        Self::DarkApostate,
        Self::Druid,
        Self::BlightCaster,
        Self::Paladin,
        Self::SacredFist,
        Self::Ranger,
        Self::DarkHunter,
        Self::Warlock,
        Self::AcolyteOfTheSkin,
    ];

    /// Returns the parent player class, if any. That is, if the class is an archetype of another
    /// class, this will return the parent class. Otherwise, this will return [`None`]
    #[must_use]
    pub const fn get_parent_class(&self) -> Option<Self> {
        match self {
            Self::AcolyteOfTheSkin => Some(Self::Warlock),
            Self::DarkHunter => Some(Self::Ranger),
            Self::SacredFist => Some(Self::Paladin),
            Self::BlightCaster => Some(Self::Druid),
            Self::Stormsinger => Some(Self::Bard),
            Self::DarkApostate => Some(Self::Cleric),
            _ => None,
        }
    }
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
