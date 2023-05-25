pub mod category;
mod traits;

pub use traits::*;

use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::{attribute::Attribute, bonus::GetBonuses};
use crate::feat::category::ProficiencyFeat;

use self::category::SkillFeat;

// TODO: Add Prerequisites

/// Represents any feat that the player can have in the game
///
/// Feats as attributes are like flags, if they are present (value greater than `0`), then the character has the feat.
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize)]
pub enum Feat {
    /// Any feat that gives bonuses to skills.
    SkillFeat(SkillFeat),
    Proficiency(ProficiencyFeat),
}

impl Feat {
    /// Takes an item that can be converted into a feat, and converts it into [`Attribute::Feat`]
    pub fn from_to_attribute<T>(value: T) -> Attribute
        where Self: From<T>
    {
        Feat::from(value).into()
    }
}

impl ToString for Feat {
    fn to_string(&self) -> String {
        match self {
            Feat::SkillFeat(feat) => feat.to_string(),
            Feat::Proficiency(prof) => prof.to_string()
        }
    }
}

impl GetBonuses for Feat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        match self {
            Feat::SkillFeat(feat) => feat.get_bonuses(value),
            Feat::Proficiency(prof) => prof.get_bonuses(value)
        }
    }
}

impl FeatTrait for Feat {
    fn get_description(&self) -> String {
        match self {
            Feat::SkillFeat(feat) => feat.get_description(),
            Feat::Proficiency(prof) => prof.get_description()
        }
    }
}

impl From<Feat> for Attribute {
    fn from(value: Feat) -> Self {
        Self::Feat(value)
    }
}
