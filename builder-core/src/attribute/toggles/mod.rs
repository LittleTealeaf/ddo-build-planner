use std::fmt::Display;

use enum_map::Enum;

use crate::bonus::Bonus;

use super::GetBonuses;

/// Toggles are interactable elements that the user is able to interact with to modify the "current state" of the character.
#[derive(Clone, Copy, PartialEq, Eq, Enum, Debug)]
pub enum Toggle {
    /// Is the character blocking
    Blocking,
    /// Is the character in reaper mode
    InReaper,
    /// Is the character attacking a tripped target
    AttackingTrippedTarget,
}
// TODO: Make a sub-toggle for "Attacking" (such as attacking a certain type of enemy)

impl Display for Toggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Toggle::Blocking => write!(f, "Blocking"),
            Toggle::InReaper => write!(f, "In Reaper"),
            Toggle::AttackingTrippedTarget => write!(f, "Attacking Tripped Target"),
        }
    }
}

impl GetBonuses for Toggle {
    fn get_bonuses(&self, _value: f32) -> Option<Vec<Bonus>> {
        None
    }
}
