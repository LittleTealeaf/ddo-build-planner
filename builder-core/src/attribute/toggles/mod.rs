mod attacking_target;

use std::fmt::Display;

use enum_map::Enum;

use crate::bonus::Bonus;

use super::{GetBonuses, Attribute};

pub use attacking_target::*;

/// Toggles are interactable elements that the user is able to interact with to modify the "current state" of the character.
#[derive(Clone, Copy, PartialEq, Eq, Enum, Debug)]
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
            Toggle::Blocking => write!(f, "Blocking"),
            Toggle::InReaper => write!(f, "In Reaper"),
            Toggle::Attacking(target) => write!(f, "Attacking {} Target", target),
        }
    }
}

impl GetBonuses for Toggle {
    fn get_bonuses(&self, _value: f32) -> Option<Vec<Bonus>> {
        None
    }
}

impl From<Toggle> for Attribute {
    fn from(value: Toggle) -> Self {
        Self::Toggle(value)
    }
}
