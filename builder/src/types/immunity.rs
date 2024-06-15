//! Immunities
use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::attribute::{Attribute, ToAttribute};

use super::flag::{Flag, ToFlag};

/// Indicates that the character is immune to certain things
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Immunity {
    /// Immunity to Sleep
    Sleep,
    /// Immunity to Fear
    Fear,
    /// Immunity to most forms of knockdown
    Knockdown,
}

impl Immunity {
    /// All possible values
    pub const VALUES: [Self; 3] = [Self::Sleep, Self::Fear, Self::Knockdown];
}

impl Display for Immunity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sleep => write!(f, "Sleep"),
            Self::Fear => write!(f, "Fear"),
            Self::Knockdown => write!(f, "Knockdown"),
        }
    }
}

impl ToAttribute for Immunity {
    fn to_attribute(self) -> Attribute {
        self.to_flag().to_attribute()
    }
}

impl ToFlag for Immunity {
    fn to_flag(self) -> Flag {
        Flag::Immunity(self)
    }
}

impl StaticValues for Immunity {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
