//! Immunities
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::attribute::ToAttribute;

use super::flag::{Flag, ToFlag};

/// Indicates that the character is immune to certain things
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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

impl ToAttribute for Immunity {
    fn to_attribute(self) -> crate::attribute::Attribute {
        self.to_flag().to_attribute()
    }
}

impl ToFlag for Immunity {
    fn to_flag(self) -> Flag {
        Flag::Immunity(self)
    }
}
