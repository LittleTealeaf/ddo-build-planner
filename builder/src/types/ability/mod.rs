//! Ability types

public_modules!(bonuses);

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::public_modules;

/// The different abilities that a character has
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Ability {
    /// All values
    All,
    /// Represents how strong the character is
    #[serde(rename = "Str")]
    Strength,
    /// Represents how flexible the character is
    #[serde(rename = "Dex")]
    Dexterity,
    /// Determines the character's health
    #[serde(rename = "Con")]
    Constitution,
    /// Represents how smart the character is
    #[serde(rename = "Int")]
    Intelligence,
    /// Represents how wise the character is.
    #[serde(rename = "Wis")]
    Wisdom,
    /// Represents how charismatic the character is.
    #[serde(rename = "Cha")]
    Charisma,
}

impl Ability {
    /// All abilities in the game.
    ///
    /// Does not include [`All`]
    ///
    /// [`All`]: Ability::All
    pub const ABILITIES: [Self; 6] = [
        Self::Strength,
        Self::Dexterity,
        Self::Constitution,
        Self::Intelligence,
        Self::Wisdom,
        Self::Charisma,
    ];
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Strength => write!(f, "Strength"),
            Self::Dexterity => write!(f, "Dexterity"),
            Self::Constitution => write!(f, "Constitution"),
            Self::Intelligence => write!(f, "Intelligence"),
            Self::Wisdom => write!(f, "Wisdom"),
            Self::Charisma => write!(f, "Charisma"),
            Self::All => write!(f, "All"),
        }
    }
}
