//! Handles any Flag that the character has.
//!
//! Most of the time, the flag is either a `1` (Has) or `0` (Not Have).
mod main_hand;
mod off_hand;

pub use main_hand::*;
pub use off_hand::*;

use std::fmt::Display;

use enum_map::Enum;

use crate::{bonus::Bonus, race::Race};

use super::{
    toggles::Toggle,
    types::{Alignment, Immunity},
    Attribute, GetBonuses,
};

/// Indicates that the character possesses some flag.
///
/// Flags are most often used for indirect effects, such as "does the character have this toggle", or other traits.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Enum, PartialOrd, Ord)]
pub enum Flag {
    /// Indicates that the user has access to a given toggle.
    HasToggle(Toggle),
    /// Indicates that the user is a given race
    Race(Race),
    /// Has an immunity to something
    Immunity(Immunity),
    /// The alignment that the character is
    Alignment(Alignment),

    /// Wielding an item in the main hand
    MainHandType(MainHandType),

    /// Item type in the off hand
    OffHandType(OffHandType),
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flag::HasToggle(toggle) => write!(f, "Has {} Toggle", toggle),
            Flag::Race(race) => write!(f, "{} Race", race),
            Flag::Immunity(immunity) => write!(f, "{} Immunity", immunity),
            Flag::Alignment(alignment) => write!(f, "Is {}", alignment),
            Flag::OffHandType(off_hand) => write!(f, "{} in the off hand", off_hand),
            Flag::MainHandType(main_hand) => write!(f, "{} in the main hand", main_hand),
        }
    }
}

impl GetBonuses for Flag {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Race(race) => race.get_bonuses(value),
            _ => None,
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

impl From<OffHandType> for Flag {
    fn from(value: OffHandType) -> Self {
        Self::OffHandType(value)
    }
}

impl From<MainHandType> for Flag {
    fn from(value: MainHandType) -> Self {
        Self::MainHandType(value)
    }
}

impl From<Flag> for Attribute {
    fn from(value: Flag) -> Self {
        Self::Flag(value)
    }
}
