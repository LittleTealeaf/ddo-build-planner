//! Each of the possile skills

use core::fmt::{self, Display};

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::{Bonus, CloneBonus},
};

use super::ability::Ability;

/// Different skills that the character can have.
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Skill {
    /// All Skills
    #[serde(rename = "a", alias = "All")]
    All,
    /// Balance Skill
    #[serde(rename = "ba", alias = "Balance")]
    Balance,
    /// Bluff Skill
    #[serde(rename = "bf", alias = "Bluff")]
    Bluff,
    /// Concentration Skill
    #[serde(rename = "cn", alias = "Concentration")]
    Concentration,
    /// Diplomacy Skill
    #[serde(rename = "dp", alias = "Diplomacy")]
    Diplomacy,
    /// Disable Device Skill
    #[serde(rename = "dd", alias = "DisableDevice")]
    DisableDevice,
    /// Haggle Skill
    #[serde(rename = "ha", alias = "Haggle")]
    Haggle,
    /// Heal Skill
    #[serde(rename = "he", alias = "Heal")]
    Heal,
    /// Hide Skill
    #[serde(rename = "hi", alias = "Hide")]
    Hide,
    /// Intimidate Skill
    #[serde(rename = "in", alias = "Intimidate")]
    Intimidate,
    /// Jump Skill
    #[serde(rename = "ju", alias = "Jump")]
    Jump,
    /// Listen Skill
    #[serde(rename = "li", alias = "Listen")]
    Listen,
    /// Move Silently Skill
    #[serde(rename = "ms", alias = "MoveSilently")]
    MoveSilently,
    /// Open Lock Skill
    #[serde(rename = "ol", alias = "OpenLock")]
    OpenLock,
    /// Perform Skill
    #[serde(rename = "pe", alias = "Perform")]
    Perform,
    /// Repair Skill
    #[serde(rename = "re", alias = "Repair")]
    Repair,
    /// Search Skill
    #[serde(rename = "se", alias = "Search")]
    Search,
    /// Spellcraft Skill
    #[serde(rename = "sc", alias = "Spellcraft")]
    Spellcraft,
    /// Spot Skill
    #[serde(rename = "sp", alias = "Spot")]
    Spot,
    /// Swim Skill
    #[serde(rename = "sw", alias = "Swim")]
    Swim,
    /// Tumble Skill
    #[serde(rename = "tu", alias = "Tumble")]
    Tumble,
    /// Use Magical Device Skill
    #[serde(rename = "um", alias = "UMD", alias = "UseMagicalDevice")]
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
    #[must_use]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl ToAttribute for Skill {
    fn to_attribute(self) -> Attribute {
        Attribute::Skill(self)
    }
}

impl CloneBonus for Skill {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::SKILLS
                .map(|skill| bonus.clone_into_attribute(skill))
                .to_vec()
        })
    }
}

impl StaticOptions for Skill {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(Self::SKILLS, [Self::All])
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
