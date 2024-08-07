//! Spell Selector
use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::{chain_tree, enums::StaticValues};

use crate::types::spell_school::SpellSchool;

use super::{damage_type::DamageType, player_class::PlayerClass, spell_power::SpellPower};

/// Specifies the spell, type of spell, spell class.
///
/// This allows for bonuses such as spell DCs or caster levels to be as specific or generic as they need to be.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpellSelector {
    /// Spells that fall under a specific spell power
    #[serde(rename = "p", alias = "SpellPower")]
    SpellPower(SpellPower),
    /// Spells of a certain school
    #[serde(rename = "s", alias = "School")]
    School(SpellSchool),
    /// Spells for a certain class
    #[serde(rename = "c", alias = "Class")]
    Class(PlayerClass),
    /// All spells
    #[serde(rename = "a", alias = "All")]
    All,
}

impl Display for SpellSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SpellPower(sp) => sp.fmt(f),
            Self::School(school) => school.fmt(f),
            Self::Class(cl) => cl.fmt(f),
            Self::All => write!(f, "All Spells"),
        }
    }
}

impl From<SpellPower> for SpellSelector {
    fn from(value: SpellPower) -> Self {
        Self::SpellPower(value)
    }
}

impl From<DamageType> for SpellSelector {
    fn from(value: DamageType) -> Self {
        SpellPower::from(value).into()
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

impl StaticValues for SpellSelector {
    fn values() -> impl Iterator<Item = Self> {
        chain_tree!(
            [Self::All],
            SpellPower::values().map(Self::SpellPower),
            SpellSchool::values().map(Self::School),
            PlayerClass::values().map(Self::Class),
        )
    }
}
