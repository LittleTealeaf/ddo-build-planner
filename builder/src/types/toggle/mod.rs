//! Any attribute that requires the user to interact / configure

mod attacking_target;

use std::fmt::Display;

use itertools::chain;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute},
    bonus::BonusTemplate,
};

pub use attacking_target::*;

use super::flag::{Flag, ToFlag};

/// Toggles are interactable elements that the user is able to interact with to modify the "current state" of the character.
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Toggle {
    /// Is the character blocking
    Blocking,
    /// Is the character in reaper mode
    InReaper,
    /// Is the character attacking a certain target
    Attacking(AttackingTarget),
}
// TODO: Make a sub-toggle for "Attacking" (such as attacking a certain type of enemy)

impl Display for Toggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blocking => write!(f, "Blocking"),
            Self::InReaper => write!(f, "In Reaper"),
            Self::Attacking(target) => write!(f, "Attacking {target} Target"),
        }
    }
}

impl GetBonuses<Self> for Toggle {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        (value > Decimal::ZERO).then(|| vec![BonusTemplate::toggle(*self, None)])
    }
}

impl ToAttribute for Toggle {
    fn to_attribute(self) -> crate::attribute::Attribute {
        Attribute::Toggle(self)
    }
}

impl ToFlag for Toggle {
    fn to_flag(self) -> Flag {
        Flag::HasToggle(self)
    }
}

/// Indicates that this object is a toggle
pub trait ToToggle {
    /// Converts this to a toggle object
    fn to_toggle(self) -> Toggle;
}

impl<T> From<T> for Toggle
where
    T: ToToggle,
{
    fn from(value: T) -> Self {
        value.to_toggle()
    }
}

impl StaticOptions for Toggle {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            [Self::Blocking, Self::InReaper],
            AttackingTarget::get_static().map(Self::Attacking)
        )
    }
}
