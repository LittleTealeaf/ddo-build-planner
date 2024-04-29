//! Handles any Flag that the character has.
//!
//! Most of the time, the flag is either a `1` (Has) or `0` (Not Have).
mod main_hand;
mod off_hand;

use core::fmt;

use itertools::chain;
pub use main_hand::*;
pub use off_hand::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use fmt::Display;

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute},
    bonus::BonusTemplate,
    types::{alignment::Alignment, immunity::Immunity, race::Race},
};

use super::{item_type::ArmorType, toggle::Toggle};

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
    /// Whether or not something is in the main hand
    HasMainHand,
    /// Wielding an item in the main hand
    MainHandType(MainHandType),
    /// Whether or not something is in the off hand
    HasOffHand,
    /// Item type in the off hand
    OffHandType(OffHandType),
    /// Wearing Armor Type
    ArmorType(ArmorType),
    /// Whether the user is using a two handed fighting weapon
    IsTwoHandedFighting,
}

impl Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HasToggle(toggle) => write!(f, "Has {toggle} Toggle"),
            Self::Race(race) => write!(f, "{race} Race"),
            Self::Immunity(immunity) => write!(f, "{immunity} Immunity"),
            Self::Alignment(alignment) => write!(f, "Is {alignment}"),
            Self::OffHandType(off_hand) => write!(f, "{off_hand} in the off hand"),
            Self::MainHandType(main_hand) => write!(f, "{main_hand} in the main hand"),
            Self::ArmorType(armor) => write!(f, "Wearing {armor} Armor"),
            Self::HasMainHand => write!(f, "Item in Main Hand"),
            Self::HasOffHand => write!(f, "Item in Off Hand"),
            Self::IsTwoHandedFighting => write!(f, "Is Two Handed Fighting"),
        }
    }
}

impl GetBonuses for Flag {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        match self {
            Self::Race(race) => race.get_bonuses(value),
            Self::MainHandType(_) => {
                (value >= Decimal::ZERO).then(|| vec![BonusTemplate::flag(Self::HasMainHand, None)])
            }
            Self::OffHandType(_) => {
                (value >= Decimal::ZERO).then(|| vec![BonusTemplate::flag(Self::HasOffHand, None)])
            }
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

impl StaticOptions for Flag {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            Toggle::get_static().map(Self::HasToggle),
            Race::get_static().map(Self::Race),
            Immunity::get_static().map(Self::Immunity),
            Alignment::get_static().map(Self::Alignment),
            OffHandType::get_static().map(Self::OffHandType),
            MainHandType::get_static().map(Self::MainHandType),
            ArmorType::get_static().map(Self::ArmorType),
            [
                Self::IsTwoHandedFighting,
                Self::HasOffHand,
                Self::HasMainHand
            ]
        )
    }
}
