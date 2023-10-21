use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusSource, BonusType},
    feat::{Feat, FeatRequirement, GetFeatRequirement},
    types::{PlayerClass, SpellSchool, SpellSelector},
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
                BonusSource::Attribute(Attribute::Feat(Feat::Spellcasting(
                    SpellcastingFeat::SpellFocus(*self),
                ))),
                None,
            )],
        })
    }
}

impl GetFeatRequirement for SpellFocusFeat {
    fn get_feat_requirements(&self) -> Option<FeatRequirement> {
        match self {
            Self::SpellFocus(_) => Some(FeatRequirement::Any(vec![
                FeatRequirement::ClassLevel(PlayerClass::Alchemist, 1),
                FeatRequirement::ClassLevel(PlayerClass::Artificer, 1),
                FeatRequirement::ClassLevel(PlayerClass::Bard, 1),
                FeatRequirement::ClassLevel(PlayerClass::Stormsinger, 1),
                FeatRequirement::ClassLevel(PlayerClass::Cleric, 1),
                FeatRequirement::ClassLevel(PlayerClass::DarkApostate, 1),
                FeatRequirement::ClassLevel(PlayerClass::Druid, 1),
                FeatRequirement::ClassLevel(PlayerClass::BlightCaster, 1),
                FeatRequirement::ClassLevel(PlayerClass::FavoredSoul, 1),
                FeatRequirement::ClassLevel(PlayerClass::Sorcerer, 1),
                FeatRequirement::ClassLevel(PlayerClass::Wizard, 1),
                FeatRequirement::ClassLevel(PlayerClass::Warlock, 1),
                FeatRequirement::ClassLevel(PlayerClass::AcolyteOfTheSkin, 1),
                FeatRequirement::ClassLevel(PlayerClass::Paladin, 4),
                FeatRequirement::ClassLevel(PlayerClass::SacredFist, 4),
                FeatRequirement::ClassLevel(PlayerClass::Ranger, 4),
                FeatRequirement::ClassLevel(PlayerClass::DarkHunter, 4),
            ])),
            Self::GreaterSpellFocus(school) => Some(FeatRequirement::Feat(Feat::Spellcasting(
                SpellcastingFeat::SpellFocus(Self::SpellFocus(*school)),
            ))),
        }
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