use std::fmt::Display;

use enum_map::Enum;

use crate::bonus::Bonus;

use super::GetBonuses;

#[derive(Clone, Copy, PartialEq, Eq, Enum, Debug)]
pub enum Toggle {
    Blocking,
    InReaper,
    AttackingTrippedTarget,
}

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
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        None
    }
}
