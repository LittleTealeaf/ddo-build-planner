//! Handles any Flag that the character has.
//!
//! Most of the time, the flag is either a `1` (Has) or `0` (Not Have).
mod main_hand;
mod off_hand;

pub use main_hand::*;
pub use off_hand::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute},
    bonus::Bonus,
    types::{alignment::Alignment, immunity::Immunity, race::Race},
};

use super::{item::ArmorType, toggle::Toggle};

/// Indicates that the character possesses some flag.
///
/// Flags are most often used for indirect effects, such as "does the character have this toggle", or other traits.
#[derive(Copy, Hash, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<Bonus>> {
        match self {
            Self::Race(race) => race.get_bonuses(value),
            _ => None,
        }
    }
}

impl ToAttribute for Flag {
    fn to_attribute(self) -> Attribute {
        Attribute::Flag(self)
    }
}

/// Indicates that this object is a flag
pub trait ToFlag {
    /// Converts this object to a flag
    fn to_flag(self) -> Flag;
}

impl<T> From<T> for Flag
where
    T: ToFlag,
{
    fn from(value: T) -> Self {
        value.to_flag()
    }
}
