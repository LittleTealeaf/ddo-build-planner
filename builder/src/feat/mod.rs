//! Feats that a character can have.

use std::fmt::Display;

use enum_map::Enum;

use crate::{race::RacialFeat, attribute::{GetBonuses, Attribute}};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum Feat {
    RacialFeat(RacialFeat),
}

impl Display for Feat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RacialFeat(feat) => feat.fmt(f),
        }
    }
}

impl GetBonuses for Feat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        match self {
            Feat::RacialFeat(feat) => feat.get_bonuses(value),
        }
    }
}

impl From<RacialFeat> for Feat {
    fn from(value: RacialFeat) -> Self {
        Self::RacialFeat(value)
    }
}

impl From<Feat> for Attribute {
    fn from(value: Feat) -> Self {
        Self::Feat(value)
    }
}
