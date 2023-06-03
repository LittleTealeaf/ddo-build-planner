//! Feats that a character can have.
mod proficiency;
pub use proficiency::*;

use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{Attribute, GetBonuses},
    race::RacialFeat,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum Feat {
    RacialFeat(RacialFeat),
    Proficiency(Proficiency),
}

impl Display for Feat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Feat::RacialFeat(feat) => feat.fmt(f),
            Feat::Proficiency(prof) => prof.fmt(f),
        }
    }
}

impl GetBonuses for Feat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        match self {
            Feat::RacialFeat(feat) => feat.get_bonuses(value),
            _ => None,
        }
    }
}

impl From<RacialFeat> for Feat {
    fn from(value: RacialFeat) -> Self {
        Self::RacialFeat(value)
    }
}

impl From<Proficiency> for Feat {
    fn from(value: Proficiency) -> Self {
        Self::Proficiency(value)
    }
}

impl From<Feat> for Attribute {
    fn from(value: Feat) -> Self {
        Self::Feat(value)
    }
}
