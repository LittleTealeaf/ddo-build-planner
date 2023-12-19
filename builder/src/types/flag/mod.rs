//! Handles any Flag that the character has.
//!
//! Most of the time, the flag is either a `1` (Has) or `0` (Not Have).
mod main_hand;
mod off_hand;

pub use main_hand::*;
pub use off_hand::*;
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::{
    attribute::GetBonuses,
    bonus::Bonus,
    types::{alignment::Alignment, immunity::Immunity, race::Race},
};

use super::{toggle::Toggle, item::ArmorType};

/// Indicates that the character possesses some flag.
///
/// Flags are most often used for indirect effects, such as "does the character have this toggle", or other traits.
#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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

    /// Wearing Armor Type
    ArmorType(ArmorType),
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HasToggle(toggle) => write!(f, "Has {toggle} Toggle"),
            Self::Race(race) => write!(f, "{race} Race"),
            Self::Immunity(immunity) => write!(f, "{immunity} Immunity"),
            Self::Alignment(alignment) => write!(f, "Is {alignment}"),
            Self::OffHandType(off_hand) => write!(f, "{off_hand} in the off hand"),
            Self::MainHandType(main_hand) => write!(f, "{main_hand} in the main hand"),
            Self::ArmorType(armor) => write!(f, "Wearing {armor} Armor"),
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

impl From<ArmorType> for Flag {
    fn from(value: ArmorType) -> Self {
        Self::ArmorType(value)
    }
}
