use std::fmt::Display;

use enum_map::Enum;
use serde::{Serialize, Deserialize};

use crate::attribute::{flags::Flag, Attribute};

/// Indicates that the character is immune to certain things
#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Immunity {
    /// Immunity to Sleep
    Sleep,
    /// Immunity to Fear
    Fear,
}

impl Display for Immunity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Immunity::Sleep => write!(f, "Sleep"),
            Immunity::Fear => write!(f, "Fear"),
        }
    }
}

impl From<Immunity> for Attribute {
    fn from(value: Immunity) -> Self {
        Flag::from(value).into()
    }
}
