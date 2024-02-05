//! Feats that a character can have.
public_modules!(feats, requirements, to_feat);

use core::fmt;
use core::fmt::Debug;

use itertools::chain;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::{enums::StaticOptions, public_modules};

use fmt::Display;

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute},
    bonus::{Bonus, BonusTemplate, CloneBonus},
};

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
    /// Past Life Feat
    PastLife(PastLife),
}

impl Display for Feat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RacialFeat(feat) => write!(f, "{feat}"),
            Self::Proficiency(prof) => write!(f, "{prof}"),
            Self::SkillFocus(feat) => write!(f, "{feat}"),
            Self::Spellcasting(feat) => write!(f, "{feat}"),
            Self::PastLife(feat) => write!(f, "{feat}"),
        }
    }
}

impl CloneBonus for Feat {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        match self {
            Self::Proficiency(feat) => feat.clone_bonus(bonus),
            _ => None,
        }
    }
}

impl GetBonuses for Feat {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        match self {
            Self::RacialFeat(feat) => feat.get_bonuses(value),
            Self::Proficiency(_) => None,
            Self::SkillFocus(feat) => feat.get_bonuses(value),
            Self::Spellcasting(feat) => feat.get_bonuses(value),
            Self::PastLife(feat) => feat.get_bonuses(value),
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

impl<T> From<T> for Feat
where
    T: ToFeat,
{
    fn from(value: T) -> Self {
        value.to_feat()
    }
}

impl ToAttribute for Feat {
    fn to_attribute(self) -> Attribute {
        Attribute::Feat(self)
    }
}

impl<T> ToAttribute for T
where
    T: ToFeat,
{
    fn to_attribute(self) -> Attribute {
        self.to_feat().to_attribute()
    }
}

impl StaticOptions for Feat {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            RacialFeat::get_static().map(Self::RacialFeat),
            Proficiency::get_static().map(Self::Proficiency),
            SkillFocus::get_static().map(Self::SkillFocus),
            SpellcastingFeat::get_static().map(Self::Spellcasting)
        )
    }
}
