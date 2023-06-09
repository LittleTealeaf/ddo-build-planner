//! Any attribute that requires the user to interact / configure

mod attacking_target;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::bonus::Bonus;

pub use attacking_target::*;

use super::GetBonuses;

/// Toggles are interactable elements that the user is able to interact with to modify the "current state" of the character.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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

impl GetBonuses for Toggle {
    fn get_bonuses(&self, _value: f32) -> Option<Vec<Bonus>> {
        None
    }
}
