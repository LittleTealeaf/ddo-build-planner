use std::fmt::Display;

use serde::{Deserialize, Serialize};


/// Different skills that the character can have.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Skill {
    /// All Skills
    All,
    /// Balance Skill
    Balance,
    /// Bluff Skill
    Bluff,
    /// Concentration Skill
    Concentration,
    /// Diplomacy Skill
    Diplomacy,
    /// Disable Device Skill
    DisableDevice,
    /// Haggle Skill
    Haggle,
    /// Heal Skill
    Heal,
    /// Hide Skill
    Hide,
    /// Intimidate Skill
    Intimidate,
    /// Jump Skill
    Jump,
    /// Listen Skill
    Listen,
    /// Move Silently Skill
    MoveSilently,
    /// Open Lock Skill
    OpenLock,
    /// Perform Skill
    Perform,
    /// Repair Skill
    Repair,
    /// Search Skill
    Search,
    /// Spellcraft Skill
    Spellcraft,
    /// Spot Skill
    Spot,
    /// Swim Skill
    Swim,
    /// Tumble Skill
    Tumble,
    /// Use Magical Device Skill
    #[serde(rename = "UMD")]
    UseMagicalDevice,
}


impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Balance => write!(f, "Balance"),
            Self::Bluff => write!(f, "Bluff"),
            Self::Concentration => write!(f, "Concentration"),
            Self::Diplomacy => write!(f, "Diplomacy"),
            Self::DisableDevice => write!(f, "Disable Device"),
            Self::Haggle => write!(f, "Haggle"),
            Self::Heal => write!(f, "Heal"),
            Self::Hide => write!(f, "Hide"),
            Self::Intimidate => write!(f, "Intimidate"),
            Self::Jump => write!(f, "Jump"),
            Self::Listen => write!(f, "Listen"),
            Self::MoveSilently => write!(f, "Move Silently"),
            Self::OpenLock => write!(f, "Open Lock"),
            Self::Perform => write!(f, "Perform"),
            Self::Repair => write!(f, "Repair"),
            Self::Search => write!(f, "Search"),
            Self::Spellcraft => write!(f, "Spellcraft"),
            Self::Spot => write!(f, "Spot"),
            Self::Swim => write!(f, "Swim"),
            Self::Tumble => write!(f, "Tumble"),
            Self::UseMagicalDevice => write!(f, "Use Magical Device"),
            Self::All => write!(f, "All Skills"),
        }
    }
}

