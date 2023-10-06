use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusSource, BonusType},
    types::{SpellSchool, SpellSelector}, feat::Feat,
};

use super::SpellcastingFeat;


#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Serialize, Deserialize, Debug)]
/// Feats that grant bonuses to the DCs of spells
pub enum SpellFocusFeat {
    /// Provides a +1 bonus to DCs of a given school
    SpellFocus(SpellSchool),
    /// Provides a +1 bonus to the DCs of a given school, requires the regular Spell Focus feat
    GreaterSpellFocus(SpellSchool),
}

impl GetBonuses for SpellFocusFeat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        (value > 0f32).then(|| match self {
            Self::SpellFocus(school) | Self::GreaterSpellFocus(school) => vec![Bonus::new(
                Attribute::SpellDC(SpellSelector::School(*school)),
                BonusType::Stacking,
                1f32.into(),
                BonusSource::Attribute(Attribute::Feat(Feat::Spellcasting(SpellcastingFeat::SpellFocus(*self)))),
                None,
            )],
        })
    }
}

impl Display for SpellFocusFeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpellFocus(school) => write!(f, "Spell Focus: {school}"),
            Self::GreaterSpellFocus(school) => write!(f, "Greater Spell Focus: {school}"),
        }
    }
}
