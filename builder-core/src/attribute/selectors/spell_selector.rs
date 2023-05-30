use std::fmt::Display;

use enum_map::Enum;

use crate::{attribute::types::{SpellPower, SpellSchool}, player_class::PlayerClass};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum SpellSelector {
    SpellPower(SpellPower),
    School(SpellSchool),
    Class(PlayerClass),
    All,
}

impl Display for SpellSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpellSelector::SpellPower(sp) => write!(f, "{} Spell Power", sp),
            SpellSelector::School(school) => write!(f, "{} Spell School", school),
            SpellSelector::Class(cl) => write!(f, "{} Spells", cl),
            SpellSelector::All => write!(f, "All Spells"),
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
