//! Each of the playable player classes and archetypes
public_modules!(bonuses);

use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::{enums::StaticOptions, public_modules};

use crate::attribute::{Attribute, ToAttribute};

/// The different Player Classes that are in the game
#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlayerClass {
    /// Alchemist Class
    #[serde(rename = "Alc", alias = "Alchemist")]
    Alchemist,
    /// Artificer Class
    #[serde(rename = "Art", alias = "Artifcer")]
    Artificer,
    /// Barbarian Class
    #[serde(rename = "Brb", alias = "Barbarian")]
    Barbarian,
    /// Fighter Class
    #[serde(rename = "Ftr", alias = "Fighter")]
    Fighter,
    /// Monk Class
    #[serde(rename = "Mnk", alias = "Monk")]
    Monk,
    /// Rogue Class
    #[serde(rename = "Rog", alias = "Rogue")]
    Rogue,
    /// Sorcerer Class
    #[serde(rename = "Sor", alias = "Sorcerer")]
    Sorcerer,
    /// Wizard Class
    #[serde(rename = "Wiz", alias = "Wizard")]
    Wizard,
    /// Favored Soul Class
    #[serde(rename = "Fvs", alias = "FavoredSoul")]
    FavoredSoul,
    /// Bard Class
    #[serde(rename = "Brd", alias = "Bard")]
    Bard,
    /// Stormsinger Class
    #[serde(rename = "Stm", alias = "Stormsinger")]
    Stormsinger,
    /// Cleric Class
    #[serde(rename = "Clr", alias = "Cleric")]
    Cleric,
    /// Dark Apostate Class
    #[serde(rename = "DarApo", alias = "DarkApostate")]
    DarkApostate,
    /// Druid Class
    #[serde(rename = "Drd", alias = "Druid")]
    Druid,
    /// Blight Caster Class
    #[serde(rename = "Bli", alias = "BlightCaster")]
    BlightCaster,
    /// Paladin Class
    #[serde(rename = "Pal", alias = "Paladin")]
    Paladin,
    /// Sacred Fist Class
    #[serde(rename = "Sac", alias = "SacredFist")]
    SacredFist,
    /// Ranger Class
    #[serde(rename = "Rgr", alias = "Ranger")]
    Ranger,
    /// Dark Hunter Class
    #[serde(rename = "DarHtr", alias = "DarkHunter")]
    DarkHunter,
    /// Warlock Class
    #[serde(rename = "War", alias = "Warlock")]
    Warlock,
    /// Acolyte Of The Skin Class
    #[serde(rename = "Aco", alias = "AcolyteOfTheSkin")]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl ToAttribute for PlayerClass {
    fn to_attribute(self) -> Attribute {
        Attribute::ClassLevel(self)
    }
}

impl StaticOptions for PlayerClass {
    fn get_static() -> impl Iterator<Item = Self> {
        Self::CLASSES.into_iter()
    }
}
