//! Feats that a character can have.
public_modules!(feats, requirements);

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::public_modules;

use std::fmt::Display;

use crate::{attribute::GetBonuses, bonus::CloneBonus, race::RacialFeat};

/// All possible feats that the player can have.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Feat {
    /// Feats pertaining to a certain race.
    RacialFeat(RacialFeat),
    /// Proficiencies for Weapons or Armor
    Proficiency(Proficiency),
    /// Skill Focus
    SkillFocus(SkillFocus),
    /// Spell Focus Feats
    Spellcasting(SpellcastingFeat),
    // SpellFocus(SpellFocusFeat),
}

impl Display for Feat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RacialFeat(feat) => feat.fmt(f),
            Self::Proficiency(prof) => prof.fmt(f),
            Self::SkillFocus(feat) => feat.fmt(f),
            Self::Spellcasting(feat) => feat.fmt(f),
        }
    }
}

impl CloneBonus for Feat {
    fn clone_bonus(&self, bonus: &crate::bonus::Bonus) -> Option<Vec<crate::bonus::Bonus>> {
        match self {
            Self::Proficiency(feat) => feat.clone_bonus(bonus),
            _ => None,
        }
    }
}

impl GetBonuses for Feat {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<crate::bonus::Bonus>> {
        match self {
            Self::RacialFeat(feat) => feat.get_bonuses(value),
            Self::Proficiency(_) => None,
            Self::SkillFocus(feat) => feat.get_bonuses(value),
            Self::Spellcasting(feat) => feat.get_bonuses(value),
        }
    }
}

impl GetFeatRequirement for Feat {
    fn get_feat_requirements(&self) -> Option<FeatRequirement> {
        match self {
            Self::SkillFocus(feat) => feat.get_feat_requirements(),
            Self::Spellcasting(feat) => feat.get_feat_requirements(),
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
