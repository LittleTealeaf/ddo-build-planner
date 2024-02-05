//! Spell Selector
use core::fmt::{self, Display};

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::types::spell_school::SpellSchool;

use super::{player_class::PlayerClass, spell_power::SpellPower};

/// Specifies the spell, type of spell, spell class.
///
/// This allows for bonuses such as spell DCs or caster levels to be as specific or generic as they need to be.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SpellPower(sp) => sp.fmt(f),
            Self::School(school) => school.fmt(f),
            Self::Class(cl) => cl.fmt(f),
            Self::All => write!(f, "Spells"),
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

impl StaticOptions for SpellSelector {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            [Self::All],
            SpellPower::get_static().map(Self::SpellPower),
            SpellSchool::get_static().map(Self::School),
            PlayerClass::get_static().map(Self::Class),
        )
    }
}
