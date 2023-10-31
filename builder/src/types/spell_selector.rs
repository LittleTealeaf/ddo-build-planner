
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::types::SpellSchool;

use super::{SpellPower, PlayerClass};

/// Specifies the spell, type of spell, spell class.
///
/// This allows for bonuses such as spell DCs or caster levels to be as specific or generic as they need to be.
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpellSelector {
    /// Spells that fall under a specific spell power
    SpellPower(SpellPower),
    /// Spells of a certain school
    School(SpellSchool),
    /// Spells for a certain class
    Class(PlayerClass),
    /// All spells
    All,
}

impl Display for SpellSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpellPower(sp) => write!(f, "{sp} Spell Power"),
            Self::School(school) => write!(f, "{school} Spell School"),
            Self::Class(cl) => write!(f, "{cl} Spells"),
            Self::All => write!(f, "All Spells"),
        }
    }
}

impl From<SpellPower> for SpellSelector {
    fn from(value: SpellPower) -> Self {
        Self::SpellPower(value)
    }
}

impl From<SpellSchool> for SpellSelector {
    fn from(value: SpellSchool) -> Self {
        Self::School(value)
    }
}

impl From<PlayerClass> for SpellSelector {
    fn from(value: PlayerClass) -> Self {
        Self::Class(value)
    }
}
