//! Playable Alignments
use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

/// Determines alignment. To create a complete alignment, two of these attributes are required.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Alignment {
    /// Good
    #[serde(rename = "g", alias = "Good")]
    Good,
    /// Evil
    #[serde(rename = "e", alias = "Evil")]
    Evil,
    /// Neutral
    #[serde(rename = "n", alias = "Neutral")]
    Neutral,
    /// Lawful
    #[serde(rename = "l", alias = "Lawful")]
    Lawful,
    /// Chaotic
    #[serde(rename = "c", alias = "Chaotic")]
    Chaotic,
}

impl Alignment {
    /// All possible values
    pub const VALUES: [Self; 5] = [
        Self::Good,
        Self::Evil,
        Self::Neutral,
        Self::Lawful,
        Self::Chaotic,
    ];
}

impl Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Good => write!(f, "Good"),
            Self::Evil => write!(f, "Evil"),
            Self::Neutral => write!(f, "Neutral"),
            Self::Lawful => write!(f, "Lawful"),
            Self::Chaotic => write!(f, "Chaotic"),
        }
    }
}

impl StaticValues for Alignment {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
