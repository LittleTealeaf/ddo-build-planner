//! Feats that a character can have.
mod proficiency;
pub use proficiency::*;
use serde::{Serialize, Deserialize};

use std::fmt::Display;


use crate::{
    attribute::{Attribute, GetBonuses},
    race::RacialFeat,
};

/// All possible feats that the player can have.
#[cfg_attr(test, derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Feat {
    /// Feats pertaining to a certain race.
    RacialFeat(RacialFeat),
    /// Proficiencies for Weapons or Armor
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

#[cfg(test)]
mod tests {
    use super::*;

    use enum_map::Enum;

    #[test]
    fn zero_bonus_returns_none() {
        for feat in (0..Feat::LENGTH).map(Feat::from_usize) {
            assert!(feat.get_bonuses(0f32).is_none());
        }
    }
}
