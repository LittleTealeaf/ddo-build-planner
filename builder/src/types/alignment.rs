//! Playable Alignbments
use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Determines alignment. To create a complete alignment, two of these attributes are required.
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Alignment {
    /// Good
    Good,
    /// Evil
    Evil,
    /// Neutral
    Neutral,
    /// Lawful
    Lawful,
    /// Chaotic
    Chaotic,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Good => write!(f, "Good"),
            Self::Evil => write!(f, "Evil"),
            Self::Neutral => write!(f, "Neutral"),
            Self::Lawful => write!(f, "Lawful"),
            Self::Chaotic => write!(f, "Chaotic"),
        }
    }
}
