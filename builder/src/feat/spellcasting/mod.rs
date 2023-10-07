use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::public_modules;

use crate::attribute::GetBonuses;

use super::GetFeatRequirement;

public_modules!(spell_focus);

#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
/// Feats thga fall under the "Spellcasting" category
pub enum SpellcastingFeat {
    /// Feats that provide bonuses to spell DCs
    SpellFocus(SpellFocusFeat),
}

impl GetBonuses for SpellcastingFeat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        match self {
            Self::SpellFocus(focus) => focus.get_bonuses(value),
        }
    }
}

impl GetFeatRequirement for SpellcastingFeat {
    fn get_feat_requirements(&self) -> Option<super::FeatRequirement> {
        match self {
            Self::SpellFocus(focus) => focus.get_feat_requirements(),
        }
    }
}

impl Display for SpellcastingFeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpellFocus(feat) => feat.fmt(f),
        }
    }
}
