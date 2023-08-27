use serde::{Deserialize, Serialize};

/// The different abilities that a character has
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
