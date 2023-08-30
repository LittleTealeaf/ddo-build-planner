use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Indicates that the character is immune to certain things
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Immunity {
    /// Immunity to Sleep
    Sleep,
    /// Immunity to Fear
    Fear,
    /// Immuntiy to most forms of knockdown
    Knockdown,
}

impl Display for Immunity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sleep => write!(f, "Sleep"),
            Self::Fear => write!(f, "Fear"),
            Self::Knockdown => write!(f, "Knockdown"),
        }
    }
}
