use std::fmt::Display;

use enum_map::Enum;

use crate::{race::Race, bonus::Bonus};

use super::{toggles::Toggle, Attribute, types::{Immunity, Alignment}, GetBonuses};

/// Indicates that the character possesses some flag.
///
/// Flags are most often used for indirect effects, such as "does the character have this toggle", or other traits.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
pub enum Flag {
    /// Indicates that the user has access to a given toggle.
    HasToggle(Toggle),
    /// Indicates that the user is a given race
    Race(Race),
    /// Has an immunity to something
    Immunity(Immunity),
    /// The alignment that the character is
    Alignment(Alignment),
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flag::HasToggle(toggle) => write!(f, "Has {} Toggle", toggle),
            Flag::Race(race) => write!(f, "{} Race", race),
            Flag::Immunity(immunity) => write!(f, "{} Immunity", immunity),
            Flag::Alignment(alignment) => write!(f, "Is {}", alignment),
        }
    }
}

impl GetBonuses for Flag {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Race(race) => race.get_bonuses(value),
            _ => None
        }
    }
}

impl From<Toggle> for Flag {
    fn from(value: Toggle) -> Self {
        Self::HasToggle(value)
    }
}

impl From<Race> for Flag {
    fn from(value: Race) -> Self {
        Self::Race(value)
    }
}

impl From<Immunity> for Flag {
    fn from(value: Immunity) -> Self {
        Self::Immunity(value)
    }
}

impl From<Flag> for Attribute {
    fn from(value: Flag) -> Self {
        Self::Flag(value)
    }
}
