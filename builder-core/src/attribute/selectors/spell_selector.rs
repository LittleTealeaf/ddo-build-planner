use std::fmt::Display;

use enum_map::Enum;

use crate::attribute::types::{SpellPower, SpellSchool};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum SpellSelector {
    SpellPower(SpellPower),
    School(SpellSchool),
}

impl Display for SpellSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpellSelector::SpellPower(sp) => write!(f, "{} Spell Power", sp),
            SpellSelector::School(school) => write!(f, "{} Spell School", school),
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
