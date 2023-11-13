use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::Ability;

/// Different skills that the character can have.
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

impl Skill {
    /// All valid skills
    ///
    /// This does not include the [`All`] entry
    ///
    /// [`All`]: Skill::All
    pub const SKILLS: [Self; 21] = [
        Self::Balance,
        Self::Bluff,
        Self::Concentration,
        Self::Diplomacy,
        Self::DisableDevice,
        Self::Haggle,
        Self::Heal,
        Self::Hide,
        Self::Intimidate,
        Self::Jump,
        Self::Listen,
        Self::MoveSilently,
        Self::OpenLock,
        Self::Perform,
        Self::Repair,
        Self::Search,
        Self::Spellcraft,
        Self::Spot,
        Self::Swim,
        Self::Tumble,
        Self::UseMagicalDevice,
    ];

    /// Returns the ability associated with this particular skill
    pub const fn get_ability(&self) -> Option<Ability> {
        match self {
            Self::Jump | Self::Swim => Some(Ability::Strength),
            Self::Tumble | Self::Hide | Self::Balance | Self::MoveSilently | Self::OpenLock => {
                Some(Ability::Dexterity)
            }
            Self::Concentration => Some(Ability::Constitution),
            Self::DisableDevice | Self::Repair | Self::Search | Self::Spellcraft => {
                Some(Ability::Intelligence)
            }
            Self::Spot | Self::Heal | Self::Listen => Some(Ability::Wisdom),
            Self::Bluff
            | Self::Diplomacy
            | Self::Haggle
            | Self::Intimidate
            | Self::Perform
            | Self::UseMagicalDevice => Some(Ability::Charisma),
            Self::All => None,
        }
    }
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn skills_have_proper_abilities() {
        let values = [
            (Some(Ability::Dexterity), Skill::Balance),
            (Some(Ability::Charisma), Skill::Bluff),
            (Some(Ability::Constitution), Skill::Concentration),
            (Some(Ability::Charisma), Skill::Diplomacy),
            (Some(Ability::Intelligence), Skill::DisableDevice),
            (Some(Ability::Charisma), Skill::Haggle),
            (Some(Ability::Wisdom), Skill::Heal),
            (Some(Ability::Dexterity), Skill::Hide),
            (Some(Ability::Charisma), Skill::Intimidate),
            (Some(Ability::Strength), Skill::Jump),
            (Some(Ability::Wisdom), Skill::Listen),
            (Some(Ability::Dexterity), Skill::MoveSilently),
            (Some(Ability::Dexterity), Skill::OpenLock),
            (Some(Ability::Charisma), Skill::Perform),
            (Some(Ability::Intelligence), Skill::Repair),
            (Some(Ability::Intelligence), Skill::Search),
            (Some(Ability::Intelligence), Skill::Spellcraft),
            (Some(Ability::Wisdom), Skill::Spot),
            (Some(Ability::Strength), Skill::Swim),
            (Some(Ability::Dexterity), Skill::Tumble),
            (Some(Ability::Charisma), Skill::UseMagicalDevice),
            (None, Skill::All),
        ];
        for (ability, skill) in values {
            assert_eq!(
                ability,
                skill.get_ability(),
                "Invalid ability for {:?}: found {:?} expected {:?}",
                skill,
                skill.get_ability(),
                ability
            );
        }
    }
}
